use crate::manager::{RequestManager, RequestStatus};
use crate::transport::Sender as WsSender;
use futures::channel::{mpsc, oneshot};
use jsonrpsee_types::v2::params::{Id, JsonRpcParams, JsonRpcSubscriptionParams, SubscriptionId};
use jsonrpsee_types::v2::request::{JsonRpcCallSer, JsonRpcNotification};
use jsonrpsee_types::v2::response::JsonRpcResponse;
use jsonrpsee_types::{v2::error::JsonRpcError, Error, RequestMessage};
use serde_json::Value as JsonValue;
use std::time::Duration;

/// Attempts to process a batch response.
///
/// On success the result is sent to the frontend.
pub fn process_batch_response(manager: &mut RequestManager, rps: Vec<JsonRpcResponse<JsonValue>>) -> Result<(), Error> {
	let mut digest = Vec::with_capacity(rps.len());
	let mut ordered_responses = vec![JsonValue::Null; rps.len()];
	let mut rps_unordered: Vec<_> = Vec::with_capacity(rps.len());

	for rp in rps {
		let id = rp.id.as_number().copied().ok_or(Error::InvalidRequestId)?;
		digest.push(id);
		rps_unordered.push((id, rp.result));
	}

	digest.sort_unstable();
	let batch_state = match manager.complete_pending_batch(digest) {
		Some(state) => state,
		None => {
			log::warn!("Received unknown batch response");
			return Err(Error::InvalidRequestId);
		}
	};

	for (id, rp) in rps_unordered {
		let pos =
			batch_state.order.get(&id).copied().expect("All request IDs valid checked by RequestManager above; qed");
		ordered_responses[pos] = rp;
	}
	let _ = batch_state.send_back.send(Ok(ordered_responses));
	Ok(())
}

/// Attempts to process a subscription response.
///
/// Returns `Ok()` if the response was successfully sent to the frontend.
/// Return `Err(None)` if the subscription was not found.
/// Returns `Err(Some(msg))` if the subscription was full.
pub fn process_subscription_response(
	manager: &mut RequestManager,
	notif: JsonRpcNotification<JsonRpcSubscriptionParams<JsonValue>>,
) -> Result<(), Option<RequestMessage>> {
	let sub_id = notif.params.subscription;
	let request_id = match manager.get_request_id_by_subscription_id(&sub_id) {
		Some(request_id) => request_id,
		None => return Err(None),
	};

	match manager.as_subscription_mut(&request_id) {
		Some(send_back_sink) => match send_back_sink.try_send(notif.params.result) {
			Ok(()) => Ok(()),
			Err(err) => {
				log::error!("Dropping subscription {:?} error: {:?}", sub_id, err);
				let msg = build_unsubscribe_message(manager, request_id, sub_id)
					.expect("request ID and subscription ID valid checked above; qed");
				Err(Some(msg))
			}
		},
		None => {
			log::error!("Subscription ID: {:?} not an active subscription", sub_id);
			Err(None)
		}
	}
}

/// Attempts to process an incoming notification
///
/// Returns Ok() if the response was successfully handled
/// Returns Err() if there was no handler for the method
pub fn process_notification(manager: &mut RequestManager, notif: JsonRpcNotification<JsonValue>) -> Result<(), Error> {
	match manager.as_notification_handler_mut(notif.method.to_owned()) {
		Some(send_back_sink) => match send_back_sink.try_send(notif.params) {
			Ok(()) => Ok(()),
			Err(err) => {
				log::error!("Error sending notification, dropping handler for {:?} error: {:?}", notif.method, err);
				let _ = manager.remove_notification_handler(notif.method.to_owned());
				Err(Error::Internal(err.into_send_error()))
			}
		},
		None => {
			log::error!("Notification: {:?} not a registered method", notif.method);
			Err(Error::UnregisteredNotification(notif.method.to_owned()))
		}
	}
}

/// Process a response from the server.
///
/// Returns `Ok(None)` if the response was successfully sent.
/// Returns `Ok(Some(_))` if the response got an error but could be handled.
/// Returns `Err(_)` if the response couldn't be handled.
pub fn process_single_response(
	manager: &mut RequestManager,
	response: JsonRpcResponse<JsonValue>,
	max_capacity_per_subscription: usize,
) -> Result<Option<RequestMessage>, Error> {
	let response_id = response.id.as_number().copied().ok_or(Error::InvalidRequestId)?;
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
				manager.complete_pending_subscription(response_id).ok_or(Error::InvalidRequestId)?;

			let sub_id: SubscriptionId = match serde_json::from_value(response.result) {
				Ok(sub_id) => sub_id,
				Err(_) => {
					let _ = send_back_oneshot.send(Err(Error::InvalidSubscriptionId));
					return Ok(None);
				}
			};

			let (subscribe_tx, subscribe_rx) = mpsc::channel(max_capacity_per_subscription);
			if manager
				.insert_subscription(response_id, unsub_id, sub_id.clone(), subscribe_tx, unsubscribe_method)
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
pub async fn stop_subscription(sender: &mut WsSender, manager: &mut RequestManager, unsub: RequestMessage) {
	if let Err(e) = sender.send(unsub.raw).await {
		log::error!("Send unsubscribe request failed: {:?}", e);
		let _ = manager.complete_pending_call(unsub.id);
	}
}

/// Builds an unsubscription message.
pub fn build_unsubscribe_message(
	manager: &mut RequestManager,
	sub_req_id: u64,
	sub_id: SubscriptionId,
) -> Option<RequestMessage> {
	let (unsub_req_id, _, unsub, sub_id) = manager.remove_subscription(sub_req_id, sub_id)?;
	let sub_id_slice: &[JsonValue] = &[sub_id.into()];
	// TODO: https://github.com/paritytech/jsonrpsee/issues/275
	let params = JsonRpcParams::ArrayRef(sub_id_slice);
	let raw = serde_json::to_string(&JsonRpcCallSer::new(Id::Number(unsub_req_id), &unsub, params)).ok()?;
	Some(RequestMessage { raw, id: unsub_req_id, send_back: None })
}

/// Attempts to process an error response.
///
/// Returns `Ok` if the response was successfully sent.
/// Returns `Err(_)` if the response ID was not found.
pub fn process_error_response(manager: &mut RequestManager, err: JsonRpcError) -> Result<(), Error> {
	let id = err.id.as_number().copied().ok_or(Error::InvalidRequestId)?;
	match manager.request_status(&id) {
		RequestStatus::PendingMethodCall => {
			let send_back = manager.complete_pending_call(id).expect("State checked above; qed");
			let _ = send_back.map(|s| s.send(Err(Error::Request(err.to_string()))));
			Ok(())
		}
		RequestStatus::PendingSubscription => {
			let (_, send_back, _) = manager.complete_pending_subscription(id).expect("State checked above; qed");
			let _ = send_back.send(Err(Error::Request(err.to_string())));
			Ok(())
		}
		_ => Err(Error::InvalidRequestId),
	}
}

/// Wait for a stream to complete within the given timeout.
pub async fn call_with_timeout<T>(
	timeout: Duration,
	rx: oneshot::Receiver<Result<T, Error>>,
) -> Result<Result<T, Error>, oneshot::Canceled> {
	let timeout = crate::tokio::sleep(timeout);
	futures::pin_mut!(rx, timeout);
	match futures::future::select(rx, timeout).await {
		futures::future::Either::Left((res, _)) => res,
		futures::future::Either::Right((_, _)) => Ok(Err(Error::RequestTimeout)),
	}
}
