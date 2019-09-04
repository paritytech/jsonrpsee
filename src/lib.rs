#![deny(unsafe_code)]
#![warn(missing_docs)]

//pub use crate::server::run;
pub use serde_json::{Value as JsonValue, Map as JsonMap, Number as JsonNumber};

pub mod raw_server;
pub mod server;

pub struct Error {
    pub code: JsonNumber,
    pub message: String,
    pub data: JsonValue,
}
