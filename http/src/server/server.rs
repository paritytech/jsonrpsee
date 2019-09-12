use crate::server::{background, response};
use async_std::net::ToSocketAddrs;
use fnv::FnvHashMap;
use futures::{channel::mpsc, channel::oneshot, lock::Mutex, prelude::*};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response};
use jsonrpsee_core::common;
use jsonrpsee_core::server::raw::RawServer;
use std::{error, io, net::SocketAddr, pin::Pin, thread};

// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.

pub struct HttpRawServer {
    /// Background thread that processes HTTP requests.
    background_thread: background::BackgroundHttp,

    /// Local address of the server.
    local_addr: SocketAddr,

    next_request_id: u64,

    /// The identifier is lineraly increasing and is never leaked on the wire or outside of this
    /// module. Therefore there is no risk of hash collision.
    requests: FnvHashMap<u64, oneshot::Sender<hyper::Response<hyper::Body>>>,
}

impl HttpRawServer {
    // TODO: `ToSocketAddrs` can be blocking
    pub async fn bind(addr: impl ToSocketAddrs) -> Result<HttpRawServer, Box<dyn error::Error + Send + Sync>> {
        let (background_thread, local_addr) = background::BackgroundHttp::bind(addr).await?;

        Ok(HttpRawServer {
            background_thread,
            local_addr,
            requests: Default::default(),
            next_request_id: 0,
        })
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
    use super::body_to_request;

    // TODO: restore test
    /*#[test]
    fn body_to_request_works() {
        futures::executor::block_on(async move {
            let mut body = hyper::Body::from("[{\"a\":\"hello\"}]");
            let json = body_to_request(body).await.unwrap();
            assert_eq!(json, serde_json::Value::from(vec![
                std::iter::once((
                    "a".to_string(),
                    serde_json::Value::from("hello")
                )).collect::<serde_json::Map<_, _>>()
            ]));
        });
    }*/

    #[test]
    fn body_to_request_size_limit_json() {
        let huge_body = serde_json::to_vec(
            &(0..32768)
                .map(|_| serde_json::Value::from("test"))
                .collect::<Vec<_>>(),
        )
        .unwrap();

        futures::executor::block_on(async move {
            let body = hyper::Body::from(huge_body);
            assert!(body_to_request(body).await.is_err());
        });
    }

    #[test]
    fn body_to_request_size_limit_garbage() {
        let huge_body = (0..100_000)
            .map(|_| rand::random::<u8>())
            .collect::<Vec<_>>();
        futures::executor::block_on(async move {
            let body = hyper::Body::from(huge_body);
            assert!(body_to_request(body).await.is_err());
        });
    }
}
