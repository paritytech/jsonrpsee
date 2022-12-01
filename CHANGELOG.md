# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

## [v0.16.2] - 2022-12-01

This release adds `Clone` and `Copy` implementations.

### [Fixed]

- fix(rpc module): make async closures Clone  ([#948](https://github.com/paritytech/jsonrpsee/pull/948))
- fix(ci): wasm tests  ([#946](https://github.com/paritytech/jsonrpsee/pull/946))

### [Added]

- add missing `Clone` and `Copy` impls  ([#951](https://github.com/paritytech/jsonrpsee/pull/951))
- TowerService should be clone-able for handling concurrent request  ([#950](https://github.com/paritytech/jsonrpsee/pull/950))

## [v0.16.1] - 2022-11-18

v0.16.1 is release that adds two new APIs to server `http_only` and `ws_only` to make it possible to allow only HTTP respectively WebSocket.

Both HTTP and WebSocket are still enabled by default.

### [Fixed]

- docs: remove outdated features ([#938](https://github.com/paritytech/jsonrpsee/pull/938))
- docs: http client url typo in examples ([#940](https://github.com/paritytech/jsonrpsee/pull/940))
- core: remove unused dependency `async-channel` ([#940](https://github.com/paritytech/jsonrpsee/pull/941))

### [Added]

- server: make it possible to enable ws/http only  ([#939](https://github.com/paritytech/jsonrpsee/pull/939))

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
-  [Bug Fix] - Incorrect trace caused by use of Span::enter in asynchronous code [#835](https://github.com/paritytech/jsonrpsee/pull/835)

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
- chore(deps): update pprof requirement from 0.7 to 0.8  [#732](https://github.com/paritytech/jsonrpsee/pull/732)
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
- ci: Create gitlab pipeline  [#534](https://github.com/paritytech/jsonrpsee/pull/534)

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
