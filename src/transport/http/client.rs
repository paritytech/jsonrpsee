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

use crate::{common, transport::TransportClient};

use futures::{channel::mpsc, prelude::*};
use std::{fmt, io, pin::Pin, thread};
use thiserror::Error;

// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// In particular, hyper can only be polled by tokio, but we don't want users to have to suffer
// from this restriction. We therefore spawn a background thread dedicated to running the tokio
// runtime.
//
// In order to perform a request, we send this request to the background thread through a channel
// and wait for an answer to come back.
//
// Addtionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

/// Implementation of a raw client for HTTP requests.
pub struct HttpTransportClient {
    /// Sender that sends requests to the background task.
    requests_tx: mpsc::Sender<FrontToBack>,
    url: String,
    /// Responses receiver.
    responses_rx: mpsc::UnboundedReceiver<Result<hyper::Response<hyper::Body>, hyper::Error>>,
    /// Responses transmitter
    responses_tx: mpsc::UnboundedSender<Result<hyper::Response<hyper::Body>, hyper::Error>>,
}

/// Message transmitted from the foreground task to the background.
struct FrontToBack {
    /// Request that the background task should perform.
    request: hyper::Request<hyper::Body>,
    /// Channel to send back to the response.
    send_back: mpsc::UnboundedSender<Result<hyper::Response<hyper::Body>, hyper::Error>>,
}

impl HttpTransportClient {
    /// Initializes a new HTTP client.
    // TODO: better type for target
    pub fn new(target: &str) -> Self {
        let (requests_tx, requests_rx) = mpsc::channel::<FrontToBack>(4);

        // Because hyper can only be polled through tokio, we spawn it in a background thread.
        thread::Builder::new()
            .name("jsonrpsee-hyper-client".to_string())
            .spawn(move || {
                let client = hyper::Client::new();
                background_thread(requests_rx, move |rq| {
                    // cloning Hyper client = cloning references
                    let client = client.clone();
                    async move {
                        let _ = rq.send_back.unbounded_send(client.request(rq.request).await);
                    }
                })
            })
            .unwrap();

        let (responses_tx, responses_rx) = futures::channel::mpsc::unbounded();
        HttpTransportClient {
            requests_tx,
            url: target.to_owned(),
            responses_rx,
            responses_tx,
        }
    }
}

impl TransportClient for HttpTransportClient {
    type Error = RequestError;

    fn send_request<'s>(
        &'s mut self,
        request: common::Request,
    ) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send + 's>> {
        let mut requests_tx = self.requests_tx.clone();

        let request = common::to_vec(&request).map(|body| {
            hyper::Request::post(&self.url)
                .header(
                    hyper::header::CONTENT_TYPE,
                    hyper::header::HeaderValue::from_static("application/json"),
                )
                .body(From::from(body))
                .expect("Uri and request headers are valid; qed") // TODO: not necessarily true for URL here
        });

        Box::pin(async move {
            let message = FrontToBack {
                request: request.map_err(RequestError::Serialization)?,
                send_back: self.responses_tx.clone(),
            };

            if requests_tx.send(message).await.is_err() {
                log::error!("JSONRPC http client background thread has shut down");
                return Err(RequestError::Http(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "background thread is down".to_string(),
                ))));
            }

            Ok(())
        })
    }

    fn next_response<'s>(
        &'s mut self,
    ) -> Pin<Box<dyn Future<Output = Result<common::Response, RequestError>> + Send + 's>> {
        Box::pin(async move {
            let hyper_response = match self.responses_rx.next().await {
                Some(Ok(r)) => r,
                Some(Err(err)) => return Err(RequestError::Http(Box::new(err))),
                None => {
                    log::error!("JSONRPC http client background thread has shut down");
                    return Err(RequestError::Http(Box::new(io::Error::new(
                        io::ErrorKind::Other,
                        "background thread is down".to_string(),
                    ))));
                }
            };

            if !hyper_response.status().is_success() {
                return Err(RequestError::RequestFailure {
                    status_code: hyper_response.status().into(),
                });
            }

            // Note that we don't check the Content-Type of the request. This is deemed
            // unnecessary, as a parsing error while happen anyway.

            // TODO: enforce a maximum size here
            let body = hyper::body::to_bytes(hyper_response.into_body())
                .await
                .map_err(|err| RequestError::Http(Box::new(err)))?;

            // TODO: use Response::from_json
            let as_json: common::Response =
                common::from_slice(&body).map_err(RequestError::ParseError)?;
            Ok(as_json)
        })
    }
}

impl fmt::Debug for HttpTransportClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("HttpTransportClient").finish()
    }
}

/// Error that can happen during a request.
#[derive(Debug, Error)]
pub enum RequestError {
    /// Error while serializing the request.
    // TODO: can that happen?
    #[error("error while serializing the request")]
    Serialization(#[source] serde_json::error::Error),

    /// Response given by the server failed to decode as UTF-8.
    #[error("response body is not UTF-8")]
    Utf8(#[source] std::string::FromUtf8Error),

    /// Error during the HTTP request, including networking errors and HTTP protocol errors.
    #[error("error while performing the HTTP request")]
    Http(Box<dyn std::error::Error + Send + Sync>),

    /// Server returned a non-success status code.
    #[error("server returned an error status code: {:?}", status_code)]
    RequestFailure {
        /// Status code returned by the server.
        status_code: u16,
    },

    /// Failed to parse the JSON returned by the server into a JSON-RPC response.
    #[error("error while parsing the response body")]
    ParseError(#[source] serde_json::error::Error),
}

/// Function that runs in a background thread.
fn background_thread<T, ProcessRequest: Future<Output = ()>>(
    mut requests_rx: mpsc::Receiver<T>,
    process_request: impl Fn(T) -> ProcessRequest,
) {
    let mut runtime = match tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(err) => {
            // Ideally, we would try to initialize the tokio runtime in the main thread then move
            // it here. That however isn't possible. If we fail to initialize the runtime, the only
            // thing we can do is print an error and shut down the background thread.
            // Initialization failures should be almost non-existant anyway, so this isn't a big
            // deal.
            log::error!("Failed to initialize tokio runtime: {:?}", err);
            return;
        }
    };

    // Running until the channel has been closed, and all requests have been completed.
    runtime.block_on(async move {
        // Collection of futures that process ongoing requests.
        let mut pending_requests = stream::FuturesUnordered::new();

        loop {
            let request = loop {
                if !pending_requests.is_empty() {
                    let event = future::select(requests_rx.next(), pending_requests.next()).await;
                    if let future::Either::Left((rq, _)) = event {
                        break rq;
                    }
                // else: one of the elements of `pending_requests` is finished, but we don't care
                } else {
                    break requests_rx.next().await;
                }
            };

            match request {
                // We received a request from the foreground.
                Some(rq) => pending_requests.push(process_request(rq)),
                // The channel with the foreground has closed.
                None => break,
            }
        }

        // Before returning, complete all pending requests.
        while let Some(_) = pending_requests.next().await {}
    });
}

#[cfg(test)]
mod tests {
    use futures::channel::oneshot;
    use super::*;

    #[test]
    fn background_thread_is_able_to_complete_requests() {
        // start background thread that returns square(passed_value) after signal
        // from 'main' thread is received
        let (mut requests_tx, requests_rx) = mpsc::channel(4);
        let background_thread = thread::spawn(move || {
            background_thread(
                requests_rx,
                move |(send_when, send_back, value): (
                    oneshot::Receiver<()>,
                    oneshot::Sender<u32>,
                    u32,
                )| async move {
                    send_when.await.unwrap();
                    send_back.send(value * value).unwrap();
                },
            )
        });

        // send two requests - there'll be two simultaneous active requests, waiting for
        // main thread' signals
        let mut pool = futures::executor::LocalPool::new();
        let (send_when_tx1, send_when_rx1) = oneshot::channel();
        let (send_when_tx2, send_when_rx2) = oneshot::channel();
        let (send_back_tx1, send_back_rx1) = oneshot::channel();
        let (send_back_tx2, send_back_rx2) = oneshot::channel();
        pool.run_until(requests_tx.send((send_when_rx1, send_back_tx1, 32)))
            .unwrap();
        pool.run_until(requests_tx.send((send_when_rx2, send_back_tx2, 1024)))
            .unwrap();

        // send both signals and wait for responses
        send_when_tx1.send(()).unwrap();
        send_when_tx2.send(()).unwrap();
        assert_eq!(pool.run_until(send_back_rx1), Ok(32 * 32));
        assert_eq!(pool.run_until(send_back_rx2), Ok(1024 * 1024));

        // drop requests sender, asking background thread to exit gently
        drop(requests_tx);
        background_thread.join().unwrap();
    }
}
