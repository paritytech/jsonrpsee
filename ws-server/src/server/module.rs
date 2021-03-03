use super::*;

pub struct Module {
	methods: Methods,
}

impl Module {
	pub fn new() -> Self {
		Module {
			methods: Methods::default()
		}
	}

	fn verify_method_name(&mut self, name: &'static str) -> Result<(), Error> {
		if self.methods.get(name).is_some() {
			return Err(Error::MethodAlreadyRegistered(name.into()));
		}

		Ok(())
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<F, R>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize,
		F: RpcMethod<R>,
	{
		self.verify_method_name(method_name)?;

		self.methods.insert(
			method_name,
			Box::new(move |id, params, tx, _| {
				let result = callback(params)?;

				send_response(id, tx, result);

				Ok(())
			}),
		);

		Ok(())
	}

	/// Register a new RPC subscription, with subscribe and unsubscribe methods.
	pub fn register_subscription(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
	) -> Result<SubscriptionSink, Error> {
		if subscribe_method_name == unsubscribe_method_name {
			return Err(Error::MethodAlreadyRegistered(subscribe_method_name.into()));
		}

		self.verify_method_name(subscribe_method_name)?;
		self.verify_method_name(unsubscribe_method_name)?;

		let subscribers = Arc::new(Mutex::new(FxHashMap::default()));

		{
			let subscribers = subscribers.clone();
			self.methods.insert(
				subscribe_method_name,
				Box::new(move |id, _, tx, conn| {
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;

						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;

						subscribers.lock().insert((conn, sub_id), tx.clone());

						sub_id
					};

					send_response(id, tx, sub_id);

					Ok(())
				}),
			);
		}

		{
			let subscribers = subscribers.clone();
			self.methods.insert(
				unsubscribe_method_name,
				Box::new(move |id, params, tx, conn| {
					let sub_id = params.one()?;

					subscribers.lock().remove(&(conn, sub_id));

					send_response(id, tx, "Unsubscribed");

					Ok(())
				}),
			);
		}

		Ok(SubscriptionSink { method: subscribe_method_name, subscribers })
	}

	pub(crate) fn into_map(self) -> Methods {
		self.methods
	}

	pub(crate) fn merge(&mut self, other: Module) -> Result<(), Error> {
		for name in other.methods.keys() {
			self.verify_method_name(name)?;
		}

		for (name, callback) in other.methods {
			self.methods.insert(name, callback);
		}

		Ok(())
	}
}
