# jsonrpsee benchmarks

This crate contains benchmarks mainly to test the server implementations of some common scenarios such as concurrent connections.
Further, running these will open lots of sockets and file descriptors, it doesn't work well on macOS for instance.

Make sure that ulimit on your system is bigger than 1024.

## Run all benchmarks

`$ cargo bench`

It's also possible to run individual benchmarks by:

`$ cargo bench --bench bench jsonrpsee_types_v2_array_ref`

## Run all benchmarks against [jsonrpc crate servers](https://github.com/paritytech/jsonrpc/)

`$ cargo bench --features jsonpc-crate`

## Run CPU profiling on the benchmarks

This will generate a flamegraph for the specific benchmark in `./target/criterion/<your benchmark>/profile/flamegraph.svg`.

`$ cargo bench --bench bench -- --profile-time=60`

It's also possible to run profiling on individual benchmarks by:

`$ cargo bench --bench bench -- --profile-time=60 sync/http_concurrent_conn_calls/1024`
