use crate::common;
use crate::server::Params;
use std::fmt;

/// Notification received on a server.
///
/// Wraps around a `common::Notification`.
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
