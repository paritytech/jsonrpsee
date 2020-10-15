use futures::io::{BufReader, BufWriter};
use serde::{Deserialize, Serialize};
use soketto::handshake;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, Tokio02AsyncReadCompatExt};

pub use hyper::{Body, HeaderMap, StatusCode, Uri};

type Error = Box<dyn std::error::Error>;

/// Request Id
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Id {
    /// No id (notification)
    Null,
    /// Numeric id
    Num(u64),
    /// String id
    Str(String),
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub header: HeaderMap,
    pub body: String,
}

/// WebSocket client to construct with arbitrary payload to construct bad payloads.
pub struct WebSocketTestClient {
    tx: soketto::Sender<BufReader<BufWriter<Compat<TcpStream>>>>,
    rx: soketto::Receiver<BufReader<BufWriter<Compat<TcpStream>>>>,
}

impl WebSocketTestClient {
    pub async fn new(url: SocketAddr) -> Result<Self, Error> {
        let socket = TcpStream::connect(url).await?;
        let mut client = handshake::Client::new(
            BufReader::new(BufWriter::new(socket.compat())),
            "test-client",
            "/",
        );
        match client.handshake().await {
            Ok(handshake::ServerResponse::Accepted { .. }) => {
                let (tx, rx) = client.into_builder().finish();
                Ok(Self { tx, rx })
            }
            r @ _ => Err(format!("WebSocketHandshake failed: {:?}", r).into()),
        }
    }

    pub async fn send_request_text(&mut self, msg: impl AsRef<str>) -> Result<String, Error> {
        self.tx.send_text(msg).await?;
        self.tx.flush().await?;
        let mut data = Vec::new();
        self.rx.receive_data(&mut data).await?;
        String::from_utf8(data).map_err(Into::into)
    }

    pub async fn send_request_binary(&mut self, msg: &[u8]) -> Result<String, Error> {
        self.tx.send_binary(msg).await?;
        self.tx.flush().await?;
        let mut data = Vec::new();
        self.rx.receive_data(&mut data).await?;
        String::from_utf8(data).map_err(Into::into)
    }

    pub async fn close(&mut self) -> Result<(), Error> {
        self.tx.close().await.map_err(Into::into)
    }
}
