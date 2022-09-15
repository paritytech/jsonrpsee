// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

//! Example showing how to add multiple loggers to the same server.

use std::net::SocketAddr;
use std::process::Command;
use std::time::Instant;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::server::logger::{HttpRequest, MethodKind};
use jsonrpsee::server::{logger, RpcModule, ServerBuilder};
use jsonrpsee::types::Params;
use jsonrpsee::ws_client::WsClientBuilder;

/// Example logger to measure call execution time.
#[derive(Clone)]
struct Timings;

impl logger::Logger for Timings {
	type Instant = Instant;

	fn on_connect(&self, remote_addr: SocketAddr, req: &HttpRequest) {
		println!("[Timings::on_connect] remote_addr {:?}, req: {:?}", remote_addr, req);
	}

	fn on_request(&self) -> Self::Instant {
		Instant::now()
	}

	fn on_call(&self, name: &str, params: Params, kind: MethodKind) {
		println!("[Timings:on_call] method: '{}', params: {:?}, kind: {}", name, params, kind);
	}

	fn on_result(&self, name: &str, success: bool, started_at: Self::Instant) {
		println!("[Timings] call={}, worked? {}, duration {:?}", name, success, started_at.elapsed());
	}

	fn on_response(&self, _result: &str, started_at: Self::Instant) {
		println!("[Timings] Response duration {:?}", started_at.elapsed());
	}

	fn on_disconnect(&self, remote_addr: SocketAddr) {
		println!("[Timings::on_disconnect] remote_addr: {:?}", remote_addr);
	}
}

/// Example logger to keep a watch on the number of total threads started in the system.
#[derive(Clone)]
struct ThreadWatcher;

impl ThreadWatcher {
	// Count the number of threads visible to this process. Counts the lines of `ps -eL` and equivalent minus one (the header).
	// Cribbed from the `palaver` crate.
	fn count_threads() -> usize {
		let out = if cfg!(any(target_os = "linux", target_os = "android")) {
			Command::new("ps").arg("-eL").output().expect("failed to execute process")
		} else if cfg!(any(target_os = "macos", target_os = "ios")) {
			Command::new("ps").arg("-eM").output().expect("failed to execute process")
		} else {
			unimplemented!()
		};
		out.stdout.split(|&x| x == b'\n').skip(1).filter(|x| !x.is_empty()).count()
	}
}

impl logger::Logger for ThreadWatcher {
	type Instant = isize;

	fn on_connect(&self, remote_addr: SocketAddr, headers: &HttpRequest) {
		println!("[ThreadWatcher::on_connect] remote_addr {:?}, headers: {:?}", remote_addr, headers);
	}

	fn on_call(&self, _method: &str, _params: Params, _kind: MethodKind) {
		let threads = Self::count_threads();
		println!("[ThreadWatcher::on_call] Threads running on the machine at the start of a call: {}", threads);
	}

	fn on_request(&self) -> Self::Instant {
		let threads = Self::count_threads();
		println!("[ThreadWatcher::on_request] Threads running on the machine at the start of a call: {}", threads);
		threads as isize
	}

	fn on_result(&self, _name: &str, _succees: bool, started_at: Self::Instant) {
		let current_nr_threads = Self::count_threads() as isize;
		println!("[ThreadWatcher::on_result] {} threads", current_nr_threads - started_at);
	}

	fn on_response(&self, _result: &str, started_at: Self::Instant) {
		let current_nr_threads = Self::count_threads() as isize;
		println!("[ThreadWatcher::on_response] {} threads", current_nr_threads - started_at);
	}

	fn on_disconnect(&self, remote_addr: SocketAddr) {
		println!("[ThreadWatcher::on_disconnect] remote_addr: {:?}", remote_addr);
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;
	let response: String = client.request("say_hello", rpc_params![]).await?;
	println!("response: {:?}", response);
	let _response: Result<String, _> = client.request("unknown_method", rpc_params![]).await;
	let _: String = client.request("say_hello", rpc_params![]).await?;
	client.request("thready", rpc_params![4]).await?;

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = ServerBuilder::new().set_logger((Timings, ThreadWatcher)).build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("lo"))?;
	module.register_method("thready", |params, _| {
		let thread_count: usize = params.one().unwrap();
		for _ in 0..thread_count {
			std::thread::spawn(|| std::thread::sleep(std::time::Duration::from_secs(1)));
		}
		Ok(())
	})?;
	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
