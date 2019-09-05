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
        let server1 = jsonrpsee::server::http();
        jsonrpsee::server::run(server1, |method, _| async move {
            panic!()
        }).await;
    });

    /*let server1 = jsonrpsee::server::HttpServer::bind("0.0.0.0:8000");
    let server2 = jsonrpsee::server::HttpServer::bind("0.0.0.0:8080");
    let server = jsonrpsee::server::join(server1, server2);

    futures::executor::block_on(jsonrpsee::run(&server, |_, _| {
        panic!();       // TODO: remove
        future::ready(jsonrpsee::JsonValue::Null)
    }));*/

    let v = futures::executor::block_on(system_name());
    println!("{:?}", v);
}
