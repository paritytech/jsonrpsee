use std::sync::Arc;

use futures_util::{future::BoxFuture, FutureExt};
use hyper::body::Bytes;
use jsonrpsee_core::{
	middleware::{Notification, RpcServiceT},
	server::{BatchResponseBuilder, MethodResponse, ResponsePayload},
	BoxError, JsonRawValue,
};
use jsonrpsee_types::{Id, Response, ResponseSuccess};
use tower::Service;

use crate::{
	transport::{Error, HttpTransportClient},
	HttpRequest, HttpResponse,
};

#[derive(Clone, Debug)]
pub struct RpcService<HttpMiddleware> {
	service: Arc<HttpTransportClient<HttpMiddleware>>,
	max_response_size: u32,
}

impl<HttpMiddleware> RpcService<HttpMiddleware> {
	pub fn new(service: HttpTransportClient<HttpMiddleware>, max_response_size: u32) -> Self {
		Self { service: Arc::new(service), max_response_size }
	}
}

impl<'a, B, HttpMiddleware> RpcServiceT<'a> for RpcService<HttpMiddleware>
where
	HttpMiddleware: Service<HttpRequest, Response = HttpResponse<B>, Error = Error> + Clone + Send + Sync + 'static,
	HttpMiddleware::Future: Send,
	B: http_body::Body<Data = Bytes> + Send + 'static,
	B::Data: Send,
	B::Error: Into<BoxError>,
{
	type Future = BoxFuture<'a, MethodResponse>;

	fn call(&self, request: jsonrpsee_types::Request<'a>) -> Self::Future {
		let raw = serde_json::to_string(&request).unwrap();
		let service = self.service.clone();
		let max_response_size = self.max_response_size;

		async move {
			let bytes = service.send_and_read_body(raw).await.map_err(BoxError::from).unwrap();
			let rp: Response<Box<JsonRawValue>> = serde_json::from_slice(&bytes).unwrap();
			MethodResponse::response(rp.id, rp.payload.into(), max_response_size as usize)
		}
		.boxed()
	}

	fn batch(&self, requests: Vec<jsonrpsee_types::Request<'a>>) -> Self::Future {
		let raw = serde_json::to_string(&requests).unwrap();
		let service = self.service.clone();
		let max_response_size = self.max_response_size;

		async move {
			let bytes = service.send_and_read_body(raw).await.map_err(BoxError::from).unwrap();
			let json_rps: Vec<Response<&JsonRawValue>> = serde_json::from_slice(&bytes).unwrap();
			let mut batch = BatchResponseBuilder::new_with_limit(max_response_size as usize);

			for rp in json_rps {
				let id = rp.id.try_parse_inner_as_number().unwrap();

				let response = match ResponseSuccess::try_from(rp) {
					Ok(r) => {
						let payload = ResponsePayload::success(r.result);
						MethodResponse::response(r.id, payload, max_response_size as usize)
					}
					Err(err) => MethodResponse::error(Id::Number(id), err),
				};

				if let Err(rp) = batch.append(response) {
					return rp;
				}
			}

			MethodResponse::from_batch(batch.finish())
		}
		.boxed()
	}

	fn notification(&self, notif: Notification<'a>) -> Self::Future {
		let raw = serde_json::to_string(&notif).unwrap();
		let service = self.service.clone();

		async move {
			service.send(raw).await.map_err(BoxError::from).unwrap();
			MethodResponse::notification()
		}
		.boxed()
	}
}
