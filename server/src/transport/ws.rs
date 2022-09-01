use crate::future::FutureDriver;
use crate::logger::{self, Logger};
use crate::server::{MethodResult, Monitored, MonitoredError, ServiceData};

use futures_channel::mpsc;
use futures_util::future::Either;
use futures_util::io::{BufReader, BufWriter};
use futures_util::{FutureExt, StreamExt, TryStreamExt};
use hyper::upgrade::Upgraded;
use jsonrpsee_core::server::helpers::{
	prepare_error, BatchResponse, BatchResponseBuilder, BoundedSubscriptions, MethodResponse, MethodSink,
};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::{ConnState, MethodKind, Methods};
use jsonrpsee_core::tracing::{rx_log_from_json, rx_log_from_str, tx_log_from_str, RpcTracing};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::Error;
use jsonrpsee_types::error::{
	reject_too_big_request, reject_too_many_subscriptions, ErrorCode, BATCHES_NOT_SUPPORTED_CODE,
	BATCHES_NOT_SUPPORTED_MSG,
};
use jsonrpsee_types::{ErrorObject, Id, Params, Request};
use soketto::connection::Error as SokettoError;
use soketto::data::ByteSlice125;
use tokio_stream::wrappers::IntervalStream;
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
pub(crate) struct Batch<'a, L: Logger> {
	pub(crate) data: Vec<u8>,
	pub(crate) call: CallData<'a, L>,
}

#[derive(Debug, Clone)]
pub(crate) struct CallData<'a, L: Logger> {
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
pub(crate) struct Call<'a, L: Logger> {
	pub(crate) params: Params<'a>,
	pub(crate) name: &'a str,
	pub(crate) call: CallData<'a, L>,
	pub(crate) id: Id<'a>,
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
pub(crate) async fn process_batch_request<L: Logger>(b: Batch<'_, L>) -> BatchResponse {
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

pub(crate) async fn process_single_request<L: Logger>(data: Vec<u8>, call: CallData<'_, L>) -> MethodResult {
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
pub(crate) async fn execute_call<L: Logger>(c: Call<'_, L>) -> MethodResult {
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

pub(crate) async fn background_task<L: Logger>(
	mut sender: Sender,
	mut receiver: Receiver,
	svc: ServiceData<L>,
) -> Result<(), Error> {
	let ServiceData {
		methods,
		resources,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		batch_requests_supported,
		stop_monitor,
		id_provider,
		ping_interval,
		max_subscriptions_per_connection,
		conn_id,
		logger,
		remote_addr,
		conn,
		..
	} = svc;

	let (tx, mut rx) = mpsc::unbounded::<String>();
	let bounded_subscriptions = BoundedSubscriptions::new(max_subscriptions_per_connection);
	let bounded_subscriptions2 = bounded_subscriptions.clone();

	let stop_monitor2 = stop_monitor.clone();
	let sink = MethodSink::new_with_limit(tx, max_response_body_size, max_log_length);

	// Send results back to the client.
	tokio::spawn(async move {
		// Received messages from the WebSocket.
		let mut rx_item = rx.next();

		// Interval to send out continuously `pings`.
		let ping_interval = IntervalStream::new(tokio::time::interval(ping_interval));
		tokio::pin!(ping_interval);
		let mut next_ping = ping_interval.next();

		while !stop_monitor2.is_shutdown_requested() {
			// Ensure select is cancel-safe by fetching and storing the `rx_item` that did not finish yet.
			// Note: Although, this is cancel-safe already, avoid using `select!` macro for future proofing.
			match futures_util::future::select(rx_item, next_ping).await {
				Either::Left((Some(response), ping)) => {
					// If websocket message send fail then terminate the connection.
					if let Err(err) = send_message(&mut sender, response).await {
						tracing::error!("Terminate connection: WS send error: {}", err);
						break;
					}
					rx_item = rx.next();
					next_ping = ping;
				}
				// Nothing else to receive.
				Either::Left((None, _)) => break,

				// Handle timer intervals.
				Either::Right((_, next_rx)) => {
					if let Err(err) = send_ping(&mut sender).await {
						tracing::error!("Terminate connection: WS send ping error: {}", err);
						break;
					}
					rx_item = next_rx;
					next_ping = ping_interval.next();
				}
			}
		}

		// Terminate connection and send close message.
		let _ = sender.close().await;

		// Notify all listeners and close down associated tasks.
		bounded_subscriptions2.close();
	});

	// Buffer for incoming data.
	let mut data = Vec::with_capacity(100);
	let mut method_executors = FutureDriver::default();
	let logger = &logger;

	let result = loop {
		data.clear();

		{
			// Need the extra scope to drop this pinned future and reclaim access to `data`
			let receive = async {
				// Identical loop to `soketto::receive_data` with debug logs for `Pong` frames.
				loop {
					match receiver.receive(&mut data).await? {
						soketto::Incoming::Data(d) => break Ok(d),
						soketto::Incoming::Pong(_) => tracing::debug!("Received pong"),
						soketto::Incoming::Closed(_) => {
							// The closing reason is already logged by `soketto` trace log level.
							// Return the `Closed` error to avoid logging unnecessary warnings on clean shutdown.
							break Err(SokettoError::Closed);
						}
					}
				}
			};

			tokio::pin!(receive);

			if let Err(err) = method_executors.select_with(Monitored::new(receive, &stop_monitor)).await {
				match err {
					MonitoredError::Selector(SokettoError::Closed) => {
						tracing::debug!("WS transport: Remote peer terminated the connection: {}", conn_id);
						sink.close();
						break Ok(());
					}
					MonitoredError::Selector(SokettoError::MessageTooLarge { current, maximum }) => {
						tracing::warn!(
							"WS transport error: Request length: {} exceeded max limit: {} bytes",
							current,
							maximum
						);
						sink.send_error(Id::Null, reject_too_big_request(max_request_body_size));
						continue;
					}
					// These errors can not be gracefully handled, so just log them and terminate the connection.
					MonitoredError::Selector(err) => {
						tracing::error!("Terminate connection {}: WS error: {}", conn_id, err);
						sink.close();
						break Err(err.into());
					}
					MonitoredError::Shutdown => break Ok(()),
				};
			};
		};

		let request_start = logger.on_request();

		let first_non_whitespace = data.iter().find(|byte| !byte.is_ascii_whitespace());
		match first_non_whitespace {
			Some(b'{') => {
				let data = std::mem::take(&mut data);
				let sink = sink.clone();
				let resources = &resources;
				let methods = &methods;
				let bounded_subscriptions = bounded_subscriptions.clone();
				let id_provider = &*id_provider;

				let fut = async move {
					let call = CallData {
						conn_id: conn_id as usize,
						resources,
						max_response_body_size,
						max_log_length,
						methods,
						bounded_subscriptions,
						sink: &sink,
						id_provider,
						logger,
						request_start,
					};

					match process_single_request(data, call).await {
						MethodResult::JustLogger(r) => {
							logger.on_response(&r.result, request_start);
						}
						MethodResult::SendAndLogger(r) => {
							logger.on_response(&r.result, request_start);
							let _ = sink.send_raw(r.result);
						}
					};
				}
				.boxed();

				method_executors.add(fut);
			}
			Some(b'[') if !batch_requests_supported => {
				let response = MethodResponse::error(
					Id::Null,
					ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
				);
				logger.on_response(&response.result, request_start);
				let _ = sink.send_raw(response.result);
			}
			Some(b'[') => {
				// Make sure the following variables are not moved into async closure below.
				let resources = &resources;
				let methods = &methods;
				let bounded_subscriptions = bounded_subscriptions.clone();
				let sink = sink.clone();
				let id_provider = id_provider.clone();
				let data = std::mem::take(&mut data);

				let fut = async move {
					let response = process_batch_request(Batch {
						data,
						call: CallData {
							conn_id: conn_id as usize,
							resources,
							max_response_body_size,
							max_log_length,
							methods,
							bounded_subscriptions,
							sink: &sink,
							id_provider: &*id_provider,
							logger,
							request_start,
						},
					})
					.await;

					tx_log_from_str(&response.result, max_log_length);
					logger.on_response(&response.result, request_start);
					let _ = sink.send_raw(response.result);
				};

				method_executors.add(Box::pin(fut));
			}
			_ => {
				sink.send_error(Id::Null, ErrorCode::ParseError.into());
			}
		}
	};

	logger.on_disconnect(remote_addr);

	// Drive all running methods to completion.
	// **NOTE** Do not return early in this function. This `await` needs to run to guarantee
	// proper drop behaviour.
	method_executors.await;

	drop(conn);

	result
}
