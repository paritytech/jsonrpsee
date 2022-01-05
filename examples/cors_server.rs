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

use std::net::SocketAddr;

use jsonrpsee::http_server::{AccessControlBuilder, HttpServerBuilder, HttpServerHandle, RpcModule};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	// Start up a JSONPRC server that allows cross origin requests.
	let (server_addr, _handle) = run_server().await?;

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

async fn run_server() -> anyhow::Result<(SocketAddr, HttpServerHandle)> {
	let acl = AccessControlBuilder::new().allow_all_headers().allow_all_origins().allow_all_hosts().build();

	let server = HttpServerBuilder::default().set_access_control(acl).build("127.0.0.1:0".parse::<SocketAddr>()?)?;

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| {
		println!("say_hello method called!");
		Ok("Hello there!!")
	})?;

	let addr = server.local_addr()?;
	let server_handle = server.start(module)?;

	Ok((addr, server_handle))
}
