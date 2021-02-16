#[cfg(all(feature = "tokio1", feature = "tokio02"))]
compile_error!("feature `tokio1` and `tokio02` are mutably exclusive");

#[cfg(not(any(feature = "tokio1", feature = "tokio02")))]
compile_error!("feature `tokio1` or `tokio02` must be enabled for this crate");

#[cfg(all(feature = "tokio1", not(feature = "tokio02")))]
extern crate hyper14 as hyper;

#[cfg(all(feature = "tokio02", not(feature = "tokio1")))]
extern crate hyper13 as hyper;

mod client;
mod transport;

#[cfg(test)]
mod tests;

pub use client::HttpClient;
pub use jsonrpsee_types::http::HttpConfig;
pub use transport::HttpTransportClient;
