# jsonrpsee

JSON-RPC library designed for async/await in Rust.

Designed to be the successor to [ParityTech's JSONRPC crate](https://github.com/paritytech/jsonrpc/).

Support `WebSocket` and `HTTP` transports for both client and server.

## Under development

The library is still under development; do not use in production.

## Sub-projects
- [jsonrpsee-http-client](./http-client) [![crates.io][http-client-image]][http-client-url]
- [jsonrpsee-http-server](./http-server) [![crates.io][http-server-image]][http-server-url]
- [jsonrpsee-proc-macros](./proc-macros) [![crates.io][proc-macros-image]][proc-macros-url]
- [jsonrpsee-ws-client](./ws-client) [![crates.io][ws-client-image]][ws-client-url]
- [jsonrpsee-ws-server](./http-server) [![crates.io][ws-server-image]][ws-server-url]

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

- [HTTP](./examples/http.rs)
- [WebSocket](./examples/ws.rs)
- [WebSocket pubsub](./examples/ws_subscription.rs)
- [API generation with proc macro](./examples/proc_macro.rs)

## Roadmap

See [tracking issue for next stable release](https://github.com/paritytech/jsonrpsee/issues/251)

## Users

If your project uses `jsonrpsee` we would like to know. Please open a pull request and add your project to the list below:
- [substrate-subxt](https://github.com/paritytech/substrate-subxt)
- [parity bridges common](https://github.com/paritytech/parity-bridges-common)
- [remote externalities](https://github.com/paritytech/substrate/tree/master/utils/frame/remote-externalities)
