# Changelog

The format is based on [Keep a Changelog].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/

## [Unreleased]

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
