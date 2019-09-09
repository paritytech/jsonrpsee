/// Applies a macro.
#[macro_export]
macro_rules! rpc_api {
    (
        $(
            $api_name:ident {
                $(
                    $(#[$attr:meta])*
                    fn $name:ident($($pn:ident: $pty:ty),*) -> $ret:ty;
                )*
            }
        )*
    ) => {
        $(
            $(
                async fn $name($($pn: $pty)*) -> $ret {
                    /*$(
                        let $pn = $crate::common::to_value($pn).unwrap();        // TODO: don't unwrap
                    )**/

                    let http = $crate::http_client("http://localhost:8000");
                    http.request(stringify!($name)).await.unwrap()
                }
            )*

            enum $api_name<'a, R, I> {
                $(
                    $name { respond: $crate::rpc_api::RpcApiResponder<'a, R, I, $ret>, $($pn: $pty),* },
                )*
            }

            impl<'a, R, I> $api_name<'a, R, I> {
                async fn next_request(server: &'a mut $crate::server::Server<R, I>) -> Result<$api_name<'a, R, I>, std::io::Error>
                    where R: $crate::server::raw::RawServer<RequestId = I>,
                          I: Clone + PartialEq + Eq + Send + Sync,
                {
                    loop {
                        let (request_id, method) = match server.next_event().await.unwrap() {        // TODO: don't unwrap
                            $crate::server::ServerEvent::Notification(n) => unimplemented!(),       // TODO:
                            $crate::server::ServerEvent::Request(r) => (r.id(), r.method().to_owned()),
                        };

                        $(
                            if method == stringify!($name) {
                                let request = server.request_by_id(&request_id).unwrap();
                                $(
                                    let $pn: $pty = {
                                        let raw_val = match request.params().get(stringify!($pn)) {
                                            Some(v) => v,
                                            None => {
                                                request.respond(Err($crate::common::Error::invalid_params("foo"))).await;       // TODO: message
                                                continue;
                                            }
                                        };

                                        match $crate::common::from_value(raw_val.clone()) {
                                            Ok(v) => v,
                                            Err(_) => {
                                                request.respond(Err($crate::common::Error::invalid_params("foo"))).await;       // TODO: message
                                                continue;
                                            }
                                        }
                                    };
                                )*

                                let respond = $crate::rpc_api::RpcApiResponder {
                                    rq: request,
                                    response_ty: std::marker::PhantomData,
                                };
                                return Ok($api_name::$name { respond, $($pn),* });
                            }
                        )*

                        server.request_by_id(&request_id).unwrap().respond(Err($crate::common::Error::method_not_found())).await;
                    }
                }
            }
        )*
    };
}

// TODO: too much pub
pub struct RpcApiResponder<'a, R, I, T> {
    pub rq: crate::server::ServerRequest<'a, R, I>,
    pub response_ty: std::marker::PhantomData<T>,
}

impl<'a, R, I, T> RpcApiResponder<'a, R, I, T>
where R: crate::server::raw::RawServer<RequestId = I>,
        I: Clone + PartialEq + Eq + Send + Sync,
{
    pub async fn respond(self, response: Result<crate::common::JsonValue, crate::common::Error>) {
        self.rq.respond(response).await
    }
}
