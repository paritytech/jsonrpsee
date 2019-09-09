#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

//pub use crate::server::run;

#[macro_use]
pub mod rpc_api; // TODO: not pub

pub mod client;
pub mod common;
pub mod server;
