//! Example of using proc macro to generate working client and server.

use std::net::SocketAddr;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::params::ArrayParams;
use jsonrpsee::core::{async_trait, RpcResult, SubscriptionResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::SubscriptionMessage;
use jsonrpsee::types::ErrorObject;
use jsonrpsee::ws_client::*;
use jsonrpsee::{rpc_params, Extensions, PendingSubscriptionSink};

#[rpc(client, server, namespace = "foo")]
pub trait Rpc {
	#[method(name = "foo", aliases = ["fooAlias", "Other"])]
	async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name = "optional_params")]
	async fn optional_params(&self, a: std::option::Option<u8>, b: String) -> RpcResult<bool>;

	#[method(name = "optional_param")]
	async fn optional_param(&self, a: Option<u8>) -> RpcResult<bool>;

	#[method(name = "array_params")]
	async fn array_params(&self, items: Vec<u64>) -> RpcResult<u64>;

	#[method(name = "rename_params", param_kind= map)]
	async fn rename_params(
		&self,
		#[argument(rename = "type")] r#type: u16,
		#[argument(rename = "halfType")] ignored_name: bool,
	) -> RpcResult<u16>;

	#[method(name = "async_conn_id", with_extensions)]
	async fn conn_id(&self) -> RpcResult<u32>;

	#[method(name = "bar")]
	fn sync_method(&self) -> RpcResult<u16>;

	#[method(name = "sync_conn_id", with_extensions)]
	fn sync_conn_id(&self) -> RpcResult<u32>;

	#[subscription(name = "subscribe", item = String)]
	async fn sub(&self) -> SubscriptionResult;

	#[subscription(name = "subscribe_conn_id", item = u32, with_extensions)]
	async fn sub_with_conn_id(&self) -> SubscriptionResult;

	#[subscription(name = "echo", unsubscribe = "unsubscribeEcho", aliases = ["ECHO"], item = u32, unsubscribe_aliases = ["NotInterested", "listenNoMore"])]
	async fn sub_with_params(&self, val: u32) -> SubscriptionResult;

	// This will send data to subscribers with the `method` field in the JSON payload set to `foo_subscribe_override`
	// because it's in the `foo` namespace.
	#[subscription(name = "subscribe_method" => "subscribe_override", item = u32)]
	async fn sub_with_override_notif_method(&self) -> SubscriptionResult;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn async_method(&self, _param_a: u8, _param_b: String) -> RpcResult<u16> {
		Ok(42u16)
	}

	async fn optional_params(&self, a: core::option::Option<u8>, _b: String) -> RpcResult<bool> {
		let res = if a.is_some() { true } else { false };
		Ok(res)
	}

	async fn optional_param(&self, a: Option<u8>) -> RpcResult<bool> {
		let res = if a.is_some() { true } else { false };
		Ok(res)
	}

	async fn array_params(&self, items: Vec<u64>) -> RpcResult<u64> {
		Ok(items.len() as u64)
	}

	async fn rename_params(&self, r#type: u16, half_type: bool) -> RpcResult<u16> {
		Ok(half_type.then(|| r#type / 2).unwrap_or(r#type))
	}

	async fn conn_id(&self, ext: &jsonrpsee::Extensions) -> RpcResult<u32> {
		ext.get::<u32>().cloned().ok_or_else(|| ErrorObject::owned(0, "No connection details found", None::<()>))
	}

	fn sync_conn_id(&self, ext: &jsonrpsee::Extensions) -> RpcResult<u32> {
		ext.get::<u32>().cloned().ok_or_else(|| ErrorObject::owned(0, "No connection details found", None::<()>))
	}

	fn sync_method(&self) -> RpcResult<u16> {
		Ok(10u16)
	}

	async fn sub(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
		let sink = pending.accept().await?;

		sink.send("Response_A".into()).await?;
		sink.send("Response_B".into()).await?;

		Ok(())
	}

	async fn sub_with_params(&self, pending: PendingSubscriptionSink, val: u32) -> SubscriptionResult {
		let sink = pending.accept().await?;
		let msg = SubscriptionMessage::from_json(&val)?;

		sink.send(msg.clone()).await?;
		sink.send(msg).await?;

		Ok(())
	}

	async fn sub_with_override_notif_method(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
		let sink = pending.accept().await?;

		let msg = SubscriptionMessage::from_json(&1)?;
		sink.send(msg).await?;

		Ok(())
	}

	async fn sub_with_conn_id(&self, pending: PendingSubscriptionSink, ext: &Extensions) -> SubscriptionResult {
		let sink = pending.accept().await?;
		let conn_id = ext
			.get::<u32>()
			.cloned()
			.ok_or_else(|| ErrorObject::owned(0, "No connection details found", None::<()>))?;
		sink.send(SubscriptionMessage::from_json(&conn_id).unwrap()).await?;
		Ok(())
	}
}

pub async fn server() -> SocketAddr {
	use hyper_util::rt::{TokioExecutor, TokioIo};
	use jsonrpsee::server::middleware::rpc::RpcServiceT;
	use jsonrpsee::server::{stop_channel, RpcServiceBuilder};
	use std::convert::Infallible;
	use std::sync::{atomic::AtomicU32, Arc};
	use tower::Service;

	#[derive(Debug, Clone)]
	struct ConnectionDetails<S> {
		inner: S,
		connection_id: u32,
	}

	impl<'a, S> RpcServiceT<'a> for ConnectionDetails<S>
	where
		S: RpcServiceT<'a>,
	{
		type Future = S::Future;

		fn call(&self, mut request: jsonrpsee::types::Request<'a>) -> Self::Future {
			request.extensions_mut().insert(self.connection_id);
			self.inner.call(request)
		}
	}

	let listener = tokio::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).await.unwrap();
	let addr = listener.local_addr().unwrap();

	let (stop_hdl, server_hdl) = stop_channel();

	tokio::spawn(async move {
		let conn_id = Arc::new(AtomicU32::new(0));
		let svc_builder = jsonrpsee::server::Server::builder().to_service_builder();
		let methods = RpcServerImpl.into_rpc();

		loop {
			let stream = tokio::select! {
				res = listener.accept() => {
					match res {
						Ok((stream, _remote_addr)) => stream,
						Err(e) => {
							eprintln!("failed to accept ipv4 connection: {:?}", e);
							continue;
						}
					}
				}
				_ = stop_hdl.clone().shutdown() => break,
			};

			let methods2 = methods.clone();
			let stop_hdl2 = stop_hdl.clone();
			let svc_builder2 = svc_builder.clone();
			let conn_id2 = conn_id.clone();
			let svc = hyper::service::service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
				let connection_id = conn_id2.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
				let rpc_middleware = RpcServiceBuilder::default()
					.layer_fn(move |service| ConnectionDetails { inner: service, connection_id });

				// Start a new service with our own connection ID.
				let mut tower_service = svc_builder2
					.clone()
					.set_rpc_middleware(rpc_middleware)
					.connection_id(connection_id)
					.build(methods2.clone(), stop_hdl2.clone());

				async move {
					let rp = tower_service.call(req).await.unwrap();
					Ok::<_, Infallible>(rp)
				}
			});

			tokio::spawn(async move {
				let builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());
				let conn = builder.serve_connection_with_upgrades(TokioIo::new(stream), svc);

				if let Err(err) = conn.await {
					eprintln!("HTTP connection failed: {:?}", err);
				}
			});
		}
	});

	tokio::spawn(server_hdl.stopped());

	addr
}

#[tokio::main]
async fn main() {
	let server_addr = server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	assert_eq!(client.async_method(10, "a".into()).await.unwrap(), 42);

	// The default param kind is `map` so test that handles renames correctly
	// both in the client and server.
	assert_eq!(client.rename_params(256, true).await.unwrap(), 128);
	assert_eq!(client.rename_params(256, false).await.unwrap(), 256);

	// Make sure that renames has no impact of ArrayParams.
	assert_eq!(client.request::<u16, ArrayParams>("foo_rename_params", rpc_params![256, true]).await.unwrap(), 128);
	assert_eq!(client.request::<u16, ArrayParams>("foo_rename_params", rpc_params![256, false]).await.unwrap(), 256);

	assert_eq!(client.sync_method().await.unwrap(), 10);
	assert_eq!(client.optional_params(None, "a".into()).await.unwrap(), false);
	assert_eq!(client.optional_params(Some(1), "a".into()).await.unwrap(), true);

	assert_eq!(client.array_params(vec![1]).await.unwrap(), 1);
	assert_eq!(
		client.request::<u64, ArrayParams>("foo_array_params", rpc_params![Vec::<u64>::new()]).await.unwrap(),
		0
	);

	assert_eq!(client.request::<bool, ArrayParams>("foo_optional_param", rpc_params![]).await.unwrap(), false);
	assert_eq!(client.request::<bool, ArrayParams>("foo_optional_param", rpc_params![1]).await.unwrap(), true);

	let mut sub = client.sub().await.unwrap();
	let first_recv = sub.next().await.transpose().unwrap();
	assert_eq!(first_recv, Some("Response_A".to_string()));
	let second_recv = sub.next().await.transpose().unwrap();
	assert_eq!(second_recv, Some("Response_B".to_string()));

	let mut sub = client.sub_with_override_notif_method().await.unwrap();
	let recv = sub.next().await.transpose().unwrap();
	assert_eq!(recv, Some(1));

	assert!(client.conn_id().await.is_ok());
	assert!(client.sync_conn_id().await.is_ok());

	let mut sub = client.sub_with_conn_id().await.unwrap();
	assert!(matches!(sub.next().await, Some(Ok(_))));
}
