use crate::types;
use futures::prelude::*;
use std::io;

pub use self::http::HttpClientPool;

pub mod http;

pub trait RawClientRef<'a> {
    type Future: Future<Output = Result<types::Response, io::Error>> + 'a;

    // TODO: decide proper type for `target`
    fn request(self, target: &str, request: types::Request) -> Self::Future;
}

pub trait RawClientRefPubSub<'a> {
    type Subscription: Stream<Item = types::Response> + 'a;
    type Future: Future<Output = Result<(types::Response, Self::Subscription), io::Error>> + 'a;

    // TODO: decide proper type for `target`
    fn request_subscribe(self, target: &str, request: types::Request) -> Self::Future;
}
