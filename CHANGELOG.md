# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

## [v0.26.0] - 2025-08-11

This is just a small release; the only breaking change is the addition of `max_frame_size` to `WsTransportClientBuilder`, which necessitates a minor version bump.

The other changes are as follows:

### [Changed]

- Fix new Rust 1.89 lifetime warnings and impl ToRpcParams on serde_json::Map ([#1594](https://github.com/paritytech/jsonrpsee/pull/1594))
- feat(keepalive): expose tcp keep-alive options ([#1583](https://github.com/paritytech/jsonrpsee/pull/1583))
- chore: expose `TowerServiceNoHttp` type ([#1588](https://github.com/paritytech/jsonrpsee/pull/1588))
- chore(deps): update socket2 requirement from 0.5.1 to 0.6.0 ([#1587](https://github.com/paritytech/jsonrpsee/pull/1587))
- Allow max websocket frame size to be set ([#1585](https://github.com/paritytech/jsonrpsee/pull/1585))
- chore(deps): update pprof requirement from 0.14 to 0.15 ([#1577](https://github.com/paritytech/jsonrpsee/pull/1577))
- Expose `jsonrpsee_http_client::RpcService` ([#1574](https://github.com/paritytech/jsonrpsee/pull/1574))

### [Fixed]

- fix: Remove username and password from URL after building Authorization header ([#1581](https://github.com/paritytech/jsonrpsee/pull/1581))

## [v0.25.1] - 2025-04-24

A small follow-up patch release that adds a `Clone impl` for the middleware RpcLogger which was missing
and broke the Clone impl for the HttpClient.

## [v0.25.0] - 2025-04-24

A new breaking release which has been in the making for a while and the biggest change is that the
`RpcServiceT trait` has been changed to support both the client and server side:

```rust
pub trait RpcServiceT {
	/// Response type for `RpcServiceT::call`.
	type MethodResponse;
	/// Response type for `RpcServiceT::notification`.
	type NotificationResponse;
	/// Response type for `RpcServiceT::batch`.
	type BatchResponse;

	/// Processes a single JSON-RPC call, which may be a subscription or regular call.
	fn call<'a>(&self, request: Request<'a>) -> impl Future<Output = Self::MethodResponse> + Send + 'a;

	/// Processes multiple JSON-RPC calls at once, similar to `RpcServiceT::call`.
	///
	/// This method wraps `RpcServiceT::call` and `RpcServiceT::notification`,
	/// but the root RPC service does not inherently recognize custom implementations
	/// of these methods.
	///
	/// As a result, if you have custom logic for individual calls or notifications,
	/// you must duplicate that implementation in this method or no middleware will be applied
	/// for calls inside the batch.
	fn batch<'a>(&self, requests: Batch<'a>) -> impl Future<Output = Self::BatchResponse> + Send + 'a;

	/// Similar to `RpcServiceT::call` but processes a JSON-RPC notification.
	fn notification<'a>(&self, n: Notification<'a>) -> impl Future<Output = Self::NotificationResponse> + Send + 'a;
}
```

The reason for this change is to make it work for the client-side as well as make it easier to
implement performantly by relying on `impl Future` instead of requiring an associated type for the `Future` (which in many cases requires boxing).

The downside of this change is that one has to duplicate the logic in the `batch` and `call` method to achieve the same
functionality as before. Thus, `call` or `notification` is not being invoked in the `batch` method and one has to implement 
them separately.
For example now it's possible to write middleware that counts the number of method calls as follows (both client and server):

```rust
#[derive(Clone)]
pub struct Counter<S> {
	service: S,
	count: Arc<AtomicUsize>,
	role: &'static str,
}

impl<S> RpcServiceT for Counter<S>
where
	S: RpcServiceT + Send + Sync + Clone + 'static,
{
	type MethodResponse = S::MethodResponse;
	type NotificationResponse = S::NotificationResponse;
	type BatchResponse = S::BatchResponse;

	fn call<'a>(&self, req: Request<'a>) -> impl Future<Output = Self::MethodResponse> + Send + 'a {
		let count = self.count.clone();
		let service = self.service.clone();
		let role = self.role;

		async move {
			let rp = service.call(req).await;
			count.fetch_add(1, Ordering::SeqCst);
			println!("{role} processed calls={} on the connection", count.load(Ordering::SeqCst));
			rp
		}
	}

	fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Self::BatchResponse> + Send + 'a {
		let len = batch.len();
		self.count.fetch_add(len, Ordering::SeqCst);
		println!("{} processed calls={} on the connection", self.role, self.count.load(Ordering::SeqCst));
		self.service.batch(batch)
	}

	fn notification<'a>(&self, n: Notification<'a>) -> impl Future<Output = Self::NotificationResponse> + Send + 'a {
		self.service.notification(n)
	}
}
```

In addition because this middleware is quite powerful it's possible to
modify requests and specifically the request ID which should be avoided
because it may break the response verification especially for the client-side.
See https://github.com/paritytech/jsonrpsee/issues/1565 for further information.

There are also a couple of other changes see the detailed changelog below.

### [Added]
- middleware: RpcServiceT distinct return types for notif, batch, call ([#1564](https://github.com/paritytech/jsonrpsee/pull/1564))
- middleware: add support for client-side ([#1521](https://github.com/paritytech/jsonrpsee/pull/1521))
- feat: add namespace_separator option for RPC methods ([#1544](https://github.com/paritytech/jsonrpsee/pull/1544))
- feat: impl Into<ErrorObject> for Infallible ([#1542](https://github.com/paritytech/jsonrpsee/pull/1542))
- client: add `request timeout` getter ([#1533](https://github.com/paritytech/jsonrpsee/pull/1533))
- server: add example how to close a connection from a rpc handler (method call or subscription) ([#1488](https://github.com/paritytech/jsonrpsee/pull/1488))
- server: add missing `ServerConfigBuilder::build` ([#1484](https://github.com/paritytech/jsonrpsee/pull/1484))

### [Fixed]
- chore(macros): fix typo in proc-macro example ([#1482](https://github.com/paritytech/jsonrpsee/pull/1482))
- chore(macros): fix typo in internal type name ([#1507](https://github.com/paritytech/jsonrpsee/pull/1507))
- http middleware: preserve the URI query in ProxyGetRequest::call ([#1512](https://github.com/paritytech/jsonrpsee/pull/1512))
- http middlware: send original error in ProxyGetRequest ([#1516](https://github.com/paritytech/jsonrpsee/pull/1516))
- docs: update comment for TOO_BIG_BATCH_RESPONSE_CODE error ([#1531](https://github.com/paritytech/jsonrpsee/pull/1531))
- fix `http request body` log ([#1540](https://github.com/paritytech/jsonrpsee/pull/1540))

### [Changed]
- unify usage of JSON via `Box<RawValue>` ([#1545](https://github.com/paritytech/jsonrpsee/pull/1545))
- server: `ServerConfigBuilder/ServerConfig` replaces `ServerBuilder` duplicate setter methods ([#1487](https://github.com/paritytech/jsonrpsee/pull/1487))
- server: make `ProxyGetRequestLayer` http middleware support multiple path-method pairs ([#1492](https://github.com/paritytech/jsonrpsee/pull/1492))
- server: propagate extensions in http response ([#1514](https://github.com/paritytech/jsonrpsee/pull/1514))
- server: add assert set_message_buffer_capacity ([#1530](https://github.com/paritytech/jsonrpsee/pull/1530))
- client: add #[derive(Clone)] for HttpClientBuilder ([#1498](https://github.com/paritytech/jsonrpsee/pull/1498))
- client: add Error::Closed for ws close ([#1497](https://github.com/paritytech/jsonrpsee/pull/1497))
- client: use native async fn in traits instead async_trait crate ([#1551](https://github.com/paritytech/jsonrpsee/pull/1551))
- refactor: move to rust edition 2024 (MSRV 1.85) ([#1528](https://github.com/paritytech/jsonrpsee/pull/1528))
- chore(deps): update tower requirement from 0.4.13 to 0.5.1 ([#1455](https://github.com/paritytech/jsonrpsee/pull/1455))
- chore(deps): update tower-http requirement from 0.5.2 to 0.6.1 ([#1463](https://github.com/paritytech/jsonrpsee/pull/1463))
- chore(deps): update pprof requirement from 0.13 to 0.14 ([#1493](https://github.com/paritytech/jsonrpsee/pull/1493))
- chore(deps): update rustls-platform-verifier requirement from 0.3 to 0.4 ([#1489](https://github.com/paritytech/jsonrpsee/pull/1489))
- chore(deps): update thiserror requirement from 1 to 2 ([#1491](https://github.com/paritytech/jsonrpsee/pull/1491))
- chore(deps): bump soketto to 0.8.1 ([#1501](https://github.com/paritytech/jsonrpsee/pull/1501))
- chore(deps): update rustls-platform-verifier requirement from 0.4 to 0.5 ([#1506](https://github.com/paritytech/jsonrpsee/pull/1506))
- chore(deps): update fast-socks5 requirement from 0.9.1 to 0.10.0 ([#1505](https://github.com/paritytech/jsonrpsee/pull/1505))
- chore(deps): tokio ^1.42 ([#1511](https://github.com/paritytech/jsonrpsee/pull/1511))
- chore: use cargo workspace dependencies ([#1502](https://github.com/paritytech/jsonrpsee/pull/1502))
- chore(deps): update rand requirement from 0.8 to 0.9 ([#1523](https://github.com/paritytech/jsonrpsee/pull/1523))

## [v0.24.9] - 2024-03-17

This is a non-breaking release that updates the dependency `rust-platform-verifier` to v0.5 to fix that
that `rust-platform-verifier` v0.3 didn't enable the `std feature` in `rustls` which caused a compilation error.
See https://github.com/paritytech/jsonrpsee/issues/1536 for further information.

Thanks to the external contributor [@prestwich](https://github.com/prestwich) who spotted and fixed this issue.

## [v0.24.8] - 2024-01-24

This is a non-breaking release that decreases the MSRV to 1.74.0.

### [Changed]
- reduce MSRV to 1.74.0 ([#1519](https://github.com/paritytech/jsonrpsee/pull/1519))

## [v0.24.7] - 2024-10-16

This is a patch release that mainly fixes the tower::Service implementation to be generic over the HttpBody to work with all middleware layers.
For instance, this makes `tower_http::compression::CompressionLayer` work, which didn't compile before.

### [Added]
- http client: add `max_concurrent_requests` ([#1473](https://github.com/paritytech/jsonrpsee/pull/1473))

### [Fixed]
- fix(server): make tower::Service impl generic over HttpBody ([#1475](https://github.com/paritytech/jsonrpsee/pull/1475))

Thanks to the external contributor [@hanabi1224](https://github.com/hanabi1224) who contributed to this release.

## [v0.24.6] - 2024-10-07

This is a bug-fix release that fixes that the `ConnectionGuard` was dropped before the future was resolved which,
could lead to that HTTP calls were not counted correctly in the `ConnectionGuard`. This impacts only the server.

### [Fixed]
- fix(server): count http calls in connection guard ([#1468](https://github.com/paritytech/jsonrpsee/pull/1468))

## [v0.24.5] - 2024-09-26

This is a patch release that mainly fixes a compilation issue for the server because the feature `tower/util` was not enabled.

### [Fixed]
- server: Enable tower util feature ([#1464](https://github.com/paritytech/jsonrpsee/pull/1464))

### [Changed]
- server: change `http method_not_allowed` message ([#1452](https://github.com/paritytech/jsonrpsee/pull/1452))

## [v0.24.4] - 2024-09-11

This is non-breaking release that changes the error variants to be `thiserror(transparent)` for wrapped errors and adds ConnectionGuard to
the extensions to make it possible to get the number of active connections.

### [Added]
- server: expose ConnectionGuard as request extension ([#1443](https://github.com/paritytech/jsonrpsee/pull/1443))

### [Fixed]
- types: use error(transparent) for wrapped errors when possible ([#1449](https://github.com/paritytech/jsonrpsee/pull/1449))

## [v0.24.3] - 2024-08-14

This is a small release that adds two new APIs to inject data via the extensions to the `RpcModule/Methods`
and it only impacts users that are using RpcModule directly via `Methods::call/subscribe/raw_json_request` (e.g., unit testing) and not the server itself.

### [Added]
- feat(server): add `Methods::extensions/extensions_mut` ([#1440](https://github.com/paritytech/jsonrpsee/pull/1440))

## [v0.24.2] - 2024-08-02

Another small release that fixes:
- Notifications without params were not handled correctly in the client, which been has been fixed.
- Improve compile times and reduce code-generation in the proc macro crate.

### [Fixed]
- client: parse notification without params ([#1436](https://github.com/paritytech/jsonrpsee/pull/1436))
- proc macros: remove direct tracing calls ([#1405](https://github.com/paritytech/jsonrpsee/pull/1405))

Thanks to the external contributor [@DaniPopes](https://github.com/DaniPopes) who contributed to this release.

## [v0.24.1] - 2024-07-30

This is a small release that forces jsonrpsee `rustls` to use the crypto backend ring which may panic if both `ring` and `aws-lc` features are enabled.
See https://github.com/rustls/rustls/issues/1877 for further information.

This has no impact on the default configuration of jsonrpsee which was already using `ring` as the default.

### [Changed]
- chore(deps): update gloo-net requirement from 0.5.0 to 0.6.0 ([#1428](https://github.com/paritytech/jsonrpsee/pull/1428))

### [Fixed]
- fix: Explicitly set rustls provider before using rustls ([#1424](https://github.com/paritytech/jsonrpsee/pull/1424))

## [v0.24.0] - 2024-07-05

A breaking release that mainly changes:

1. `tls` feature for the client has been divided into `tls` and `tls-platform-verifier` where the `tls` feature
will only include `rustls` and no specific certificate store but the default one is still `tls-rustls-platform-verifier`.
This is useful if one wants to avoid bring on openssl dependencies.
2. Remove dependencies `anyhow` and `beef` from the codebase.

### [Changed]
- types: serialize `id` in `Response` before `result`/`error` fields ([#1421](https://github.com/paritytech/jsonrpsee/pull/1421))
- refactor(client+transport)!: split `tls` into `tls` and `tls-rustls-platform-verifier` features ([#1419](https://github.com/paritytech/jsonrpsee/pull/1419))
- chore(deps): update rustc-hash requirement from 1 to 2 ([#1410](https://github.com/paritytech/jsonrpsee/pull/1410))
- deps: remove anyhow ([#1402](https://github.com/paritytech/jsonrpsee/pull/1402))
- deps: remove beef ([#1401](https://github.com/paritytech/jsonrpsee/pull/1401))

## [v0.23.2] - 2024-06-26

This a small patch release that fixes a couple of bugs and adds a couple of new APIs.

The bug fixes are:
- The `server::ws::on_connect` was not working properly due to a merge nit when upgrading to hyper v1.0 
  This impacts only users that are using the low-level API and not the server itself.
- `WsTransport::build_with_stream` shouldn't not resolve the socket addresses and it's fixed now, [see #1411 for further info](https://github.com/paritytech/jsonrpsee/issues/1411). 
  This impacts users that are inject their own TcpStream directly into the `WsTransport`.

### [Added]
- server: add `RpcModule::remove` ([#1416](https://github.com/paritytech/jsonrpsee/pull/1416))
- server: add `capacity and max_capacity` to the subscription API ([#1414](https://github.com/paritytech/jsonrpsee/pull/1414))
- server: add `PendingSubscriptionSink::method_name` ([#1413](https://github.com/paritytech/jsonrpsee/pull/1413))

### [Fixed]
- server: make `ws::on_connect` work again ([#1418](https://github.com/paritytech/jsonrpsee/pull/1418))
- client: `WsTransport::build_with_stream` don't resolve sockaddrs ([#1412](https://github.com/paritytech/jsonrpsee/pull/1412))

## [v0.23.1] - 2024-06-10

This is a patch release that injects the ConnectionId in
the extensions when using a RpcModule without a server. This impacts
users that are using RpcModule directly (e.g., unit testing) and not the
server itself.

### [Changed]
- types: remove anyhow dependency ([#1398](https://github.com/paritytech/jsonrpsee/pull/1398))

### [Fixed]
- rpc module: inject ConnectionId in extensions ([#1399](https://github.com/paritytech/jsonrpsee/pull/1399))

## [v0.23.0] - 2024-06-07

This is a new breaking release, and let's go through the changes.

### hyper v1.0

jsonrpsee has been upgraded to use hyper v1.0 and this mainly impacts users that are using
the low-level API and rely on the `hyper::service::make_service_fn`
which has been removed, and from now on you need to manage the socket yourself.

The `hyper::service::make_service_fn` can be replaced by the following example template:

```rust
async fn start_server() {
  let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();

  loop {
    let sock = tokio::select! {
    res = listener.accept() => {
        match res {
          Ok((stream, _remote_addr)) => stream,
          Err(e) => {
            tracing::error!("failed to accept v4 connection: {:?}", e);
            continue;
          }
        }
      }
      _ = per_conn.stop_handle.clone().shutdown() => break,
    };

    let svc = tower::service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
      let mut jsonrpsee_svc = svc_builder
        .set_rpc_middleware(rpc_middleware)
        .build(methods, stop_handle);
      // https://github.com/rust-lang/rust/issues/102211 the error type can't be inferred
      // to be `Box<dyn std::error::Error + Send + Sync>` so we need to convert it to a concrete type
      // as workaround.
      jsonrpsee_svc
        .call(req)
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))
    });

    tokio::spawn(jsonrpsee::server::serve_with_graceful_shutdown(
      sock,
      svc,
      stop_handle.clone().shutdown(),
    ));
  }
}
```

Also, be aware that `tower::service_fn` and `hyper::service::service_fn` are different and it's recommended to use `tower::service_fn` from now.

### Extensions

Because it was not possible/easy to share state between RPC middleware layers
jsonrpsee has added `Extensions` to the Request and Response.
To allow users to inject arbitrary data that can be accessed in the RPC middleware
and RPC handlers.

Please be careful when injecting large amounts of data into the extensions because
It's cloned for each RPC call, which can increase memory usage significantly.

The connection ID from the jsonrpsee-server is injected in the extensions by default.
and it is possible to fetch it as follows:

```rust
struct LogConnectionId<S>(S);

impl<'a, S: RpcServiceT<'a>> RpcServiceT<'a> for LogConnectionId<S> {
	type Future = S::Future;

	fn call(&self, request: jsonrpsee::types::Request<'a>) -> Self::Future {
		let conn_id = request.extensions().get::<ConnectionId>().unwrap();
		tracing::info!("Connection ID {}", conn_id.0);

		self.0.call(request)
	}
}
```

In addition the `Extensions` is not added in the proc-macro API by default and
one has to enable `with_extensions` attr for that to be available:

```rust
#[rpc(client, server)]
pub trait Rpc {
	// legacy
	#[method(name = "foo"])
	async fn async_method(&self) -> u16>;

	// with extensions
	#[method(name = "with_ext", with_extensions)]
	async fn f(&self) -> bool;
}

impl RpcServer for () {
	async fn async_method(&self) -> u16 {
		12
	}

	// NOTE: ext is injected just after self in the API
	async fn f(&self, ext: &Extensions: b: String) -> {
		ext.get::<u32>().is_ok()
	}
}
```

### client - TLS certificate store changed

The default TLS certificate store has been changed to
`rustls-platform-verifier` to decide the best certificate
store for each platform.

In addition it's now possible to inject a custom certificate store
if one wants need some special certificate store.

### client - Subscription API modified

The subscription API has been modified:
- The error type has been changed to `serde_json::Error`
  to indicate that error can only occur if the decoding of T fails.
- It has been some confusion when the subscription is closed which can occur if the client "lagged" or the connection is closed.
  Now it's possible to call `Subscription::close_reason` after the subscription closed (i.e. has return None) to know why.

If one wants to replace old messages in case of lagging it is recommended to write your own adaptor on top of the subscription:

```rust
fn drop_oldest_when_lagging<T: Clone + DeserializeOwned + Send + Sync + 'static>(
    mut sub: Subscription<T>,
    buffer_size: usize,
) -> impl Stream<Item = Result<T, BroadcastStreamRecvError>> {
    let (tx, rx) = tokio::sync::broadcast::channel(buffer_size);

    tokio::spawn(async move {
        // Poll the subscription which ignores errors
        while let Some(n) = sub.next().await {
            let msg = match n {
                Ok(msg) => msg,
                Err(e) => {
                    tracing::error!("Failed to decode the subscription message: {e}");
                    continue;
                }
            };

            // Only fails if the receiver has been dropped
            if tx.send(msg).is_err() {
                return;
            }
        }
    });

    BroadcastStream::new(rx)
}
```

### [Added]
- server: add `serve` and `serve_with_graceful_shutdown` helpers ([#1382](https://github.com/paritytech/jsonrpsee/pull/1382))
- server: pass `extensions` from http layer ([#1389](https://github.com/paritytech/jsonrpsee/pull/1389))
- macros: add macro attr `with_extensions` ([#1380](https://github.com/paritytech/jsonrpsee/pull/1380))
- server: inject connection id in extensions ([#1381](https://github.com/paritytech/jsonrpsee/pull/1381))
- feat: add `Extensions` to Request/MethodResponse ([#1306](https://github.com/paritytech/jsonrpsee/pull/1306))
- proc-macros: rename parameter names ([#1365](https://github.com/paritytech/jsonrpsee/pull/1365))
- client: add `Subscription::close_reason` ([#1320](https://github.com/paritytech/jsonrpsee/pull/1320))

### [Changed]
- chore(deps): tokio ^1.23.1 ([#1393](https://github.com/paritytech/jsonrpsee/pull/1393))
- server: use `ConnectionId` in subscription APIs ([#1392](https://github.com/paritytech/jsonrpsee/pull/1392))
- server: add logs when connection closed by `ws ping/pong` ([#1386](https://github.com/paritytech/jsonrpsee/pull/1386))
- client: set `authorization header` from the URL ([#1384](https://github.com/paritytech/jsonrpsee/pull/1384))
- client: use rustls-platform-verifier cert store ([#1373](https://github.com/paritytech/jsonrpsee/pull/1373))
- client: remove MaxSlots limit ([#1377](https://github.com/paritytech/jsonrpsee/pull/1377))
- upgrade to hyper v1.0 ([#1368](https://github.com/paritytech/jsonrpsee/pull/1368))

## [v0.22.5] - 2024-04-29

A small bug-fix release, see each commit below for further information.

### [Fixed]
- proc macros: collision between generated code name and proc macro API ([#1363](https://github.com/paritytech/jsonrpsee/pull/1363))
- proc-macros: `feature server-core` compiles without `feature server` ([#1360](https://github.com/paritytech/jsonrpsee/pull/1360))
- client: add check in `max_buffer_capacity_per_subscription` that the buffer size > ([#1358](https://github.com/paritytech/jsonrpsee/pull/1358))
- types: Response type ignore unknown fields ([#1353](https://github.com/paritytech/jsonrpsee/pull/1353))

## [v0.22.4] - 2024-04-08

Yet another rather small release that fixes a cancel-safety issue that
could cause an unexpected panic when reading disconnect reason from the background task.

Also this makes the API `Client::disconnect_reason` cancel-safe.

### [Added]
- client: support batched notifications ([#1327](https://github.com/paritytech/jsonrpsee/pull/1327))
- client: support batched subscription notifs ([#1332](https://github.com/paritytech/jsonrpsee/pull/1332))

### [Changed]
- client: downgrade logs from error/warn -> debug ([#1343](https://github.com/paritytech/jsonrpsee/pull/1343))

### [Fixed]
- Update MSRV to 1.74.1 in Cargo.toml ([#1338](https://github.com/paritytech/jsonrpsee/pull/1338))
- client: disconnect_reason/read_error is now cancel-safe ([#1347](https://github.com/paritytech/jsonrpsee/pull/1347))

## [v0.22.3] - 2024-03-20

Another small release that adds a new API for RpcModule if one already has the state in an `Arc`
and a couple of bug fixes.

### [Added]
- add `RpcModule::from_arc` ([#1324](https://github.com/paritytech/jsonrpsee/pull/1324))

### [Fixed]
- Revert "fix(server): return err on WS handshake err (#1288)" ([#1326](https://github.com/paritytech/jsonrpsee/pull/1326))
- export `AlreadyStoppedError` ([#1325](https://github.com/paritytech/jsonrpsee/pull/1325))

Thanks to the external contributors [@mattsse](https://github.com/mattsse) and [@aatifsyed](https://github.com/mattsse) who contributed to this release.

## [v0.22.2] - 2024-03-05

This is a small patch release that exposes the connection details in server method implementations without breaking changes.
We plan to extend this functionality in jsonrpsee v1.0, although this will necessitate a breaking change.

### [Added]
- server: Register raw method with connection ID ([#1297](https://github.com/paritytech/jsonrpsee/pull/1297))

### [Changed]
- Update Syn 1.0 -> 2.0 ([#1304](https://github.com/paritytech/jsonrpsee/pull/1304))

## [v0.22.1] - 2024-02-19

This is a small patch release that internally changes `AtomicU64` to `AtomicUsize`
to support more targets.

### [Fixed]
- fix(docs): part of proc-macro documentation not rendering correctly in IDE ([#1294](https://github.com/paritytech/jsonrpsee/pull/1294))
- fix(client): change to `AtomicU64` to `AtomicUsize` ([#1293](https://github.com/paritytech/jsonrpsee/pull/1293))
- fix(server): low-level API return err on WS handshake err ([#1288](https://github.com/paritytech/jsonrpsee/pull/1288))

## [v0.22.0] - 2024-02-07

Another breaking release where a new `ResponsePayload` type is introduced in order
to make it possible to determine whether a response has been processed.

Unfortunately, the `IntoResponse trait` was modified to enable that
and some minor changes were made to make more fields private to avoid further
breakage.

### Example of the async `ResponsePayload API`

```rust
#[rpc(server)]
pub trait Api {
	#[method(name = "x")]
	fn x(&self) -> ResponsePayload<'static, String>;
}

impl RpcServer for () {
	fn x(&self) -> ResponsePayload<'static, String> {
		let (rp, rp_done) = ResponsePayload::success("ehheeheh".to_string()).notify_on_completion();

		tokio::spawn(async move {
			if rp_done.await.is_ok() {
				do_task_that_depend_x();
			}
		});

		rp
	}
}
```

### Roadmap

We are getting closer to releasing jsonrpsee v1.0 and
the following work is planned:
- Native async traits
- Upgrade hyper to v1.0
- Better subscription API for the client.

Thanks to the external contributor [@dan-starkware](https://github.com/dan-starkware) who contributed to this release.

### [Added]
- feat(server): add `TowerService::on_session_close` ([#1284](https://github.com/paritytech/jsonrpsee/pull/1284))
- feat(server): async API when `Response` has been processed. ([#1281](https://github.com/paritytech/jsonrpsee/pull/1281))

### [Changed]
- client(error): make display impl less verbose ([#1283](https://github.com/paritytech/jsonrpsee/pull/1283))
- fix: allow application/json-rpc http content type ([#1277](https://github.com/paritytech/jsonrpsee/pull/1277))
- refactor(rpc_module): RpcModule::raw_json_request -> String ([#1287](https://github.com/paritytech/jsonrpsee/pull/1287))

## [v0.21.0] - 2023-12-13

This release contains big changes and let's go over the main ones:

### JSON-RPC specific middleware

After getting plenty of feedback regarding a JSON-RPC specific middleware,
this release introduces a composable "tower-like" middleware that applies per JSON-RPC method call.
The new middleware also replaces the old `RpcLogger` which may break some use-cases, such as if
JSON-RPC was made on a WebSocket or HTTP transport, but it's possible to implement that by
using `jsonrpsee as a tower service` or `the low-level server API`.

An example how write such middleware:

```rust
#[derive(Clone)]
pub struct ModifyRequestIf<S>(S);

impl<'a, S> RpcServiceT<'a> for ModifyRequestIf<S>
where
	S: Send + Sync + RpcServiceT<'a>,
{
	type Future = S::Future;

	fn call(&self, mut req: Request<'a>) -> Self::Future {
		// Example how to modify the params in the call.
		if req.method == "say_hello" {
			// It's a bit awkward to create new params in the request
			// but this shows how to do it.
			let raw_value = serde_json::value::to_raw_value("myparams").unwrap();
			req.params = Some(StdCow::Owned(raw_value));
		}
		// Re-direct all calls that isn't `say_hello` to `say_goodbye`
		else if req.method != "say_hello" {
			req.method = "say_goodbye".into();
		}

		self.0.call(req)
	}
}

async fn run_server() {
	// Construct our middleware and build the server.
	let rpc_middleware = RpcServiceBuilder::new().layer_fn(|service| ModifyRequestIf(service));
	let server = Server::builder().set_rpc_middleware(rpc_middleware).build("127.0.0.1:0").await.unwrap();

	// Start the server.
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| "lo").unwrap();
	module.register_method("say_goodbye", |_, _| "goodbye").unwrap();

	let handle = server.start(module);
	handle.stopped().await;
}
```

### jsonrpsee server as a tower service

For users who want to get full control of the HTTP request, it's now possible to utilize jsonrpsee as a tower service
[example here](./examples/examples/jsonrpsee_as_service.rs)

### jsonrpsee server low-level API

For users who want to get low-level access and for example to disconnect
misbehaving peers that is now possible as well [example here](./examples/examples/jsonrpsee_server_low_level_api.rs)

### Logging in the server

Logging of RPC calls has been disabled by default,
but it's possible to enable that with the RPC logger middleware or provide
your own middleware for that.

```rust
let rpc_middleware = RpcServiceBuilder::new().rpc_logger(1024);
let server = Server::builder().set_rpc_middleware(rpc_middleware).build("127.0.0.1:0").await?;
```

### WebSocket ping/pong API

The WebSocket ping/pong APIs have been refactored to be able
to disconnect inactive connections both by from the server and client-side.

Thanks to the external contributors [@oleonardolima](https://github.com/oleonardolima)
and [@venugopv](https://github.com/venugopv) who contributed to this release.

### [Changed]
- chore(deps): update tokio-rustls requirement from 0.24 to 0.25 ([#1256](https://github.com/paritytech/jsonrpsee/pull/1256))
- chore(deps): update gloo-net requirement from 0.4.0 to 0.5.0 ([#1260](https://github.com/paritytech/jsonrpsee/pull/1260))
- chore(deps): update async-lock requirement from 2.4 to 3.0 ([#1226](https://github.com/paritytech/jsonrpsee/pull/1226))
- chore(deps): update proc-macro-crate requirement from 1 to 2 ([#1211](https://github.com/paritytech/jsonrpsee/pull/1211))
- chore(deps): update console-subscriber requirement from 0.1.8 to 0.2.0 ([#1210](https://github.com/paritytech/jsonrpsee/pull/1210))
- refactor: split client and server errors ([#1122](https://github.com/paritytech/jsonrpsee/pull/1122))
- refactor(ws client): impl tokio:{AsyncRead, AsyncWrite} for EitherStream ([#1249](https://github.com/paritytech/jsonrpsee/pull/1249))
- refactor(http client): enable all http versions ([#1252](https://github.com/paritytech/jsonrpsee/pull/1252))
- refactor(server): change ws ping API ([#1248](https://github.com/paritytech/jsonrpsee/pull/1248))
- refactor(ws client): generic over data stream ([#1168](https://github.com/paritytech/jsonrpsee/pull/1168))
- refactor(client): unify ws ping/pong API with the server ([#1258](https://github.com/paritytech/jsonrpsee/pull/1258)
- refactor: set `tcp_nodelay == true` by default ([#1263])(https://github.com/paritytech/jsonrpsee/pull/1263)

### [Added]
- feat(client): add `disconnect_reason` API ([#1246](https://github.com/paritytech/jsonrpsee/pull/1246))
- feat(server): jsonrpsee as `service` and `low-level API for more fine-grained API to disconnect peers etc` ([#1224](https://github.com/paritytech/jsonrpsee/pull/1224))
- feat(server): JSON-RPC specific middleware ([#1215](https://github.com/paritytech/jsonrpsee/pull/1215))
- feat(middleware): add `HostFilterLayer::disable` ([#1213](https://github.com/paritytech/jsonrpsee/pull/1213))

### [Fixed]
- fix(host filtering): support hosts with multiple ports ([#1227](https://github.com/paritytech/jsonrpsee/pull/1227))

## [v0.20.3] - 2023-10-24

This release fixes a cancel-safety issue in the server's graceful shutdown which could lead to high CPU usage.

### [Fixed]
- server: graceful shutdown distinguish between stopped and conn closed ([#1220](https://github.com/paritytech/jsonrpsee/pull/1220))
- server: graceful shutdown fix cancel-safety issue ([#1218](https://github.com/paritytech/jsonrpsee/pull/1218))
- server: graceful shutdown check `Incoming::Closed` ([#1216](https://github.com/paritytech/jsonrpsee/pull/1216))

## [v0.20.2] - 2023-10-13

This release removes the bounded buffer check which was intended to provide
backpressure all the way down to the TCP layer but it didn't work well.

For subscriptions the backpressure will be handled by implementation itself
and just rely on that.

### [Changed]
- server: remove bounded channel check ([#1209](https://github.com/paritytech/jsonrpsee/pull/1209))

## [v0.20.1] - 2023-09-15

This release adds support for `synchronous subscriptions` and fixes a leak in WebSocket server
where FuturesUnordered was not getting polled until shutdown, so it was accumulating tasks forever.

### [Changed]
- client: downgrade log for unknown subscription to DEBUG ([#1185](https://github.com/paritytech/jsonrpsee/pull/1185))
- refactor(http client): use HTTP connector on http URLs ([#1187](https://github.com/paritytech/jsonrpsee/pull/1187))
- refactor(server): less alloc per method call ([#1188](https://github.com/paritytech/jsonrpsee/pull/1188))

### [Fixed]
- fix: remove needless clone in ws background task ([#1203](https://github.com/paritytech/jsonrpsee/pull/1203))
- async client: save latest Waker ([#1198](https://github.com/paritytech/jsonrpsee/pull/1198))
- chore(deps): bump actions/checkout from 3.6.0 to 4.0.0 ([#1197](https://github.com/paritytech/jsonrpsee/pull/1197))
- fix(server): fix leak in FuturesUnordered ([#1204](https://github.com/paritytech/jsonrpsee/pull/1204))

### [Added]
- feat(server): add sync subscription API `register_subscription_raw` ([#1182](https://github.com/paritytech/jsonrpsee/pull/1182))

## [v0.20.0] - 2023-08-11

Another breaking release where the major changes are:
- `host filtering` has been moved to tower middleware instead of the server API.
- the clients now supports default port number such `wss://my.server.com`
- the background task for the async client has been refactored to multiplex send and read operations.

Regarding host filtering prior to this release one had to do:

```rust
let acl = AllowHosts::Only(vec!["http://localhost:*".into(), "http://127.0.0.1:*".into()]);
let server = ServerBuilder::default().set_host_filtering(acl).build("127.0.0.1:0").await.unwrap();
```

After this release then one have to do:

```rust
let middleware = tower::ServiceBuilder::new().layer(HostFilterLayer::new(["example.com"]).unwrap());
let server = Server::builder().set_middleware(middleware).build("127.0.0.1:0".parse::<SocketAddr>()?).await?;
```

Thanks to the external contributors [@polachok](https://github.com/polachok), [@bobs4462](https://github.com/bobs4462) and [@aj3n](https://github.com/aj3n) that contributed to this release.

### [Added]
- feat(server): add `SubscriptionMessage::new` ([#1176](https://github.com/paritytech/jsonrpsee/pull/1176))
- feat(server): add `SubscriptionSink::connection_id` ([#1175](https://github.com/paritytech/jsonrpsee/pull/1175))
- feat(server): add `Params::get` ([#1173](https://github.com/paritytech/jsonrpsee/pull/1173))
- feat(server): add `PendingSubscriptionSink::connection_id` ([#1163](https://github.com/paritytech/jsonrpsee/pull/1163))

### [Fixed]
- fix(server): host filtering URI read authority ([#1178](https://github.com/paritytech/jsonrpsee/pull/1178))

### [Changed]
- refactor: make ErrorObject::borrowed accept `&str` ([#1160](https://github.com/paritytech/jsonrpsee/pull/1160))
- refactor(client): support default port number ([#1172](https://github.com/paritytech/jsonrpsee/pull/1172))
- refactor(server): server host filtering ([#1174](https://github.com/paritytech/jsonrpsee/pull/1174))
- refactor(client): refactor background task ([#1145](https://github.com/paritytech/jsonrpsee/pull/1145))
- refactor: use `RootCertStore::add_trust_anchors` ([#1165](https://github.com/paritytech/jsonrpsee/pull/1165))
- chore(deps): update criterion v0.5 and pprof 0.12 ([#1161](https://github.com/paritytech/jsonrpsee/pull/1161))
- chore(deps): update webpki-roots requirement from 0.24 to 0.25 ([#1158](https://github.com/paritytech/jsonrpsee/pull/1158))
- refactor(server): move host filtering to tower middleware ([#1179](https://github.com/paritytech/jsonrpsee/pull/1179))

## [v0.19.0] - 2023-07-20

### [Fixed]

- Fixed connections processing await on server shutdown ([#1153](https://github.com/paritytech/jsonrpsee/pull/1153))
- fix: include error code in RpcLogger ([#1135](https://github.com/paritytech/jsonrpsee/pull/1135))
- fix: downgrade more logs to `debug` ([#1127](https://github.com/paritytech/jsonrpsee/pull/1127))
- fix(server): remove `MethodSinkPermit` to fix backpressure issue on concurrent subscriptions ([#1126](https://github.com/paritytech/jsonrpsee/pull/1126))
- fix readme links ([#1152](https://github.com/paritytech/jsonrpsee/pull/1152))

### [Changed]

- server: downgrade connection logs to debug ([#1123](https://github.com/paritytech/jsonrpsee/pull/1123))
- refactor(server): make `Server::start` infallible and add `fn builder()` ([#1137](https://github.com/paritytech/jsonrpsee/pull/1137))

## [v0.18.2] - 2023-05-10

This release improves error message for `too big batch response` and exposes the `BatchRequestConfig type` in order to make it possible to use `ServerBuilder::set_batch_request_config`

### [Fixed]

- server: export BatchRequestConfig ([#1112](https://github.com/paritytech/jsonrpsee/pull/1112))
- fix(server): improve too big batch response msg ([#1107](https://github.com/paritytech/jsonrpsee/pull/1107))

## [v0.18.1] - 2023-04-26

This release fixes a couple bugs and improves the ergonomics for the HTTP client
when no tower middleware is enabled.

### [Changed]

- http client: add default generic param for the backend ([#1099](https://github.com/paritytech/jsonrpsee/pull/1099))

### [Fixed]

- rpc module: fix race in subscription close callback ([#1098](https://github.com/paritytech/jsonrpsee/pull/1098))
- client: add missing batch request tracing span ([#1097](https://github.com/paritytech/jsonrpsee/pull/1097))
- ws server: don't wait for graceful shutdown when connection already closed ([#1103](https://github.com/paritytech/jsonrpsee/pull/1103))

## [v0.18.0] - 2023-04-21

This is a breaking release that removes the `CallError` which was used to represent a JSON-RPC error object that
could happen during JSON-RPC method call and one could assign application specific error code, message and data in a
specific implementation.

Previously jsonrpsee provided `CallError` that could be converted to/from `jsonrpsee::core::Error`
and in some scenarios the error code was automatically assigned by jsonrpsee. After jsonrpsee
added support for custom error types the `CallError` doesn't provide any benefit because one has to implement `Into<ErrorObjectOwned>`
on the error type anyway.

Thus, `jsonrpsee::core::Error` can't be used in the proc macro API anymore and the type alias
`RpcResult` has been modified to `Result<(), ErrorObjectOwned>` instead.

Before it was possible to do:

```rust
#[derive(thiserror::Error)]
enum Error {
	A,
	B,
}

#[rpc(server, client)]
pub trait Rpc
{
	#[method(name = "getKeys")]
	async fn keys(&self) -> Result<String, jsonrpsee::core::Error> {
		Err(jsonrpsee::core::Error::to_call_error(Error::A))
		// or jsonrpsee::core::Error::Call(CallError::Custom(ErrorObject::owned(1, "a", None::<()>)))
	}
}
```

After this change one has to do:

```rust
pub enum Error {
	A,
	B,
}

impl From<Error> for ErrorObjectOwned {
	fn from(e: Error) -> Self {
		match e {
			Error::A => ErrorObject::owned(1, "a", None::<()>),
			Error::B => ErrorObject::owned(2, "b", None::<()>),
		}
	}
}

#[rpc(server, client)]
pub trait Rpc {
	// Use a custom error type that implements `Into<ErrorObject>`
	#[method(name = "custom_err_ty")]
	async fn custom_err_type(&self) -> Result<String, Error> {
		Err(Error::A)
	}

	// Use `ErrorObject` as error type directly.
	#[method(name = "err_obj")]
	async fn error_obj(&self) -> RpcResult<String> {
		Err(ErrorObjectOwned::owned(1, "c", None::<()>))
	}
}
```

### [Changed]
- remove `CallError` ([#1087](https://github.com/paritytech/jsonrpsee/pull/1087))

### [Fixed]
- fix(proc macros): support parsing params !Result ([#1094](https://github.com/paritytech/jsonrpsee/pull/1094))

## [v0.17.1] - 2023-04-21

This release fixes HTTP graceful shutdown for the server.

### [Fixed]
- server: fix http graceful shutdown ([#1090](https://github.com/paritytech/jsonrpsee/pull/1090))

## [v0.17.0] - 2023-04-17

This is a significant release and the major breaking changes to be aware of are:

### Server backpressure

This release changes the server to be "backpressured" and it mostly concerns subscriptions.
New APIs has been introduced because of that and the API `pipe_from_stream` has been removed.

Before it was possible to do:

```rust
	module
		.register_subscription("sub", "s", "unsub", |_, sink, _| async move {
			let stream = stream_of_integers();

			tokio::spawn(async move {
				sink.pipe_from_stream(stream)
			});
		})
		.unwrap();
```

After this release one must do something like:

```rust
	// This is just a example helper.
	//
	// Other examples:
	// - <https://github.com/paritytech/jsonrpsee/blob/master/examples/examples/ws_pubsub_broadcast.rs>
	// - <https://github.com/paritytech/jsonrpsee/blob/master/examples/examples/ws_pubsub_with_params.rs>
	async fn pipe_from_stream<T: Serialize>(
		pending: PendingSubscriptionSink,
		mut stream: impl Stream<Item = T> + Unpin,
	) -> Result<(), anyhow::Error> {
		let mut sink = pending.accept().await?;

		loop {
			tokio::select! {
				_ = sink.closed() => break Ok(()),

				maybe_item = stream.next() => {
					let Some(item) = match maybe_item else {
						break Ok(()),
					};

					let msg = SubscriptionMessage::from_json(&item)?;

					if let Err(e) = sink.send_timeout(msg, Duration::from_secs(60)).await {
						match e {
							// The subscription or connection was closed.
							SendTimeoutError::Closed(_) => break Ok(()),
							/// The subscription send timeout expired
							/// the message is returned and you could save that message
							/// and retry again later.
							SendTimeoutError::Timeout(_) => break Err(anyhow::anyhow!("Subscription timeout expired")),
						}
					}
				}
			}
		}
	}


	module
		.register_subscription("sub", "s", "unsub", |_, pending, _, _| async move {
			let stream = stream();
			pipe_from_stream(sink, stream).await
		})
		.unwrap();
```

### Method call return type is more flexible

This release also introduces a trait called `IntoResponse` which is makes it possible to return custom types and/or error
types instead of enforcing everything to return `Result<T, jsonrpsee::core::Error>`

This affects the APIs `RpcModule::register_method`, `RpcModule::register_async_method` and `RpcModule::register_blocking_method`
and when these are used in the proc macro API are affected by this change.
Be aware that [the client APIs don't support this yet](https://github.com/paritytech/jsonrpsee/issues/1067)

The `IntoResponse` trait is already implemented for `Result<T, jsonrpsee::core::Error>` and for the primitive types

Before it was possible to do:

```rust
	// This would return Result<&str, jsonrpsee::core::Error>
	module.register_method("say_hello", |_, _| Ok("lo"))?;
```

After this release it's possible to do:

```rust
	// Note, this method call is infallible and you might not want to return Result.
	module.register_method("say_hello", |_, _| "lo")?;
```

### Subscription API is changed.

jsonrpsee now spawns the subscriptions via `tokio::spawn` and it's sufficient to provide an async block in `register_subscription`

Further, the subscription API had an explicit close API for closing subscriptions which was hard to understand and
to get right. This has been removed and everything is handled by the return value/type of the async block instead.

Example:

```rust
	module
		.register_subscription::<RpcResult<(), _, _>::("sub", "s", "unsub", |_, pending, _, _| async move {
			// This just answers the RPC call and if this fails => no close notification is sent out.
			pending.accept().await?;
			// This is sent out as a `close notification/message`.
			Err(anyhow::anyhow!("The subscription failed"))?;
		})
		.unwrap();
```

The return value in the example above needs to implement `IntoSubscriptionCloseResponse` and
any value that is returned after that the subscription has been accepted will be treated as a `IntoSubscriptionCloseResponse`.

Because `Result<(), E>` is used here the close notification will be sent out as error notification but it's possible to
disable the subscription close response by using `()` instead of `Result<(), E>` or implement `IntoSubscriptionCloseResponse` for other behaviour.

### [Added]
- feat(server): configurable limit for batch requests. ([#1073](https://github.com/paritytech/jsonrpsee/pull/1073))
- feat(http client): add tower middleware ([#981](https://github.com/paritytech/jsonrpsee/pull/981))

### [Fixed]
- add tests for ErrorObject ([#1078](https://github.com/paritytech/jsonrpsee/pull/1078))
- fix: tokio v1.27 ([#1062](https://github.com/paritytech/jsonrpsee/pull/1062))
- fix: remove needless `Semaphore::(u32::MAX)` ([#1051](https://github.com/paritytech/jsonrpsee/pull/1051))
- fix server: don't send error on JSON-RPC notifications ([#1021](https://github.com/paritytech/jsonrpsee/pull/1021))
- fix: add `max_log_length` APIs and use missing configs ([#956](https://github.com/paritytech/jsonrpsee/pull/956))
- fix(rpc module): subscription close bug ([#1011](https://github.com/paritytech/jsonrpsee/pull/1011))
- fix: customized server error codes ([#1004](https://github.com/paritytech/jsonrpsee/pull/1004))

### [Changed]
- docs: introduce workspace attributes and add keywords ([#1077](https://github.com/paritytech/jsonrpsee/pull/1077))
- refactor(server): downgrade connection log ([#1076](https://github.com/paritytech/jsonrpsee/pull/1076))
- chore(deps): update webpki-roots and tls ([#1068](https://github.com/paritytech/jsonrpsee/pull/1068))
- rpc module: refactor subscriptions to return `impl IntoSubscriptionResponse` ([#1034](https://github.com/paritytech/jsonrpsee/pull/1034))
- add `IntoResponse` trait for method calls ([#1057](https://github.com/paritytech/jsonrpsee/pull/1057))
- Make `jsonrpc` protocol version field in `Response` as `Option` ([#1046](https://github.com/paritytech/jsonrpsee/pull/1046))
- server: remove dependency http ([#1037](https://github.com/paritytech/jsonrpsee/pull/1037))
- chore(deps): update tower-http requirement from 0.3.4 to 0.4.0 ([#1033](https://github.com/paritytech/jsonrpsee/pull/1033))
- chore(deps): update socket2 requirement from 0.4.7 to 0.5.1 ([#1032](https://github.com/paritytech/jsonrpsee/pull/1032))
- Update bound type name ([#1029](https://github.com/paritytech/jsonrpsee/pull/1029))
- rpc module: remove `SubscriptionAnswer` ([#1025](https://github.com/paritytech/jsonrpsee/pull/1025))
- make verify_and_insert pub ([#1028](https://github.com/paritytech/jsonrpsee/pull/1028))
- update MethodKind ([#1026](https://github.com/paritytech/jsonrpsee/pull/1026))
- remove batch response ([#1020](https://github.com/paritytech/jsonrpsee/pull/1020))
- remove debug log ([#1024](https://github.com/paritytech/jsonrpsee/pull/1024))
- client: rename `max_notifs_per_subscription` to `max_buffer_capacity_per_subscription` ([#1012](https://github.com/paritytech/jsonrpsee/pull/1012))
- client: feature gate tls cert store ([#994](https://github.com/paritytech/jsonrpsee/pull/994))
- server: bounded channels and backpressure ([#962](https://github.com/paritytech/jsonrpsee/pull/962))
- client: use tokio channels ([#999](https://github.com/paritytech/jsonrpsee/pull/999))
- chore: update gloo-net ^0.2.6 ([#978](https://github.com/paritytech/jsonrpsee/pull/978))
- Custom errors ([#977](https://github.com/paritytech/jsonrpsee/pull/977))
- client: distinct APIs to configure max request and response sizes ([#967](https://github.com/paritytech/jsonrpsee/pull/967))
- server: replace `FutureDriver` with `tokio::spawn` ([#1080](https://github.com/paritytech/jsonrpsee/pull/1080))
- server: uniform whitespace handling in rpc calls ([#1082](https://github.com/paritytech/jsonrpsee/pull/1082))

## [v0.16.2] - 2022-12-01

This release adds `Clone` and `Copy` implementations.

### [Fixed]

- fix(rpc module): make async closures Clone ([#948](https://github.com/paritytech/jsonrpsee/pull/948))
- fix(ci): wasm tests ([#946](https://github.com/paritytech/jsonrpsee/pull/946))

### [Added]

- add missing `Clone` and `Copy` impls ([#951](https://github.com/paritytech/jsonrpsee/pull/951))
- TowerService should be clone-able for handling concurrent request ([#950](https://github.com/paritytech/jsonrpsee/pull/950))

## [v0.16.1] - 2022-11-18

v0.16.1 is release that adds two new APIs to server `http_only` and `ws_only` to make it possible to allow only HTTP respectively WebSocket.

Both HTTP and WebSocket are still enabled by default.

### [Fixed]

- docs: remove outdated features ([#938](https://github.com/paritytech/jsonrpsee/pull/938))
- docs: http client url typo in examples ([#940](https://github.com/paritytech/jsonrpsee/pull/940))
- core: remove unused dependency `async-channel` ([#940](https://github.com/paritytech/jsonrpsee/pull/941))

### [Added]

- server: make it possible to enable ws/http only ([#939](https://github.com/paritytech/jsonrpsee/pull/939))

## [v0.16.0] - 2022-11-09

v0.16.0 is a breaking release and the major changes are:

- The server now support WS and HTTP on the same socket and the `jsonrpsee-http-server` and `jsonrpsee-ws-server` crates are moved to the `jsonrpsee-server` crate instead.
- The client batch request API is improved such as the errors and valid responses can be iterated over.
- The server has `tower middleware` support.
- The server now adds a tracing span for each connection to distinguish logs per connection.
- CORS has been moved to `tower middleware`.

### [Fixed]

- server: read accepted conns properly ([#929](https://github.com/paritytech/jsonrpsee/pull/929))
- server: proper handling of batch errors and mixed calls ([#917](https://github.com/paritytech/jsonrpsee/pull/917))
- jsonrpsee: add `types` to server feature ([#891](https://github.com/paritytech/jsonrpsee/pull/891))
- http client: more user-friendly error messages when decoding fails ([#853](https://github.com/paritytech/jsonrpsee/pull/853))
- http_server: handle http2 requests host filtering correctly ([#866](https://github.com/paritytech/jsonrpsee/pull/866))
- server: `RpcModule::call` decode response correctly ([#839](https://github.com/paritytech/jsonrpsee/pull/839))

### [Added]

- proc macro: support camelCase & snake_case for object params ([#921](https://github.com/paritytech/jsonrpsee/pull/921))
- server: add connection span ([#922](https://github.com/paritytech/jsonrpsee/pull/922))
- server: Expose the subscription ID ([#900](https://github.com/paritytech/jsonrpsee/pull/900))
- jsonrpsee wrapper crate: add feature async_wasm_client ([#893](https://github.com/paritytech/jsonrpsee/pull/893))
- server: add `transport protocol details` to the logger trait ([#886](https://github.com/paritytech/jsonrpsee/pull/886))
- middleware: Implement proxy URI paths to RPC methods ([#859](https://github.com/paritytech/jsonrpsee/pull/859))
- client: Implement `notify_on_disconnect` ([#837](https://github.com/paritytech/jsonrpsee/pull/837))
- Add `bytes_len()` to Params ([#848](https://github.com/paritytech/jsonrpsee/pull/848))
- Benchmarks for different HTTP header sizes ([#824](https://github.com/paritytech/jsonrpsee/pull/824))

### [Changed]

- replace `WS and HTTP servers` with a server that supports both `WS and HTTP` ([#863](https://github.com/paritytech/jsonrpsee/pull/863))
- Optimize serialization for client parameters ([#864](https://github.com/paritytech/jsonrpsee/pull/864))
- Uniform log messages ([#855](https://github.com/paritytech/jsonrpsee/pull/855))
- Move CORS logic to tower middleware CorsLayer ([#851](https://github.com/paritytech/jsonrpsee/pull/851))
- server: add log for the http request ([#854](https://github.com/paritytech/jsonrpsee/pull/854))
- server: add `tower` support ([#831](https://github.com/paritytech/jsonrpsee/pull/831))
- jsonrpsee: less deps when defining RPC API. ([#849](https://github.com/paritytech/jsonrpsee/pull/849))
- server: rename `Middleware` to `Logger` ([#845](https://github.com/paritytech/jsonrpsee/pull/845))
- client: adjust TransportSenderT ([#852](https://github.com/paritytech/jsonrpsee/pull/852))
- client: improve batch request API ([#910](https://github.com/paritytech/jsonrpsee/pull/910))
- server: Optimize sending for `SubscriptionSink::pipe_from_stream` ([#901](https://github.com/paritytech/jsonrpsee/pull/901))
- ws-client: downgrade connection log to debug ([#865](https://github.com/paritytech/jsonrpsee/pull/865))
- use tracing instrument macro ([#846](https://github.com/paritytech/jsonrpsee/pull/846))

## [v0.15.1] - 2022-07-29

This release fixes some incorrect tracing spans.

### [Fixed]
- [Bug Fix] - Incorrect trace caused by use of Span::enter in asynchronous code [#835](https://github.com/paritytech/jsonrpsee/pull/835)

## [v0.15.0] - 2022-07-20

v0.15.0 is a breaking release. The main changes are:

- It's now possible to apply resource limits to subscriptions as well as regular calls.
- We now allow trait bounds to be overridden in the proc macros. See `examples/examples/proc_macro_bounds.rs` for examples.
- We've tidied up the subscription API, removing the `PendingSink` concept (you can still manually accept or reject a sink, but otherwise it'll be accepted automatically if you send a message down it) ([#799](https://github.com/paritytech/jsonrpsee/pull/799)).
- Our logging `Middleware` trait has been split into `HttpMiddleware` and `WsMiddleware` to better capture the differences between the two. if you use custom middleware, you'll need to implement one or the other trait on it depending on your used transport method ([#793](https://github.com/paritytech/jsonrpsee/pull/793)). We also provide params and the method type to middleware calls now, too ([#820](https://github.com/paritytech/jsonrpsee/pull/820)).
- We've consistified the API for setting headers across HTTP and WS clients ([#799](https://github.com/paritytech/jsonrpsee/pull/814)).

Here's the full list of changes:

### [Fixed]

- Fix client generation with param_kind = map [#805](https://github.com/paritytech/jsonrpsee/pull/805)
- ws-server: Handle soketto::Incoming::Closed frames [#815](https://github.com/paritytech/jsonrpsee/pull/815)
- fix(ws server): reply HTTP 403 on all failed conns [#819](https://github.com/paritytech/jsonrpsee/pull/819)
- fix clippy [#817](https://github.com/paritytech/jsonrpsee/pull/817)

### [Added]

- Add resource limiting for Subscriptions [#786](https://github.com/paritytech/jsonrpsee/pull/786)
- feat(logging): add tracing span per JSON-RPC call [#722](https://github.com/paritytech/jsonrpsee/pull/722)
- feat(clients): add explicit unsubscribe API [#789](https://github.com/paritytech/jsonrpsee/pull/789)
- Allow trait bounds to be overridden in macro [#808](https://github.com/paritytech/jsonrpsee/pull/808)

### [Changed]

- Point to a new v1.0 milestone in the README.md [#801](https://github.com/paritytech/jsonrpsee/pull/801)
- chore(deps): upgrade tracing v0.1.34 [#800](https://github.com/paritytech/jsonrpsee/pull/800)
- Replace cargo-nextest with cargo-test for running tests [#802](https://github.com/paritytech/jsonrpsee/pull/802)
- Remove deny_unknown_fields from Request and Response [#803](https://github.com/paritytech/jsonrpsee/pull/803)
- substrate-subxt -> subxt [#807](https://github.com/paritytech/jsonrpsee/pull/807)
- chore(deps): update pprof requirement from 0.9 to 0.10 [#810](https://github.com/paritytech/jsonrpsee/pull/810)
- Return error from subscription callbacks [#799](https://github.com/paritytech/jsonrpsee/pull/799)
- middleware refactoring [#793](https://github.com/paritytech/jsonrpsee/pull/793)
- feat(middleware): expose type of the method call [#820](https://github.com/paritytech/jsonrpsee/pull/820)
- Uniform API for custom headers between clients [#814](https://github.com/paritytech/jsonrpsee/pull/814)
- Update links to client directories. [#822](https://github.com/paritytech/jsonrpsee/pull/822)

## [v0.14.0] - 2022-06-14

v0.14.0 is breaking release which changes the `health and access control APIs` and a bunch of bug fixes.

### [Fixed]
- fix(servers): more descriptive errors when calls fail [#790](https://github.com/paritytech/jsonrpsee/pull/790)
- fix(ws server): support `*` in host and origin filtering [#781](https://github.com/paritytech/jsonrpsee/pull/781)
- fix(rpc module): register failed `unsubscribe calls` in middleware [#792](https://github.com/paritytech/jsonrpsee/pull/792)
- fix(http server): omit jsonrpc details in health API [#785](https://github.com/paritytech/jsonrpsee/pull/785)
- fix(servers): skip leading whitespace in JSON deserialization [#783](https://github.com/paritytech/jsonrpsee/pull/783)
- fix(ws-server): Submit ping regardless of WS messages [#788](https://github.com/paritytech/jsonrpsee/pull/788)
- fix(rpc_module): remove expect in `fn call` [#774](https://github.com/paritytech/jsonrpsee/pull/774)

### [Added]
- feat(ws-client): `ping-pong` for WebSocket clients [#772](https://github.com/paritytech/jsonrpsee/pull/772)
- feat(ws-server): Implement `ping-pong` for WebSocket server [#782](https://github.com/paritytech/jsonrpsee/pull/782)

### [Changed]
- chore(deps): bump Swatinem/rust-cache from 1.3.0 to 1.4.0 [#778](https://github.com/paritytech/jsonrpsee/pull/778)
- chore(deps): bump actions/checkout from 2.4.0 to 3.0.2 [#779](https://github.com/paritytech/jsonrpsee/pull/779)
- chore(ci): bring back daily benchmarks [#777](https://github.com/paritytech/jsonrpsee/pull/777)
- chore(examples): Move examples under dedicated folder to simplify `Cargo.toml` [#769](https://github.com/paritytech/jsonrpsee/pull/769)

## [v0.13.1] - 2022-05-13

v0.13.1 is a release that fixes the documentation for feature-gated items on `docs.rs`.

### [Fixed]
- fix: generate docs for all features on docs.rs [#767](https://github.com/paritytech/jsonrpsee/pull/767)

### [Changed]
- chore(deps): update pprof requirement from 0.8 to 0.9 [#761](https://github.com/paritytech/jsonrpsee/pull/761)

## [v0.13.0] - 2022-05-11

v0.13.0 is release that adds health API support for the HTTP server and a few bug fixes.

### [Added]
feat: add http health API [#763](https://github.com/paritytech/jsonrpsee/pull/763)

### [Fixed]
- hide internal macros from public interface [#755](https://github.com/paritytech/jsonrpsee/pull/755)
- fix: add `core` behind `http-server` feature [#760](https://github.com/paritytech/jsonrpsee/pull/760)


## [v0.12.0] - 2022-05-06

v0.12.0 is mainly a patch release with some minor features added.

### [Added]
- Make it possible to disable batch requests support [#744](https://github.com/paritytech/jsonrpsee/pull/744)
- feat: add a way to limit the number of subscriptions per connection [#739](https://github.com/paritytech/jsonrpsee/pull/739)

### [Fixed]
- fix(http client): use https connector for https [#750](https://github.com/paritytech/jsonrpsee/pull/750)
- fix(rpc module): close subscription task when a subscription is `unsubscribed` via the `unsubscribe call` [#743](https://github.com/paritytech/jsonrpsee/pull/743)
- fix(jsonrpsee): generate docs behind features [#741](https://github.com/paritytech/jsonrpsee/pull/741)

### [Changed]
- remove vault from ci [#745](https://github.com/paritytech/jsonrpsee/pull/745)
- chore(deps): update pprof requirement from 0.7 to 0.8 [#732](https://github.com/paritytech/jsonrpsee/pull/732)
- chore(deps): update gloo-net requirement from 0.1.0 to 0.2.0 [#733](https://github.com/paritytech/jsonrpsee/pull/733)

## [v0.11.0] - 2022-04-21

v0.11.0 is a breaking release that reworks how subscriptions are handled by the servers where the users have to explicitly reject or accept each subscription.
The reason for this is that the actual params in the subscription is passed to the callback and if the application decides the params are invalid and the server can't know if the call is going to fail or pass when dispatching the call.
Thus, the actual subscription method call is only answered when the subscription is accepted or rejected.

Additionally, the servers before sent a `SubscriptionClosed message` which is now disabled by default because it might break other implementations.
It is still possible to respond with a `SubscriptionClosed message` but one has to match on the result from `SubscriptionSink::pipe_from_stream`.

This release also adds support for `JSON-RPC WASM client` using web-sys bindings.

### [Added]
- feat: WASM client via web-sys transport [#648](https://github.com/paritytech/jsonrpsee/pull/648)

### [Changed]
- CI: bump Swatinem/rust-cache from 1.3.0 to 1.4.0 [#730](https://github.com/paritytech/jsonrpsee/pull/730)

### [Fixed]
- fix(rpc module): fail subscription calls with bad params [#728](https://github.com/paritytech/jsonrpsee/pull/728)


## [v0.10.1] - 2022-04-05

v0.10.1 is a release that fixes a regression in the HTTP server where the backlog was hardcoded to 128 (this is now set to 1024 by default but also configurable), introduces a couple of new APIs and a few minor bug fixes.

If your usage expects a high rate of new HTTP connections you are encouraged to update or manually configure the socket based on the traffic characteristics.

### [Changed]
- [proc macros]: only generate unsub method if not provided [#702](https://github.com/paritytech/jsonrpsee/pull/702)
- [examples]: update pubsub examples [#705](https://github.com/paritytech/jsonrpsee/pull/705)
- core: remove `Error::Request` variant [#717](https://github.com/paritytech/jsonrpsee/pull/717)
- Replace async-channel [#708](https://github.com/paritytech/jsonrpsee/pull/708)
- chore(deps): bump actions/checkout from 2.4.0 to 3 [#710](https://github.com/paritytech/jsonrpsee/pull/710)
- CI: cache cargo hack installation [#706](https://github.com/paritytech/jsonrpsee/pull/706)
- CI: try nextest [#701](https://github.com/paritytech/jsonrpsee/pull/701)
- chore(deps): update tokio-util requirement from 0.6 to 0.7 [#695](https://github.com/paritytech/jsonrpsee/pull/695)
- CI: Move CI script to new location [#694](https://github.com/paritytech/jsonrpsee/pull/694)
- refactor(log): downgrade send errors to warn [#726](https://github.com/paritytech/jsonrpsee/pull/726)

### [Fixed]
- fix(client): close subscription when server sent `SubscriptionClosed` notification [#721](https://github.com/paritytech/jsonrpsee/pull/721)
- fix(http client): set reuseaddr and nodelay. [#687](https://github.com/paritytech/jsonrpsee/pull/687)
- fix(rpc module): unsubscribe according ethereum pubsub spec [#693](https://github.com/paritytech/jsonrpsee/pull/693)
- http server: fix regression set backlog to 1024 [#718](https://github.com/paritytech/jsonrpsee/pull/718)
- README.MD: fix link to `ws server` [#703](https://github.com/paritytech/jsonrpsee/pull/703)
- fix(ws server): close all subscription when the connection is closed [#725](https://github.com/paritytech/jsonrpsee/pull/725)
- perf: don't send messages when client is gone [#724](https://github.com/paritytech/jsonrpsee/pull/724)

### [Added]
- feat(http server): add new builder APIs `build_from_tcp` and `build_from_hyper` [#719](https://github.com/paritytech/jsonrpsee/pull/719)
- feat(servers): add `SubscriptionSink::pipe_from_try_stream` to support streams that returns `Result` [#720](https://github.com/paritytech/jsonrpsee/pull/720)
- feat(servers): add max_response_size [#711](https://github.com/paritytech/jsonrpsee/pull/711)

## [v0.10.0] - 2022-04-04 [YANKED]

Yanked due to a leak when closing subscriptions in WebSocket server.

## [v0.9.0] - 2022-02-03

v0.9.0 is technically a breaking release because of the `Debug` bound of the `IdProvider` trait changed which is used by WebSocket server. In practise it should be a non-breaking upgrade for most users.

### [Changed]
refactor(ws server): impl IdProvider for Box<T> [#684](https://github.com/paritytech/jsonrpsee/pull/684)
chore(deps): update parking_lot requirement from 0.11 to 0.12 [#682](https://github.com/paritytech/jsonrpsee/pull/682)

## [v0.8.0] - 2022-01-21

v0.8.0 is a breaking release for the way subscription closing is handled, along with a few other minor tweaks and fixes.

### [Added]

- feat(client): support request id as Strings. [#659](https://github.com/paritytech/jsonrpsee/pull/659)
- feat(rpc module) Add a method to RpcModule that transforms the module into a RpcModule<()>, i.e. removes the context. [#660](https://github.com/paritytech/jsonrpsee/pull/660)
- feat(rpc module): stream API for SubscriptionSink [#639](https://github.com/paritytech/jsonrpsee/pull/639)

### [Fixed]

- fix: nit in WsError [#662](https://github.com/paritytech/jsonrpsee/pull/662)
- fix(jsonrpsee): feature macros include client types [#656](https://github.com/paritytech/jsonrpsee/pull/656)
- fix(ws client): export WsClient [#646](https://github.com/paritytech/jsonrpsee/pull/646)
- fix(ws client): improve error message bad URL [#642](https://github.com/paritytech/jsonrpsee/pull/642)
- fix(ws client): expose tls feature. [#640](https://github.com/paritytech/jsonrpsee/pull/640)
- fix(http server): handle post and option HTTP requests properly. [#637](https://github.com/paritytech/jsonrpsee/pull/637)

## [v0.7.0] - 2021-12-22

v0.7.0 is a breaking release that contains a big refactoring of the crate structure. The `types` and
`utils` crates are split up as `types` and `core` to clarify the difference between the two.

`core`: common types used in various places.
`types`: includes JSON-RPC specification related types.

### [Added]

- servers: configurable subscriptionID [#604](https://github.com/paritytech/jsonrpsee/pull/604)
- client: impl Stream on Subscription and tweak built-in next() method [#601](https://github.com/paritytech/jsonrpsee/pull/601)
- ci: Create gitlab pipeline [#534](https://github.com/paritytech/jsonrpsee/pull/534)

### [Changed]

- chore: migrate to rust 2021 [#618](https://github.com/paritytech/jsonrpsee/pull/618)
- extract async client abstraction. [#580](https://github.com/paritytech/jsonrpsee/pull/580)
- Crate restructuring [#590](https://github.com/paritytech/jsonrpsee/pull/590)
- servers: refactor `SubscriptionClosed` [#612](https://github.com/paritytech/jsonrpsee/pull/612)
- ci: Add job to publish benchmark results to github pages [#603](https://github.com/paritytech/jsonrpsee/pull/603)
- rpc module: refactor calls/subs without a server [#591](https://github.com/paritytech/jsonrpsee/pull/591)
- types: make subscription ID a CoW String. [#594](https://github.com/paritytech/jsonrpsee/pull/594)
- ci: remove GHA daily benchmark [#598](https://github.com/paritytech/jsonrpsee/pull/598)
- examples: Remove usage of the `palaver` crate in an example [#597](https://github.com/paritytech/jsonrpsee/pull/597)
- clients: use `FxHashMap` instead `FnvHashMap` [#592](https://github.com/paritytech/jsonrpsee/pull/592)
- clients: feature gate `tls` [#545](https://github.com/paritytech/jsonrpsee/pull/545)

### [Fixed]

- benches: fix image in check-bench job [#621](https://github.com/paritytech/jsonrpsee/pull/621)
- benches: update publish script [#619](https://github.com/paritytech/jsonrpsee/pull/619)
- chore(http client): remove needless clone [#620](https://github.com/paritytech/jsonrpsee/pull/620)
- jsonrpsee wrapper: make ws tls configurable [#616](https://github.com/paritytech/jsonrpsee/pull/616)
- deps: Upgrade `tracing-subscriber` [#615](https://github.com/paritytech/jsonrpsee/pull/615)
- proc macros: Fix span for underscore_token for tests to be equivalent on stable and nightly [#614](https://github.com/paritytech/jsonrpsee/pull/614)
- proc macros: Better error messages for method arguments ignored with a `_` [#611](https://github.com/paritytech/jsonrpsee/pull/611)
- http client: re-export transport types. [#607](https://github.com/paritytech/jsonrpsee/pull/607)
- benches: Fix job to publish benchmark results to gh-pages [#608](https://github.com/paritytech/jsonrpsee/pull/608)
- benches: make jsonrpc crates optional [#596](https://github.com/paritytech/jsonrpsee/pull/596)
- deps: duplicate env logger deps [#595](https://github.com/paritytech/jsonrpsee/pull/595)

## [v0.6.1] – 2021-12-07

### [Added]

- rpc module: add call_and_subscribe [#588](https://github.com/paritytech/jsonrpsee/pull/588)


## [v0.6.0] – 2021-12-01

v0.6 is a breaking release

### [Added]

- Servers: Middleware for metrics [#576](https://github.com/paritytech/jsonrpsee/pull/576)
- http client: impl Clone [#583](https://github.com/paritytech/jsonrpsee/pull/583)

### [Fixed]
- types: use Cow for deserializing str [#584](https://github.com/paritytech/jsonrpsee/pull/584)
- deps: require tokio ^1.8 [#586](https://github.com/paritytech/jsonrpsee/pull/586)


## [v0.5.1] – 2021-11-26

The v0.5.1 release is a bug fix.

### [Fixed]

- rpc error: support escaped strings [#578](https://github.com/paritytech/jsonrpsee/pull/578)

## [v0.5.0] – 2021-11-23

v0.5 is a breaking release

### [Added]

- Add register_blocking_method [#523](https://github.com/paritytech/jsonrpsee/pull/523)
- Re-introduce object param parsing [#526](https://github.com/paritytech/jsonrpsee/pull/526)
- clients: add support for webpki and native certificate stores [#533](https://github.com/paritytech/jsonrpsee/pull/533)
- feat(ws client): support custom headers. [#535](https://github.com/paritytech/jsonrpsee/pull/535)
- Proc macro support for map param [#544](https://github.com/paritytech/jsonrpsee/pull/544)
- feat: make it possible to try several sockaddrs when starting server [#567](https://github.com/paritytech/jsonrpsee/pull/567)
- feat: make it possible to override method name in subscriptions [#568](https://github.com/paritytech/jsonrpsee/pull/568)
- proc-macros: Support deprecated methods for rpc client [#570](https://github.com/paritytech/jsonrpsee/pull/570)

### [Change]

- DRY error handling for methods [#515](https://github.com/paritytech/jsonrpsee/pull/515)
- deps: replace log with tracing [#525](https://github.com/paritytech/jsonrpsee/pull/525)
- benches: add option to run benchmarks against jsonrpc crate servers [#527](https://github.com/paritytech/jsonrpsee/pull/527)
- clients: request ID as RAII guard [#543](https://github.com/paritytech/jsonrpsee/pull/543)
- Allow awaiting on server handles [#550](https://github.com/paritytech/jsonrpsee/pull/550)
- ws server: reject too big response [#553](https://github.com/paritytech/jsonrpsee/pull/553)
- Array syntax aliases [#557](https://github.com/paritytech/jsonrpsee/pull/557)
- rpc module: report error on invalid subscription [#561](https://github.com/paritytech/jsonrpsee/pull/561)
- [rpc module]: improve TestSubscription to return None when closed [#566](https://github.com/paritytech/jsonrpsee/pull/566)

### [Fixed]

- ws server: respect max limit for received messages [#537](https://github.com/paritytech/jsonrpsee/pull/537)
- fix(ws server): batch wait until all methods has been executed. [#542](https://github.com/paritytech/jsonrpsee/pull/542)
- Re-export tracing for macros [#555](https://github.com/paritytech/jsonrpsee/pull/555)
- Periodically wake DriverSelect so we can poll whether or not stop had been called. [#556](https://github.com/paritytech/jsonrpsee/pull/556)
- Implement SubscriptionClient for HttpClient [#563](https://github.com/paritytech/jsonrpsee/pull/563)
- fix: better log for failed unsubscription call [#575](https://github.com/paritytech/jsonrpsee/pull/575)

## [v0.4.1] – 2021-10-12

The v0.4.1 release is a bug fix.

### [Fixed]

- fix: nit in ServerBuilder::custom_tokio_runtime [#512](https://github.com/paritytech/jsonrpsee/pull/512)

## [v0.4.0] – 2021-10-12

The v0.4 release is a breaking change.

### [Added]

- Document resource limiting [#510](https://github.com/paritytech/jsonrpsee/pull/510)

- Resource limiting [#500](https://github.com/paritytech/jsonrpsee/pull/500)

- Support http redirects when doing the ws handshake [#397](https://github.com/paritytech/jsonrpsee/pull/397)

- Add convenience `rpc_params` macro to build params in http and ws clients [#498](https://github.com/paritytech/jsonrpsee/pull/498)

- Method alias attribute for proc macros [#442](https://github.com/paritytech/jsonrpsee/pull/442)

- Add benchmarks for concurrent connections [#430](https://github.com/paritytech/jsonrpsee/pull/430)

- Support generic type params in the proc macro [#436](https://github.com/paritytech/jsonrpsee/pull/436)


### [Changed]

- use tokio::spawn internally in `HttpServer::start` and return `StopHandle` [#402](https://github.com/paritytech/jsonrpsee/pull/402)

- remove `ParamsSer::NoParams` [#501](https://github.com/paritytech/jsonrpsee/pull/501)

- http server uses similar API for host and origin filtering as `WS` [#473](https://github.com/paritytech/jsonrpsee/pull/473)

- `SubscriptionClosed` errors carry more information [#504](https://github.com/paritytech/jsonrpsee/pull/504)

- Improve feature configuration for faster builds and leaner build artifacts [#494](https://github.com/paritytech/jsonrpsee/pull/494)

- Unbox async futures [#495](https://github.com/paritytech/jsonrpsee/pull/495)

- WS clients default subscription buffer set to 1024 items [#475](https://github.com/paritytech/jsonrpsee/pull/475)

- Re-export `v2` submodules [#469](https://github.com/paritytech/jsonrpsee/pull/469)

- Replace internal `array_impl macro` with const generics [#470](https://github.com/paritytech/jsonrpsee/pull/470)

- Rename and reorganize many public types [#462](https://github.com/paritytech/jsonrpsee/pull/462)

- Export acl types [#466](https://github.com/paritytech/jsonrpsee/pull/466)

- Propagate cause of `InvalidParams` [#463](https://github.com/paritytech/jsonrpsee/pull/463)

- Reject overflowing connection with status code 429 [#456](https://github.com/paritytech/jsonrpsee/pull/456)

- Test helper for calling and converting types to JSON-RPC params [#458](https://github.com/paritytech/jsonrpsee/pull/458)

- Make it possible to treat empty JSON response as no params [#446](https://github.com/paritytech/jsonrpsee/pull/446)

- Methods generated by the proc macro return `Result` [#435](https://github.com/paritytech/jsonrpsee/pull/435)

- Concurrent polling on async methods [#424](https://github.com/paritytech/jsonrpsee/pull/424)

- Sniff the first byte to glean if the incoming request is a single or batch request [#419](https://github.com/paritytech/jsonrpsee/pull/419)

- Upgrade hyper to ^0.14.10 [#427](https://github.com/paritytech/jsonrpsee/pull/427)

- Proc macro params optimizations and tests. [#421](https://github.com/paritytech/jsonrpsee/pull/421)


### [Fixed]

- Proc macro Argument parsing should permit commas inside angle brackets [#509](https://github.com/paritytech/jsonrpsee/pull/509)

- Fix http client bench with request limit [#506](https://github.com/paritytech/jsonrpsee/pull/506)

- Fixed flaky test on windows [#491](https://github.com/paritytech/jsonrpsee/pull/491)

- Share the request id code between the http and websocket clients [#490](https://github.com/paritytech/jsonrpsee/pull/490)

- WS server terminates subscriptions when connection is closed by the client. [#483](https://github.com/paritytech/jsonrpsee/pull/483)

- Subscription code generated by the proc macro generated returns `Result` [#455](https://github.com/paritytech/jsonrpsee/pull/455)

- Proc macro generates documentation for trait methods. [#453](https://github.com/paritytech/jsonrpsee/pull/453)

- Fix errors with generics when using the proc macro [#433](https://github.com/paritytech/jsonrpsee/pull/433)

- WS client uses query part of the URL [#429](https://github.com/paritytech/jsonrpsee/pull/429)


### [Removed]

- Remove rustls [#502](https://github.com/paritytech/jsonrpsee/pull/502)

- Remove cors_max_age [#466](https://github.com/paritytech/jsonrpsee/pull/466)

- Remove support for tokio 0.2 runtimes [#432](https://github.com/paritytech/jsonrpsee/pull/432)


## [v0.3.0] – 2021-07-12

[changed] Module API refactor [#412](https://github.com/paritytech/jsonrpsee/pull/412)

[changed] Pass owned `RpcParams` to async methods [#410](https://github.com/paritytech/jsonrpsee/pull/410)

[changed] Re-work re-exported types for clarity and consistency [#409](https://github.com/paritytech/jsonrpsee/pull/409)

[changed] All requests time out [#406](https://github.com/paritytech/jsonrpsee/pull/406)

[changed] Streaming `RpcParams` parsing for optional arguments [#401](https://github.com/paritytech/jsonrpsee/pull/401)

[changed] Set allowed Host header values [#399](https://github.com/paritytech/jsonrpsee/pull/399)

[changed] Terminate already established ws connection(s) when the server is stopped [#396](https://github.com/paritytech/jsonrpsee/pull/396)

[added] Customizable JSON-RPC error codes via new enum variant on `CallErrror` [#394](https://github.com/paritytech/jsonrpsee/pull/394)

[changed] Unify a few types and more tests [#389](https://github.com/paritytech/jsonrpsee/pull/389)

[changed] Synchronization-less async connections in ws-server [#388](https://github.com/paritytech/jsonrpsee/pull/388)

[added] Server proc macros [#387](https://github.com/paritytech/jsonrpsee/pull/387)

[added] Add a way to stop servers [#386](https://github.com/paritytech/jsonrpsee/pull/386)

[changed] Refactor benchmarks to use Criterion's async bencher [#385]https://github.com/paritytech/jsonrpsee/pull/385)

[added] Support RPC method aliases and make `RpcModule` be `Clone` [#383]https://github.com/paritytech/jsonrpsee/pull/383)

[added] CORS support and use `soketto` v0.6 [#375](https://github.com/paritytech/jsonrpsee/pull/375)

[changed] Ws switch from sending TEXT instead of BINARY [#374](https://github.com/paritytech/jsonrpsee/pull/374)

[added] Benchmarks for async methods and subscriptions [#372](https://github.com/paritytech/jsonrpsee/pull/372)


## [v0.2.0] – 2021-06-04

[changed] The crate structure changed to several smaller crates, enabling users to pick and choose. The `jsonrpsee` crate works as a façade crate for users to pick&chose what components they wish to use.

[changed] Starting with this release, the project is assuming `tokio` is the async executor.

[changed] Revamped RPC subscription/method definition: users now provide closures when initializing the server and it is no longer possible to register new methods after the server started.

[changed] Refactored the internals from the ground up.

[added] Support for async methods

[added] Support for batch requests (http/ws)

[changed] the proc macros are currently limited to client side.

[added] crate publication script

## [v0.1.0] - 2020-02-28
