macro_rules! cfg_client {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "client")]
            #[cfg_attr(docsrs, doc(cfg(feature = "client")))]
            $item
        )*
    }
}

macro_rules! cfg_server {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "server")]
            #[cfg_attr(docsrs, doc(cfg(feature = "server")))]
            $item
        )*
    }
}

macro_rules! cfg_http_helpers {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "http-helpers")]
            #[cfg_attr(docsrs, doc(cfg(feature = "http-helpers")))]
            $item
        )*
    }
}

macro_rules! cfg_async_client {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "async-wasm-client", feature = "async-client"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "async-client")))]
            #[cfg_attr(docsrs, doc(cfg(feature = "async-wasm-client")))]
            $item
        )*
    }
}
