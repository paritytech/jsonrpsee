macro_rules! cfg_client {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "jsonrpsee-http-client", feature = "jsonrpsee-ws-client", feature = "client", feature = "async-client"))]
            $item
        )*
    }
}

macro_rules! cfg_http_client {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "jsonrpsee-http-client")]
            #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee-http-client")))]
            $item
        )*
    }
}

macro_rules! cfg_ws_client {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "jsonrpsee-ws-client")]
            #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee-ws-client")))]
            $item
        )*
    }
}

macro_rules! cfg_wasm_client {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "jsonrpsee-wasm-client")]
            #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee-wasm-client")))]
            $item
        )*
    }
}

macro_rules! cfg_async_client {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "async-client")]
            #[cfg_attr(docsrs, doc(cfg(feature = "async-client")))]
            $item
        )*
    }
}

macro_rules! cfg_client_transport {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "jsonrpsee-client-transport")]
            #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee-client-transport")))]
            $item
        )*
    }
}

macro_rules! cfg_server {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "jsonrpsee-http-server", feature = "jsonrpsee-ws-server"))]
            $item
        )*
    }
}

macro_rules! cfg_http_server {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "jsonrpsee-http-server")]
            #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee-http-server")))]
            $item
        )*
    }
}

macro_rules! cfg_ws_server {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "jsonrpsee-ws-server")]
            #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee-ws-server")))]
            $item
        )*
    }
}

macro_rules! cfg_proc_macros {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "jsonrpsee-proc-macros")]
            #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee-proc-macros")))]
            $item
        )*
    }
}

macro_rules! cfg_types {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "jsonrpsee-types")]
            #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee-types")))]
            $item
        )*
    }
}

macro_rules! cfg_client_or_server {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "jsonrpsee-http-client", feature = "jsonrpsee-ws-client", feature = "client", feature = "async-client", feature = "jsonrpsee-ws-server", feature = "jsonrpsee-http-client"))]
            $item
        )*
    }
}
