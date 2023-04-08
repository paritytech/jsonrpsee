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
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_util::future::FutureExt;
use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;
use jsonrpsee_core::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{watch, OwnedSemaphorePermit, Semaphore, TryAcquireError};

/// This is a flexible collection of futures that need to be driven to completion
/// alongside some other future, such as connection handlers that need to be
/// handled along with a listener for new connections.
///
/// In order to `.await` on these futures and drive them to completion, call
/// `select_with` providing some other future, the result of which you need.
///
///

/// This is a glorified select `Future` that will attempt to drive all
/// connection futures `F` to completion on each `poll`, while also
/// handling incoming connections.
pub(crate) struct DriverSelect<'a, S, F> {
	selector: S,
	futures: &'a mut FuturesUnordered<F>,
}

impl<'a, S, F> DriverSelect<'a, S, F>
where
	S: Future + Unpin,
	F: Future + Unpin,
{
	pub(crate) fn new(selector: S, futures: &'a mut FuturesUnordered<F>) -> Self {
		Self { selector, futures }
	}
}

impl<'a, S, F> Future for DriverSelect<'a, S, F>
where
	S: Future + Unpin,
	F: Future + Unpin,
{
	type Output = S::Output;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		// Wakeup the list of pending futures and just check one item.
		if !this.futures.is_empty() {
			// don't care about the result, just remove the completed one from list of pending futures.
			_ = this.futures.poll_next_unpin(cx);
		}

		this.selector.poll_unpin(cx)
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

	pub(crate) fn shutdown_requested(&self) -> bool {
		self.0.has_changed().unwrap_or(true)
	}
}

/// This is a glorified select listening for new messages, while also checking the `stop_receiver` signal.
pub(crate) struct Monitored<'a, F> {
	future: F,
	stop_monitor: &'a StopHandle,
}

impl<'a, F> Monitored<'a, F> {
	pub(crate) fn new(future: F, stop_monitor: &'a StopHandle) -> Self {
		Monitored { future, stop_monitor }
	}
}

pub(crate) enum MonitoredError<E> {
	Shutdown,
	Selector(E),
}

pub(crate) struct Incoming(pub(crate) TcpListener);

impl<'a> Future for Monitored<'a, Incoming> {
	type Output = Result<(TcpStream, SocketAddr), MonitoredError<std::io::Error>>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		if this.stop_monitor.shutdown_requested() {
			return Poll::Ready(Err(MonitoredError::Shutdown));
		}

		this.future.0.poll_accept(cx).map_err(MonitoredError::Selector)
	}
}

impl<'a, 'f, F, T, E> Future for Monitored<'a, Pin<&'f mut F>>
where
	F: Future<Output = Result<T, E>>,
{
	type Output = Result<T, MonitoredError<E>>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		if this.stop_monitor.shutdown_requested() {
			return Poll::Ready(Err(MonitoredError::Shutdown));
		}

		this.future.poll_unpin(cx).map_err(MonitoredError::Selector)
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
