use crate::server::MethodResult;

use futures_util::io::{BufReader, BufWriter};
use futures_util::TryStreamExt;
use hyper::upgrade::Upgraded;
use jsonrpsee_core::logger::{self, WsLogger};
use jsonrpsee_core::server::helpers::{
	prepare_error, BatchResponse, BatchResponseBuilder, BoundedSubscriptions, MethodResponse, MethodSink,
};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::{ConnState, MethodKind, Methods};
use jsonrpsee_core::tracing::{rx_log_from_json, rx_log_from_str, RpcTracing};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::Error;
use jsonrpsee_types::error::{reject_too_many_subscriptions, ErrorCode};
use jsonrpsee_types::{ErrorObject, Id, Params, Request};
use soketto::data::ByteSlice125;
use tokio_util::compat::Compat;
use tracing_futures::Instrument;

pub(crate) type Sender = soketto::Sender<BufReader<BufWriter<Compat<Upgraded>>>>;
pub(crate) type Receiver = soketto::Receiver<BufReader<BufWriter<Compat<Upgraded>>>>;

pub(crate) async fn send_message(sender: &mut Sender, response: String) -> Result<(), Error> {
	sender.send_text_owned(response).await?;
	sender.flush().await.map_err(Into::into)
}

pub(crate) async fn send_ping(sender: &mut Sender) -> Result<(), Error> {
	tracing::debug!("Send ping");
	// Submit empty slice as "optional" parameter.
	let slice: &[u8] = &[];
	// Byte slice fails if the provided slice is larger than 125 bytes.
	let byte_slice = ByteSlice125::try_from(slice).expect("Empty slice should fit into ByteSlice125");
	sender.send_ping(byte_slice).await?;
	sender.flush().await.map_err(Into::into)
}

#[derive(Debug, Clone)]
pub(crate) struct Batch<'a, L: WsLogger> {
	pub(crate) data: Vec<u8>,
	pub(crate) call: CallData<'a, L>,
}

#[derive(Debug, Clone)]
pub(crate) struct CallData<'a, L: WsLogger> {
	pub(crate) conn_id: usize,
	pub(crate) bounded_subscriptions: BoundedSubscriptions,
	pub(crate) id_provider: &'a dyn IdProvider,
	pub(crate) methods: &'a Methods,
	pub(crate) max_response_body_size: u32,
	pub(crate) max_log_length: u32,
	pub(crate) resources: &'a Resources,
	pub(crate) sink: &'a MethodSink,
	pub(crate) logger: &'a L,
	pub(crate) request_start: L::Instant,
}

#[derive(Debug, Clone)]
pub(crate) struct Call<'a, L: WsLogger> {
	pub(crate) params: Params<'a>,
	pub(crate) name: &'a str,
	pub(crate) call: CallData<'a, L>,
	pub(crate) id: Id<'a>,
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
pub(crate) async fn process_batch_request<L: WsLogger>(b: Batch<'_, L>) -> BatchResponse {
	let Batch { data, call } = b;

	if let Ok(batch) = serde_json::from_slice::<Vec<Request>>(&data) {
		return if !batch.is_empty() {
			let batch = batch.into_iter().map(|req| Ok((req, call.clone())));
			let batch_stream = futures_util::stream::iter(batch);

			let trace = RpcTracing::batch();

			return async {
				let max_response_size = call.max_response_body_size;

				let batch_response = batch_stream
					.try_fold(
						BatchResponseBuilder::new_with_limit(max_response_size as usize),
						|batch_response, (req, call)| async move {
							let params = Params::new(req.params.map(|params| params.get()));
							let response = execute_call(Call { name: &req.method, params, id: req.id, call }).await;
							batch_response.append(response.as_inner())
						},
					)
					.await;

				match batch_response {
					Ok(batch) => batch.finish(),
					Err(batch_err) => batch_err,
				}
			}
			.instrument(trace.into_span())
			.await;
		} else {
			BatchResponse::error(Id::Null, ErrorObject::from(ErrorCode::InvalidRequest))
		};
	}

	let (id, code) = prepare_error(&data);
	BatchResponse::error(id, ErrorObject::from(code))
}

pub(crate) async fn process_single_request<L: WsLogger>(data: Vec<u8>, call: CallData<'_, L>) -> MethodResult {
	if let Ok(req) = serde_json::from_slice::<Request>(&data) {
		let trace = RpcTracing::method_call(&req.method);

		async {
			rx_log_from_json(&req, call.max_log_length);

			let params = Params::new(req.params.map(|params| params.get()));
			let name = &req.method;
			let id = req.id;

			execute_call(Call { name, params, id, call }).await
		}
		.instrument(trace.into_span())
		.await
	} else {
		let (id, code) = prepare_error(&data);
		MethodResult::SendAndLogger(MethodResponse::error(id, ErrorObject::from(code)))
	}
}

/// Execute a call which returns result of the call with a additional sink
/// to fire a signal once the subscription call has been answered.
///
/// Returns `(MethodResponse, None)` on every call that isn't a subscription
/// Otherwise `(MethodResponse, Some(PendingSubscriptionCallTx)`.
pub(crate) async fn execute_call<L: WsLogger>(c: Call<'_, L>) -> MethodResult {
	let Call { name, id, params, call } = c;
	let CallData {
		resources,
		methods,
		max_response_body_size,
		max_log_length,
		conn_id,
		bounded_subscriptions,
		id_provider,
		sink,
		logger,
		request_start,
	} = call;

	let response = match methods.method_with_name(name) {
		None => {
			logger.on_call(name, params.clone(), logger::MethodKind::Unknown);
			let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound));
			MethodResult::SendAndLogger(response)
		}
		Some((name, method)) => match &method.inner() {
			MethodKind::Sync(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall);
				match method.claim(name, resources) {
					Ok(guard) => {
						let r = (callback)(id, params, max_response_body_size as usize);
						drop(guard);
						MethodResult::SendAndLogger(r)
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
						let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy));
						MethodResult::SendAndLogger(response)
					}
				}
			}
			MethodKind::Async(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall);
				match method.claim(name, resources) {
					Ok(guard) => {
						let id = id.into_owned();
						let params = params.into_owned();

						let response =
							(callback)(id, params, conn_id, max_response_body_size as usize, Some(guard)).await;
						MethodResult::SendAndLogger(response)
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
						let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy));
						MethodResult::SendAndLogger(response)
					}
				}
			}
			MethodKind::Subscription(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::Subscription);
				match method.claim(name, resources) {
					Ok(guard) => {
						if let Some(cn) = bounded_subscriptions.acquire() {
							let conn_state = ConnState { conn_id, close_notify: cn, id_provider };
							let response = callback(id.clone(), params, sink.clone(), conn_state, Some(guard)).await;
							MethodResult::JustLogger(response)
						} else {
							let response =
								MethodResponse::error(id, reject_too_many_subscriptions(bounded_subscriptions.max()));
							MethodResult::SendAndLogger(response)
						}
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
						let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy));
						MethodResult::SendAndLogger(response)
					}
				}
			}
			MethodKind::Unsubscription(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::Unsubscription);

				// Don't adhere to any resource or subscription limits; always let unsubscribing happen!
				let result = callback(id, params, conn_id, max_response_body_size as usize);
				MethodResult::SendAndLogger(result)
			}
		},
	};

	let r = response.as_inner();

	rx_log_from_str(&r.result, max_log_length);
	logger.on_result(name, r.success, request_start);
	response
}
