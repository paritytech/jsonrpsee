
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

            enum $api_name {
                $(
                    $name { respond: (), $($pn: $pty),* },
                )*
            }

            impl $api_name {
                async fn next_request<S>(server: &mut $crate::server::Server<S>) -> Result<Self, std::io::Error>
                    where for<'r> &'r mut S: $crate::server::raw::RawServerRef<'r>
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

                                // TODO: give a way to respond
                                return Ok($api_name::$name { respond: (), $($pn),* });
                            }
                        )*

                        request.respond(Err($crate::common::Error::method_not_found())).await;
                    }
                }
            }
        )*
    };
}
