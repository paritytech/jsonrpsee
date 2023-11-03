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

//! Example middleware to rate limit based on the number
//! JSON-RPC calls.
//!
//! As demonstrated in this example any state must be
//! stored in something to provide interior mutability
//! such as `Arc<Mutex>`

use jsonrpsee::core::{async_trait, client::ClientT};
use jsonrpsee::server::middleware::rpc::{RpcServiceBuilder, RpcServiceT, TransportProtocol};
use jsonrpsee::server::Server;
use jsonrpsee::types::{ErrorObject, Request};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, MethodResponse, RpcModule};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone)]
struct Rate {
	num: u64,
	period: Duration,
}

#[derive(Debug, Copy, Clone)]
enum State {
	Deny { until: Instant },
	Allow { until: Instant, rem: u64 },
}

/// Depending on how the rate limit is instantiated
/// it's possible to select whether the rate limit
/// is be applied per connection or shared by
/// all connections.
///
/// Have a look at `async fn run_server` below which
/// shows how do it.
#[derive(Clone)]
pub struct RateLimit<S> {
	service: S,
	state: Arc<Mutex<State>>,
	rate: Rate,
}

impl<S> RateLimit<S> {
	fn new(service: S, rate: Rate) -> Self {
		let period = rate.period;
		let num = rate.num;

		Self {
			service,
			rate,
			state: Arc::new(Mutex::new(State::Allow { until: Instant::now() + period, rem: num + 1 })),
		}
	}
}

#[async_trait]
impl<'a, S> RpcServiceT<'a> for RateLimit<S>
where
	S: Send + Sync + RpcServiceT<'a>,
{
	async fn call(&self, req: Request<'a>, t: TransportProtocol) -> MethodResponse {
		let now = Instant::now();

		let is_denied = {
			let mut lock = self.state.lock().unwrap();
			let next_state = match *lock {
				State::Deny { until } => {
					if now > until {
						State::Allow { until: now + self.rate.period, rem: self.rate.num - 1 }
					} else {
						State::Deny { until }
					}
				}
				State::Allow { until, rem } => {
					if now > until {
						State::Allow { until: now + self.rate.period, rem: self.rate.num - 1 }
					} else {
						let n = rem - 1;
						if n > 0 {
							State::Allow { until: now + self.rate.period, rem: n }
						} else {
							State::Deny { until }
						}
					}
				}
			};

			*lock = next_state;
			matches!(next_state, State::Deny { .. })
		};

		if is_denied {
			MethodResponse::error(req.id, ErrorObject::borrowed(-32000, "RPC rate limit", None))
		} else {
			self.service.call(req, t).await
		}
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

	let client1 = WsClientBuilder::default().build(&url).await?;
	let _response: String = client1.request("say_hello", rpc_params![]).await?;

	// The rate limit should trigger an error here.
	let _response = client1.request::<String, _>("unknown_method", rpc_params![]).await.unwrap_err();

	// Make a new connection and the server will allow it because our `RateLimit`
	// applies per connection and not globally on the server.
	let client2 = WsClientBuilder::default().build(&url).await?;
	let _response: String = client2.request("say_hello", rpc_params![]).await?;

	// The first connection should allow a call now again.
	tokio::time::sleep(Duration::from_secs(2)).await;
	let _response: String = client1.request("say_hello", rpc_params![]).await?;

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	// This will create a new `RateLimit` per connection.
	//
	// In this particular example the server will only
	// allow one RPC call per second.
	//
	// Have a look at the `rpc_middleware example` if you want see an example
	// how to share state of the "middleware" for all connections on the server.
	let rpc_middleware = RpcServiceBuilder::new()
		.layer_fn(|service| RateLimit::new(service, Rate { num: 1, period: Duration::from_secs(1) }));

	let server = Server::builder().set_rpc_middleware(rpc_middleware).build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| "lo")?;
	module.register_method("say_goodbye", |_, _| "goodbye")?;
	let addr = server.local_addr()?;

	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
