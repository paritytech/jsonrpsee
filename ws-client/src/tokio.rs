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

//! Compatibility layer for supporting both tokio v0.2 and v1.

// Check that either v1 or v0.2 feature is enabled.
#[cfg(not(any(feature = "tokio1", feature = "tokio02")))]
compile_error!("Either `tokio1` or `tokio02` feature must be enabled");

// Also check that only *one* of them is enabled.
#[cfg(all(feature = "tokio1", feature = "tokio02"))]
compile_error!("feature `tokio1` and `tokio02` are mutually exclusive");

pub(crate) use tokio_impl::*;

#[cfg(feature = "tokio1")]
mod tokio_impl {
	// Required for `tokio::test` to work correctly.
	#[cfg(test)]
	pub(crate) use tokioV1::{runtime, test};

	pub(crate) use tokioV1::{net::TcpStream, spawn, sync::Mutex};
	pub(crate) use tokioV1_rustls::{
		client::TlsStream,
		webpki::{DNSNameRef, InvalidDNSNameError},
		TlsConnector,
	};
	pub(crate) use tokioV1_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

	pub(crate) use tokioV1::time::sleep;

	pub(crate) use tokioV1::select;
}

// Note that we check for `not(feature = "tokio1")` here, but not above.
// This is required so that in case of both features enabled, `tokio_impl`
// will only be defined once. This way, the only error user will get is
// the compile error about features being mutually exclusive, which will
// provide better DevEx.
#[cfg(all(feature = "tokio02", not(feature = "tokio1")))]
mod tokio_impl {
	// Required for `tokio::test` to work correctly.
	#[cfg(test)]
	pub(crate) use tokioV02::{runtime, test};

	pub(crate) use tokioV02::{net::TcpStream, spawn, sync::Mutex};
	pub(crate) use tokioV02_rustls::{
		client::TlsStream,
		webpki::{DNSNameRef, InvalidDNSNameError},
		TlsConnector,
	};
	pub(crate) use tokioV02_util::compat::{
		Tokio02AsyncReadCompatExt as TokioAsyncReadCompatExt, Tokio02AsyncWriteCompatExt as TokioAsyncWriteCompatExt,
	};

	// In 0.2 `tokio::time::sleep` had different name.
	pub(crate) use tokioV02::time::delay_for as sleep;

	pub(crate) use tokioV02::select;
}
