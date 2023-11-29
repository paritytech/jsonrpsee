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

//! Utilities for handling async code.

use futures_util::{Stream, StreamExt};
use jsonrpsee_core::Error;
use pin_project::pin_project;
use std::{
	pin::Pin,
	sync::Arc,
	task::{Context, Poll},
};
use tokio::{
	sync::{watch, OwnedSemaphorePermit, Semaphore, TryAcquireError},
	time::interval,
};

/// Create channel to determine whether
/// the server shall continue to run or not.
pub fn stop_channel() -> (StopHandle, ServerHandle) {
	let (tx, rx) = tokio::sync::watch::channel(());
	(StopHandle::new(rx), ServerHandle::new(tx))
}

/// Represent a stop handle which is a wrapper over a `multi-consumer receiver`
/// and cloning [`StopHandle`] will get a separate instance of the underlying receiver.
#[derive(Debug, Clone)]
pub struct StopHandle(watch::Receiver<()>);

impl StopHandle {
	/// Create a new stop handle.
	pub(crate) fn new(rx: watch::Receiver<()>) -> Self {
		Self(rx)
	}

	/// A future that resolves when server has been stopped
	/// it consumes the stop handle.
	pub async fn shutdown(mut self) {
		let _ = self.0.changed().await;
	}
}

/// Server handle.
///
/// When all [`StopHandle`]'s have been `dropped` or `stop` has been called
/// the server will be stopped.
#[derive(Debug, Clone)]
pub struct ServerHandle(Arc<watch::Sender<()>>);

impl ServerHandle {
	/// Create a new server handle.
	pub(crate) fn new(tx: watch::Sender<()>) -> Self {
		Self(Arc::new(tx))
	}

	/// Tell the server to stop without waiting for the server to stop.
	pub fn stop(&self) -> Result<(), Error> {
		self.0.send(()).map_err(|_| Error::AlreadyStopped)
	}

	/// Wait for the server to stop.
	pub async fn stopped(self) {
		self.0.closed().await
	}

	/// Check if the server has been stopped.
	pub fn is_stopped(&self) -> bool {
		self.0.is_closed()
	}
}

/// Limits the number of connections.
#[derive(Clone, Debug)]
pub struct ConnectionGuard {
	inner: Arc<Semaphore>,
	max: usize,
}

impl ConnectionGuard {
	/// Create a new connection guard.
	pub fn new(limit: usize) -> Self {
		Self { inner: Arc::new(Semaphore::new(limit)), max: limit }
	}

	/// Acquire a connection permit.
	pub fn try_acquire(&self) -> Option<ConnectionPermit> {
		match self.inner.clone().try_acquire_owned() {
			Ok(guard) => Some(guard),
			Err(TryAcquireError::Closed) => unreachable!("Semaphore::Close is never called and can't be closed; qed"),
			Err(TryAcquireError::NoPermits) => None,
		}
	}

	/// Get the number of available connection slots.
	pub fn available_connections(&self) -> usize {
		self.inner.available_permits()
	}

	/// Get the maximum number of connections.
	pub fn max_connections(&self) -> usize {
		self.max
	}
}

/// Connection permit.
pub type ConnectionPermit = OwnedSemaphorePermit;

#[pin_project]
pub(crate) struct IntervalStream(#[pin] Option<tokio_stream::wrappers::IntervalStream>);

impl IntervalStream {
	/// Creates a stream which never returns any elements.
	pub(crate) fn pending() -> Self {
		Self(None)
	}

	/// Creates a stream which produces elements with `period`.
	pub(crate) async fn new(period: std::time::Duration) -> Self {
		let mut interval = interval(period);
		interval.tick().await;

		Self(Some(tokio_stream::wrappers::IntervalStream::new(interval)))
	}
}

impl Stream for IntervalStream {
	type Item = tokio::time::Instant;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		if let Some(mut stream) = self.project().0.as_pin_mut() {
			stream.poll_next_unpin(cx)
		} else {
			// NOTE: this will not be woken up again and it's by design
			// to be a pending stream that never returns.
			Poll::Pending
		}
	}
}
