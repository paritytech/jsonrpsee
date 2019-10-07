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

use crate::common;
use crate::server::Params;
use std::fmt;

/// Notification received on a server.
///
/// Wraps around a `common::Notification`.
#[derive(PartialEq)]
pub struct Notification(common::Notification);

impl From<common::Notification> for Notification {
    fn from(notif: common::Notification) -> Notification {
        Notification(notif)
    }
}

impl From<Notification> for common::Notification {
    fn from(notif: Notification) -> common::Notification {
        notif.0
    }
}

impl Notification {
    /// Returns the method of this notification.
    pub fn method(&self) -> &str {
        &self.0.method
    }

    /// Returns the parameters of the notification.
    pub fn params(&self) -> Params {
        Params::from(&self.0.params)
    }
}

impl fmt::Debug for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Notification")
            .field("method", &self.method())
            .field("params", &self.params())
            .finish()
    }
}
