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

use std::fmt::Formatter;
use std::future::Future;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_util::future::{BoxFuture, FutureExt};
use jsonrpsee_core::Error;
use tokio::sync::{watch, OwnedSemaphorePermit, Semaphore, TryAcquireError};
use tokio::time::{self, Duration, Interval};

/// Polling for server stop monitor interval in milliseconds.
const STOP_MONITOR_POLLING_INTERVAL: Duration = Duration::from_millis(1000);

/// This is a flexible collection of futures that need to be driven to completion
/// alongside some other future, such as connection handlers that need to be
/// handled along with a listener for new connections.
///
/// In order to `.await` on these futures and drive them to completion, call
/// `select_with` providing some other future, the result of which you need.
pub(crate) struct FutureDriver<F> {
	futures: Vec<F>,
	stop_monitor_heartbeat: Interval,
}

impl<F> Default for FutureDriver<F> {
	fn default() -> Self {
		let mut heartbeat = time::interval(STOP_MONITOR_POLLING_INTERVAL);

		heartbeat.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

		FutureDriver { futures: Vec::new(), stop_monitor_heartbeat: heartbeat }
	}
}

impl<F> FutureDriver<F> {
	/// Add a new future to this driver
	pub(crate) fn add(&mut self, future: F) {
		self.futures.push(future);
	}
}

impl<F> FutureDriver<F>
where
	F: Future + Unpin,
{
	pub(crate) async fn select_with<S: Future>(&mut self, selector: S) -> S::Output {
		tokio::pin!(selector);

		DriverSelect { selector, driver: self }.await
	}

	fn drive(&mut self, cx: &mut Context) {
		let mut i = 0;

		while i < self.futures.len() {
			if self.futures[i].poll_unpin(cx).is_ready() {
				// Using `swap_remove` since we don't care about ordering
				// but we do care about removing being `O(1)`.
				//
				// We don't increment `i` in this branch, since we now
				// have a shorter length, and potentially a new value at
				// current index
				self.futures.swap_remove(i);
			} else {
				i += 1;
			}
		}
	}

	fn poll_stop_monitor_heartbeat(&mut self, cx: &mut Context) {
		// We don't care about the ticks of the heartbeat, it's here only
		// to periodically wake the `Waker` on `cx`.
		let _ = self.stop_monitor_heartbeat.poll_tick(cx);
	}
}

impl<F> Future for FutureDriver<F>
where
	F: Future + Unpin,
{
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		this.drive(cx);

		if this.futures.is_empty() {
			Poll::Ready(())
		} else {
			Poll::Pending
		}
	}
}

/// This is a glorified select `Future` that will attempt to drive all
/// connection futures `F` to completion on each `poll`, while also
/// handling incoming connections.
struct DriverSelect<'a, S, F> {
	selector: S,
	driver: &'a mut FutureDriver<F>,
}

impl<'a, R, F> Future for DriverSelect<'a, R, F>
where
	R: Future + Unpin,
	F: Future + Unpin,
{
	type Output = R::Output;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		this.driver.drive(cx);
		this.driver.poll_stop_monitor_heartbeat(cx);

		this.selector.poll_unpin(cx)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct StopHandle(watch::Receiver<()>);

impl StopHandle {
	pub(crate) fn new(rx: watch::Receiver<()>) -> Self {
		Self(rx)
	}

	pub(crate) fn shutdown_requested(&self) -> bool {
		// if a message has been seen, it means that `stop` has been called.
		self.0.has_changed().unwrap_or(true)
	}

	pub(crate) async fn shutdown(&mut self) {
		// Err(_) implies that the `sender` has been dropped.
		// Ok(_) implies that `stop` has been called.
		let _ = self.0.changed().await;
	}
}

/// Server handle.
///
/// When all [`StopHandle`]'s have been `dropped` or `stop` has been called
/// the server will be stopped.
pub struct ServerHandle {
	inner: &'static watch::Sender<()>,
	future: BoxFuture<'static, ()>,
}

impl std::fmt::Debug for ServerHandle {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.inner)
	}
}

impl ServerHandle {
	/// Create a new server handle.
	pub fn new(tx: watch::Sender<()>) -> Self {
		// This is a case of self-referencing structs, that are completely
		// sane for this scenario. However, we need to circumvent the
		// compiler's restrictions.
		//
		// The boxed future needs to live for long enough to ensure the
		// compiler's compliance. This means that the sender must be
		// 'static, as the future relies upon `self.inner.closed()` to live
		// long enough.
		//
		// Obtain a static reference by leaking the box on the heap,
		// then reconstruct and clean-up the memory manually.
		//
		// Using `Box::leak` as opposed to `Box::into_raw` has two main advantages:
		//  - It provides `&'a mut T` that can be shared `Send` bounded for the tokio's task
		//  - Allows us to safely use the reference, while still being able to convert it to a pointer.
		//
		// This wouldn't be necessary if the future we are waiting for was independent of `self`.
		let tx = Box::new(tx);
		let inner = Box::leak(Box::new(tx));

		// We need to poll on the same future to wake up when it is completed.
		// Therefore, we can't simply create the `Box::pin` inside the Future's `poll` method.
		// This is the self-referencing part.
		let future = Box::pin(inner.closed());
		Self { inner, future }
	}

	/// Stop the server
	pub fn stop(self) -> Result<impl Future<Output = ()>, Error> {
		self.inner.send(()).map_err(|_| Error::AlreadyStopped)?;
		Ok(self)
	}

	/// Check if the server has been stopped.
	pub fn is_stopped(&self) -> bool {
		self.inner.is_closed()
	}
}

impl Future for ServerHandle {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.future.as_mut().poll(cx)
	}
}

impl Drop for ServerHandle {
	fn drop(&mut self) {
		// We leaked the `watch::Sender<()>` in the `new` constructor.
		// It's time to free up the heap allocation.
		//
		// To go from `&'a [mut] T` to `*[mut] T` we normally would have to go through
		// `Box::leak` first.
		//
		// This is because `Box` is recognized as a "unique pointer" by the compiler's
		// borrow checker, but internally its nothing more than a raw pointer.
		// Directly turning the reference into a raw pointer would not permit aliased raw accesses,
		// and is rejected by the compiler.
		//
		// However, we used exactly `Box::leak` to get the `&'static watch::Sender<()>` reference.
		// Therefore, it is safe to turn that into a pointer, but not by casting.
		let ptr = NonNull::from(self.inner).as_ptr();

		// Reconstruct the `Box` created by `Box::new(tx)` in the `new` constructor
		// to automatically clean up the memory.
		let _free_ptr = unsafe { Box::from_raw(ptr) };
	}
}
/// Limits the number of connections.
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
