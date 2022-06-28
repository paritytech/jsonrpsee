use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::RpcResult;

pub trait Config {
	type Hash: Send + Sync + 'static;
	type NotUsed;
}

#[rpc(client, namespace = "foo", client_bounds(Conf::Hash: jsonrpsee::core::DeserializeOwned))]
pub trait MyRpcClient<Conf: Config> {
	#[method(name = "bar")]
	fn method(&self) -> RpcResult<Conf::Hash>;
}

#[rpc(server, namespace = "foo", server_bounds(Conf::Hash: jsonrpsee::core::Serialize))]
pub trait MyRpcServer<Conf: Config> {
	#[method(name = "bar")]
	fn method(&self) -> RpcResult<Conf::Hash>;
}

#[rpc(server, client, namespace = "foo", client_bounds(Conf::Hash: jsonrpsee::core::DeserializeOwned), server_bounds(Conf::Hash: jsonrpsee::core::Serialize))]
pub trait MyRpcServerClient<Conf: Config> {
	#[method(name = "bar")]
	fn method(&self) -> RpcResult<Conf::Hash>;
}

fn main() {}
