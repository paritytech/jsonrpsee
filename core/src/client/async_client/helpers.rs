// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::client::async_client::manager::{RequestManager, RequestStatus};
use crate::client::{RequestMessage, TransportSenderT};
use crate::params::ArrayParams;
use crate::traits::ToRpcParams;
use crate::Error;

use futures_channel::mpsc;
use futures_timer::Delay;
use futures_util::future::{self, Either};

use jsonrpsee_types::error::CallError;
use jsonrpsee_types::response::SubscriptionError;
use jsonrpsee_types::{
	ErrorObject, ErrorResponse, Id, Notification, RequestSer, Response, SubscriptionId, SubscriptionResponse,
};
use serde_json::Value as JsonValue;
use std::ops::Range;

#[derive(Debug, Clone)]
pub(crate) struct InnerBatchResponse {
	pub(crate) id: u64,
	pub(crate) result: Result<JsonValue, ErrorObject<'static>>,
}

/// Attempts to process a batch response.
///
/// On success the result is sent to the frontend.
pub(crate) fn process_batch_response(
	manager: &mut RequestManager,
	rps: Vec<InnerBatchResponse>,
	range: Range<u64>,
) -> Result<(), Error> {
	let mut responses = Vec::with_capacity(rps.len());

	let start_idx = range.start;

	let batch_state = match manager.complete_pending_batch(range.clone()) {
		Some(state) => state,
		None => {
			tracing::warn!("Received unknown batch response");
			return Err(Error::InvalidRequestId);
		}
	};

	for _ in range {
		let err_obj = ErrorObject::borrowed(0, &"", None);
		responses.push(Err(err_obj));
	}

	for rp in rps {
		let maybe_elem =
			rp.id.checked_sub(start_idx).and_then(|p| p.try_into().ok()).and_then(|p: usize| responses.get_mut(p));

		if let Some(elem) = maybe_elem {
			*elem = rp.result;
		} else {
			return Err(Error::InvalidRequestId);
		}
	}

	let _ = batch_state.send_back.send(Ok(responses));
	Ok(())
}

/// Attempts to process a subscription response.
///
/// Returns `Ok()` if the response was successfully sent to the frontend.
/// Return `Err(None)` if the subscription was not found.
/// Returns `Err(Some(msg))` if the channel to the `Subscription` was full.
pub(crate) fn process_subscription_response(
	manager: &mut RequestManager,
	response: SubscriptionResponse<JsonValue>,
) -> Result<(), Option<RequestMessage>> {
	let sub_id = response.params.subscription.into_owned();
	let request_id = match manager.get_request_id_by_subscription_id(&sub_id) {
		Some(request_id) => request_id,
		None => {
			tracing::warn!("Subscription {:?} is not active", sub_id);
			return Err(None);
		}
	};

	match manager.as_subscription_mut(&request_id) {
		Some(send_back_sink) => match send_back_sink.try_send(response.params.result) {
			Ok(()) => Ok(()),
			Err(err) => {
				tracing::error!("Dropping subscription {:?} error: {:?}", sub_id, err);
				let msg = build_unsubscribe_message(manager, request_id, sub_id)
					.expect("request ID and subscription ID valid checked above; qed");
				Err(Some(msg))
			}
		},
		None => {
			tracing::warn!("Subscription {:?} is not active", sub_id);
			Err(None)
		}
	}
}

/// Attempts to close a subscription when a [`SubscriptionError`] is received.
///
/// Returns `Ok(())` if the subscription was removed
/// Return `Err(e)` if the subscription was not found.
pub(crate) fn process_subscription_close_response(
	manager: &mut RequestManager,
	response: SubscriptionError<JsonValue>,
) -> Result<(), Error> {
	let sub_id = response.params.subscription.into_owned();
	let request_id = match manager.get_request_id_by_subscription_id(&sub_id) {
		Some(request_id) => request_id,
		None => {
			tracing::error!("The server tried to close an invalid subscription: {:?}", sub_id);
			return Err(Error::InvalidSubscriptionId);
		}
	};

	manager.remove_subscription(request_id, sub_id).expect("Both request ID and sub ID in RequestManager; qed");
	Ok(())
}

/// Attempts to process an incoming notification
///
/// Returns Ok() if the response was successfully handled
/// Returns Err() if there was no handler for the method
pub(crate) fn process_notification(manager: &mut RequestManager, notif: Notification<JsonValue>) -> Result<(), Error> {
	match manager.as_notification_handler_mut(notif.method.to_string()) {
		Some(send_back_sink) => match send_back_sink.try_send(notif.params) {
			Ok(()) => Ok(()),
			Err(err) => {
				tracing::error!("Error sending notification, dropping handler for {:?} error: {:?}", notif.method, err);
				let _ = manager.remove_notification_handler(notif.method.into_owned());
				Err(err.into_send_error().into())
			}
		},
		None => {
			tracing::error!("Notification: {:?} not a registered method", notif.method);
			Err(Error::UnregisteredNotification(notif.method.into_owned()))
		}
	}
}

/// Process a response from the server.
///
/// Returns `Ok(None)` if the response was successfully sent.
/// Returns `Ok(Some(_))` if the response got an error but could be handled.
/// Returns `Err(_)` if the response couldn't be handled.
pub(crate) fn process_single_response(
	manager: &mut RequestManager,
	response: Response<JsonValue>,
	max_capacity_per_subscription: usize,
) -> Result<Option<RequestMessage>, Error> {
	let response_id = response.id.into_owned();
	match manager.request_status(&response_id) {
		RequestStatus::PendingMethodCall => {
			let send_back_oneshot = match manager.complete_pending_call(response_id) {
				Some(Some(send)) => send,
				Some(None) => return Ok(None),
				None => return Err(Error::InvalidRequestId),
			};
			let _ = send_back_oneshot.send(Ok(response.result));
			Ok(None)
		}
		RequestStatus::PendingSubscription => {
			let (unsub_id, send_back_oneshot, unsubscribe_method) =
				manager.complete_pending_subscription(response_id.clone()).ok_or(Error::InvalidRequestId)?;

			let sub_id: Result<SubscriptionId, _> = response.result.try_into();
			let sub_id = match sub_id {
				Ok(sub_id) => sub_id,
				Err(_) => {
					let _ = send_back_oneshot.send(Err(Error::InvalidSubscriptionId));
					return Ok(None);
				}
			};

			let (subscribe_tx, subscribe_rx) = mpsc::channel(max_capacity_per_subscription);
			if manager
				.insert_subscription(response_id.clone(), unsub_id, sub_id.clone(), subscribe_tx, unsubscribe_method)
				.is_ok()
			{
				match send_back_oneshot.send(Ok((subscribe_rx, sub_id.clone()))) {
					Ok(_) => Ok(None),
					Err(_) => Ok(build_unsubscribe_message(manager, response_id, sub_id)),
				}
			} else {
				let _ = send_back_oneshot.send(Err(Error::InvalidSubscriptionId));
				Ok(None)
			}
		}
		RequestStatus::Subscription | RequestStatus::Invalid => Err(Error::InvalidRequestId),
	}
}

/// Sends an unsubscribe to request to server to indicate
/// that the client is not interested in the subscription anymore.
//
// NOTE: we don't count this a concurrent request as it's part of a subscription.
pub(crate) async fn stop_subscription(
	sender: &mut impl TransportSenderT,
	manager: &mut RequestManager,
	unsub: RequestMessage,
) {
	if let Err(e) = sender.send(unsub.raw).await {
		tracing::error!("Send unsubscribe request failed: {:?}", e);
		let _ = manager.complete_pending_call(unsub.id);
	}
}

/// Builds an unsubscription message.
pub(crate) fn build_unsubscribe_message(
	manager: &mut RequestManager,
	sub_req_id: Id<'static>,
	sub_id: SubscriptionId<'static>,
) -> Option<RequestMessage> {
	let (unsub_req_id, _, unsub, sub_id) = manager.remove_subscription(sub_req_id, sub_id)?;

	let mut params = ArrayParams::new();
	params.insert(sub_id).ok()?;
	let params = params.to_rpc_params().ok()?;

	let raw = serde_json::to_string(&RequestSer::owned(unsub_req_id.clone(), unsub, params)).ok()?;
	Some(RequestMessage { raw, id: unsub_req_id, send_back: None })
}

/// Attempts to process an error response.
///
/// Returns `Ok` if the response was successfully sent.
/// Returns `Err(_)` if the response ID was not found.
pub(crate) fn process_error_response(manager: &mut RequestManager, err: ErrorResponse) -> Result<(), Error> {
	let id = err.id().clone().into_owned();

	match manager.request_status(&id) {
		RequestStatus::PendingMethodCall => {
			let send_back = manager.complete_pending_call(id).expect("State checked above; qed");
			let _ =
				send_back.map(|s| s.send(Err(Error::Call(CallError::Custom(err.error_object().clone().into_owned())))));
			Ok(())
		}
		RequestStatus::PendingSubscription => {
			let (_, send_back, _) = manager.complete_pending_subscription(id).expect("State checked above; qed");
			let _ = send_back.send(Err(Error::Call(CallError::Custom(err.error_object().clone().into_owned()))));
			Ok(())
		}
		_ => Err(Error::InvalidRequestId),
	}
}

/// Wait for a stream to complete within the given timeout.
pub(crate) async fn call_with_timeout<T>(
	timeout: std::time::Duration,
	rx: futures_channel::oneshot::Receiver<Result<T, Error>>,
) -> Result<Result<T, Error>, futures_channel::oneshot::Canceled> {
	match future::select(rx, Delay::new(timeout)).await {
		Either::Left((res, _)) => res,
		Either::Right((_, _)) => Ok(Err(Error::RequestTimeout)),
	}
}
