# jsonrpsee

[![GitLab Status](https://gitlab.parity.io/parity/jsonrpsee/badges/master/pipeline.svg)](https://gitlab.parity.io/parity/jsonrpsee/pipelines)
[![crates.io](https://img.shields.io/crates/v/jsonrpsee)](https://crates.io/crates/jsonrpsee)
[![Docs](https://docs.rs/jsonrpsee/badge.svg)](https://docs.rs/jsonrpsee)
![MIT](https://img.shields.io/crates/l/jsonrpsee.svg)
[![CI](https://github.com/paritytech/jsonrpsee/actions/workflows/ci.yml/badge.svg)](https://github.com/paritytech/jsonrpsee/actions/workflows/ci.yml)
[![Benchmarks](https://github.com/paritytech/jsonrpsee/actions/workflows/benchmarks.yml/badge.svg)](https://github.com/paritytech/jsonrpsee/actions/workflows/benchmarks.yml)
[![dependency status](https://deps.rs/crate/jsonrpsee/0.16.1/status.svg)](https://deps.rs/crate/jsonrpsee/0.16.1)

JSON-RPC library designed for async/await in Rust.

Designed to be the successor to [ParityTech's JSONRPC crate](https://github.com/paritytech/jsonrpc/).

## Features
- Client/server HTTP/HTTP2 support
- Client/server WebSocket support
- Client WASM support via web-sys
- Client transport abstraction to provide custom transports
- Middleware
- Logger

## Documentation
- [API Documentation](https://docs.rs/jsonrpsee)

## Examples

- [HTTP](./examples/examples/http.rs)
- [WebSocket](./examples/examples/ws.rs)
- [WebSocket pubsub](./examples/examples/ws_pubsub_broadcast.rs)
- [API generation with proc macro](./examples/examples/proc_macro.rs)
- [Logger](./examples/examples/multi_logger.rs)
- [CORS server](./examples/examples/cors_server.rs)
- [Core client](./examples/examples/core_client.rs)
- [HTTP proxy middleware](./examples/examples/http_proxy_middleware.rs)

See [this directory](./examples/examples) for more examples

## Roadmap

See [our tracking milestone](https://github.com/paritytech/jsonrpsee/milestone/2) for the upcoming stable v1.0 release.

## Users

If your project uses `jsonrpsee` we would like to know. Please open a pull request and add your project to the list below:
- [subxt](https://github.com/paritytech/subxt)
- [parity bridges common](https://github.com/paritytech/parity-bridges-common)
- [remote externalities](https://github.com/paritytech/substrate/tree/master/utils/frame/remote-externalities)
- [substrate](https://github.com/paritytech/substrate)

## Benchmarks

Daily benchmarks for jsonrpsee can be found:
- Github action machine: <https://paritytech.github.io/jsonrpsee/bench/dev>
- Gitlab machine (experimental): <https://paritytech.github.io/jsonrpsee/bench/dev2>
