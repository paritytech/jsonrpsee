# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

## [Unreleased]

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
- deps: require tokio ^1.8  [#586](https://github.com/paritytech/jsonrpsee/pull/586)


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

-  fix: nit in ServerBuilder::custom_tokio_runtime [#512](https://github.com/paritytech/jsonrpsee/pull/512)

## [v0.4.0] – 2021-10-12

The v0.4 release is a breaking change.

### [Added]

- Document resource limiting (#510)

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
