//! Compatibility layer for supporting both tokio v0.2 and v1.

pub(crate) use tokio_impl::*;

#[cfg(feature = "tokioV1")]
mod tokio_impl {
	// Required for `tokio::test` to work correctly.
	#[cfg(test)]
	pub(crate) use tokio1::{runtime, test};

	pub(crate) use tokio1::{net::TcpStream, spawn, sync::Mutex};
	pub(crate) use tokio1_rustls::{
		client::TlsStream,
		webpki::{DNSNameRef, InvalidDNSNameError},
		TlsConnector,
	};
	pub(crate) use tokio1_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

	pub(crate) use tokio1::time::sleep;
}

#[cfg(feature = "tokioV02")]
mod tokio_impl {
	// Required for `tokio::test` to work correctly.
	#[cfg(test)]
	pub(crate) use tokio02::{runtime, test};

	pub(crate) use tokio02::{net::TcpStream, spawn, sync::Mutex};
	pub(crate) use tokio02_rustls::{
		client::TlsStream,
		webpki::{DNSNameRef, InvalidDNSNameError},
		TlsConnector,
	};
	pub(crate) use tokio02_util::compat::{
		Tokio02AsyncReadCompatExt as TokioAsyncReadCompatExt, Tokio02AsyncWriteCompatExt as TokioAsyncWriteCompatExt,
	};

	// In 0.2 `tokio::time::sleep` had different name.
	pub(crate) use tokio02::time::delay_for as sleep;
}

#[cfg(not(any(feature = "tokioV1", feature = "tokioV02")))]
mod tokio_impl {
	compile_error!("Either `tokiov1` or `tokiov02` feature must be enabled");
}
