use crate::common;
use super::RawServer;
use futures::prelude::*;
use std::{io, pin::Pin};

/// Joins two servers into one.
///
/// The combination of the two will produce a request whenever one of them produces a request.
//  TODO: example
pub fn join<A, B>(left: A, right: B) -> Join<A, B> {
    Join { left, right }
}

/// Joins two servers into one.
#[derive(Debug)]
pub struct Join<A, B> {
    left: A,
    right: B,
}

/// Request ID corresponding to the [`Join`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinRequestId<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> RawServer for Join<A, B>
where
    A: RawServer,
    B: RawServer,
{
    type RequestId = JoinRequestId<A::RequestId, B::RequestId>;

    fn next_request<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(Self::RequestId, common::Request), ()>> + Send + 'a>> {
        unimplemented!()
    }

    fn finish<'a>(
        &'a mut self,
        request_id: &'a Self::RequestId,
        response: Option<&'a common::Response>,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        match request_id {
            JoinRequestId::Left(request_id) => self.left.finish(request_id, response),
            JoinRequestId::Right(request_id) => self.right.finish(request_id, response),
        }
    }

    fn supports_resuming(&self, request_id: &Self::RequestId) -> bool {
        match request_id {
            JoinRequestId::Left(request_id) => self.left.supports_resuming(request_id),
            JoinRequestId::Right(request_id) => self.right.supports_resuming(request_id),
        }
    }

    fn send<'a>(
        &'a mut self,
        request_id: &'a Self::RequestId,
        response: &'a common::Response,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
        match request_id {
            JoinRequestId::Left(request_id) => self.left.send(request_id, response),
            JoinRequestId::Right(request_id) => self.right.send(request_id, response),
        }
    }
}
