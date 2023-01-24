// Copyright 2022 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Example how to use `tokio-console` to debug async tasks `jsonrpsee`.
//! For further information see https://docs.rs/console-subscriber.
//!
//! To run it:
//! `$ cargo install --locked tokio-console`
//! `$ RUSTFLAGS="--cfg tokio_unstable" cargo run --example tokio_console`
//! `$ tokio-console`
//!
//! It will start a server on http://127.0.0.1:6669 for `tokio-console` to connect to.

use std::net::SocketAddr;

use jsonrpsee::core::Error;
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::RpcModule;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	console_subscriber::init();

	let _ = run_server().await?;

	futures::future::pending().await
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = ServerBuilder::default().build("127.0.0.1:9944").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("lo"))?;
	module.register_method("memory_call", |_, _| Ok("A".repeat(1024 * 1024)))?;
	module.register_async_method("sleep", |_, _| async {
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;
		Result::<_, Error>::Ok("lo")
	})?;

	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	// In this example we don't care about doing a stopping the server so let it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
