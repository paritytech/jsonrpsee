use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::Error;

pub trait Config {
	type Hash: Send + Sync + 'static;
}

/// Client bound must be `Conf::Hash: jsonrpsee::core::DeserializeOwned`
/// Server bound must be `Conf::Hash: jsonrpsee::core::Serialize`
#[rpc(server, client, namespace = "foo", client_bounds(), server_bounds())]
pub trait EmptyBounds<Conf: Config> {
	#[method(name = "bar")]
	fn method(&self) -> Result<Conf::Hash, Error>;
}

fn main() {}
