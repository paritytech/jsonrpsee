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

//! Convenience wrapper for a stream (AsyncRead + AsyncWrite) which can either be plain TCP or TLS.

use futures::{
	io::{IoSlice, IoSliceMut},
	prelude::*,
};
use pin_project::pin_project;
use std::{io::Error as IoError, pin::Pin, task::Context, task::Poll};

/// Stream to represent either a unencrypted or encrypted socket stream.
#[pin_project(project = EitherStreamProj)]
#[derive(Debug, Copy, Clone)]
pub enum EitherStream<S, T> {
	/// Unencrypted socket stream.
	Plain(#[pin] S),
	/// Encrypted socket stream.
	Tls(#[pin] T),
}

impl<S, T> AsyncRead for EitherStream<S, T>
where
	S: AsyncRead,
	T: AsyncRead,
{
	fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<Result<usize, IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => AsyncRead::poll_read(s, cx, buf),
			EitherStreamProj::Tls(t) => AsyncRead::poll_read(t, cx, buf),
		}
	}

	fn poll_read_vectored(
		self: Pin<&mut Self>,
		cx: &mut Context<'_>,
		bufs: &mut [IoSliceMut<'_>],
	) -> Poll<Result<usize, IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => AsyncRead::poll_read_vectored(s, cx, bufs),
			EitherStreamProj::Tls(t) => AsyncRead::poll_read_vectored(t, cx, bufs),
		}
	}
}

impl<S, T> AsyncWrite for EitherStream<S, T>
where
	S: AsyncWrite,
	T: AsyncWrite,
{
	fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => AsyncWrite::poll_write(s, cx, buf),
			EitherStreamProj::Tls(t) => AsyncWrite::poll_write(t, cx, buf),
		}
	}

	fn poll_write_vectored(
		self: Pin<&mut Self>,
		cx: &mut Context<'_>,
		bufs: &[IoSlice<'_>],
	) -> Poll<Result<usize, IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => AsyncWrite::poll_write_vectored(s, cx, bufs),
			EitherStreamProj::Tls(t) => AsyncWrite::poll_write_vectored(t, cx, bufs),
		}
	}

	fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => AsyncWrite::poll_flush(s, cx),
			EitherStreamProj::Tls(t) => AsyncWrite::poll_flush(t, cx),
		}
	}

	fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), IoError>> {
		match self.project() {
			EitherStreamProj::Plain(s) => AsyncWrite::poll_close(s, cx),
			EitherStreamProj::Tls(t) => AsyncWrite::poll_close(t, cx),
		}
	}
}
