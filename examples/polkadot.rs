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
    });
}
