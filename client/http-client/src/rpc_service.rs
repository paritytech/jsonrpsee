use std::sync::Arc;

use futures_util::{FutureExt, future::BoxFuture};
use hyper::{body::Bytes, http::Extensions};
use jsonrpsee_core::{
	BoxError, JsonRawValue,
	client::{Error, MethodResponse},
	middleware::{Batch, Notification, Request, RpcServiceT},
};
use jsonrpsee_types::{Response, ResponseSuccess};
use tower::Service;

use crate::{
	HttpRequest, HttpResponse,
	transport::{Error as TransportError, HttpTransportClient},
};

#[derive(Clone, Debug)]
pub struct RpcService<HttpMiddleware> {
	service: Arc<HttpTransportClient<HttpMiddleware>>,
}

impl<HttpMiddleware> RpcService<HttpMiddleware> {
	pub fn new(service: HttpTransportClient<HttpMiddleware>) -> Self {
		Self { service: Arc::new(service) }
	}
}

impl<'a, B, HttpMiddleware> RpcServiceT<'a> for RpcService<HttpMiddleware>
where
	HttpMiddleware:
		Service<HttpRequest, Response = HttpResponse<B>, Error = TransportError> + Clone + Send + Sync + 'static,
	HttpMiddleware::Future: Send,
	B: http_body::Body<Data = Bytes> + Send + 'static,
	B::Data: Send,
	B::Error: Into<BoxError>,
{
	type Future = BoxFuture<'a, Result<Self::Response, Self::Error>>;
	type Error = Error;
	type Response = MethodResponse;

	fn call(&self, request: Request<'a>) -> Self::Future {
		let service = self.service.clone();

		async move {
			let raw = serde_json::to_string(&request)?;
			let bytes = service.send_and_read_body(raw).await.map_err(|e| Error::Transport(e.into()))?;
			let json_rp: Response<Box<JsonRawValue>> = serde_json::from_slice(&bytes)?;
			let success = ResponseSuccess::try_from(json_rp)?;
			Ok(MethodResponse::method_call(success.result, request.extensions, success.id.into_owned()))
		}
		.boxed()
	}

	fn batch(&self, batch: Batch<'a>) -> Self::Future {
		let service = self.service.clone();

		async move {
			let raw = serde_json::to_string(&batch)?;
			let bytes = service.send_and_read_body(raw).await.map_err(|e| Error::Transport(e.into()))?;
			let json: Vec<Box<JsonRawValue>> = serde_json::from_slice(&bytes)?;

			let mut extensions = Extensions::new();

			for call in batch.into_iter() {
				extensions.extend(call.into_extensions());
			}

			let json = json.into_iter().map(Ok).collect();

			Ok(MethodResponse::batch(json, extensions))
		}
		.boxed()
	}

	fn notification(&self, notif: Notification<'a>) -> Self::Future {
		let service = self.service.clone();

		async move {
			let raw = serde_json::to_string(&notif)?;
			service.send(raw).await.map_err(|e| Error::Transport(e.into()))?;
			Ok(MethodResponse::notification(notif.extensions))
		}
		.boxed()
	}
}
