#![cfg(test)]

use crate::common::{Id, JsonValue};
use crate::ws::WsServer;
use futures::channel::oneshot::{self, Sender};
use futures::future::FutureExt;
use futures::io::{BufReader, BufWriter};
use futures::{pin_mut, select};
use soketto::handshake;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, Tokio02AsyncReadCompatExt};

type Error = Box<dyn std::error::Error>;

struct WebSocketTestClient {
    tx: soketto::Sender<BufReader<BufWriter<Compat<TcpStream>>>>,
    rx: soketto::Receiver<BufReader<BufWriter<Compat<TcpStream>>>>,
}

impl WebSocketTestClient {
    async fn new(url: SocketAddr) -> Result<Self, Error> {
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

    async fn send_request_text(&mut self, msg: impl AsRef<str>) -> Result<String, Error> {
        self.tx.send_text(msg).await?;
        self.tx.flush().await?;
        let mut data = Vec::new();
        self.rx.receive_data(&mut data).await?;
        String::from_utf8(data).map_err(Into::into)
    }

    async fn send_request_binary(&mut self, msg: &[u8]) -> Result<String, Error> {
        self.tx.send_binary(msg).await?;
        self.tx.flush().await?;
        let mut data = Vec::new();
        self.rx.receive_data(&mut data).await?;
        String::from_utf8(data).map_err(Into::into)
    }

    async fn close(&mut self) -> Result<(), Error> {
        self.tx.close().await.map_err(Into::into)
    }
}

/// Spawn a WebSocket server where the OS assigns the port number and
/// sends back the `local_addr` via a channel.
async fn server(server_started_tx: Sender<SocketAddr>) {
    let server = WsServer::new("127.0.0.1:0").await.unwrap();
    let mut hello = server.register_method("say_hello".to_owned()).unwrap();
    let mut add = server.register_method("add".to_owned()).unwrap();
    let mut notif = server
        .register_notification("notif".to_owned(), false)
        .unwrap();
    server_started_tx.send(*server.local_addr()).unwrap();

    loop {
        let hello_fut = async {
            let handle = hello.next().await;
            log::debug!("server respond to hello");
            handle
                .respond(Ok(JsonValue::String("hello".to_owned())))
                .await;
        }
        .fuse();

        let add_fut = async {
            let handle = add.next().await;
            let params: Vec<u64> = handle.params().clone().parse().unwrap();
            let sum: u64 = params.iter().sum();
            handle.respond(Ok(JsonValue::Number(sum.into()))).await;
        }
        .fuse();

        let notif_fut = async {
            let params = notif.next().await;
            println!("received notification: say_hello params[{:?}]", params);
        }
        .fuse();

        pin_mut!(hello_fut, add_fut, notif_fut);
        select! {
            say_hello = hello_fut => (),
            add = add_fut => (),
            notif = notif_fut => (),
            complete => (),
        };
    }
}

fn ok_response(result: JsonValue, id: Id) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","result":{},"id":{}}}"#,
        result,
        serde_json::to_string(&id).unwrap()
    )
}

fn method_not_found(id: Id) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","error":{{"code":-32601,"message":"Method not found"}},"id":{}}}"#,
        serde_json::to_string(&id).unwrap()
    )
}

fn parse_error(id: Id) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","error":{{"code":-32700,"message":"Parse error"}},"id":{}}}"#,
        serde_json::to_string(&id).unwrap()
    )
}

fn invalid_request(id: Id) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","error":{{"code":-32600,"message":"Invalid request"}},"id":{}}}"#,
        serde_json::to_string(&id).unwrap()
    )
}

fn invalid_params(id: Id) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","error":{{"code":-32602,"message":"Invalid params"}},"id":{}}}"#,
        serde_json::to_string(&id).unwrap()
    )
}

#[tokio::test]
async fn single_method_call_works() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
    tokio::spawn(server(server_started_tx));
    let server_addr = server_started_rx.await.unwrap();
    let mut client = WebSocketTestClient::new(server_addr).await.unwrap();

    for i in 0..10 {
        let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
        let response = client.send_request_text(req).await.unwrap();
        assert_eq!(
            response,
            ok_response(JsonValue::String("hello".to_owned()), Id::Num(i))
        );
    }
}

#[tokio::test]
async fn single_method_call_with_params() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
    tokio::spawn(server(server_started_tx));
    let server_addr = server_started_rx.await.unwrap();
    let mut client = WebSocketTestClient::new(server_addr).await.unwrap();

    let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
    let response = client.send_request_text(req).await.unwrap();
    assert_eq!(
        response,
        ok_response(JsonValue::Number(3.into()), Id::Num(1))
    );
}

#[tokio::test]
async fn single_method_send_binary() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
    tokio::spawn(server(server_started_tx));
    let server_addr = server_started_rx.await.unwrap();
    let mut client = WebSocketTestClient::new(server_addr).await.unwrap();

    let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
    let response = client.send_request_binary(req.as_bytes()).await.unwrap();
    assert_eq!(
        response,
        ok_response(JsonValue::Number(3.into()), Id::Num(1))
    );
}

#[tokio::test]
async fn should_return_method_not_found() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
    tokio::spawn(server(server_started_tx));
    let server_addr = server_started_rx.await.unwrap();
    let mut client = WebSocketTestClient::new(server_addr).await.unwrap();

    let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
    let response = client.send_request_text(req).await.unwrap();
    assert_eq!(response, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
    tokio::spawn(server(server_started_tx));
    let server_addr = server_started_rx.await.unwrap();

    let mut client = WebSocketTestClient::new(server_addr).await.unwrap();
    let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
    let response = client.send_request_text(req).await.unwrap();
    // If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
    assert_eq!(response, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
    tokio::spawn(server(server_started_tx));
    let server_addr = server_started_rx.await.unwrap();

    let mut client = WebSocketTestClient::new(server_addr).await.unwrap();
    let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
    let response = client.send_request_text(req).await.unwrap();
    assert_eq!(response, invalid_request(Id::Num(1)));
}

#[tokio::test]
async fn register_methods_works() {
    let server = WsServer::new("127.0.0.1:0").await.unwrap();
    assert!(server.register_method("say_hello".to_owned()).is_ok());
    assert!(server.register_method("say_hello".to_owned()).is_err());
    assert!(server
        .register_notification("notif".to_owned(), false)
        .is_ok());
    assert!(server
        .register_notification("notif".to_owned(), false)
        .is_err());
    assert!(server
        .register_subscription("subscribe_hello".to_owned(), "unsubscribe_hello".to_owned())
        .is_ok());
    assert!(server
        .register_subscription("subscribe_hello_again".to_owned(), "notif".to_owned())
        .is_err());
    assert!(
        server
            .register_method("subscribe_hello_again".to_owned())
            .is_ok(),
        "Failed register_subscription should not have side-effects"
    );
}
