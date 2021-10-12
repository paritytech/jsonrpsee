# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

## [Unreleased]

## [v0.4.0] – 2021-10-12

[changed] [http server]: use tokio::spawn internally in `HttpServer::start` and return `StopHandle` [#402](https://github.com/paritytech/jsonrpsee/pull/402)

[changed] Proc macro Argument parsing should permit commas inside angle brackets [#509](https://github.com/paritytech/jsonrpsee/pull/509)

[changed] ParamsSer::NoParams no more [#508](https://github.com/paritytech/jsonrpsee/pull/508)

[changed] [http server]: use similar API for host and origin filtering as `WS` [#473](https://github.com/paritytech/jsonrpsee/pull/473)

[changed] remove `ParamsSer::NoParams` [#501](https://github.com/paritytech/jsonrpsee/pull/501)

[changed] improve SubscriptionClosed error [#504](https://github.com/paritytech/jsonrpsee/pull/504)

[changed] Resource Limiting [#500](https://github.com/paritytech/jsonrpsee/pull/500)

[changed] fix http client bench with request limit [#506](https://github.com/paritytech/jsonrpsee/pull/506)

[changed] ws client redirections [#397](https://github.com/paritytech/jsonrpsee/pull/397)

[changed] deps: remove rustls [#502](https://github.com/paritytech/jsonrpsee/pull/502)

[changed] Update soketto requirement from 0.6 to 0.7 [#493](https://github.com/paritytech/jsonrpsee/pull/493)

[changed] expose the `rpc_params` macro to both http and ws clients [#498](https://github.com/paritytech/jsonrpsee/pull/498)

[changed] Improve feature configuration [#494](https://github.com/paritytech/jsonrpsee/pull/494)

[changed] Add macro to build params [#496](https://github.com/paritytech/jsonrpsee/pull/496)

[changed] Unbox async futures [#495](https://github.com/paritytech/jsonrpsee/pull/495)

[changed] Un-ignore test that is flaky on windows [#491](https://github.com/paritytech/jsonrpsee/pull/491)

[changed] Share the request id code between the http and websocket clients [#490](https://github.com/paritytech/jsonrpsee/pull/490)

[changed] wrapper struct for test subscription [#489](https://github.com/paritytech/jsonrpsee/pull/489)

[changed] fix: ws server terminate subscriptions when connection is closed by the client. [#483](https://github.com/paritytech/jsonrpsee/pull/483)

[changed] less deps [#484](https://github.com/paritytech/jsonrpsee/pull/484)

[changed] examples: remove weather [#479](https://github.com/paritytech/jsonrpsee/pull/479)

[changed] [ws client]: default subscription buffer 1024 [#475](https://github.com/paritytech/jsonrpsee/pull/475)

[changed] Ignore troublesome test on windows [#471](https://github.com/paritytech/jsonrpsee/pull/471)

[changed] Re-export `v2` submodules [#469](https://github.com/paritytech/jsonrpsee/pull/469)

[changed] replace `array_impl macro` with const generics [#470](https://github.com/paritytech/jsonrpsee/pull/470)

[changed] Rename and reorg types [#462](https://github.com/paritytech/jsonrpsee/pull/462)

[changed] [http server]: export acl types + remove cors_max_age [#466](https://github.com/paritytech/jsonrpsee/pull/466)

[changed] Fix build warnings [#465](https://github.com/paritytech/jsonrpsee/pull/465)

[changed] Propagate cause of `InvalidParams` [#463](https://github.com/paritytech/jsonrpsee/pull/463)

[changed] Reject overflowing connection with status code 429 [#456](https://github.com/paritytech/jsonrpsee/pull/456)

[changed] Remove unstable rustfmt comment wrapping/limiting [#464](https://github.com/paritytech/jsonrpsee/pull/464)

[changed] [rpc module] test helper for calling and converting types to JSON-RPC params   [#458](https://github.com/paritytech/jsonrpsee/pull/458)

[changed] Let rustfmt wrap comments at the 120 width boundary [#461](https://github.com/paritytech/jsonrpsee/pull/461)

[changed] fix(proc macros): subscriptions must return result [#455](https://github.com/paritytech/jsonrpsee/pull/455)

[changed] cleanup after #453 [#454](https://github.com/paritytech/jsonrpsee/pull/454)

[changed] fix(proc macros): generate documentation for trait methods. [#453](https://github.com/paritytech/jsonrpsee/pull/453)

[changed] Tidy `StopHandle` [#425](https://github.com/paritytech/jsonrpsee/pull/425)

[changed] feat: alias attribute for proc macros [#442](https://github.com/paritytech/jsonrpsee/pull/442)

[changed] Make it possible to treat empty JSON response as no params [#446](https://github.com/paritytech/jsonrpsee/pull/446)

[changed] benches: add benchmark for concurrent connections [#430](https://github.com/paritytech/jsonrpsee/pull/430)

[changed] [proc macros]: support generic type params [#436](https://github.com/paritytech/jsonrpsee/pull/436)

[changed] Add license headers where missing and update year [#439](https://github.com/paritytech/jsonrpsee/pull/439)

[changed] Cleanup proc-macros [#438](https://github.com/paritytech/jsonrpsee/pull/438)

[changed] [proc macros] force proc macro api to return `Result` [#435](https://github.com/paritytech/jsonrpsee/pull/435)

[changed] Fix errors with generics when using the proc macro [#433](https://github.com/paritytech/jsonrpsee/pull/433)

[changed] Concurrent polling on async methods [#424](https://github.com/paritytech/jsonrpsee/pull/424)

[changed] Don't allocate until we know it's worth it [#420](https://github.com/paritytech/jsonrpsee/pull/420)

[changed] [clients]: remove tokio 0.2 runtime support [#432](https://github.com/paritytech/jsonrpsee/pull/432)

[changed] Sniff the first byte to glean if the incoming request is a single or batch request [#419](https://github.com/paritytech/jsonrpsee/pull/419)

[changed] [proc macros]: remove old code and tests. [#431](https://github.com/paritytech/jsonrpsee/pull/431)

[changed] fix most clippy warnings [#434](https://github.com/paritytech/jsonrpsee/pull/434)

[changed] deps(hyper): require 0.14.10 [#427](https://github.com/paritytech/jsonrpsee/pull/427)

[changed] fix(ws client): use query part of URL. [#429](https://github.com/paritytech/jsonrpsee/pull/429)

[changed] Proc macro params optimizations and tests. [#421](https://github.com/paritytech/jsonrpsee/pull/421)

[changed] Update env_logger requirement from 0.8 to 0.9 [#418](https://github.com/paritytech/jsonrpsee/pull/418)


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
