use jsonrpsee::proc_macros::rpc;

// Subscription method must not use the same override name.
#[rpc(client, server)]
pub trait DupOverride {
	#[subscription(name = "subscribeOne" => "override", item = u8)]
	fn one(&self);
	#[subscription(name = "subscribeTwo" => "override", item = u8)]
	fn two(&self);
}

fn main() {}
