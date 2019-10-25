use crate::server::response;
use futures::{channel::mpsc, channel::oneshot, prelude::*};
use hyper::service::{make_service_fn, service_fn};
use hyper::Error;
use jsonrpsee_core::common;
use jsonrpsee_server_utils::cors;
use jsonrpsee_server_utils::access_control::AccessControl;
use std::{io, net::SocketAddr, thread};
use crate::utils;


/// Background thread that serves HTTP requests.
pub(super) struct BackgroundHttp {
    /// Receiver for requests coming from the background thread.
    rx: mpsc::Receiver<Request>,
}

/// Request generated from the background thread.
pub(super) struct Request {
    /// Sender for the body of the response to send on the network.
    pub send_back: oneshot::Sender<hyper::Response<hyper::Body>>,
    /// The JSON body that was sent by the client.
    pub request: common::Request,
}

impl BackgroundHttp {
    /// Tries to create an HTTP server listening on the given address and start a background
    /// thread.
    ///
    /// In addition to `Self`, also returns the local address the server ends up listening on,
    /// which might be different than the one passed as parameter.
    pub fn bind(addr: &SocketAddr) -> Result<(BackgroundHttp, SocketAddr), hyper::Error> {
        Self::bind_with_acl(addr, AccessControl::default())
    }

    pub fn bind_with_acl(addr: &SocketAddr, access_control: AccessControl) -> Result<(BackgroundHttp, SocketAddr), hyper::Error> {
        let (tx, rx) = mpsc::channel(4);

        let make_service = make_service_fn(move |_| {
            let tx = tx.clone();
            let access_control = access_control.clone();
            async move {
                Ok::<_, Error>(service_fn(move |req| {
                    let mut tx = tx.clone();
                    let access_control = access_control.clone();
                    async move { Ok::<_, Error>(process_request(req, &mut tx, &access_control).await) }
                }))
            }
        });

        let server = hyper::Server::try_bind(addr)?.serve(make_service);
        let local_addr = server.local_addr();

        // Because hyper can only be polled through tokio, we spawn it in a background thread.
        thread::Builder::new()
            .name("jsonrpsee-hyper-server".to_string())
            .spawn(move || {
                let mut runtime = match tokio::runtime::current_thread::Runtime::new() {
                    Ok(r) => r,
                    Err(err) => {
                        log::error!(
                            "Failed to initialize tokio runtime in HTTP JSON-RPC server: {}",
                            err
                        );
                        return;
                    }
                };

                runtime.block_on(async move {
                    if let Err(err) = server.await {
                        log::error!("HTTP JSON-RPC server closed with an error: {}", err);
                    }
                });
            })
            .unwrap();

        Ok((BackgroundHttp { rx }, local_addr))
    }

    /// Returns the next request, or an error if the background thread has unexpectedly closed.
    pub async fn next(&mut self) -> Result<Request, ()> {
        self.rx.next().await.ok_or(())
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
    fg_process_tx: &mut mpsc::Sender<Request>,
    access_control: &AccessControl,
) -> hyper::Response<hyper::Body> {
    // Process access control 
    if !utils::is_host_allowed(&request, &access_control.allow_hosts) {
        return response::host_not_allowed();
    }

    let cors_allow_origin = utils::cors_allow_origin(&request, &access_control.cors_allow_origin);
    if cors_allow_origin == cors::AllowCors::Invalid && !access_control.continue_on_invalid_cors {
        return response::invalid_allow_origin();
    }

    let cors_allow_headers = utils::cors_allow_headers(&request, &access_control.cors_allow_headers);
    if cors_allow_headers == cors::AllowCors::Invalid && !access_control.continue_on_invalid_cors {
        return response::invalid_allow_headers();
    }
    
    /*
    // Read metadata
    let metadata = self.jsonrpc_handler.extractor.read_metadata(&request);
    */

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

            let json_body = match body_to_request(request.into_body()).await {
                Ok(b) => b,
                Err(_) => {
                    unimplemented!()        // TODO:
                }
            };

            let (tx, rx) = oneshot::channel();
            let user_facing_rq = Request {
                send_back: tx,
                request: json_body,
            };
            if fg_process_tx.send(user_facing_rq).await.is_err() {
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

/// Converts a `hyper` body into a structured JSON object.
///
/// Enforces a size limit on the body.
async fn body_to_request(mut body: hyper::Body) -> Result<common::Request, io::Error> {
    let mut json_body = Vec::new();
    while let Some(chunk) = body.next().await {
        let chunk = match chunk {
            Ok(c) => c,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())), // TODO:
        };
        json_body.extend_from_slice(&chunk.into_bytes());
        if json_body.len() >= 16384 {
            // TODO: some limit
            return Err(io::Error::new(io::ErrorKind::Other, "request too large"));
        }
    }

    Ok(serde_json::from_slice(&json_body)?)
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
