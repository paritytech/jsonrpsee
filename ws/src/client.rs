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

use async_std::net::{ToSocketAddrs, TcpStream};
use err_derive::*;
use futures::prelude::*;
use jsonrpsee_core::{client::Client, client::RawClient, common};
use soketto::connection::Connection;
use soketto::handshake::client::{ServerResponse, Client as WsClient};
use std::{fmt, io, pin::Pin};

/// Implementation of a raw client for WebSockets requests.
pub struct WsRawClient {
    /// TCP/IP connection wrapped around a WebSocket encoder/decoder.
    inner: Connection<TcpStream>,
}

impl WsRawClient {
    /// Initializes a new HTTP client.
    // TODO: better type for target
    pub async fn new(target: impl ToSocketAddrs) -> Result<Client<Self>, WsNewError> {
        Ok(Client::new(Self::new_raw(target).await?))
    }

    /// Initializes a new HTTP client.
    // TODO: better type for target
    pub async fn new_raw(target: impl ToSocketAddrs) -> Result<Self, WsNewError> {
        let tcp_stream = TcpStream::connect(target).await?;
        let mut client = WsClient::new(tcp_stream, "127.0.0.1:9944" /* TODO: */, "/");
        client.set_origin("https://polkadot.js.org");     // TODO: ??
        match client.handshake().await? {
            ServerResponse::Accepted { .. } => {},
            ServerResponse::Rejected { status_code } |
            ServerResponse::Redirect { status_code, .. } => {
                // TODO: redirects also lead here
                return Err(WsNewError::Rejected { status_code });
            },
        }

        let mut connection = client.into_connection();
        connection.validate_utf8(true);
        Ok(WsRawClient {
            inner: connection,
        })
    }
}

impl RawClient for WsRawClient {
    type Error = WsConnecError;

    fn send_request<'a>(
        &'a mut self,
        request: common::Request,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        Box::pin(async move {
            let request = common::to_vec(&request)
                .map_err(WsConnecError::Serialization)?;
            self.inner.send_text(&mut From::from(request)).await?;
            self.inner.flush().await?;
            Ok(())
        })
    }

    fn next_response<'a>(&'a mut self)
        -> Pin<Box<dyn Future<Output = Result<common::Response, Self::Error>> + Send + 'a>>
    {
        Box::pin(async move {
            let (data, _is_text) = self.inner.receive().await?;
            let response = common::from_slice(&data)
                .map_err(WsConnecError::ParseError)?;
            Ok(response)
        })
    }
}

impl fmt::Debug for WsRawClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("WsRawClient").finish()
    }
}

/// Error that can happen during the initial handshake.
#[derive(Debug, Error)]
pub enum WsNewError {
    /// Error when opening the TCP socket.
    #[error(display = "Error when opening the TCP socket: {}", 0)]
    Io(io::Error),

    /// Error in the WebSocket handshake.
    #[error(display = "Error in the WebSocket handshake: {}", 0)]
    Handshake(#[error(cause)] soketto::handshake::Error),

    /// Server rejected our handshake.
    #[error(display = "Server returned an error status code: {}", status_code)]
    Rejected {
        /// HTTP status code that the server returned.
        status_code: u16,
    }
}

impl From<io::Error> for WsNewError {
    fn from(err: io::Error) -> WsNewError {
        WsNewError::Io(err)
    }
}

impl From<soketto::handshake::Error> for WsNewError {
    fn from(err: soketto::handshake::Error) -> WsNewError {
        WsNewError::Handshake(err)
    }
}

/// Error that can happen during a request.
#[derive(Debug, Error)]
pub enum WsConnecError {
    /// Error while serializing the request.
    // TODO: can that happen?
    #[error(display = "error while serializing the request")]
    Serialization(#[error(cause)] serde_json::error::Error),

    /// Error in the WebSocket connection.
    #[error(display = "error in the WebSocket connection")]
    Ws(#[error(cause)] soketto::connection::Error),

    /// Failed to parse the JSON returned by the server into a JSON-RPC response.
    #[error(display = "error while parsing the response body")]
    ParseError(#[error(cause)] serde_json::error::Error),
}

impl From<soketto::connection::Error> for WsConnecError {
    fn from(err: soketto::connection::Error) -> WsConnecError {
        WsConnecError::Ws(err)
    }
}
