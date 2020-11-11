// Copyright 2019 Parity Technologies (UK) Ltd.
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

#![cfg(test)]

use crate::client::HttpTransportClient;
use crate::http::{HttpRawServer, HttpRawServerEvent, HttpTransportServer};
use crate::types::jsonrpc::{self, Call, MethodCall, Notification, Params, Request, Version};
use serde_json::Value;

async fn connection_context() -> (HttpTransportClient, HttpRawServer) {
	let server = HttpTransportServer::new(&"127.0.0.1:0".parse().unwrap()).await.unwrap();
	let uri = format!("http://{}", server.local_addr());
	let client = HttpTransportClient::new(&uri, 10 * 1024 * 1024).unwrap();
	(client, server.into())
}

// TODO(niklasad1): fix before eventual merge
#[tokio::test]
#[ignore]
async fn request_work() {
	let (client, mut server) = connection_context().await;
	tokio::spawn(async move {
		let call = Call::MethodCall(MethodCall {
			jsonrpc: Version::V2,
			method: "hello_world".to_owned(),
			params: Params::Array(vec![Value::from(1), Value::from(2)]),
			id: jsonrpc::Id::Num(3),
		});
		client.send_request_and_wait_for_response(Request::Single(call)).await.unwrap();
	});

	match server.next_event().await {
		HttpRawServerEvent::Request(r) => {
			assert_eq!(r.method(), "hello_world");
			let p1: i32 = r.params().get(0).unwrap();
			let p2: i32 = r.params().get(1).unwrap();
			assert_eq!(p1, 1);
			assert_eq!(p2, 2);
			assert_eq!(r.request_id(), &jsonrpc::Id::Num(3));
		}
		e @ _ => panic!("Invalid server event: {:?} expected Request", e),
	}
}

// TODO(niklasad1): fix before eventual merge
#[tokio::test]
#[ignore]
async fn notification_work() {
	let (client, mut server) = connection_context().await;
	tokio::spawn(async move {
		let n = Notification {
			jsonrpc: Version::V2,
			method: "hello_world".to_owned(),
			params: Params::Array(vec![Value::from("lo"), Value::from(2)]),
		};
		client.send_request_and_wait_for_response(Request::Single(Call::Notification(n))).await.unwrap();
	});

	match server.next_event().await {
		HttpRawServerEvent::Notification(r) => {
			assert_eq!(r.method(), "hello_world");
			let p1: String = r.params().get(0).unwrap();
			let p2: i32 = r.params().get(1).unwrap();
			assert_eq!(p1, "lo");
			assert_eq!(p2, 2);
		}
		e @ _ => panic!("Invalid server event: {:?} expected Notification", e),
	}
}
