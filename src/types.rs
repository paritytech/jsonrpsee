//! Type definitions for JSON-RPC.
//!
//! All these types implement the `Serialize` and `Deserialize` traits of the `serde` library.

pub mod error;
pub mod id;
pub mod params;
pub mod request;
pub mod response;
pub mod version;

pub use serde_json::{from_value, to_string, value::to_value};
pub use serde_json::Map as JsonMap;
pub use serde_json::Number as JsonNumber;
pub use serde_json::Value as JsonValue;

pub use self::error::{Error, ErrorCode};
pub use self::id::Id;
pub use self::params::Params;
pub use self::request::{Call, MethodCall, Notification, Request};
pub use self::response::{Failure, Output, Response, Success};
pub use self::version::Version;
