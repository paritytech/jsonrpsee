#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

//pub use crate::server::run;

#[macro_use]
mod rpc_api;

pub mod client;
pub mod server;
pub mod types;
