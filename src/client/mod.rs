#[cfg(feature = "http")]
mod http;
#[cfg(feature = "ws")]
mod ws;

// TODO: just export `Client` because the underlying layers is not likely to be used.
// Unless we want the user to have to possibility to not spawn a background thread to
// handle responses.
#[cfg(feature = "http")]
pub use http::{Client as HttpClient, HttpTransportClient, RawClient as HttpRawClient};
#[cfg(feature = "ws")]
pub use ws::{Client as WsClient, RawClient as RawWsClient, Subscription as WsSubscription, WsTransportClient};
