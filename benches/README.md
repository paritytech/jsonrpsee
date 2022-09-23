# jsonrpsee benchmarks

This crate contains benchmarks mainly to test the server implementations of some common scenarios such as concurrent connections.
Further, running these will open lots of sockets and file descriptors.

Note that on MacOS inparticular, you may need to increase some limits to be
able to open a large number of connections. Try commands like:

```sh
sudo sysctl -w kern.maxfiles=100000
sudo sysctl -w kern.maxfilesperproc=100000
ulimit -n 100000
sudo sysctl -w kern.ipc.somaxconn=100000
sudo sysctl -w kern.ipc.maxsockbuf=16777216
```

In general, if you run into issues, it may be better to run this on a linux
box; MacOS seems to hit limits quicker in general.

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

## Measurement time of benchmarks

Some of the benchmarks are quite expensive to run and doesn't run with enough samples with the default values
provided by criterion. Currently the default values are very conversative which can be modified by the following environment variables:

    - "SLOW_MEASUREMENT_TIME" - sets the measurement time for slow benchmarks (default is 250 seconds)
    - "MEASUREMENT_TIME" - sets the measurement time for fast benchmarks (default is 10 seconds)