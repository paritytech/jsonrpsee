use crate::server::{Server, ServerJsonRequest, ServerRef, ServerRefRq};
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
                    async move { process_request(req, &mut tx).await }
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

impl<'a> ServerRef<'a> for &'a HttpServer {
    type ServerRefRq = HttpServerRefRq;

    fn next_request(self) -> Pin<Box<dyn Future<Output = Result<Self::ServerRefRq, ()>> + 'a>> {
        Box::pin(async move {
            let mut rx = self.rx.lock().await;
            let packet = rx.next().await.ok_or_else(|| ())?;
            let (tx, rx) = oneshot::channel();
            Ok(HttpServerRefRq { send_back: tx })
        })
    }
}

/// HTTP request that must be answered.
pub struct HttpServerRefRq {
    send_back: oneshot::Sender<Vec<u8>>,
}

impl ServerRefRq for HttpServerRefRq {
    fn method(&self) -> &str {
        "test"      // TODO: no
    }

    fn respond(self) -> Pin<Box<dyn Future<Output = Result<(), io::Error>>>> {
        Box::pin(future::pending())
    }
}

/// Accepts an HTTP request as parameter.
async fn process_request(
    request: hyper::Request<hyper::Body>,
    tx: &mut mpsc::Sender<HttpServerRefRq>,
) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
    /*if self.cors_allow_origin == cors::AllowCors::Invalid && !continue_on_invalid_cors {
        return RpcHandlerState::Writing(Response::invalid_allow_origin());
    }

    if self.cors_allow_headers == cors::AllowCors::Invalid && !continue_on_invalid_cors {
        return RpcHandlerState::Writing(Response::invalid_allow_headers());
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
            //body: request.into_body()
        }
        /*Method::POST if /*self.rest_api == RestApi::Unsecure &&*/ request.uri().path().split('/').count() > 2 => {
            RpcHandlerState::ProcessRest {
                metadata,
                uri: request.uri().clone(),
            }
        }
        // Just return error for unsupported content type
        Method::POST => RpcHandlerState::Writing(Response::unsupported_content_type()),
        // Don't validate content type on options
        Method::OPTIONS => RpcHandlerState::Writing(Response::empty()),
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
        _ => return Ok(method_not_allowed()),
    };

    Ok(hyper::Response::new(hyper::Body::from("Hello World!")))
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

/// Create a response for disallowed method used.
fn method_not_allowed() -> hyper::Response<hyper::Body> {
    hyper::Response::builder()
        .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
        .header("Content-Type", hyper::header::HeaderValue::from_static("text/plain; charset=utf-8"))
        .body(hyper::Body::from("Used HTTP Method is not allowed. POST or OPTIONS is required\n"))
        .unwrap()
}
