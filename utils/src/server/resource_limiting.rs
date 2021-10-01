use std::sync::Arc;

use arrayvec::ArrayVec;
use jsonrpsee_types::error::Error;
use parking_lot::Mutex;

const RESOURCE_COUNT: usize = 8;

/// Fixed size table mapping a resource to some value
pub type ResourceTable<T> = [T; RESOURCE_COUNT];
/// Variable size table mapping a resource to some value
pub type ResourceVec<T> = ArrayVec<T, RESOURCE_COUNT>;

/// User defined resources used by the JSON-RPC server.
#[derive(Debug, Default, Clone)]
pub struct Resources {
	/// Current unit values that are being used by concurrent method executions (0 for empty slots)
	totals: Arc<Mutex<ResourceTable<u16>>>,
	/// Unit capacities for every registered resource (0 for empty slots)
	pub capacities: ResourceTable<u16>,
	/// Default unit values a method execution uses for every registered resource (0 for empty slots)
	pub defaults: ResourceTable<u16>,
	/// Labels for every registered resource
	pub labels: ResourceVec<&'static str>,
}

impl Resources {
	/// Register a new resource type. Errors if `label` was already registered, or if number of
	/// registered resources would exceed 8.
	pub fn register(&mut self, label: &'static str, capacity: u16, default: u16) -> Result<(), Error> {
		if self.labels.iter().any(|&l| l == label) {
			return Err(Error::ResourceNameAlreadyTaken(label));
		}

		let idx = self.labels.len();

		self.labels.try_push(label).map_err(|_| Error::MaxResourcesReached)?;

		self.capacities[idx] = capacity;
		self.defaults[idx] = default;

		Ok(())
	}

	/// Attempt to claim units for each resource, incrementing current totals.
	/// If successful returns a `ResourceGuard` which decrements the totals by the same
	/// amounts once dropped.
	pub fn claim(&self, units: ResourceTable<u16>) -> Result<ResourceGuard, Error> {
		let mut totals = self.totals.lock();
		let mut sum = *totals;

		for (idx, sum) in sum.iter_mut().enumerate() {
			match sum.checked_add(units[idx]) {
				Some(s) if s <= self.capacities[idx] => *sum = s,
				_ => {
					let label = self.labels.get(idx).copied().unwrap_or("<UNKNOWN>");

					return Err(Error::ResourceAtCapacity(label));
				}
			}
		}

		*totals = sum;

		Ok(ResourceGuard { totals: self.totals.clone(), units })
	}
}

/// RAII style "lock" for claimed resources, will automatically release them once dropped.
#[derive(Debug)]
pub struct ResourceGuard {
	totals: Arc<Mutex<ResourceTable<u16>>>,
	units: ResourceTable<u16>,
}

impl Drop for ResourceGuard {
	fn drop(&mut self) {
		for (sum, claimed) in self.totals.lock().iter_mut().zip(self.units) {
			*sum -= claimed;
		}
	}
}
