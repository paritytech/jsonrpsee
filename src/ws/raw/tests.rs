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

use crate::client::WsTransportClient;
use crate::common::{self, Call, MethodCall, Notification, Params, Request, Version};
use crate::ws::{RawWsServer, RawWsServerEvent, WsTransportServer};
use jsonrpsee_test_utils::helpers::*;
use serde_json::Value;
use std::net::SocketAddr;

async fn raw_server() -> (RawWsServer, SocketAddr) {
	let server = WsTransportServer::builder("127.0.0.1:0".parse().unwrap()).build().await.unwrap();
	let addr = *server.local_addr();
	(server.into(), addr)
}

#[tokio::test]
async fn request_work() {
	let (mut server, server_addr) = raw_server().await;

	tokio::spawn(async move {
		let mut client = WsTransportClient::new(&to_ws_uri_string(server_addr)).await.unwrap();
		let call = Call::MethodCall(MethodCall {
			jsonrpc: Version::V2,
			method: "hello_world".to_owned(),
			params: Params::Array(vec![Value::from(1), Value::from(2)]),
			id: common::Id::Num(3),
		});
		client.send_request(Request::Single(call)).await.unwrap();
	});

	match server.next_event().await {
		RawWsServerEvent::Request(r) => {
			assert_eq!(r.method(), "hello_world");
			let p1: i32 = r.params().get(0).unwrap();
			let p2: i32 = r.params().get(1).unwrap();
			assert_eq!(p1, 1);
			assert_eq!(p2, 2);
			assert_eq!(r.request_id(), &common::Id::Num(3));
		}
		e @ _ => panic!("Invalid server event: {:?} expected Request", e),
	}
}

#[tokio::test]
async fn notification_work() {
	let (mut server, server_addr) = raw_server().await;

	tokio::spawn(async move {
		let mut client = WsTransportClient::new(&to_ws_uri_string(server_addr)).await.unwrap();
		let n = Notification {
			jsonrpc: Version::V2,
			method: "hello_world".to_owned(),
			params: Params::Array(vec![Value::from("lo"), Value::from(2)]),
		};
		client.send_request(Request::Single(Call::Notification(n))).await.unwrap();
	});

	match server.next_event().await {
		RawWsServerEvent::Notification(r) => {
			assert_eq!(r.method(), "hello_world");
			let p1: String = r.params().get(0).unwrap();
			let p2: i32 = r.params().get(1).unwrap();
			assert_eq!(p1, "lo");
			assert_eq!(p2, 2);
		}
		e @ _ => panic!("Invalid server event: {:?} expected Notification", e),
	}
}
