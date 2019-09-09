//! Type definitions from the JSON-RPC specifications.
//!
//! All these common implement the `Serialize` and `Deserialize` traits of the `serde` library
//! and can be serialize/deserialized using the `to_string`/`to_vec`/`from_slice` methods.

mod error;
mod id;
mod params;
mod request;
mod response;
mod version;

pub use serde_json::Map as JsonMap;
pub use serde_json::Number as JsonNumber;
pub use serde_json::Value as JsonValue;
pub use serde_json::{from_slice, from_value, to_string, to_value, to_vec};

pub use self::error::{Error, ErrorCode};
pub use self::id::Id;
pub use self::params::Params;
pub use self::request::{Call, MethodCall, Notification, Request};
pub use self::response::{Failure, Output, Response, Success};
pub use self::version::Version;
