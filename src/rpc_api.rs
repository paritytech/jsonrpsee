
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

            enum $api_name<'a,  R> {
                $(
                    $name { respond: $crate::rpc_api::RpcApiResponder<'a, R, $ret>, $($pn: $pty),* },
                )*
            }

            impl<'a, R> $api_name<'a, R> {
                async fn next_request<S>(server: &'a mut $crate::server::Server<S>) -> Result<Self, std::io::Error>
                    where &'a mut S: $crate::server::raw::RawServerRef<'a, Request = R>,
                        R: $crate::server::raw::RawServerRq<'a>,
                {
                    loop {
                        let request = server.next_request().await.unwrap();     // TODO: don't unwrap

                        $(
                            if request.method() == stringify!($name) {
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

                        request.respond(Err($crate::common::Error::method_not_found())).await;
                    }
                }
            }
        )*
    };
}

// TODO: too much pub
pub struct RpcApiResponder<'a, R, T> {
    pub rq: crate::server::ServerRq<'a, R>,
    pub response_ty: std::marker::PhantomData<T>,
}

impl<'a, R, T> RpcApiResponder<'a, R, T> {

}
