use futures::{
    io::{IoSlice, IoSliceMut},
    prelude::*,
};
use pin_project::{pin_project, project};
use std::{fmt, io::Error as IoError, pin::Pin, task::Context, task::Poll};

#[pin_project]
#[derive(Debug, Copy, Clone)]
pub enum EitherStream<S, T> {
    /// Unencrypted socket stream.
    Plain(#[pin] S),
    /// Encrypted socket stream.
    Tls(#[pin] T),
}

impl<S, T> AsyncRead for EitherStream<S, T>
where
    S: AsyncRead,
    T: AsyncRead,
{
    #[project]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, IoError>> {
        #[project]
        match self.project() {
            EitherStream::Plain(s) => AsyncRead::poll_read(s, cx, buf),
            EitherStream::Tls(t) => AsyncRead::poll_read(t, cx, buf),
        }
    }

    #[project]
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &mut [IoSliceMut],
    ) -> Poll<Result<usize, IoError>> {
        #[project]
        match self.project() {
            EitherStream::Plain(s) => AsyncRead::poll_read_vectored(s, cx, bufs),
            EitherStream::Tls(t) => AsyncRead::poll_read_vectored(t, cx, bufs),
        }
    }
}

impl<S, T> AsyncWrite for EitherStream<S, T>
where
    S: AsyncWrite,
    T: AsyncWrite,
{
    #[project]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, IoError>> {
        #[project]
        match self.project() {
            EitherStream::Plain(s) => AsyncWrite::poll_write(s, cx, buf),
            EitherStream::Tls(t) => AsyncWrite::poll_write(t, cx, buf),
        }
    }

    #[project]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context,
        bufs: &[IoSlice],
    ) -> Poll<Result<usize, IoError>> {
        #[project]
        match self.project() {
            EitherStream::Plain(s) => AsyncWrite::poll_write_vectored(s, cx, bufs),
            EitherStream::Tls(t) => AsyncWrite::poll_write_vectored(t, cx, bufs),
        }
    }

    #[project]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), IoError>> {
        #[project]
        match self.project() {
            EitherStream::Plain(s) => AsyncWrite::poll_flush(s, cx),
            EitherStream::Tls(t) => AsyncWrite::poll_flush(t, cx),
        }
    }

    #[project]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), IoError>> {
        #[project]
        match self.project() {
            EitherStream::Plain(s) => AsyncWrite::poll_close(s, cx),
            EitherStream::Tls(t) => AsyncWrite::poll_close(t, cx),
        }
    }
}
