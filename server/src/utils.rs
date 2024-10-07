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

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use crate::{HttpBody, HttpRequest, LOG_TARGET};

use futures_util::future::{self, Either};
use hyper_util::rt::{TokioExecutor, TokioIo};
use jsonrpsee_core::BoxError;
use pin_project::pin_project;
use tower::util::Oneshot;
use tower::ServiceExt;

#[derive(Debug, Copy, Clone)]
pub(crate) struct TowerToHyperService<S> {
	service: S,
}

impl<S> TowerToHyperService<S> {
	pub(crate) fn new(service: S) -> Self {
		Self { service }
	}
}

impl<S> hyper::service::Service<HttpRequest<hyper::body::Incoming>> for TowerToHyperService<S>
where
	S: tower::Service<HttpRequest> + Clone,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = TowerToHyperServiceFuture<S, HttpRequest>;

	fn call(&self, req: HttpRequest<hyper::body::Incoming>) -> Self::Future {
		let req = req.map(HttpBody::new);
		TowerToHyperServiceFuture { future: self.service.clone().oneshot(req) }
	}
}

#[pin_project]
pub(crate) struct TowerToHyperServiceFuture<S, R>
where
	S: tower::Service<R>,
{
	#[pin]
	future: Oneshot<S, R>,
}

impl<S, R> std::future::Future for TowerToHyperServiceFuture<S, R>
where
	S: tower::Service<R>,
{
	type Output = Result<S::Response, S::Error>;

	#[inline]
	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.project().future.poll(cx)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct PendingPings {
	list: VecDeque<Instant>,
	max_missed_pings: usize,
	missed_pings: usize,
	max_inactivity_dur: Duration,
	conn_id: u32,
}

impl PendingPings {
	pub(crate) fn new(max_missed_pings: usize, max_inactivity_dur: Duration, conn_id: u32) -> Self {
		Self { list: VecDeque::new(), max_missed_pings, max_inactivity_dur, missed_pings: 0, conn_id }
	}

	fn log_ping_expired(elapsed: Duration, conn_id: u32, max_inactivity_dur: Duration) {
		tracing::debug!(target: LOG_TARGET, "Ping/pong keep alive for conn_id={conn_id}, elapsed={}ms/max={}ms", elapsed.as_millis(), max_inactivity_dur.as_millis());
	}

	fn log_connection_closed(missed_pings: usize, conn_id: u32) {
		tracing::debug!(target: LOG_TARGET, "Missed {missed_pings} ping/pongs for conn_id={conn_id}; closing connection");
	}

	pub(crate) fn push(&mut self, instant: Instant) {
		self.list.push_back(instant);
	}

	/// Check if there are any pending pings that have expired
	///
	/// It's different from [`PendingPing::alive_response`] because
	/// this shouldn't be used when data is received.
	///
	/// It's just way to ensure that pings are checked despite no message is received on the
	/// connection.
	///
	/// Returns `true` if the connection is still alive, `false` otherwise.
	pub(crate) fn check_alive(&mut self) -> bool {
		let mut list = VecDeque::new();

		for ping_start in self.list.drain(..) {
			if ping_start.elapsed() >= self.max_inactivity_dur {
				self.missed_pings += 1;
				Self::log_ping_expired(ping_start.elapsed(), self.conn_id, self.max_inactivity_dur);
			} else {
				list.push_back(ping_start);
			}

			if self.missed_pings >= self.max_missed_pings {
				Self::log_connection_closed(self.missed_pings, self.conn_id);
				return false;
			}
		}

		self.list = list;
		true
	}

	/// Register a alive response.
	///
	/// Returns `true` if the pong was answered in time, `false` otherwise.
	pub(crate) fn alive_response(&mut self, end: Instant) -> bool {
		for ping_start in self.list.drain(..) {
			// Calculate the round-trip time (RTT) of the ping/pong.
			// We adjust for the time when the pong was received.
			let elapsed = ping_start.elapsed().saturating_sub(end.elapsed());

			tracing::trace!(target: LOG_TARGET, "ws_ping_pong_rtt={}ms, conn_id={}", elapsed.as_millis(), self.conn_id);

			if elapsed >= self.max_inactivity_dur {
				self.missed_pings += 1;
				Self::log_ping_expired(ping_start.elapsed(), self.conn_id, self.max_inactivity_dur);
			} else {
				self.missed_pings = 0;
			}

			if self.missed_pings >= self.max_missed_pings {
				Self::log_connection_closed(self.missed_pings, self.conn_id);
				return false;
			}
		}

		true
	}
}

/// Serve a service over a TCP connection without graceful shutdown.
/// This means that pending requests will be dropped when the server is stopped.
///
/// If you want to gracefully shutdown the server, use [`serve_with_graceful_shutdown`] instead.
pub async fn serve<S, B, I>(io: I, service: S) -> Result<(), BoxError>
where
	S: tower::Service<http::Request<hyper::body::Incoming>, Response = http::Response<B>> + Clone + Send + 'static,
	S::Future: Send,
	S::Response: Send,
	S::Error: Into<BoxError>,
	B: http_body::Body<Data = hyper::body::Bytes> + Send + 'static,
	B::Error: Into<BoxError>,
	I: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send + Unpin + 'static,
{
	let service = hyper_util::service::TowerToHyperService::new(service);
	let io = TokioIo::new(io);

	let builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());
	let conn = builder.serve_connection_with_upgrades(io, service);
	conn.await
}

/// Serve a service over a TCP connection with graceful shutdown.
/// This means that pending requests will be completed before the server is stopped.
pub async fn serve_with_graceful_shutdown<S, B, I>(
	io: I,
	service: S,
	stopped: impl Future<Output = ()>,
) -> Result<(), BoxError>
where
	S: tower::Service<http::Request<hyper::body::Incoming>, Response = http::Response<B>> + Clone + Send + 'static,
	S::Future: Send,
	S::Response: Send,
	S::Error: Into<BoxError>,
	B: http_body::Body<Data = hyper::body::Bytes> + Send + 'static,
	B::Error: Into<BoxError>,
	I: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send + Unpin + 'static,
{
	let service = hyper_util::service::TowerToHyperService::new(service);
	let io = TokioIo::new(io);

	let builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());
	let conn = builder.serve_connection_with_upgrades(io, service);

	tokio::pin!(stopped, conn);

	match future::select(conn, stopped).await {
		// Return if the connection was completed.
		Either::Left((conn, _)) => conn,
		// If the server is stopped, we should gracefully shutdown
		// the connection and poll it until it finishes.
		Either::Right((_, mut conn)) => {
			conn.as_mut().graceful_shutdown();
			conn.await
		}
	}
}

/// Helpers to deserialize a request with extensions.
pub(crate) mod deserialize {
	/// Helper to deserialize a request with extensions.
	pub(crate) fn from_slice_with_extensions(
		data: &[u8],
		extensions: http::Extensions,
	) -> Result<jsonrpsee_types::Request, serde_json::Error> {
		let mut req: jsonrpsee_types::Request = serde_json::from_slice(data)?;
		*req.extensions_mut() = extensions;
		Ok(req)
	}

	/// Helper to deserialize a request with extensions.
	pub(crate) fn from_str_with_extensions(
		data: &str,
		extensions: http::Extensions,
	) -> Result<jsonrpsee_types::Request, serde_json::Error> {
		let mut req: jsonrpsee_types::Request = serde_json::from_str(data)?;
		*req.extensions_mut() = extensions;
		Ok(req)
	}
}

#[cfg(test)]
mod tests {
	use super::PendingPings;
	use std::time::{Duration, Instant};

	#[test]
	fn pending_ping_works() {
		let mut pending_pings = PendingPings::new(1, std::time::Duration::from_secs(1), 0);

		pending_pings.push(Instant::now());
		assert!(pending_pings.alive_response(std::time::Instant::now()));
		assert!(pending_pings.list.is_empty());
		assert_eq!(pending_pings.missed_pings, 0);
	}

	#[test]
	fn inactive_too_long() {
		let mut pending_pings = PendingPings::new(2, std::time::Duration::from_millis(100), 0);

		pending_pings.push(Instant::now());
		pending_pings.push(Instant::now());

		std::thread::sleep(Duration::from_millis(200));

		assert!(!pending_pings.check_alive());
		assert_eq!(pending_pings.missed_pings, 2);
	}

	#[test]
	fn active_reset_counter() {
		let mut pending_pings = PendingPings::new(2, std::time::Duration::from_millis(100), 0);
		pending_pings.push(std::time::Instant::now());

		std::thread::sleep(Duration::from_millis(200));

		assert!(pending_pings.check_alive());
		assert_eq!(pending_pings.missed_pings, 1);

		pending_pings.push(std::time::Instant::now());
		assert!(pending_pings.alive_response(Instant::now()));
		assert_eq!(pending_pings.missed_pings, 0);
	}
}
