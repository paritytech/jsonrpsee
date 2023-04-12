use jsonrpsee::core::Error;
use jsonrpsee::proc_macros::rpc;

pub trait Config {
	type Hash: Send + Sync + 'static;
}

/// Client bound must be `Conf::Hash: jsonrpsee::core::DeserializeOwned`
/// Server bound must be `Conf::Hash: jsonrpsee::core::Serialize + Clone`
#[rpc(server, client, namespace = "foo", client_bounds(), server_bounds())]
pub trait EmptyBounds<Conf: Config> {
	#[method(name = "bar")]
	fn method(&self) -> Result<Conf::Hash, Error>;
}

fn main() {}
