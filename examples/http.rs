
jsonrpsee::rpc_api! {
    Health {
        /// Test
        fn system_name() -> String;

        /// Test2
        fn system_name2() -> String;
    }

    System {
        fn test_foo() -> String;
    }
}

fn main() {
    // Spawning a server in a background task.
    async_std::task::spawn(async move {
        let mut server1 = jsonrpsee::http_server("127.0.0.1:8000").await;

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
    // TODO: URL is hardcoded in the library at the moment
    let v = futures::executor::block_on(Health::system_name());
    println!("{:?}", v);
}
