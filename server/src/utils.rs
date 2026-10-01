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

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::{HttpBody, HttpRequest};

use futures_util::future;
use hyper_util::rt::{TokioExecutor, TokioIo};
use jsonrpsee_core::BoxError;
use pin_project::pin_project;
use tokio::sync::Notify;
use tower::ServiceExt;
use tower::util::Oneshot;

/// Default for how long a connection may take to send its first request.
pub(crate) const DEFAULT_FIRST_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

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

/// Notifies `request_received` whenever hyper passes a request to the service.
#[derive(Debug, Clone)]
pub(crate) struct NotifyOnRequest<S> {
	service: S,
	request_received: Arc<Notify>,
}

impl<S> NotifyOnRequest<S> {
	pub(crate) fn new(service: S, request_received: Arc<Notify>) -> Self {
		Self { service, request_received }
	}
}

impl<S, R> hyper::service::Service<R> for NotifyOnRequest<S>
where
	S: hyper::service::Service<R>,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = S::Future;

	fn call(&self, request: R) -> Self::Future {
		self.request_received.notify_one();
		self.service.call(request)
	}
}

/// Resolves with an error if no request was received within `timeout`, otherwise never.
pub(crate) async fn no_request_within(request_received: &Notify, timeout: Option<Duration>) -> BoxError {
	if let Some(timeout) = timeout {
		if tokio::time::timeout(timeout, request_received.notified()).await.is_err() {
			return io::Error::new(io::ErrorKind::TimedOut, "no request received in time").into();
		}
	}

	future::pending().await
}

/// Serve a service over a TCP connection without graceful shutdown.
/// This means that pending requests will be dropped when the server is stopped.
///
/// Connections which don't send a request within 30 seconds are closed.
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
	let request_received = Arc::new(Notify::new());
	let service =
		NotifyOnRequest::new(hyper_util::service::TowerToHyperService::new(service), request_received.clone());
	let io = TokioIo::new(io);

	let builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());
	let conn = builder.serve_connection_with_upgrades(io, service);

	tokio::select! {
		res = conn => res,
		err = no_request_within(&request_received, Some(DEFAULT_FIRST_REQUEST_TIMEOUT)) => Err(err),
	}
}

/// Serve a service over a TCP connection with graceful shutdown.
/// This means that pending requests will be completed before the server is stopped.
///
/// Connections which don't send a request within 30 seconds are closed.
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
	let request_received = Arc::new(Notify::new());
	let service =
		NotifyOnRequest::new(hyper_util::service::TowerToHyperService::new(service), request_received.clone());
	let io = TokioIo::new(io);

	let builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());
	let conn = builder.serve_connection_with_upgrades(io, service);

	tokio::pin!(stopped, conn);

	tokio::select! {
		// Return if the connection was completed.
		res = &mut conn => res,
		// If the server is stopped, we should gracefully shutdown
		// the connection and poll it until it finishes.
		_ = &mut stopped => {
			conn.as_mut().graceful_shutdown();
			conn.await
		}
		err = no_request_within(&request_received, Some(DEFAULT_FIRST_REQUEST_TIMEOUT)) => Err(err),
	}
}

/// Deserialize calls, notifications and responses with HTTP extensions.
pub mod deserialize_with_ext {
	/// Method call.
	pub mod call {
		use jsonrpsee_types::Request;

		/// Wrapper over `serde_json::from_slice` that sets the extensions.
		pub fn from_slice<'a>(
			data: &'a [u8],
			extensions: &'a http::Extensions,
		) -> Result<Request<'a>, serde_json::Error> {
			let mut req: Request = serde_json::from_slice(data)?;
			*req.extensions_mut() = extensions.clone();
			Ok(req)
		}

		/// Wrapper over `serde_json::from_str` that sets the extensions.
		pub fn from_str<'a>(data: &'a str, extensions: &'a http::Extensions) -> Result<Request<'a>, serde_json::Error> {
			let mut req: Request = serde_json::from_str(data)?;
			*req.extensions_mut() = extensions.clone();
			Ok(req)
		}
	}

	/// Notification.
	pub mod notif {
		use jsonrpsee_types::Notification;

		/// Wrapper over `serde_json::from_slice` that sets the extensions.
		pub fn from_slice<'a, T>(
			data: &'a [u8],
			extensions: &'a http::Extensions,
		) -> Result<Notification<'a, T>, serde_json::Error>
		where
			T: serde::Deserialize<'a>,
		{
			let mut notif: Notification<T> = serde_json::from_slice(data)?;
			*notif.extensions_mut() = extensions.clone();
			Ok(notif)
		}

		/// Wrapper over `serde_json::from_str` that sets the extensions.
		pub fn from_str<'a, T>(
			data: &'a str,
			extensions: &http::Extensions,
		) -> Result<Notification<'a, T>, serde_json::Error>
		where
			T: serde::Deserialize<'a>,
		{
			let mut notif: Notification<T> = serde_json::from_str(data)?;
			*notif.extensions_mut() = extensions.clone();
			Ok(notif)
		}
	}
}
