jsonrpsee::rpc_api! {
    Test {
        fn allowed(foo: bool) -> bool;
    }
}

macro_rules! spawn_server {
    ($server:expr) => {
        async_std::task::spawn(async move {
            while let Ok(request) = Test::next_request(&mut $server).await {
                match request {
                    Test::Allowed { respond, foo } => {
                        respond.ok(foo).await;
                    }
                }
            }
        });
    };
}

extern crate jsonrpsee;
use jsonrpsee::raw::{RawClient, RawClientError, RawServer};
use jsonrpsee::transport::http::{
    self, access_control::Host, RequestError, HttpTransportClient,
};
use std::net::SocketAddr;

fn spawn_client(res: bool, port: u16) -> (RawClient<HttpTransportClient>, jsonrpsee::common::Params) {
    let transport = jsonrpsee::transport::http::HttpTransportClient::new(&format!("http://localhost:{}", port));
    let client = jsonrpsee::raw::RawClient::new(transport);
    let params = {
        let mut map = jsonrpsee::common::JsonMap::new();
        map.insert("foo".to_owned(), res.into());
        jsonrpsee::common::Params::Map(map)
    };
    (client, params)
}

#[test]
fn host_allow_any() {
    async_std::task::block_on(async {
        let ip: SocketAddr = "0.0.0.0:8080".parse().unwrap();
        let acl = http::access_control::AccessControlBuilder::new().build();
        let transport_server = jsonrpsee::transport::http::HttpTransportServer::bind_with_acl(&ip, acl).await.unwrap();
        let mut server = RawServer::new(transport_server);
        spawn_server!(server);
        let res = true;
        let (mut client, params) = spawn_client(res, 8080);
        let id = client.start_request("allowed", params).await.unwrap();
        let v: bool =
            jsonrpsee::common::from_value(client.request_by_id(id).unwrap().await.unwrap())
                .unwrap();
        assert_eq!(v, res);
    });
}

#[test]
fn host_allow_by_being_white_listed() {
    async_std::task::block_on(async {
        let ip: SocketAddr = "0.0.0.0:8081".parse().unwrap();
        let acl = http::access_control::AccessControlBuilder::new()
            .allow_host(Host::parse("localhost:*"))
            .build();
        let transport_server = jsonrpsee::transport::http::HttpTransportServer::bind_with_acl(&ip, acl).await.unwrap();
        let mut server = RawServer::new(transport_server);
        spawn_server!(server);
        let res = true;
        let (mut client, params) = spawn_client(res, 8081);
        let id = client.start_request("allowed", params).await.unwrap();
        let v: bool =
            jsonrpsee::common::from_value(client.request_by_id(id).unwrap().await.unwrap())
                .unwrap();
        assert_eq!(v, res);
    });
}

#[test]
fn host_deny_by_not_being_white_listed() {
    async_std::task::block_on(async {
        let ip: SocketAddr = "0.0.0.0:8082".parse().unwrap();
        let acl = http::access_control::AccessControlBuilder::new()
            .allow_host(Host::parse("1.2.3.4"))
            .build();
        let transport_server = jsonrpsee::transport::http::HttpTransportServer::bind_with_acl(&ip, acl).await.unwrap();
        let mut server = RawServer::new(transport_server);
        spawn_server!(server);
        let (mut client, params) = spawn_client(false, 8082);
        let id = client.start_request("allowed", params).await.unwrap();
        let denied = match client.request_by_id(id).unwrap().await {
            Ok(_) => false,
            Err(err) => match err {
                RawClientError::Inner(failure) => match failure {
                    RequestError::RequestFailure { status_code } => status_code == 403,
                    _ => false,
                },
                _ => false,
            },
        };
        assert_eq!(denied, true);
    });
}
