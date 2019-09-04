
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
                    use $crate::raw_client::RawClientRef;

                    $(
                        let $pn = $crate::types::to_value($pn).unwrap();        // TODO: don't unwrap
                    )*

                    let method_call = $crate::types::MethodCall {
                        jsonrpc: Some($crate::types::Version::V2),
                        method: stringify!($name).to_owned(),
                        params: $crate::types::Params::None/*::Map(
                            Default::default()      // TODO:
                        )*/,
                        id: $crate::types::Id::Num(5),
                    };
                    let request = $crate::types::Request::Single($crate::types::Call::MethodCall(method_call));

                    let http = $crate::raw_client::HttpClientPool::new().unwrap();      // TODO: don't unwrap
                    let result = http.request("http://localhost:9933", request).await.unwrap();

                    let val = match result {
                        $crate::types::Response::Single($crate::types::Output::Success(s)) => s,
                        _ => panic!("error in request")       // TODO: no
                    };

                    $crate::types::from_value(val.result).unwrap()     // TODO: don't unwrap
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
