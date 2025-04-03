use std::sync::Arc;

use futures_util::FutureExt;
use hyper::{body::Bytes, http::Extensions};
use jsonrpsee_core::{
	BoxError, JsonRawValue,
	client::{Error, MethodResponse},
	middleware::{Batch, Notification, Request, RpcServiceT},
};
use jsonrpsee_types::Response;
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

impl<B, HttpMiddleware> RpcServiceT for RpcService<HttpMiddleware>
where
	HttpMiddleware:
		Service<HttpRequest, Response = HttpResponse<B>, Error = TransportError> + Clone + Send + Sync + 'static,
	HttpMiddleware::Future: Send,
	B: http_body::Body<Data = Bytes> + Send + 'static,
	B::Data: Send,
	B::Error: Into<BoxError>,
{
	type Error = Error;
	type Response = MethodResponse;

	fn call<'a>(&self, request: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let service = self.service.clone();

		async move {
			let raw = serde_json::to_string(&request)?;
			let bytes = service.send_and_read_body(raw).await.map_err(|e| Error::Transport(e.into()))?;
			let json_rp: Response<Box<JsonRawValue>> = serde_json::from_slice(&bytes)?;
			Ok(MethodResponse::method_call(json_rp.into_owned().into(), request.extensions))
		}
		.boxed()
	}

	fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let service = self.service.clone();

		async move {
			let raw = serde_json::to_string(&batch)?;
			let bytes = service.send_and_read_body(raw).await.map_err(|e| Error::Transport(e.into()))?;
			let rp: Vec<_> = serde_json::from_slice::<Vec<Response<Box<JsonRawValue>>>>(&bytes)?
				.into_iter()
				.map(|r| r.into_owned().into())
				.collect();

			let mut extensions = Extensions::new();

			for call in batch.into_batch_entries() {
				let Ok(call) = call else {
					continue;
				};

				extensions.extend(call.into_extensions());
			}

			Ok(MethodResponse::batch(rp, extensions))
		}
		.boxed()
	}

	fn notification<'a>(
		&self,
		notif: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let service = self.service.clone();

		async move {
			let raw = serde_json::to_string(&notif)?;
			service.send(raw).await.map_err(|e| Error::Transport(e.into()))?;
			Ok(MethodResponse::notification(notif.extensions))
		}
		.boxed()
	}
}
