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

//! Convenience wrapper for a stream (AsyncRead + AsyncWrite) which can either be plain TCP or TLS.

use std::io::Error as IoError;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures_util::io::{IoSlice, IoSliceMut};
use futures_util::*;
use pin_project::pin_project;
use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

/// Stream to represent either a unencrypted or encrypted socket stream.
#[pin_project(project = EitherStreamProj)]
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum EitherStream {
	/// Unencrypted socket stream.
	Plain(#[pin] TcpStream),
	/// Encrypted socket stream.
	#[cfg(feature = "tls")]
	Tls(#[pin] tokio_rustls::client::TlsStream<TcpStream>),
}

impl AsyncRead for EitherStream {
	fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<Result<usize, IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => {
				let compat = s.compat();
				futures_util::pin_mut!(compat);
				AsyncRead::poll_read(compat, cx, buf)
			}
			#[cfg(feature = "tls")]
			EitherStreamProj::Tls(t) => {
				let compat = t.compat();
				futures_util::pin_mut!(compat);
				AsyncRead::poll_read(compat, cx, buf)
			}
		}
	}

	fn poll_read_vectored(
		self: Pin<&mut Self>,
		cx: &mut Context,
		bufs: &mut [IoSliceMut],
	) -> Poll<Result<usize, IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => {
				let compat = s.compat();
				futures_util::pin_mut!(compat);
				AsyncRead::poll_read_vectored(compat, cx, bufs)
			}
			#[cfg(feature = "tls")]
			EitherStreamProj::Tls(t) => {
				let compat = t.compat();
				futures_util::pin_mut!(compat);
				AsyncRead::poll_read_vectored(compat, cx, bufs)
			}
		}
	}
}

impl AsyncWrite for EitherStream {
	fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<Result<usize, IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => {
				let compat = s.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_write(compat, cx, buf)
			}
			#[cfg(feature = "tls")]
			EitherStreamProj::Tls(t) => {
				let compat = t.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_write(compat, cx, buf)
			}
		}
	}

	fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context, bufs: &[IoSlice]) -> Poll<Result<usize, IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => {
				let compat = s.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_write_vectored(compat, cx, bufs)
			}
			#[cfg(feature = "tls")]
			EitherStreamProj::Tls(t) => {
				let compat = t.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_write_vectored(compat, cx, bufs)
			}
		}
	}

	fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => {
				let compat = s.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_flush(compat, cx)
			}
			#[cfg(feature = "tls")]
			EitherStreamProj::Tls(t) => {
				let compat = t.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_flush(compat, cx)
			}
		}
	}

	fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => {
				let compat = s.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_close(compat, cx)
			}
			#[cfg(feature = "tls")]
			EitherStreamProj::Tls(t) => {
				let compat = t.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_close(compat, cx)
			}
		}
	}
}
