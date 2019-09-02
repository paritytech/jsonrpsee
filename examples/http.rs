fn main() {
    let server1 = jsonrpsee::server::HttpServer::bind("0.0.0.0:8000");
    let server2 = jsonrpsee::server::HttpServer::bind("0.0.0.0:8080");
    let server = jsonrpsee::server::join(server1, server2);

    futures::executor::block_on(jsonrpsee::run_server(&server));
}
