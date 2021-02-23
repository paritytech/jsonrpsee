use crate::{
	manager::{RequestManager, RequestStatus},
	FrontToBack,
};
use core::convert::TryInto;
use futures::{channel::mpsc, prelude::*};
use jsonrpsee_types::error::Error;
use jsonrpsee_types::jsonrpc::{self, JsonValue, SubscriptionId};
use jsonrpsee_types::traits::{TransportReceiver, TransportSender};

/// Function being run in the background that processes messages from the frontend.
pub async fn background_task<S, R>(
	sender: S,
	receiver: R,
	mut frontend: mpsc::Receiver<FrontToBack>,
	max_capacity_per_subscription: usize,
) where
	S: TransportSender + Send + 'static,
	R: TransportReceiver + Send + 'static,
{
	let mut manager = RequestManager::new();
	let mut sender: crate::jsonrpc_sender::Sender<_> = sender.into();

	let backend_event = futures::stream::unfold(receiver, |mut receiver| async {
		let res = receiver.receive().await;
		Some((res, receiver))
	});

	futures::pin_mut!(backend_event);

	loop {
		let next_frontend = frontend.next();
		let next_backend = backend_event.next();
		futures::pin_mut!(next_frontend, next_backend);

		futures::select! {
			event = next_frontend => match event {
				// User dropped the sender side of the channel.
				None => {
					log::trace!("[backend]: frontend channel dropped; terminate client");
					break
				}
				// User called `notification` on the front-end
				Some(FrontToBack::Notification { method, params }) => {
					log::trace!("[backend]: client prepares to send notification");
					let _ = sender.send_notification(method, params).await;
				}
				// User called `request` on the front-end
				Some(FrontToBack::StartRequest { method, params, send_back }) => {
					log::trace!("[backend]: client prepares to send request={:?}", method);
					match sender.start_request(method, params).await {
						Ok(id) => {
							if let Err(send_back) = manager.insert_pending_call(id, send_back) {
								let _ = send_back.send(Err(Error::DuplicateRequestId));
							}
						}
						Err(err) => {
							log::warn!("[backend]: client request failed: {:?}", err);
							let _ = send_back.send(Err(Error::TransportError(Box::new(err))));
						}
					}
				}
				// User called `subscribe` on the front-end.
				Some(FrontToBack::Subscribe { subscribe_method, unsubscribe_method, params, send_back }) => {
						log::trace!(
						"[backend]: client prepares to start subscription, subscribe_method={:?} unsubscribe_method:{:?}",
						subscribe_method,
						unsubscribe_method
					);
					match sender.start_subscription(subscribe_method, params).await {
						Ok(id) => {
							if let Err(send_back) = manager.insert_pending_subscription(id, send_back, unsubscribe_method) {
								let _ = send_back.send(Err(Error::DuplicateRequestId));
							}
						}
						Err(err) => {
							log::warn!("[backend]: client subscription failed: {:?}", err);
							let _ = send_back.send(Err(Error::TransportError(Box::new(err))));
						}
					}
				}
				// User dropped a subscription.
				Some(FrontToBack::SubscriptionClosed(sub_id)) => {
							log::trace!("Closing in subscription: {:?}", sub_id);
					// NOTE: The subscription may have been closed earlier if
					// the channel was full or disconnected.
					if let Some(request_id) = manager.get_request_id_by_subscription_id(&sub_id) {
						if let Some((_sink, unsubscribe_method)) = manager.remove_subscription(request_id, sub_id.clone()) {
							if let Ok(json_sub_id) = jsonrpc::to_value(sub_id) {
								let params = jsonrpc::Params::Array(vec![json_sub_id]);
								let _ = sender.start_request(unsubscribe_method, params).await;
							}
						}
					}
				}
			},
			event = next_backend => match event {
				None => {
					log::trace!("[backend]: backend channel dropped; terminate client");
					break;
				}
				Some(Ok(jsonrpc::Response::Single(response))) => {
					match process_response(&mut manager, response, max_capacity_per_subscription) {
						Ok(Some((unsubscribe, params))) => {
							if let Err(e) = sender.start_request(unsubscribe, params).await {
								log::error!("Failed to send unsubscription response: {:?}", e);
							}
						}
						Ok(None) => (),
						Err(e) => {
							log::error!("Error: {:?} terminating client", e);
							break;
						}
					}
				}
				Some(Ok(jsonrpc::Response::Batch(_responses))) => {
					todo!("batch requests^^")
				}
				Some(Ok(jsonrpc::Response::Notif(notif))) => {
					let sub_id = notif.params.subscription;
					let request_id = match manager.get_request_id_by_subscription_id(&sub_id) {
						Some(r) => r,
						None => {
							log::error!("Subscription ID: {:?} not found", sub_id);
							continue;
						}
					};

					match manager.as_subscription_mut(&request_id) {
						Some(send_back_sink) => {
							if let Err(e) = send_back_sink.try_send(notif.params.result) {
								log::error!("Dropping subscription {:?} error: {:?}", sub_id, e);
								manager.remove_subscription(request_id, sub_id).expect("subscription is active; checked above");
							}
						}
						None => {
							log::error!("Subscription ID: {:?} not an active subscription", sub_id);
						},
					}
				}
				Some(Err(e)) => {
					log::error!("Error: {:?} terminating client", e);
					break;
				}
			}
		}
	}
}

/// Process a response from the server.
///
/// Returns `Ok(_)` if the response was successful or if the error could be handled.
/// Returns `Err(_)` if the response couldn't be handled.
fn process_response(
	manager: &mut RequestManager,
	response: jsonrpc::Output,
	max_capacity_per_subscription: usize,
) -> Result<Option<(String, jsonrpc::Params)>, Error> {
	let response_id = *response.id().as_number().ok_or(Error::InvalidRequestId)?;

	match manager.request_status(&response_id) {
		RequestStatus::PendingMethodCall => {
			let send_back_oneshot = manager.complete_pending_call(response_id).ok_or(Error::InvalidRequestId)?;
			let response = response.try_into().map_err(Error::Request);
			match send_back_oneshot.send(response) {
				Err(Err(e)) => Err(e),
				Err(Ok(_)) => Err(Error::Custom("Frontend channel closed".into())),
				Ok(_) => Ok(None),
			}
		}
		RequestStatus::PendingSubscription => {
			let (send_back_oneshot, unsubscribe_method) =
				manager.complete_pending_subscription(response_id).ok_or(Error::InvalidRequestId)?;
			let json_sub_id: JsonValue = match response.try_into() {
				Ok(response) => response,
				Err(e) => {
					return match send_back_oneshot.send(Err(Error::Request(e))) {
						Err(Err(e)) => Err(e),
						Err(Ok(_)) => unreachable!("Error sent above; qed"),
						_ => Ok(None),
					};
				}
			};

			let sub_id: SubscriptionId = match jsonrpc::from_value(json_sub_id.clone()) {
				Ok(sub_id) => sub_id,
				Err(_) => {
					return match send_back_oneshot.send(Err(Error::InvalidSubscriptionId)) {
						Err(Err(e)) => Err(e),
						Err(Ok(_)) => unreachable!("Error sent above; qed"),
						_ => Ok(None),
					}
				}
			};

			let (subscribe_tx, subscribe_rx) = mpsc::channel(max_capacity_per_subscription);
			if manager.insert_subscription(response_id, sub_id.clone(), subscribe_tx, unsubscribe_method).is_ok() {
				match send_back_oneshot.send(Ok((subscribe_rx, sub_id.clone()))) {
					Ok(_) => Ok(None),
					Err(_) => {
						let (_, unsubscribe_method) =
							manager.remove_subscription(response_id, sub_id).expect("Subscription inserted above; qed");
						let params = jsonrpc::Params::Array(vec![json_sub_id]);
						Ok(Some((unsubscribe_method, params)))
					}
				}
			} else {
				match send_back_oneshot.send(Err(Error::InvalidSubscriptionId)) {
					Err(Err(e)) => Err(e),
					Err(Ok(_)) => unreachable!("Error sent above; qed"),
					_ => Ok(None),
				}
			}
		}
		RequestStatus::Subscription | RequestStatus::Invalid => Err(Error::InvalidRequestId),
	}
}
