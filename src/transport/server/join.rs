// Copyright 2019 Parity Technologies (UK) Ltd.
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

use super::{TransportServer, TransportServerEvent};
use crate::common;

use alloc::boxed::Box;
use core::pin::Pin;
use futures::prelude::*;

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

impl<A, B> TransportServer for Join<A, B>
where
    A: TransportServer + Send,
    B: TransportServer + Send,
{
    type RequestId = JoinRequestId<A::RequestId, B::RequestId>;

    fn next_request<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = TransportServerEvent<Self::RequestId>> + Send + 'a>> {
        Box::pin(async move {
            match future::select(self.left.next_request(), self.right.next_request()).await {
                future::Either::Left((TransportServerEvent::Request { id, request }, _)) => {
                    TransportServerEvent::Request {
                        id: JoinRequestId::Left(id),
                        request,
                    }
                }
                future::Either::Left((TransportServerEvent::Closed(id), _)) => {
                    TransportServerEvent::Closed(JoinRequestId::Left(id))
                }
                future::Either::Right((TransportServerEvent::Request { id, request }, _)) => {
                    TransportServerEvent::Request {
                        id: JoinRequestId::Right(id),
                        request,
                    }
                }
                future::Either::Right((TransportServerEvent::Closed(id), _)) => {
                    TransportServerEvent::Closed(JoinRequestId::Right(id))
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
