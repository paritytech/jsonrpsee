#[cfg(feature = "http")]
mod http;
#[cfg(feature = "ws")]
mod ws;

// TODO: just export `Client` because the underlying layers is not likely to be used.
// Unless we want the user to have to possibility to not spawn a background thread to
// handle responses.
#[cfg(feature = "http")]
pub use http::{HttpClient, HttpConfig, HttpTransportClient};
#[cfg(feature = "ws")]
pub use ws::{
	Client as WsClient, Config as WsConfig, Receiver as WsReceiver, Sender as WsSender, Subscription as WsSubscription,
};
