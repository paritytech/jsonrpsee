#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

pub use crate::client::raw::RawClient;
pub use crate::client::Client;
pub use crate::local::local;
pub use crate::server::raw::RawServer;
pub use crate::server::{Server, ServerEvent, ServerRequestId, ServerSubscriptionId};

pub mod client;
pub mod common;
pub mod local;
pub mod server;
