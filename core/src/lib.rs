#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

use std::{error, fmt};

pub use crate::client::raw::RawClient;
pub use crate::client::{Client, ClientEvent, ClientRequestId};
pub use crate::local::local_raw;
pub use crate::server::raw::{RawServer, RawServerEvent};
pub use crate::server::{Server, ServerEvent, ServerRequestId, ServerSubscriptionId};

pub mod client;
pub mod common;
pub mod local;
pub mod server;
