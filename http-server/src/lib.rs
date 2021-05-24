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

mod access_control;
mod response;
mod server;

use std::fmt::{self, Debug, Display};

pub use access_control::{AccessControl, AccessControlBuilder, AllowHosts, Host};
pub use jsonrpsee_types::{Error, TEN_MB_SIZE_BYTES};
pub use jsonrpsee_utils::server::rpc_module::{Methods, RpcContextModule, RpcModule};
pub use server::{Builder as HttpServerBuilder, Server as HttpServer};

#[cfg(test)]
mod tests;

#[derive(Debug, thiserror::Error)]
/// Http-specific error type.
pub struct HttpError(Error);

impl Display for HttpError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<hyper::Error> for HttpError {
	fn from(hyper_err: hyper::Error) -> HttpError {
		HttpError(Error::Transport(Box::new(hyper_err)))
	}
}

impl From<std::io::Error> for HttpError {
	fn from(io_err: std::io::Error) -> HttpError {
		HttpError(Error::Transport(Box::new(io_err)))
	}
}

impl From<HttpError> for Error {
	fn from(http_err: HttpError) -> Error {
		http_err.0
	}
}
