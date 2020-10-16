use futures::channel::mpsc::{self, Receiver, Sender};
use futures::future;
use futures::io::{BufReader, BufWriter};
use futures::stream::{self, StreamExt};
use futures::sink::SinkExt;
use serde::{Deserialize, Serialize};
use soketto::handshake;
use soketto::handshake::{server::Response, Server};
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

/// JSONRPC v2 dummy WebSocket server that sends a hardcoded response.
pub struct WebSocketTestServer {
    local_addr: SocketAddr,
    exit: Sender<()>,
}

impl WebSocketTestServer {
    pub async fn with_hardcoded_response(sockaddr: SocketAddr, answer: String) -> Self {
        let listener = async_std::net::TcpListener::bind(sockaddr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel::<()>(4);
        tokio::spawn(server_backend(listener, rx, answer));

        Self { local_addr, exit: tx }
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    pub async fn close(&mut self) {
        self.exit.send(()).await.unwrap();
    }
}

async fn server_backend(listener: async_std::net::TcpListener, mut exit: Receiver<()>, answer: String) {
    let mut connections = Vec::new();

    loop {
        let next_conn = listener.accept();
        let next_exit = exit.next();
        futures::pin_mut!(next_exit, next_conn);

        match future::select(next_exit, next_conn).await {
            future::Either::Left(_) => break,
            future::Either::Right((Ok((stream, _)), _)) => {
                let (tx, rx) = mpsc::channel::<()>(4);
                let handle = tokio::spawn(connection_task(stream, answer.clone(), rx));
                connections.push((handle, tx));
            }
            future::Either::Right((Err(_), _)) => {}
        };
    }

    // close connections
    for (handle, mut exit) in connections {
        exit.send(()).await.unwrap();
        handle.await.unwrap();
    }
}

async fn connection_task(socket: async_std::net::TcpStream, answer: String, mut exit: Receiver<()>) {
    let mut server = Server::new(socket);

    let websocket_key = match server.receive_request().await {
        Ok(req) => req.into_key(),
        Err(_) => return,
    };

    let accept = server.send_response(&Response::Accept {
         key: &websocket_key,
         protocol: None,
    }).await;

    if accept.is_err() {
        return;
    }

    let (mut sender, receiver) = server.into_builder().finish();

    let ws_stream = stream::unfold(receiver, move |mut receiver| async {
        let mut buf = Vec::new();
        let ret = match receiver.receive_data(&mut buf).await {
            Ok(_) => Ok(buf),
            Err(err) => Err(err),
        };
        Some((ret, receiver))
    });
    futures::pin_mut!(ws_stream);

    loop {
        let next_ws = ws_stream.next();
        let next_exit = exit.next();
        futures::pin_mut!(next_exit, next_ws);

        match future::select(next_exit, next_ws).await {
            future::Either::Left(_) => break,
            // don't care about the received data
            future::Either::Right((Some(Ok(_)), _)) => {
                let _ = sender.send_text(&answer).await.unwrap();
            }
            future::Either::Right((e, _)) => {
                log::warn!("server error: {:?}", e);
            }
        }
    }
}
