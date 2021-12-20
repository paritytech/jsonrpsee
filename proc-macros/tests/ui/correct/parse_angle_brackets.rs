use jsonrpsee::{proc_macros::rpc, core::RpcResult};

fn main() {
	#[rpc(server)]
	pub trait Rpc {
		#[subscription(
			name = "submitAndWatchExtrinsic",
			aliases = ["author_extrinsicUpdate"],
			unsubscribe_aliases = ["author_unwatchExtrinsic"],
			// Arguments are being parsed the nearest comma,
			// angle braces need to be accounted for manually.
			item = TransactionStatus<Hash, BlockHash>,
		)]
		fn dummy_subscription(&self) -> RpcResult<()>;
	}
}
