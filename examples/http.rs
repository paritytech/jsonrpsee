jsonrpsee::rpc_api! {
    Health {
        /// Test
        fn system_name() -> String;

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
                Health::SystemName { respond } => {
                    respond.ok("hello").await;
                }
                Health::SystemName2 { respond } => {
                    respond.ok("hello 2").await;
                }
            }
        }
    });

    // Client demo.
    let mut client = jsonrpsee::http_client("http://127.0.0.1:8000");
    let v = async_std::task::block_on(Health::system_name(&mut client));
    println!("{:?}", v);
}
