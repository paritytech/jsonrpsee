use crate::types;
use super::RawClient;
use futures::{prelude::*, channel::mpsc, channel::oneshot};
use std::{fmt, io, net::SocketAddr, pin::Pin, thread};

// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// In particular, hyper can only be polled by tokio, but we don't want users to have to suffer
// from this restriction. We therefore spawn a background thread dedicated to running the tokio
// runtime.

pub struct HttpClientPool {
    /// Sender that sends requests to the background task.
    tx: mpsc::Sender<(hyper::Request<hyper::Body>, oneshot::Sender<Result<hyper::Response<hyper::Body>, hyper::Error>>)>,
}

impl HttpClientPool {
    pub fn new() -> Result<Self, io::Error> {
        let (mut tx, mut rx) = mpsc::channel::<(hyper::Request<hyper::Body>, oneshot::Sender<Result<hyper::Response<hyper::Body>, hyper::Error>>)>(4);

        let client = hyper::Client::new();

        // Because hyper can only be polled through tokio, we spawn it in a background thread.
        thread::Builder::new()
            .name("jsonrpc-hyper-client".to_string())
            .spawn(move || {
                // TODO: don't unwrap
                let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
                runtime.block_on(async move {
                    while let Some((rq, send_back)) = rx.next().await {
                        // TODO: no, as that makes queries serially one by one
                        let _ = send_back.send(client.request(rq).await);
                    }
                });
            })
            .unwrap();

        Ok(HttpClientPool { tx })
    }
}

impl fmt::Debug for HttpClientPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("HttpClientPool").finish()
    }
}

impl RawClient for HttpClientPool {
    type Future = Pin<Box<dyn Future<Output = Result<types::Response, io::Error>> + Send>>;

    fn request(&self, target: &str, request: types::Request) -> Self::Future {
        let mut tx = self.tx.clone();

        let request = types::to_vec(&request).map(|body| {
            hyper::Request::post(target)
                .header(
                    hyper::header::CONTENT_TYPE,
                    hyper::header::HeaderValue::from_static("application/json"),
                )
                .body(From::from(body))
                .expect("Uri and request headers are valid; qed")      // TODO: not necessarily true for URL here
        });

        Box::pin(async move {
            let request = request?;

            let (send_back_tx, send_back_rx) = oneshot::channel();
            if tx.send((request, send_back_tx)).await.is_err() {
                log::error!("JSONRPC http client cackground thread has shut down");
                return Err(io::Error::new(io::ErrorKind::Other, "background thread is down"))
            }

            let hyper_response = send_back_rx.await;
            unimplemented!()        // TODO: finish
        })
    }
}
