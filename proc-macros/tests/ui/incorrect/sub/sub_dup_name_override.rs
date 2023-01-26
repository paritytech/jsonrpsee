use jsonrpsee::proc_macros::rpc;

// Subscription method must not use the same override name.
#[rpc(client, server)]
pub trait DupOverride {
	#[subscription(name = "subscribeOne" => "override", item = u8)]
	async fn one(&self) -> jsonrpsee::core::SubscriptionResult;
	#[subscription(name = "subscribeTwo" => "override", item = u8)]
	async fn two(&self) -> jsonrpsee::core::SubscriptionResult;
}

fn main() {}
