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
//!         match server.next_event().await.unwrap() {
//!             ServerEvent::Request(request) => {
//!                 request.respond(Ok(From::from("hello".to_owned()))).await;
//!             },
//!             _ => {}
//!         }
//!     }
//! });
//!
//! let rq: String = futures::executor::block_on(async move {
//!     client.request("test", jsonrpsee_core::common::Params::None).await
//! }).unwrap();
//! println!("result: {:?}", rq);
//! ```
//!

use crate::{common, RawClient, RawServer, RawServerEvent};
use err_derive::*;
use fnv::FnvHashMap;
use futures::{channel::mpsc, prelude::*};
use std::{collections::hash_map::Entry, fmt, pin::Pin};

/// Builds a new client and a new server that are connected to each other.
pub fn local_raw() -> (LocalRawClient, LocalRawServer) {
    let (rq_tx, rq_rx) = mpsc::channel(4);
    let client = LocalRawClient { rq_tx };
    let server = LocalRawServer {
        rq_rx,
        next_request_id: 0,
        requests: Default::default(),
    };
    (client, server)
}

/// Client connected to a [`LocalRawServer`]. Can be created using [`local_raw`].
///
/// Can be cloned in order to have multiple clients connected to the same server.
#[derive(Clone)]
pub struct LocalRawClient {
    /// Channel to the server. Send a request and a way to send back a response.
    rq_tx: mpsc::Sender<(common::Request, mpsc::Sender<common::Response>)>,
}

/// Server connected to a [`LocalRawClient`]. Can be created using [`local_raw`].
pub struct LocalRawServer {
    /// Receiver connected to the client(s). Receive requests and a way to send back a response.
    rq_rx: mpsc::Receiver<(common::Request, mpsc::Sender<common::Response>)>,
    /// Id of the next request to insert in the `requests` hashmap.
    next_request_id: u64,
    /// List of requests waiting for an answer. Each entry is the sender that sends back a
    /// response to the client.
    requests: FnvHashMap<u64, mpsc::Sender<common::Response>>,
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

    fn request<'a>(
        &'a mut self,
        request: common::Request,
    ) -> Pin<Box<dyn Future<Output = Result<common::Response, Self::Error>> + Send + 'a>> {
        Box::pin(async move {
            let (tx, mut rx) = mpsc::channel(4);
            self.rq_tx
                .send((request, tx))
                .await
                .map_err(|_| LocalRawClientErr::ServerClosed)?;
            rx.next().await.ok_or(LocalRawClientErr::ServerClosed)
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
    ) -> Pin<Box<dyn Future<Output = RawServerEvent<Self::RequestId>> + Send + 'a>>
    {
        Box::pin(async move {
            let (request, send_back) = match self.rq_rx.next().await {
                Some(v) => v,
                None => return RawServerEvent::ServerClosed,
            };

            loop {
                let id = self.next_request_id;
                self.next_request_id = self.next_request_id.wrapping_add(1);
                match self.requests.entry(id) {
                    Entry::Occupied(_) => continue,
                    Entry::Vacant(e) => e.insert(send_back),
                };
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
            if let Some(mut rq) = self.requests.remove(&request_id) {
                if let Some(response) = response {
                    rq.send(response.clone()).await.map_err(|_| ())
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        })
    }

    fn supports_resuming(&self, request_id: &Self::RequestId) -> Result<bool, ()> {
        if self.requests.contains_key(request_id) {
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
            if let Some(rq) = self.requests.get_mut(&request_id) {
                rq.send(response.clone()).await.map_err(|_| ())
            } else {
                Err(())
            }
        })
    }
}

impl fmt::Debug for LocalRawServer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LocalRawServer")
            .field(
                "pending_requests",
                &self.requests.keys().cloned().collect::<Vec<_>>(),
            )
            .finish()
    }
}
