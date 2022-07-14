[![GitLab Status](https://gitlab.parity.io/parity/jsonrpsee/badges/master/pipeline.svg)](https://gitlab.parity.io/parity/jsonrpsee/pipelines)

# jsonrpsee

JSON-RPC library designed for async/await in Rust.

Designed to be the successor to [ParityTech's JSONRPC crate](https://github.com/paritytech/jsonrpc/).

Support `WebSocket` and `HTTP` transports for both client and server.

## Sub-projects
- [jsonrpsee-http-client](./client/http-client) [![crates.io][http-client-image]][http-client-url]
- [jsonrpsee-http-server](./http-server) [![crates.io][http-server-image]][http-server-url]
- [jsonrpsee-proc-macros](./proc-macros) [![crates.io][proc-macros-image]][proc-macros-url]
- [jsonrpsee-ws-client](./client/ws-client) [![crates.io][ws-client-image]][ws-client-url]
- [jsonrpsee-ws-server](./ws-server) [![crates.io][ws-server-image]][ws-server-url]

[http-client-image]: https://img.shields.io/crates/v/jsonrpsee-http-client.svg
[http-client-url]: https://crates.io/crates/jsonrpsee-http-client
[http-server-image]: https://img.shields.io/crates/v/jsonrpsee-http-server.svg
[http-server-url]: https://crates.io/crates/jsonrpsee-http-server
[proc-macros-url]: https://crates.io/crates/jsonrpsee-proc-macros
[proc-macros-image]: https://img.shields.io/crates/v/jsonrpsee-proc-macros.svg
[ws-client-image]: https://img.shields.io/crates/v/jsonrpsee-ws-client.svg
[ws-client-url]: https://crates.io/crates/jsonrpsee-ws-client
[ws-server-image]: https://img.shields.io/crates/v/jsonrpsee-ws-server.svg
[ws-server-url]: https://crates.io/crates/jsonrpsee-ws-server

## Examples

- [HTTP](./examples/examples/http.rs)
- [WebSocket](./examples/examples/ws.rs)
- [WebSocket pubsub](./examples/examples/ws_pubsub_broadcast.rs)
- [API generation with proc macro](./examples/examples/proc_macro.rs)
- [Middleware](./examples/examples/multi_middleware.rs)
- [CORS server](./examples/examples/cors_server.rs)
- [Core client](./examples/examples/core_client.rs)

## Roadmap

See [our tracking milestone](https://github.com/paritytech/jsonrpsee/milestone/2) for the upcoming stable v1.0 release.

## Users

If your project uses `jsonrpsee` we would like to know. Please open a pull request and add your project to the list below:
- [subxt](https://github.com/paritytech/subxt)
- [parity bridges common](https://github.com/paritytech/parity-bridges-common)
- [remote externalities](https://github.com/paritytech/substrate/tree/master/utils/frame/remote-externalities)
