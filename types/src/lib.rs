// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

//! JSON-RPC specific types.

#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

/// JSON-RPC params related types.
pub mod params;

/// JSON-RPC request object related types
pub mod request;

/// JSON-RPC response object related types.
pub mod response;

/// JSON-RPC response error object related types.
pub mod error;

pub use error::{ErrorObject, ErrorObjectOwned, ErrorResponse};
pub use params::{Id, Params, ParamsSequence, SubscriptionId, TwoPointZero};
pub use request::{InvalidRequest, Notification, NotificationSer, Request, RequestSer};
pub use response::{Response, SubscriptionPayload, SubscriptionResponse};
