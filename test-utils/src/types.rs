use futures::channel::mpsc::{self, Receiver, Sender};
use futures::future::FutureExt;
use futures::io::{BufReader, BufWriter};
use futures::sink::SinkExt;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use soketto::handshake;
use soketto::handshake::{server::Response, Server};
use std::net::SocketAddr;
use std::time::Duration;
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
		let mut client = handshake::Client::new(BufReader::new(BufWriter::new(socket.compat())), "test-client", "/");
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

#[derive(Debug, Clone)]
pub enum ServerMode {
    // Send out a hardcoded response on every connection.
    Response(String),
    // Send out a subscription ID and continuously send out data on the subscription.
    Subscription((String, String)),
}

/// JSONRPC v2 dummy WebSocket server that sends a hardcoded response.
pub struct WebSocketTestServer {
    local_addr: SocketAddr,
    exit: Sender<()>,
}

impl WebSocketTestServer {
    // Spawns a dummy `JSONRPC v2` WebSocket server that sends out a pre-configured `hardcoded response` for every connection.
    pub async fn with_hardcoded_response(sockaddr: SocketAddr, response: String) -> Self {
        let listener = async_std::net::TcpListener::bind(sockaddr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel::<()>(4);
        tokio::spawn(server_backend(listener, rx, ServerMode::Response(response)));

        Self {
            local_addr,
            exit: tx,
        }
    }

    // Spawns a dummy `JSONRPC v2` WebSocket server that sends out a pre-configured subscription ID and subscription response.
    //
    // NOTE: ignores the actual subscription and unsubscription method.
    pub async fn with_hardcoded_subscription(
        sockaddr: SocketAddr,
        subscription_id_response: String,
        subscription_response: String,
    ) -> Self {
        let listener = async_std::net::TcpListener::bind(sockaddr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel::<()>(4);
        tokio::spawn(server_backend(
            listener,
            rx,
            ServerMode::Subscription((subscription_id_response, subscription_response)),
        ));

        Self {
            local_addr,
            exit: tx,
        }
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    pub async fn close(&mut self) {
        self.exit.send(()).await.unwrap();
    }
}

async fn server_backend(
    listener: async_std::net::TcpListener,
    mut exit: Receiver<()>,
    mode: ServerMode,
) {
    let mut connections = Vec::new();

    loop {
        let conn_fut = listener.accept().fuse();
        let exit_fut = exit.next();
        futures::pin_mut!(exit_fut, conn_fut);

        futures::select! {
            exit = exit_fut => break,
            conn = conn_fut => {
                if let Ok((stream, _)) = conn {
                    let (tx, rx) = mpsc::channel::<()>(4);
                    let handle = tokio::spawn(connection_task(stream, mode.clone(), rx));
                    connections.push((handle, tx));
                }
            }
        }
    }

    // close connections
    for (handle, mut exit) in connections {
        // If the actual connection was never established i.e., returned early
        // It will most likely be caught on the client-side but just to explicit.
        exit.send(()).await.expect("WebSocket connection was never established");
        handle.await.unwrap();
    }
}

async fn connection_task(
    socket: async_std::net::TcpStream,
    mode: ServerMode,
    mut exit: Receiver<()>,
) {
    let mut server = Server::new(socket);

    let websocket_key = match server.receive_request().await {
        Ok(req) => req.into_key(),
        Err(_) => return,
    };

    let accept = server
        .send_response(&Response::Accept {
            key: &websocket_key,
            protocol: None,
        })
        .await;

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
        let next_ws = ws_stream.next().fuse();
        let next_exit = exit.next().fuse();
        let time_out = tokio::time::delay_for(Duration::from_secs(1)).fuse();
        futures::pin_mut!(time_out, next_exit, next_ws);

        futures::select! {
            subscription = time_out => {
                if let ServerMode::Subscription((_, r)) = &mode {
                    if let Err(e) = sender.send_text(&r).await {
                        log::warn!("send response to subscription: {:?}", e);
                    }
                }
            }
            ws = next_ws => {
                // Got a request on the connection but don't care about the contents.
                // Just send out the pre-configured hardcoded responses.
                if let Some(Ok(_)) = ws {
                    match &mode {
                        ServerMode::Response(r) => {
                            if let Err(e) = sender.send_text(&r).await {
                                log::warn!("send response to request error: {:?}", e);
                            }
                        }
                        ServerMode::Subscription((sub_id, sub_response)) => {
                            if let Err(e) = sender.send_text(&sub_id).await {
                                log::warn!("send subscription id error: {:?}", e);
                            }
                        }
                    }
                }
            }
            exit = next_exit => break,
        }
    }
}
