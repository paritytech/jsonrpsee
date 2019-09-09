#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

#[cfg(feature = "http")]
pub use http::{http_client, http_server};

#[doc(inline)]
pub use jsonrpsee_core as core;
#[doc(inline)]
#[cfg(feature = "http")]
pub use jsonrpsee_http as http;
