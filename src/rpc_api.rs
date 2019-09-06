
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

                    let http = $crate::client::Client::new($crate::client::raw::HttpClientPool::new().unwrap());      // TODO: don't unwrap
                    http.request(stringify!($name)).await.unwrap()
                }
            )*

            enum $api_name {
                $(
                    $name { $($pn: $pty),* },
                )*
            }

            impl $api_name {
                async fn next_request<S>(server: &mut $crate::server::Server<S>) -> Result<Self, std::io::Error>
                    where for<'r> &'r mut S: $crate::server::raw::RawServerRef<'r>
                {
                    panic!()
                }
            }
        )*
    };
}
