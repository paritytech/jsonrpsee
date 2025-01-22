use std::sync::Arc;

use futures_util::{future::BoxFuture, FutureExt};
use hyper::body::Bytes;
use jsonrpsee_core::{
	middleware::RpcServiceT,
	server::{MethodResponse, ResponsePayload},
	BoxError, JsonRawValue,
};
use jsonrpsee_types::{Id, Response};
use tower::Service;

use crate::{
	transport::{Error, HttpTransportClient},
	HttpRequest, HttpResponse, IsNotification,
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

		let is_notification = request.extensions().get::<IsNotification>().is_some();

		async move {
			if is_notification {
				service.send(raw).await.map_err(BoxError::from).unwrap();
				MethodResponse::response(Id::Null, ResponsePayload::success(""), max_response_size as usize)
			} else {
				let bytes = service.send_and_read_body(raw).await.map_err(BoxError::from).unwrap();
				let rp: Response<Box<JsonRawValue>> = serde_json::from_slice(&bytes).unwrap();
				MethodResponse::response(rp.id, rp.payload.into(), max_response_size as usize)
			}
		}
		.boxed()
	}
}
