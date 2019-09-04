use crate::types;
use futures::prelude::*;
use std::io;

pub use self::http::HttpClientPool;

pub mod http;

pub trait RawClient {
    type Future: Future<Output = Result<types::Response, io::Error>>;

    // TODO: decide proper type for `target`
    fn request(&self, target: &str, request: types::Request) -> Self::Future;
}
