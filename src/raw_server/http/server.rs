use crate::JsonValue;
use crate::raw_server::http::response;
use crate::raw_server::{RawServerRef, RawServerRefRq};
use futures::{channel::mpsc, channel::oneshot, lock::Mutex, prelude::*};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response};
use std::{io, net::ToSocketAddrs, pin::Pin, thread};

// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.

pub struct HttpServer {
    rx: Mutex<mpsc::Receiver<HttpServerRefRq>>,
}

impl HttpServer {
    // TODO: `ToSocketAddrs` can be blocking
    pub fn bind(addr: impl ToSocketAddrs) -> HttpServer {
        let (mut tx, rx) = mpsc::channel(4);

        let addr = addr.to_socket_addrs().unwrap().next().unwrap(); // TODO: no

        let make_service = make_service_fn(move |_| {
            let mut tx = tx.clone();
            async move {
                Ok::<_, Error>(service_fn(move |req| {
                    let mut tx = tx.clone();
                    async move { Ok::<_, Error>(process_request(req, &mut tx).await) }
                }))
            }
        });

        let server = hyper::Server::bind(&addr).serve(make_service);

        // Because hyper can only be polled through tokio, we spawn it in a background thread.
        thread::spawn(move || {
            let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
            runtime.block_on(async move {
                //future::select(shutdown_rx, server);
                if let Err(err) = server.await {
                    panic!("{:?}", err);
                    // TODO: log
                }
                panic!("HTTP server closed");
            });
        });

        HttpServer { rx: Mutex::new(rx) }
    }
}

impl<'a> RawServerRef<'a> for &'a HttpServer {
    type RawServerRefRq = HttpServerRefRq;

    fn next_payload(self) -> Pin<Box<dyn Future<Output = Result<Self::RawServerRefRq, ()>> + 'a>> {
        Box::pin(async move {
            let mut rx = self.rx.lock().await;
            rx.next().await.ok_or_else(|| ())
        })
    }
}

/// HTTP request that must be answered.
pub struct HttpServerRefRq {
    send_back: oneshot::Sender<hyper::Response<hyper::Body>>,
    /// The JSON body that was sent by the user.
    json: JsonValue,
}

impl RawServerRefRq for HttpServerRefRq {
    fn json(&self) -> &JsonValue {
        &self.json
    }

    fn respond<'a>(self, response: &'a JsonValue) -> Pin<Box<dyn Future<Output = Result<(), io::Error>> + 'a>> {
        Box::pin(async move {
            let bytes = match serde_json::to_vec(response) {
                Ok(b) => b,
                Err(_) => panic!()      // TODO: no
            };

            let response = hyper::Response::builder()
                .status(hyper::StatusCode::OK)
                .header(
                    "Content-Type",
                    hyper::header::HeaderValue::from_static("application/json; charset=utf-8"),
                )
                .body(hyper::Body::from(bytes))
                .expect("Unable to parse response body for type conversion");

            self.send_back.send(response).map_err(|_| io::Error::from(io::ErrorKind::Other));      // TODO:
            Ok(())
        })
    }
}

/// Process an HTTP request and sends back a response.
///
/// This function is the main method invoked whenever we receive an HTTP request.
///
/// In order to process JSON-RPC requests, it has access to `fg_process_tx`. Objects sent on this
/// channel will be dispatched to the user.
async fn process_request(
    request: hyper::Request<hyper::Body>,
    fg_process_tx: &mut mpsc::Sender<HttpServerRefRq>,
) -> hyper::Response<hyper::Body> {
    /*if self.cors_allow_origin == cors::AllowCors::Invalid && !continue_on_invalid_cors {
        return response::invalid_allow_origin();
    }

    if self.cors_allow_headers == cors::AllowCors::Invalid && !continue_on_invalid_cors {
        return response::invalid_allow_headers();
    }

    // Read metadata
    let metadata = self.jsonrpc_handler.extractor.read_metadata(&request);*/

    // Proceed
    match *request.method() {
        // Validate the ContentType header
        // to prevent Cross-Origin XHRs with text/plain
        hyper::Method::POST if is_json(request.headers().get("content-type")) => {
            let uri = //if self.rest_api != RestApi::Disabled {
                Some(request.uri().clone())
            /*} else {
                None
            }*/;

            let (tx, rx) = oneshot::channel();
            let user_facing_rq = HttpServerRefRq {
                send_back: tx,
                json: JsonValue::Null,      // FIXME:
            };
            if let Err(_) = fg_process_tx.send(user_facing_rq).await {
                return response::internal_error("JSON requests processing channel has shut down");
            }
            match rx.await {
                Ok(response) => response,
                Err(_) => return response::internal_error("JSON request send back channel has shut down"),
            }
        }
        /*Method::POST if /*self.rest_api == RestApi::Unsecure &&*/ request.uri().path().split('/').count() > 2 => {
            RpcHandlerState::ProcessRest {
                metadata,
                uri: request.uri().clone(),
            }
        }
        // Just return error for unsupported content type
        Method::POST => response::unsupported_content_type(),
        // Don't validate content type on options
        Method::OPTIONS => response::empty(),
        // Respond to health API request if there is one configured.
        Method::GET if self.health_api.as_ref().map(|x| &*x.0) == Some(request.uri().path()) => {
            RpcHandlerState::ProcessHealth {
                metadata,
                method: self
                    .health_api
                    .as_ref()
                    .map(|x| x.1.clone())
                    .expect("Health api is defined since the URI matched."),
            }
        }*/
        // Disallow other methods.
        _ => response::method_not_allowed(),
    }
}

/// Returns true if the `content_type` header indicates a valid JSON message.
fn is_json(content_type: Option<&hyper::header::HeaderValue>) -> bool {
    match content_type.and_then(|val| val.to_str().ok()) {
        Some(ref content)
            if content.eq_ignore_ascii_case("application/json")
                || content.eq_ignore_ascii_case("application/json; charset=utf-8")
                || content.eq_ignore_ascii_case("application/json;charset=utf-8") =>
        {
            true
        }
        _ => false,
    }
}
