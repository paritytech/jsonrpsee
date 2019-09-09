use futures::prelude::*;

jsonrpsee::rpc_api! {
    Health {
        /// Test
        fn system_name() -> String;
    }
}

fn main() {
    // Spawning a server in a background task.
    async_std::task::spawn(async move {
        let mut server1 = jsonrpsee::http_server("127.0.0.1:8000").await;

        while let Ok(request) = Health::next_request(&mut server1).await {
            match request {
                Health::system_name { .. } => (), // TODO:
                                                  // Health::system_name { send_back } => send_back.respond("hello");
            }
        }
    });

    // Client demo.
    // TODO: URL is hardcoded in the library at the moment
    let v = futures::executor::block_on(system_name());
    println!("{:?}", v);
}
