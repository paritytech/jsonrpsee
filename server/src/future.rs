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

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_util::future::FutureExt;
use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;
use jsonrpsee_core::Error;
use tokio::sync::{watch, OwnedSemaphorePermit, Semaphore, TryAcquireError};

/// This is a flexible collection of futures that need to be driven to completion
/// alongside some other future, such as connection handlers that need to be
/// handled along with a listener for new connections.
pub(crate) struct DriveFutures<'a, Fut1, Fut2> {
	future: Fut1,
	futures: &'a mut FuturesUnordered<Fut2>,
}

impl<'a, Fut1, Fut2> DriveFutures<'a, Fut1, Fut2>
where
	Fut1: Future + Unpin,
	Fut2: Future + Unpin,
{
	pub(crate) fn new(future: Fut1, futures: &'a mut FuturesUnordered<Fut2>) -> Self {
		Self { future, futures }
	}
}

impl<'a, Fut1, Fut2> Future for DriveFutures<'a, Fut1, Fut2>
where
	Fut1: Future + Unpin,
	Fut2: Future + Unpin,
{
	type Output = Fut1::Output;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		// Wakeup the list of pending futures and poll just one item.
		if !this.futures.is_empty() {
			// Don't care about the result, just remove the completed one from list of pending futures.
			_ = this.futures.poll_next_unpin(cx);
		}

		this.future.poll_unpin(cx)
	}
}

/// Represent a stop handle which is a wrapper over a `multi-consumer receiver`
/// and cloning [`StopHandle`] will get a separate instance of the underlying receiver.
#[derive(Debug, Clone)]
pub(crate) struct StopHandle(watch::Receiver<()>);

impl StopHandle {
	pub(crate) fn new(rx: watch::Receiver<()>) -> Self {
		Self(rx)
	}

	/// A future that resolves when server has been stopped
	/// it consumes the stop handle.
	pub(crate) async fn shutdown(mut self) {
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
	pub fn new(tx: watch::Sender<()>) -> Self {
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
#[derive(Debug)]
pub(crate) struct ConnectionGuard(Arc<Semaphore>);

impl ConnectionGuard {
	pub(crate) fn new(limit: usize) -> Self {
		Self(Arc::new(Semaphore::new(limit)))
	}

	pub(crate) fn try_acquire(&self) -> Option<OwnedSemaphorePermit> {
		match self.0.clone().try_acquire_owned() {
			Ok(guard) => Some(guard),
			Err(TryAcquireError::Closed) => unreachable!("Semaphore::Close is never called and can't be closed; qed"),
			Err(TryAcquireError::NoPermits) => None,
		}
	}

	pub(crate) fn available_connections(&self) -> usize {
		self.0.available_permits()
	}
}
