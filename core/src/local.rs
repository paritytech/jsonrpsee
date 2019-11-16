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

//! Implementation of a [`RawClient`](crate::RawClient) and a [`RawServer`](crate::RawServer)
//! that communicate through a memory channel.
//!
//! # Usage
//!
//! Call the [`local_raw`](crate::local_raw()) function to build a set of a client and a server.
//!
//! The [`LocalRawClient`](crate::local::LocalRawClient) is clonable.
//!
//! ```
//! use jsonrpsee_core::client::Client;
//! use jsonrpsee_core::server::{Server, ServerEvent};
//!
//! let (raw_client, raw_server) = jsonrpsee_core::local_raw();
//! let mut client = Client::new(raw_client);
//! let mut server = Server::new(raw_server);
//!
//! async_std::task::spawn(async move {
//!     loop {
//!         match server.next_event().await {
//!             ServerEvent::Request(request) => {
//!                 request.respond(Ok(From::from("hello".to_owned()))).await;
//!             },
//!             _ => {}
//!         }
//!     }
//! });
//!
//! let rq: String = futures::executor::block_on(async move {
//!     let request_id = client.start_request("test", jsonrpsee_core::common::Params::None).await.unwrap();
//!     jsonrpsee_core::common::from_value(client.request_by_id(request_id).unwrap().await.unwrap())
//! }).unwrap();
//! println!("result: {:?}", rq);
//! ```
//!

use crate::{common, RawClient, RawServer, RawServerEvent};
use err_derive::*;
use fnv::FnvHashSet;
use futures::{channel::mpsc, prelude::*};
use std::{fmt, pin::Pin};

/// Builds a new client and a new server that are connected to each other.
pub fn local_raw() -> (LocalRawClient, LocalRawServer) {
    let (to_server, from_client) = mpsc::channel(4);
    let (to_client, from_server) = mpsc::channel(4);
    let client = LocalRawClient {
        to_server,
        from_server,
    };
    let server = LocalRawServer {
        to_client,
        from_client,
        next_request_id: 0,
        requests: Default::default(),
    };
    (client, server)
}

/// Client connected to a [`LocalRawServer`]. Can be created using [`local_raw`].
///
/// Can be cloned in order to have multiple clients connected to the same server.
// TODO: restore #[derive(Clone)])
pub struct LocalRawClient {
    /// Channel to the server.
    to_server: mpsc::Sender<common::Request<'static>>,
    /// Channel from the server.
    from_server: mpsc::Receiver<common::Response<'static>>,
}

/// Server connected to a [`LocalRawClient`]. Can be created using [`local_raw`].
pub struct LocalRawServer {
    /// Channel to the client.
    to_client: mpsc::Sender<common::Response<'static>>,
    /// Channel from the client.
    from_client: mpsc::Receiver<common::Request<'static>>,
    /// Id of the next request to insert in the `requests` hashset.
    next_request_id: u64,
    /// List of requests waiting for an answer.
    requests: FnvHashSet<u64>,
}

/// Error that can happen on the client side.
#[derive(Debug, Error)]
pub enum LocalRawClientErr {
    /// The [`LocalRawServer`] no longer exists.
    #[error(display = "Server has been closed")]
    ServerClosed,
}

impl RawClient for LocalRawClient {
    type Error = LocalRawClientErr;

    fn send_request<'a>(
        &'a mut self,
        request: &'a common::Request<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        Box::pin(async move {
            self.to_server
                .send(request.into_owned())
                .await
                .map_err(|_| LocalRawClientErr::ServerClosed)?;
            Ok(())
        })
    }

    fn next_response<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<common::Response<'static>, Self::Error>> + Send + 'a>> {
        Box::pin(async move {
            self.from_server
                .next()
                .await
                .ok_or(LocalRawClientErr::ServerClosed)
        })
    }
}

impl fmt::Debug for LocalRawClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("LocalRawClient").finish()
    }
}

impl RawServer for LocalRawServer {
    type RequestId = u64;

    fn next_request<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = RawServerEvent<Self::RequestId>> + Send + 'a>> {
        Box::pin(async move {
            let request = match self.from_client.next().await {
                Some(v) => v,
                None => {
                    if let Some(rq_id) = self.requests.iter().cloned().next() {
                        self.requests.remove(&rq_id);
                        return RawServerEvent::Closed(rq_id);
                    } else {
                        loop {
                            futures::pending!()
                        }
                    }
                }
            };

            loop {
                let id = self.next_request_id;
                self.next_request_id = self.next_request_id.wrapping_add(1);
                if !self.requests.insert(id) {
                    continue;
                }
                return RawServerEvent::Request { id, request };
            }
        })
    }

    fn finish<'a>(
        &'a mut self,
        request_id: &'a Self::RequestId,
        response: Option<&'a common::Response>,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        Box::pin(async move {
            if self.requests.remove(&request_id) {
                if let Some(response) = response {
                    self.to_client.send(response.to_owned()).await.map_err(|_| ())
                } else {
                    Ok(())
                }
            } else {
                Err(())
            }
        })
    }

    fn supports_resuming(&self, request_id: &Self::RequestId) -> Result<bool, ()> {
        if self.requests.contains(request_id) {
            Ok(true)
        } else {
            Err(())
        }
    }

    fn send<'a>(
        &'a mut self,
        request_id: &'a Self::RequestId,
        response: &'a common::Response,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        Box::pin(async move {
            if self.requests.contains(&request_id) {
                self.to_client.send(response.to_owned()).await.map_err(|_| ())
            } else {
                Err(())
            }
        })
    }
}

impl fmt::Debug for LocalRawServer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LocalRawServer").finish()
    }
}
