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

//! Utilities for handling async code.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_util::future::FutureExt;

/// This is a flexible collection of futures that need to be driven to completion
/// alongside some other future, such as connection handlers that need to be
/// handled along with a listener for new connections.
///
/// In order to `.await` on these futures and drive them to completion, call
/// `select_with` providing some other future, the result of which you need.
pub(crate) struct FutureDriver<F> {
	futures: Vec<F>,
}

impl<F> Default for FutureDriver<F> {
	fn default() -> Self {
		FutureDriver {
			futures: Vec::new()
		}
	}
}

impl<F> FutureDriver<F> {
	/// Get the count of remaining futures on this driver
	pub(crate) fn count(&self) -> usize {
		self.futures.len()
	}

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

		this.selector.poll_unpin(cx)
	}
}
