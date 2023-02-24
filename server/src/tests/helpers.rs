use std::{fmt, net::SocketAddr};

use crate::{types::error::CallError, RpcModule, ServerBuilder, ServerHandle};

use anyhow::anyhow;
use jsonrpsee_core::{DeserializeOwned, Error};
use jsonrpsee_test_utils::{mocks::TestContext, TimeoutFutureExt};
use jsonrpsee_types::Response;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

/// Spawns a dummy JSON-RPC server.
pub(crate) async fn server() -> SocketAddr {
	let (addr, handle) = server_with_handles().await;
	tokio::spawn(handle.stopped());
	addr
}

/// Spawns a dummy JSON-RPC server.
///
/// It has the following methods:
///     sync methods: `say_hello` and `add`
///     async: `say_hello_async` and `add_sync`
///     other: `invalid_params` (always returns `CallError::InvalidParams`),
///            `call_fail` (always returns `CallError::Failed`),
///            `sleep_for`
///            `subscribe_hello` (starts a subscription that doesn't send anything)
///
/// Returns the address together with handle for the server.
pub(crate) async fn server_with_handles() -> (SocketAddr, ServerHandle) {
	let server = ServerBuilder::default()
		.build("127.0.0.1:0")
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	let ctx = TestContext;
	let mut module = RpcModule::new(ctx);
	module
		.register_method("say_hello", |_, _| {
			tracing::debug!("server respond to hello");
			Ok("hello")
		})
		.unwrap();
	module
		.register_method("add", |params, _| {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	module
		.register_method("multiparam", |params, _| {
			let params: (String, String, Vec<u8>) = params.parse()?;
			let r = format!("string1={}, string2={}, vec={}", params.0.len(), params.1.len(), params.2.len());
			Ok(r)
		})
		.unwrap();
	module
		.register_async_method("say_hello_async", |_, _| {
			async move {
				tracing::debug!("server respond to hello");
				// Call some async function inside.
				futures_util::future::ready(()).await;
				Result::<_, Error>::Ok("hello")
			}
		})
		.unwrap();
	module
		.register_async_method("add_async", |params, _| async move {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Result::<_, Error>::Ok(sum)
		})
		.unwrap();
	module
		.register_method("invalid_params", |_params, _| Err::<(), _>(CallError::InvalidParams(anyhow!("buh!")).into()))
		.unwrap();
	module
		.register_method("call_fail", |_params, _| Err::<(), _>(Error::to_call_error(MyAppError)))
		.unwrap();
	module
		.register_method("sleep_for", |params, _| {
			let sleep: Vec<u64> = params.parse()?;
			std::thread::sleep(std::time::Duration::from_millis(sleep[0]));
			Ok("Yawn!")
		})
		.unwrap();
	module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, pending, _| async move {
			let sink = pending.accept().await?;

			loop {
				let _ = &sink;
				tokio::time::sleep(std::time::Duration::from_secs(30)).await;
			}
		})
		.unwrap();

	module.register_method("notif", |_, _| Ok("")).unwrap();
	module
		.register_method("should_err", |_, ctx| {
			ctx.err().map_err(CallError::Failed)?;
			Ok("err")
		})
		.unwrap();

	module
		.register_method("should_ok", |_, ctx| {
			ctx.ok().map_err(CallError::Failed)?;
			Ok("ok")
		})
		.unwrap();
	module
		.register_async_method("should_ok_async", |_p, ctx| async move {
			ctx.ok().map_err(CallError::Failed)?;
			Result::<_, Error>::Ok("ok")
		})
		.unwrap();

	let addr = server.local_addr().unwrap();

	let server_handle = server.start(module).unwrap();
	(addr, server_handle)
}

/// Run server with user provided context.
pub(crate) async fn server_with_context() -> SocketAddr {
	let server = ServerBuilder::default()
		.build("127.0.0.1:0")
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();

	let ctx = TestContext;
	let mut rpc_module = RpcModule::new(ctx);

	rpc_module
		.register_method("should_err", |_p, ctx| {
			ctx.err().map_err(CallError::Failed)?;
			Ok("err")
		})
		.unwrap();

	rpc_module
		.register_method("should_ok", |_p, ctx| {
			ctx.ok().map_err(CallError::Failed)?;
			Ok("ok")
		})
		.unwrap();

	rpc_module
		.register_async_method("should_ok_async", |_p, ctx| async move {
			ctx.ok().map_err(CallError::Failed)?;
			// Call some async function inside.
			Result::<_, Error>::Ok(futures_util::future::ready("ok!").await)
		})
		.unwrap();

	rpc_module
		.register_async_method("err_async", |_p, ctx| async move {
			ctx.ok().map_err(CallError::Failed)?;
			// Async work that returns an error
			futures_util::future::err::<(), Error>(anyhow!("nah").into()).await
		})
		.unwrap();

	let addr = server.local_addr().unwrap();
	let handle = server.start(rpc_module).unwrap();

	tokio::spawn(handle.stopped());
	addr
}

pub(crate) fn init_logger() {
	let _ = FmtSubscriber::builder()
		.with_env_filter(EnvFilter::from_default_env())
		.try_init();
}

pub(crate) fn deser_call<T: DeserializeOwned>(raw: String) -> T {
	let out: Response<T> = serde_json::from_str(&raw).unwrap();
	out.result
}

/// Applications can/should provide their own error.
#[derive(Debug)]
struct MyAppError;
impl fmt::Display for MyAppError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "MyAppError")
	}
}
impl std::error::Error for MyAppError {}
