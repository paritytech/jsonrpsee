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
    System {
        /// Get the node's implementation name. Plain old string.
        //#[rpc(method = "system_name")]        // TODO: https://github.com/paritytech/jsonrpsee/issues/26
        fn system_name() -> String;
    }
}

fn main() {
    async_std::task::block_on(async move {
        let mut client = jsonrpsee::ws_client("127.0.0.1:9944").await.unwrap();
        let v = System::system_name(&mut client).await.unwrap();
        println!("{:?}", v);

        let _ = client.start_subscription("chain_subscribeNewHeads", jsonrpsee::core::common::Params::None).await;
        while let ev = client.next_event().await {
            println!("ev: {:?}", ev);
        }
    });
}
