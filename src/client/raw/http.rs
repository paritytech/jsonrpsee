use crate::common;
use super::RawClientRef;
use derive_more::*;
use err_derive::*;
use futures::{prelude::*, channel::mpsc, channel::oneshot};
use std::{fmt, io, net::SocketAddr, pin::Pin, thread};

// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// In particular, hyper can only be polled by tokio, but we don't want users to have to suffer
// from this restriction. We therefore spawn a background thread dedicated to running the tokio
// runtime.
//
// In order to perform a request, we send this request to the background thread through a channel
// and wait for an answer to come back.

/// Implementation of a raw client for HTTP requests.
pub struct HttpClientPool {
    /// Sender that sends requests to the background task.
    requests_tx: mpsc::Sender<FrontToBack>,
}

/// Message transmitted from the foreground task to the background.
struct FrontToBack {
    /// Request that the background task should perform.
    request: hyper::Request<hyper::Body>,
    /// Channel to send back to the response.
    send_back: oneshot::Sender<Result<hyper::Response<hyper::Body>, hyper::Error>>,
}

impl HttpClientPool {
    /// Initializes a new pool for HTTP client reuests.
    pub fn new() -> Result<Self, io::Error> {
        let (requests_tx, requests_rx) = mpsc::channel::<FrontToBack>(4);

        // Because hyper can only be polled through tokio, we spawn it in a background thread.
        thread::Builder::new()
            .name("jsonrpc-hyper-client".to_string())
            .spawn(move || background_thread(requests_rx))
            .unwrap();

        Ok(HttpClientPool { requests_tx })
    }

    /// Borrows the `HttpClientPool` and builds an object that can perform request towards the
    /// given URL.
    pub fn with_server<'a, 'b>(&'a self, url: &'b str) -> WithServer<'a, 'b> {
        WithServer {
            pool: self,
            url,
        }
    }
}

impl fmt::Debug for HttpClientPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("HttpClientPool").finish()
    }
}

/// Borrows an [`HttpClientPool`] and a target URL.
pub struct WithServer<'a, 'b> {
    pool: &'a HttpClientPool,
    url: &'b str,
}

impl<'a, 'b> RawClientRef<'a> for WithServer<'a, 'b> {
    type Request = Pin<Box<dyn Future<Output = Result<common::Response, RequestError>> + Send + 'a>>;
    type Error = RequestError;

    fn request(self, request: common::Request) -> Self::Request {
        let mut requests_tx = self.pool.requests_tx.clone();

        let request = common::to_vec(&request).map(|body| {
            hyper::Request::post(self.url)
                .header(
                    hyper::header::CONTENT_TYPE,
                    hyper::header::HeaderValue::from_static("application/json"),
                )
                .body(From::from(body))
                .expect("Uri and request headers are valid; qed")      // TODO: not necessarily true for URL here
        });

        Box::pin(async move {
            let (send_back_tx, send_back_rx) = oneshot::channel();
            let message = FrontToBack {
                request: request.map_err(RequestError::Serialization)?,
                send_back: send_back_tx,
            };

            if requests_tx.send(message).await.is_err() {
                log::error!("JSONRPC http client background thread has shut down");
                return Err(RequestError::Io(
                    io::Error::new(io::ErrorKind::Other, "background thread is down")
                ))
            }

            let hyper_response = match send_back_rx.await {
                Ok(Ok(r)) => r,
                Ok(Err(err)) => return Err(RequestError::Http(Box::new(err))),
                Err(_) => {
                    log::error!("JSONRPC http client background thread has shut down");
                    return Err(RequestError::Io(
                        io::Error::new(io::ErrorKind::Other, "background thread is down")
                    ))
                }
            };

            if !hyper_response.status().is_success() {
                return Err(RequestError::RequestFailure {
                    status_code: hyper_response.status().into(),
                })
            }

            // Note that we don't check the Content-Type of the request. This is deemed
            // unnecessary, as a parsing error while happen anyway.

            // TODO: enforce a maximum size here
            let body: hyper::Chunk = hyper_response.into_body().try_concat().await
                .map_err(|err| RequestError::Http(Box::new(err)))?;

            // TODO: use Response::from_json
            let as_json: common::Response = common::from_slice(&body)
                .map_err(|err| RequestError::ParseError(err))?;
            Ok(as_json)
        })
    }
}

/// Error that can happen during a request.
#[derive(Debug, From, Error)]
pub enum RequestError {
    // TODO: remove
    #[error(display = "network error while performing the request")]
    Io(#[error(cause)] io::Error),

    #[error(display = "error while serializing the request")]
    Serialization(#[error(cause)] serde_json::error::Error),

    #[error(display = "response body is not UTF-8")]
    Utf8(std::string::FromUtf8Error),

    #[error(display = "error while performing the HTTP request")]
    Http(Box<std::error::Error + Send + Sync>),

    #[error(display = "error while parsing the response body")]
    ParseError(#[error(cause)] serde_json::error::Error),

    #[error(display = "server returned an error status code: {:?}", status_code)]
    RequestFailure {
        status_code: u16,
    },
}

/// Function that runs in a background thread.
fn background_thread(mut requests_rx: mpsc::Receiver<FrontToBack>) {
    let client = hyper::Client::new();

    let mut runtime = match tokio::runtime::current_thread::Runtime::new() {
        Ok(r) => r,
        Err(err) => {
            // Ideally, we would try to initialize the tokio runtime in the main thread then move
            // it here. That however isn't possible. If we fail to initialize the runtime, the only
            // thing we can do is print an error and shut down the background thread.
            // Initialization failures should be almost non-existant anyway, so this isn't a big
            // deal.
            log::error!("Failed to initialize tokio runtime: {:?}", err);
            return
        },
    };

    // Running until the channel has been closed, and all requests have been completed.
    runtime.block_on(async move {
        // Collection of futures that process ongoing requests.
        let mut pending_requests = stream::FuturesUnordered::new();

        loop {
            let rq = match future::select(requests_rx.next(), pending_requests.next()).await {
                // We received a request from the foreground.
                future::Either::Left((Some(rq), _)) => rq,
                // The channel with the foreground has closed.
                future::Either::Left((None, _)) => break,
                // One of the elements of `pending_requests` is finished.
                future::Either::Right(_) => continue,
            };

            pending_requests.push(async {
                let _ = rq.send_back.send(client.request(rq.request).await);
            });
        }

        // Before returning, complete all pending requests.
        while let Some(_) = pending_requests.next().await {}
    });
}
