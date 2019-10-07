// Copyright (c) 2019 Parity Technologies Limited
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
    Health {
        /// Test
        fn system_name(foo: String, bar: i32) -> String;

        /// Test2
        #[rpc(method = "foo")]
        fn system_name2() -> String;
    }

    System {
        fn test_foo() -> String;
    }
}

fn main() {
    // Spawning a server in a background task.
    async_std::task::spawn(async move {
        let listen_addr = "127.0.0.1:8000".parse().unwrap();
        let mut server1 = jsonrpsee::http_server(&listen_addr).await.unwrap();

        while let Ok(request) = Health::next_request(&mut server1).await {
            match request {
                Health::SystemName { respond, foo, bar } => {
                    let value = format!("{}, {}", foo, bar);
                    respond.ok(value).await;
                }
                Health::SystemName2 { respond } => {
                    respond.ok("hello 2").await;
                }
            }
        }
    });

    // Client demo.
    let mut client = jsonrpsee::http_client("http://127.0.0.1:8000");
    let v = async_std::task::block_on(Health::system_name(&mut client, "hello", 5)).unwrap();
    println!("{:?}", v);
}
