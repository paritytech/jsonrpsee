#![cfg(test)]

use crate::common::{self, Id, JsonValue, Response};
use crate::ws::{WsServer, RegisteredMethod};
use futures::channel::oneshot::{self, Sender};
use futures::future::{Fuse, FusedFuture, FutureExt};
use futures::io::{BufReader, BufWriter};
use futures::{pin_mut, select};
use soketto::{BoxedError, connection, handshake};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, Tokio02AsyncReadCompatExt};

type Error = Box<dyn std::error::Error>;

struct WebSocketTestClient {
    tx: soketto::Sender<BufReader<BufWriter<Compat<TcpStream>>>>,
    rx: soketto::Receiver<BufReader<BufWriter<Compat<TcpStream>>>>,
}

impl WebSocketTestClient {
    async fn new(url: &str) -> Result<Self, Error> {
        let socket = TcpStream::connect(url).await?;
        let mut client = handshake::Client::new(BufReader::new(BufWriter::new(socket.compat())), url, "foo");
        match client.handshake().await {
            Ok(handshake::ServerResponse::Accepted {..}) => {
                let (tx, rx) = client.into_builder().finish();
                Ok(Self { tx, rx })
            },
            r @ _ => Err(format!("WebSocketHandshake failed: {:?}", r).into())
        }
    }

    async fn send_request(&mut self, msg: impl AsRef<str>) -> Result<String, Error> {
        self.tx.send_text(msg).await?;
        self.tx.flush().await?;
        let mut data = Vec::new();
        self.rx.receive(&mut data).await?;
        String::from_utf8(data).map_err(Into::into)
    }

    async fn close(&mut self) -> Result<(), Error> {
        self.tx.close().await.map_err(Into::into)
    }
}

async fn server(sockaddr: &str, server_started_tx: Sender<()>) {
    let mut server = WsServer::new(sockaddr).await.unwrap();
    let mut hello = server.register_method("say_hello".to_owned()).unwrap();
    let mut add = server.register_method("add".to_owned()).unwrap();
    let mut notif = server
        .register_notification("notif".to_owned(), false)
        .unwrap();
    server_started_tx.send(()).unwrap();

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
    env_logger::init();

    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64201", server_started_tx));
    server_started_rx.await.unwrap();

    let mut client = WebSocketTestClient::new("127.0.0.1:64201").await.unwrap();

    for i in 0..10 {
        let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
        let response = client.send_request(req)
            .await
            .unwrap();
        assert_eq!(
            response,
            ok_response(JsonValue::String("hello".to_owned()), Id::Num(i))
        );
    }
}

#[tokio::test]
async fn single_method_call_with_params() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64202", server_started_tx));
    server_started_rx.await.unwrap();

    let mut client = WebSocketTestClient::new("127.0.0.1:64202").await.unwrap();
    let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
    let response = client.send_request(req).await.unwrap();
    assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn should_return_method_not_found() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64203", server_started_tx));
    server_started_rx.await.unwrap();

    let mut client = WebSocketTestClient::new("127.0.0.1:64203").await.unwrap();
    let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
    let response = client.send_request(req)
        .await
        .unwrap();
    assert_eq!(response, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64204", server_started_tx));
    server_started_rx.await.unwrap();

    let mut client = WebSocketTestClient::new("127.0.0.1:64204").await.unwrap();
    let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
    let response = client.send_request(req)
        .await
        .unwrap();
    // If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
    assert_eq!(response, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64205", server_started_tx));
    server_started_rx.await.unwrap();

    let mut client = WebSocketTestClient::new("127.0.0.1:64205").await.unwrap();
    let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
    let response = client.send_request(req)
        .await
        .unwrap();
    assert_eq!(response, invalid_request(Id::Num(1)));
}
