// Copyright 2019 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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

pub use serde::{de::DeserializeOwned, ser::Serialize};
pub use serde_json::Map as JsonMap;
pub use serde_json::Number as JsonNumber;
pub use serde_json::Value as JsonValue;
pub use serde_json::{from_slice, from_value, to_string, to_value, to_vec};

pub use self::error::{Error, ErrorCode};
pub use self::id::Id;
pub use self::params::Params;
pub use self::request::{Call, MethodCall, Notification, Request};
pub use self::response::{
    Failure, Output, Response, SubscriptionId, SubscriptionNotif, SubscriptionNotifParams, Success,
};
pub use self::version::Version;
