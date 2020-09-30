#![cfg(test)]

use crate::common::{self, Id, JsonValue};
use crate::http::{HttpServer, RegisteredMethod};
use futures::channel::oneshot::{self, Sender};
use futures::future::{Fuse, FusedFuture, FutureExt};
use futures::{pin_mut, select};
use hyper::{Body, HeaderMap, StatusCode, Uri};

#[derive(Debug)]
struct Response {
    status: StatusCode,
    header: HeaderMap,
    body: String,
}

async fn request(body: Body, uri: Uri) -> Result<Response, String> {
    let client = hyper::Client::new();
    let r = hyper::Request::post(uri)
        .header(
            hyper::header::CONTENT_TYPE,
            hyper::header::HeaderValue::from_static("application/json"),
        )
        .body(body)
        .expect("uri and request headers are valid; qed");
    let res = client.request(r).await.map_err(|e| format!("{:?}", e))?;

    let (parts, body) = res.into_parts();
    let bytes = hyper::body::to_bytes(body).await.unwrap();

    Ok(Response {
        status: parts.status,
        header: parts.headers,
        body: String::from_utf8(bytes.to_vec()).unwrap(),
    })
}

async fn server(sockaddr: &str, server_started_tx: Sender<()>) {
    let mut server = HttpServer::new(sockaddr).await.unwrap();
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
    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64001", server_started_tx));
    server_started_rx.await.unwrap();

    for i in 0..10 {
        let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
        let response = request(req.into(), Uri::from_static("http://127.0.0.1:64001"))
            .await
            .unwrap();
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(
            response.body,
            ok_response(JsonValue::String("hello".to_owned()), Id::Num(i))
        );
    }
}

#[tokio::test]
async fn single_method_call_with_params() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64011", server_started_tx));
    server_started_rx.await.unwrap();

    let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
    let response = request(req.into(), Uri::from_static("http://127.0.0.1:64011"))
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(
        response.body,
        ok_response(JsonValue::Number(3.into()), Id::Num(1))
    );
}

#[tokio::test]
async fn should_return_method_not_found() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64002", server_started_tx));
    server_started_rx.await.unwrap();

    let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
    let response = request(req.into(), Uri::from_static("http://127.0.0.1:64002"))
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64003", server_started_tx));
    server_started_rx.await.unwrap();

    let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
    let response = request(req.into(), Uri::from_static("http://127.0.0.1:64003"))
        .await
        .unwrap();
    // If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
    assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
    let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
    tokio::spawn(server("127.0.0.1:64004", server_started_tx));
    server_started_rx.await.unwrap();

    let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
    let response = request(req.into(), Uri::from_static("http://127.0.0.1:64004"))
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body, invalid_request(Id::Num(1)));
}
