# jsonrpsee

JSON-RPC library designed for async/await in Rust.

Designed to be the successor to [ParityTech's JSONRPC crate](https://github.com/paritytech/jsonrpc/).

Supports `WebSocket` and `HTTP` transports for both client-side and server-ide.

## Under development

The library is still under development and do not use in production.

## Sub-projects
- [jsonrpsee-http-client](./http-client) [![crates.io][ws-client-image]][ws-client-url]
- [jsonrpsee-http-server UNSTABLE/NOT RELEASED](./http-server) [![crates.io][http-server-image]][http-server-url]
- [jsonrpsee-proc-macros](./proc-macros) [![crates.io][proc-macros-image]][proc-macros-url]
- [jsonrpsee-ws-client](./ws-client) [![crates.io][ws-client-image]][ws-client-url]
- [jsonrpsee-ws-server UNSTABLE/NOT RELEASED](./http-server) [![crates.io][http-server-image]][http-server-url]


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
- [WebSocket pubsub](./examples/examples/ws_subscription.rs)

## Roadmap

See [tracking issue for next stable release](https://github.com/paritytech/jsonrpsee/issues/251)

## Users

If your project uses `jsonrpsee` we like to know please open a pull request and add your project to the list below:
- [substrate-subxt](https://github.com/paritytech/substrate-subxt)
