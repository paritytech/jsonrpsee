
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
                        let $pn = $crate::types::to_value($pn).unwrap();        // TODO: don't unwrap
                    )**/

                    let http = $crate::client::Client::new($crate::raw_client::HttpClientPool::new().unwrap());      // TODO: don't unwrap
                    http.request(stringify!($name)).await.unwrap()
                }
            )*

            enum $api_name {
                $(
                    $name { $($pn: $pty),* },
                )*
            }
        )*
    };
}
