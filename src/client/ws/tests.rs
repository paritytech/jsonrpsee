#![cfg(test)]

use crate::client::{WsClient, WsRequestError};
use crate::common::{Error, ErrorCode, JsonValue, Params};

use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, WebSocketTestServer};

fn assert_error_response(
    response: Result<JsonValue, WsRequestError>,
    code: ErrorCode,
    message: String,
) {
    match response {
        Err(WsRequestError::Request(err)) => {
            assert_eq!(
                err,
                Error {
                    code,
                    message,
                    data: None,
                }
            );
        }
        e @ _ => panic!("Expected error: \"Method not\", got: {:?}", e),
    };
}

#[tokio::test]
async fn method_call_works() {
    let server = WebSocketTestServer::with_hardcoded_response(
        "127.0.0.1:0".parse().unwrap(),
        ok_response("hello".into(), Id::Num(0_u64)),
    )
    .await;
    let uri = to_ws_uri_string(server.local_addr());
    let client = WsClient::new(&uri).await.unwrap();
    let response: Result<JsonValue, WsRequestError> =
        client.request("say_hello", Params::None).await;
    let exp = JsonValue::String("hello".to_string());
    assert!(matches!(response, Ok(exp)));
}

#[tokio::test]
async fn notif_doesnt_hang() {
    // this empty string shouldn't be read because the server shouldn't respond to notifications.
    let server =
        WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), String::new())
            .await;
    let uri = to_ws_uri_string(server.local_addr());
    let client = WsClient::new(&uri).await.unwrap();
    client.notification("notif", Params::None).await;
}

#[tokio::test]
async fn method_not_found_works() {
    let server = WebSocketTestServer::with_hardcoded_response(
        "127.0.0.1:0".parse().unwrap(),
        method_not_found(Id::Num(0_u64)),
    )
    .await;
    let uri = to_ws_uri_string(server.local_addr());
    let client = WsClient::new(&uri).await.unwrap();
    let response: Result<JsonValue, WsRequestError> =
        client.request("say_hello", Params::None).await;
    assert_error_response(response, ErrorCode::MethodNotFound, METHOD_NOT_FOUND.into());
}

#[tokio::test]
async fn parse_error_works() {
    let server = WebSocketTestServer::with_hardcoded_response(
        "127.0.0.1:0".parse().unwrap(),
        parse_error(Id::Num(0_u64)),
    )
    .await;
    let uri = to_ws_uri_string(server.local_addr());
    let client = WsClient::new(&uri).await.unwrap();
    let response: Result<JsonValue, WsRequestError> =
        client.request("say_hello", Params::None).await;
    assert_error_response(response, ErrorCode::ParseError, PARSE_ERROR.into());
}

#[tokio::test]
async fn invalid_request_works() {
    let server = WebSocketTestServer::with_hardcoded_response(
        "127.0.0.1:0".parse().unwrap(),
        invalid_request(Id::Num(0_u64)),
    )
    .await;
    let uri = to_ws_uri_string(server.local_addr());
    let client = WsClient::new(&uri).await.unwrap();
    let response: Result<JsonValue, WsRequestError> =
        client.request("say_hello", Params::None).await;
    assert_error_response(response, ErrorCode::InvalidRequest, INVALID_REQUEST.into());
}

#[tokio::test]
async fn invalid_params_works() {
    let server = WebSocketTestServer::with_hardcoded_response(
        "127.0.0.1:0".parse().unwrap(),
        invalid_params(Id::Num(0_u64)),
    )
    .await;
    let uri = to_ws_uri_string(server.local_addr());
    let client = WsClient::new(&uri).await.unwrap();
    let response: Result<JsonValue, WsRequestError> =
        client.request("say_hello", Params::None).await;
    assert_error_response(response, ErrorCode::InvalidParams, INVALID_PARAMS.into());
}

#[tokio::test]
async fn internal_error_works() {
    let server = WebSocketTestServer::with_hardcoded_response(
        "127.0.0.1:0".parse().unwrap(),
        internal_error(Id::Num(0_u64)),
    )
    .await;
    let uri = to_ws_uri_string(server.local_addr());
    let client = WsClient::new(&uri).await.unwrap();
    let response: Result<JsonValue, WsRequestError> =
        client.request("say_hello", Params::None).await;
    assert_error_response(response, ErrorCode::InternalError, INTERNAL_ERROR.into());
}

#[tokio::test]
async fn close_server_on_pending_request() {
    let mut server = WebSocketTestServer::with_hardcoded_response(
        "127.0.0.1:0".parse().unwrap(),
        r#"{}"#.into(),
    )
    .await;
    let uri = to_ws_uri_string(server.local_addr());
    let client = WsClient::new(&uri).await.unwrap();
    server.close().await;
    let response: Result<JsonValue, WsRequestError> =
        client.request("say_hello", Params::None).await;
    assert!(matches!(response, Err(WsRequestError::TransportError(_))));
}
