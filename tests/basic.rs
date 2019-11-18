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

    let v: String = async_std::task::block_on(async {
        let id = client.start_request("concat", params).await.unwrap();
        jsonrpsee::core::common::from_value(client.request_by_id(id).unwrap().await.unwrap())
    })
    .unwrap();
    assert_eq!(v, "hello, 5");
}
