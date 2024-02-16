use std::error::Error as StdError;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{fmt, sync::atomic::AtomicUsize};

use crate::{stop_channel, RpcModule, Server, ServerBuilder, ServerHandle};

use futures_util::FutureExt;
use hyper::server::conn::AddrStream;
use jsonrpsee_core::server::Methods;
use jsonrpsee_core::{DeserializeOwned, RpcResult, StringError};
use jsonrpsee_test_utils::TimeoutFutureExt;
use jsonrpsee_types::{error::ErrorCode, ErrorObject, ErrorObjectOwned, Response, ResponseSuccess};
use tower::Service;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub(crate) struct TestContext;

impl TestContext {
	pub(crate) fn ok(&self) -> Result<(), MyAppError> {
		Ok(())
	}
	pub(crate) fn err(&self) -> Result<(), MyAppError> {
		Err(MyAppError)
	}
}

/// Spawns a dummy JSON-RPC server.
pub(crate) async fn server() -> SocketAddr {
	let (addr, handle) = server_with_handles().await;
	tokio::spawn(handle.stopped());
	addr
}

/// Spawns a dummy JSON-RPC server.
///
/// Returns the address together with handle for the server.
pub(crate) async fn server_with_handles() -> (SocketAddr, ServerHandle) {
	let server = ServerBuilder::default().build("127.0.0.1:0").with_default_timeout().await.unwrap().unwrap();
	let ctx = TestContext;
	let mut module = RpcModule::new(ctx);
	module
		.register_method("say_hello", |_, _, _| {
			tracing::debug!("server respond to hello");
			"hello"
		})
		.unwrap();
	module
		.register_method::<Result<u64, ErrorObjectOwned>, _>("add", |params, _, _| {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	module
		.register_method::<Result<String, ErrorObjectOwned>, _>("multiparam", |params, _, _| {
			let params: (String, String, Vec<u8>) = params.parse()?;
			let r = format!("string1={}, string2={}, vec={}", params.0.len(), params.1.len(), params.2.len());
			Ok(r)
		})
		.unwrap();
	module
		.register_async_method("say_hello_async", |_, _, _ctx| {
			async move {
				tracing::debug!("server respond to hello");
				// Call some async function inside.
				futures_util::future::ready(()).await;
				"hello"
			}
		})
		.unwrap();
	module
		.register_async_method::<Result<u64, ErrorObjectOwned>, _, _>("add_async", |params, _, _| async move {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	module.register_method("invalid_params", |_params, _, _| Err::<(), _>(invalid_params())).unwrap();
	module.register_method("call_fail", |_params, _, _| Err::<(), _>(MyAppError)).unwrap();
	module
		.register_method::<Result<&str, ErrorObjectOwned>, _>("sleep_for", |params, _, _| {
			let sleep: Vec<u64> = params.parse()?;
			std::thread::sleep(std::time::Duration::from_millis(sleep[0]));
			Ok("Yawn!")
		})
		.unwrap();
	module
		.register_subscription::<Result<(), StringError>, _, _>(
			"subscribe_hello",
			"subscribe_hello",
			"unsubscribe_hello",
			|_, pending, _| async move {
				let sink = pending.accept().await?;

				loop {
					let _ = &sink;
					tokio::time::sleep(std::time::Duration::from_secs(30)).await;
				}
			},
		)
		.unwrap();

	module.register_method("notif", |_, _, _ctx| "").unwrap();
	module
		.register_method("should_err", |_, _, ctx| {
			ctx.err()?;
			RpcResult::Ok("err")
		})
		.unwrap();

	module
		.register_method("should_ok", |_, _, ctx| {
			ctx.ok()?;
			RpcResult::Ok("ok")
		})
		.unwrap();
	module
		.register_async_method("should_ok_async", |_p, _, ctx| async move {
			ctx.ok()?;
			Ok::<_, MyAppError>("ok")
		})
		.unwrap();

	let addr = server.local_addr().unwrap();

	let server_handle = server.start(module);
	(addr, server_handle)
}

/// Run server with user provided context.
pub(crate) async fn server_with_context() -> SocketAddr {
	let server = ServerBuilder::default().build("127.0.0.1:0").with_default_timeout().await.unwrap().unwrap();

	let ctx = TestContext;
	let mut rpc_module = RpcModule::new(ctx);

	rpc_module
		.register_method("should_err", |_p, _, ctx| {
			ctx.err()?;
			RpcResult::Ok("err")
		})
		.unwrap();

	rpc_module
		.register_method("should_ok", |_p, _, ctx| {
			ctx.ok()?;
			RpcResult::Ok("ok")
		})
		.unwrap();

	rpc_module
		.register_async_method("should_ok_async", |_p, _, ctx| async move {
			ctx.ok()?;
			// Call some async function inside.
			Result::<_, MyAppError>::Ok(futures_util::future::ready("ok!").await)
		})
		.unwrap();

	rpc_module
		.register_async_method("err_async", |_p, _, ctx| async move {
			ctx.ok()?;
			// Async work that returns an error
			futures_util::future::err::<(), _>(MyAppError).await
		})
		.unwrap();

	let addr = server.local_addr().unwrap();
	let handle = server.start(rpc_module);

	tokio::spawn(handle.stopped());
	addr
}

pub(crate) fn init_logger() {
	let _ = FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).try_init();
}

pub(crate) fn deser_call<T: DeserializeOwned + fmt::Debug + Clone>(raw: String) -> T {
	let rp: Response<T> = serde_json::from_str(&raw).unwrap();
	ResponseSuccess::try_from(rp).expect("Successful call").result
}

/// Applications can/should provide their own error.
#[derive(Copy, Clone, Debug)]
pub struct MyAppError;
impl fmt::Display for MyAppError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "MyAppError")
	}
}
impl std::error::Error for MyAppError {}

impl From<MyAppError> for ErrorObjectOwned {
	fn from(_: MyAppError) -> Self {
		ErrorObject::owned(-32000, "MyAppError", None::<()>)
	}
}

fn invalid_params() -> ErrorObjectOwned {
	ErrorCode::InvalidParams.into()
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Metrics {
	pub(crate) ws_sessions_opened: Arc<AtomicUsize>,
	pub(crate) ws_sessions_closed: Arc<AtomicUsize>,
}

pub(crate) fn ws_server_with_stats(metrics: Metrics) -> SocketAddr {
	use hyper::service::{make_service_fn, service_fn};

	let addr = SocketAddr::from(([127, 0, 0, 1], 0));
	let (stop_handle, server_handle) = stop_channel();
	let stop_handle2 = stop_handle.clone();

	// And a MakeService to handle each connection...
	let make_service = make_service_fn(move |_conn: &AddrStream| {
		let stop_handle = stop_handle2.clone();
		let metrics = metrics.clone();

		async move {
			Ok::<_, Box<dyn StdError + Send + Sync>>(service_fn(move |req| {
				let is_websocket = crate::ws::is_upgrade_request(&req);
				let metrics = metrics.clone();
				let stop_handle = stop_handle.clone();

				let mut svc =
					Server::builder().max_connections(33).to_service_builder().build(Methods::new(), stop_handle);

				if is_websocket {
					// This should work for each callback.
					let session_close1 = svc.on_session_closed();
					let session_close2 = svc.on_session_closed();

					tokio::spawn(async move {
						metrics.ws_sessions_opened.fetch_add(1, Ordering::SeqCst);
						tokio::join!(session_close2, session_close1);
						metrics.ws_sessions_closed.fetch_add(1, Ordering::SeqCst);
					});

					async move { svc.call(req).await }.boxed()
				} else {
					// HTTP.
					async move { svc.call(req).await }.boxed()
				}
			}))
		}
	});

	let server = hyper::Server::bind(&addr).serve(make_service);

	let addr = server.local_addr();

	tokio::spawn(async move {
		let graceful = server.with_graceful_shutdown(async move { stop_handle.shutdown().await });
		graceful.await.unwrap();
		drop(server_handle)
	});

	addr
}
