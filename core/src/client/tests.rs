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

use crate::common;
use crate::local_raw;
use crate::server::raw::{RawServer, RawServerEvent};
use crate::{client::ClientError, Client};

#[test]
fn basic_request_works() {
    let (raw_client, mut raw_server) = local_raw();
    let mut client = Client::new(raw_client);

    async_std::task::spawn(async move {
        if let RawServerEvent::Request { id, request } = raw_server.next_request().await {
            let client_rq_id = request
                .into_single()
                .unwrap()
                .into_method_call()
                .unwrap()
                .id;
            raw_server.finish(
                &id,
                Some(&common::Response::Single(common::Output::Success(
                    common::Success {
                        jsonrpc: common::Version::V2,
                        result: common::JsonValue::String(From::from("foo")),
                        id: client_rq_id,
                    },
                ))),
            );
        }
    });

    async_std::task::block_on(async {
        let rq_id = client
            .start_request(
                "foo",
                common::Params::Array(vec![From::from("test"), From::from(12u32)]),
            )
            .await
            .unwrap();

        match client.request_by_id(rq_id).unwrap().await {
            Ok(rp) => {
                let rp = rp.into_output().unwrap().into_success().unwrap();
                assert_eq!(rp.result, common::JsonValue::String(From::from("foo")));
            }
            _ => panic!(),
        }
    })
}

#[test]
fn request_errors_if_connec_closed() {
    let (raw_client, mut raw_server) = local_raw();
    let mut client = Client::new(raw_client);

    async_std::task::spawn(async move {
        if let RawServerEvent::Request { id, .. } = raw_server.next_request().await {
            raw_server.finish(&id, None);
        }
    });

    async_std::task::block_on(async {
        let rq_id = client
            .start_request(
                "foo",
                common::Params::Array(vec![From::from("test"), From::from(12u32)]),
            )
            .await
            .unwrap();

        match client.request_by_id(rq_id).unwrap().await {
            Err(ClientError::Inner(_)) => {}
            _ => panic!(),
        }
    })
}

#[test]
fn request_errors_if_server_dead() {
    let (raw_client, mut raw_server) = local_raw();
    let mut client = Client::new(raw_client);

    async_std::task::spawn(async move {
        raw_server.next_request().await;
    });

    async_std::task::block_on(async {
        let rq_id = client
            .start_request(
                "foo",
                common::Params::Array(vec![From::from("test"), From::from(12u32)]),
            )
            .await
            .unwrap();

        match client.request_by_id(rq_id).unwrap().await {
            Err(ClientError::Inner(_)) => {}
            _ => panic!(),
        }
    })
}
