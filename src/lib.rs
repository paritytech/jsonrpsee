#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

//pub use crate::server::run;
pub use self::wrappers::*;

#[macro_use]
pub mod rpc_api; // TODO: not pub
mod wrappers;

pub mod client;
pub mod common;
pub mod server;
