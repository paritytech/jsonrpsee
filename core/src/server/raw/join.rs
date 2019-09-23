use super::{RawServer, RawServerEvent};
use crate::common;
use futures::prelude::*;
use std::pin::Pin;

/// Joins two servers into one.
///
/// The combination of the two will produce a request whenever one of them produces a request.
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JoinRequestId<A, B> {
    /// The request belongs to the first server.
    Left(A),
    /// The request belongs to the second server.
    Right(B),
}

impl<A, B> RawServer for Join<A, B>
where
    A: RawServer + Send,
    B: RawServer + Send,
{
    type RequestId = JoinRequestId<A::RequestId, B::RequestId>;

    fn next_request<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = RawServerEvent<Self::RequestId>> + Send + 'a>>
    {
        Box::pin(async move {
            match future::select(self.left.next_request(), self.right.next_request()).await {
                future::Either::Left((RawServerEvent::Request { id, request }, _)) => {
                    RawServerEvent::Request { id: JoinRequestId::Left(id), request }
                }
                future::Either::Left((RawServerEvent::Closed(id), _)) => {
                    RawServerEvent::Closed(JoinRequestId::Left(id))
                }
                future::Either::Left((RawServerEvent::ServerClosed, _)) => {
                    RawServerEvent::ServerClosed
                }
                future::Either::Right((RawServerEvent::Request { id, request }, _)) => {
                    RawServerEvent::Request { id: JoinRequestId::Right(id), request }
                }
                future::Either::Right((RawServerEvent::Closed(id), _)) => {
                    RawServerEvent::Closed(JoinRequestId::Right(id))
                }
                future::Either::Right((RawServerEvent::ServerClosed, _)) => {
                    RawServerEvent::ServerClosed
                }
            }
        })
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

    fn supports_resuming(&self, request_id: &Self::RequestId) -> Result<bool, ()> {
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
