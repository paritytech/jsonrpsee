jsonrpsee::rpc_api! {
    Health {
        /// Test
        fn system_name(foo: String, bar: i32) -> String;

        fn test_notif(foo: String, bar: i32);

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
                Health::TestNotif { foo, bar } => {
                    println!("server got notif: {:?} {:?}", foo, bar);
                }
            }
        }
    });

    // Client demo.
    let mut client = jsonrpsee::http_client("http://127.0.0.1:8000");
    let v = async_std::task::block_on(async {
        Health::test_notif(&mut client, "notif_string", 192).await.unwrap();
        Health::system_name(&mut client, "hello", 5).await.unwrap()
    });
    println!("{:?}", v);
}
