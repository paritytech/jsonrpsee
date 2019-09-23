jsonrpsee::rpc_api! {
    Test {
        fn concat(foo: String, bar: i32) -> String;
    }
}

macro_rules! spawn_server {
    ($server:expr) => {
        async_std::task::spawn(async move {
            while let Ok(request) = Test::next_request(&mut $server).await {
                match request {
                    Test::Concat { respond, foo, bar } => {
                        let value = format!("{}, {}", foo, bar);
                        respond.ok(value).await;
                    }
                }
            }
        });
    };
}

#[test]
fn client_server_works() {
    let (mut client, mut server) = jsonrpsee::local();
    spawn_server!(server);

    let v = async_std::task::block_on(Test::concat(&mut client, "hello", 5)).unwrap();
    assert_eq!(v, "hello, 5");
}

#[test]
fn server_works_the_expected_way() {
    let (mut client, mut server) = jsonrpsee::local();
    spawn_server!(server);

    let params = {
        let mut map = jsonrpsee::core::common::JsonMap::new();
        map.insert("foo".to_owned(), "hello".into());
        map.insert("bar".to_owned(), 5i32.into());
        jsonrpsee::core::common::Params::Map(map)
    };

    let v: String = async_std::task::block_on(client.request("concat", params)).unwrap();
    assert_eq!(v, "hello, 5");
}
