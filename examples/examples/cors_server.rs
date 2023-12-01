// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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

//! This example adds upstream CORS layers to the RPC service,
//! with access control allowing requests from all hosts.

use hyper::Method;
use jsonrpsee::server::{RpcModule, Server};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	// Start up a JSON-RPC server that allows cross origin requests.
	let server_addr = run_server().await?;

	// Print instructions for testing CORS from a browser.
	println!("Run the following snippet in the developer console in any Website.");
	println!(
		r#"
        fetch("http://{}", {{
            method: 'POST',
            mode: 'cors',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify({{
                jsonrpc: '2.0',
                method: 'say_hello',
                id: 1
            }})
        }}).then(res => {{
            console.log("Response:", res);
            return res.text()
        }}).then(body => {{
            console.log("Response Body:", body)
        }});
    "#,
		server_addr
	);

	futures::future::pending().await
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	// Add a CORS middleware for handling HTTP requests.
	// This middleware does affect the response, including appropriate
	// headers to satisfy CORS. Because any origins are allowed, the
	// "Access-Control-Allow-Origin: *" header is appended to the response.
	let cors = CorsLayer::new()
		// Allow `POST` when accessing the resource
		.allow_methods([Method::POST])
		// Allow requests from any origin
		.allow_origin(Any)
		.allow_headers([hyper::header::CONTENT_TYPE]);
	let middleware = tower::ServiceBuilder::new().layer(cors);

	// The RPC exposes the access control for filtering and the middleware for
	// modifying requests / responses. These features are independent of one another
	// and can also be used separately.
	// In this example, we use both features.
	let server = Server::builder().set_http_middleware(middleware).build("127.0.0.1:0".parse::<SocketAddr>()?).await?;

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| {
		println!("say_hello method called!");
		"Hello there!!"
	})?;

	let addr = server.local_addr()?;
	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
