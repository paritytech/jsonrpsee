#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::server::Server;

pub mod server;

pub async fn run_server(server: &impl server::Server) {
    while let Ok(req) = server.next_request().await {
        println!("request!");
    }
}
