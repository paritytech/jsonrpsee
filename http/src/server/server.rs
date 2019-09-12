use crate::server::background;
use fnv::FnvHashMap;
use futures::{channel::oneshot, prelude::*};
use jsonrpsee_core::{common, server::raw::RawServer};
use std::{error, net::SocketAddr, pin::Pin};

// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.

/// Implementation of the [`RawServer`](jsonrpsee_core::server::raw::RawServer) trait for HTTP.
pub struct HttpRawServer {
    /// Background thread that processes HTTP requests.
    background_thread: background::BackgroundHttp,

    /// Local address of the server.
    local_addr: SocketAddr,

    /// Next identifier to use when inserting an element in `requests`.
    next_request_id: u64,

    /// The identifier is lineraly increasing and is never leaked on the wire or outside of this
    /// module. Therefore there is no risk of hash collision and using a `FnvHashMap` is safe.
    requests: FnvHashMap<u64, oneshot::Sender<hyper::Response<hyper::Body>>>,
}

impl HttpRawServer {
    /// Tries to start an HTTP server that listens on the given address.
    ///
    /// Returns an error if we fail to start listening, which generally happens if the port is
    /// already occupied.
    //
    // > Note: This function is `async` despite not performing any asynchronous operation. Normally
    // >       starting to listen on a port is an asynchronous operation, but the hyper library
    // >       hides this to us. In order to be future-proof, this function is async, so that we
    // >       might switch out to a different library later without breaking the API.
    pub async fn bind(
        addr: &SocketAddr,
    ) -> Result<HttpRawServer, Box<dyn error::Error + Send + Sync>> {
        let (background_thread, local_addr) = background::BackgroundHttp::bind(addr)?;

        Ok(HttpRawServer {
            background_thread,
            local_addr,
            requests: Default::default(),
            next_request_id: 0,
        })
    }

    /// Returns the address we are actually listening on, which might be different from the one
    /// passed as parameter.
    pub fn local_addr(&self) -> &SocketAddr {
        &self.local_addr
    }
}

impl RawServer for HttpRawServer {
    type RequestId = u64;

    fn next_request<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(Self::RequestId, common::Request), ()>> + Send + 'a>>
    {
        Box::pin(async move {
            let request = self.background_thread.next().await?;
            let request_id = {
                let id = self.next_request_id;
                self.next_request_id += 1;
                id
            };

            // TODO: we actually don't need to insert the request
            // most requests are answered without being dropped, so as an optimization we can
            // return the request itself, and insert it later if we drop it
            self.requests.insert(request_id, request.send_back);

            // Every 128 requests, we call `shrink_to_fit` on the list.
            if request_id % 128 == 0 {
                self.requests.shrink_to_fit();
            }

            Ok((request_id, request.request))
        })
    }

    fn finish<'a>(
        &'a mut self,
        request_id: &'a Self::RequestId,
        response: Option<&'a common::Response>,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        Box::pin(async move {
            let send_back = match self.requests.remove(request_id) {
                Some(rq) => rq,
                None => return Err(()),
            };

            let response = match response.map(|r| serde_json::to_vec(r)) {
                Some(Ok(bytes)) => hyper::Response::builder()
                    .status(hyper::StatusCode::OK)
                    .header(
                        "Content-Type",
                        hyper::header::HeaderValue::from_static("application/json; charset=utf-8"),
                    )
                    .body(hyper::Body::from(bytes))
                    .expect("Unable to parse response body for type conversion"),
                Some(Err(_)) => panic!(), // TODO: no
                None => {
                    // TODO: is that a good idea? should the param really be an Option?
                    hyper::Response::builder()
                        .status(hyper::StatusCode::NO_CONTENT)
                        .body(hyper::Body::empty())
                        .expect("Unable to parse response body for type conversion")
                }
            };

            if send_back.send(response).is_err() {
                log::error!("Couldn't send back JSON-RPC response, as background task has crashed");
            }

            Ok(())
        })
    }

    fn supports_resuming(&self, _: &u64) -> bool {
        false
    }

    fn send<'a>(
        &'a mut self,
        _: &'a Self::RequestId,
        _: &'a common::Response,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        Box::pin(async move { Err(()) })
    }
}

#[cfg(test)]
mod tests {
    use super::HttpRawServer;

    #[test]
    fn error_if_port_occupied() {
        futures::executor::block_on(async move {
            let addr = "127.0.0.1:0".parse().unwrap();
            let server1 = HttpRawServer::bind(&addr).await.unwrap();
            assert!(HttpRawServer::bind(server1.local_addr()).await.is_err());
        });
    }
}
