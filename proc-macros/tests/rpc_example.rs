//! Example of using proc macro to generate working client and server.

use jsonrpsee_proc_macros::rpc;

// #[rpc(client, server, namespace = "foo")]
pub trait Rpc {
	// #[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> u16;

	// #[method(name = "bar")]
	fn sync_method(&self) -> u16;

	// #[subscription(name = "sub", unsub = "unsub", item = String)]
	fn sub(&self);
}

#[jsonrpsee::types::__reexports::async_trait]
#[doc = "Server trait implementation for the `Rpc` RPC API."]
pub trait RpcServer: Sized + Send + Sync + 'static {
    async fn async_method(&self, param_a: u8, param_b: String) -> u16;
    fn sync_method(&self) -> u16;
    fn sub(&self, subscription_sink: jsonrpsee::SubscriptionSink);
    #[doc = "Collects all the methods and subscriptions defined in the trait and adds them into a single `RpcModule`."]
    fn into_rpc(self) -> jsonrpsee::RpcModule<Self> {
        let inner = move || -> Result<jsonrpsee::RpcModule<Self>, jsonrpsee::types::Error> {
            let mut rpc = jsonrpsee::RpcModule::new(self);
            rpc.register_async_method("foo_foo", |params, context| {
                let fut = async move {
                    let (param_a, param_b) = if params.is_object() {
                        #[derive(jsonrpsee::types::__reexports::serde::Deserialize)]
                        struct ParamsObject {
                            param_a: u8,
                            param_b: String,
                        }
                        panic!("Not supported!");
                    } else {
                        let mut seq = params.sequence();
                        let param_a: u8 = seq.next()?;
                        let param_b: String = seq.next()?;
                        (param_a, param_b)
                    };
                    Ok(context.as_ref().async_method(param_a, param_b).await)
                };
                Box::pin(fut)
            })?;
            rpc.register_method("foo_bar", |params, context| Ok(context.sync_method()))?;
            rpc.register_subscription("foo_sub", "foo_unsub", |params, sink, context| {
                Ok(context.as_ref().sub(sink))
            })?;
            Ok(rpc)
        };
        inner().expect("RPC macro method names should never conflict")
    }
}
