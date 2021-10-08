use std::sync::Arc;

use arrayvec::ArrayVec;
use jsonrpsee_types::error::Error;
use parking_lot::Mutex;

// The number of kinds of resources that can be used for limiting.
const RESOURCE_COUNT: usize = 8;

/// Fixed size table, mapping a resource to a (unitless) value indicating the amount of the resource that is available to RPC calls.
pub type ResourceTable = [u16; RESOURCE_COUNT];
/// Variable size table, mapping a resource to a (unitless) value indicating the amount of the resource that is available to RPC calls.
pub type ResourceVec<T> = ArrayVec<T, RESOURCE_COUNT>;

/// User defined resources available to be used by calls on the JSON-RPC server.
/// Each of the 8 possible resource kinds, for instance "cpu", "io", "nanobots",
/// store a maximum `capacity` and a default. A value of `0` means no limits for the given resource.
#[derive(Debug, Default, Clone)]
pub struct Resources {
	/// Resources currently in use by executing calls. 0 for unused resource kinds.
	totals: Arc<Mutex<ResourceTable>>,
	/// Max capacity for all resource kinds
	pub capacities: ResourceTable,
	/// Default value for all resource kinds; unless a method has a resource limit defined, this is the cost of a call (0 means no default limit)
	pub defaults: ResourceTable,
	/// Labels for every registered resource
	pub labels: ResourceVec<&'static str>,
}

impl Resources {
	/// Register a new resource kind. Errors if `label` is already registered, or if the total number of
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

	/// Attempt to claim `units` units for each resource, incrementing current totals.
	/// If successful, returns a [`ResourceGuard`] which decrements the totals by the same
	/// amounts once dropped.
	pub fn claim(&self, units: ResourceTable) -> Result<ResourceGuard, Error> {
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
	totals: Arc<Mutex<ResourceTable>>,
	units: ResourceTable,
}

impl Drop for ResourceGuard {
	fn drop(&mut self) {
		for (sum, claimed) in self.totals.lock().iter_mut().zip(self.units) {
			*sum -= claimed;
		}
	}
}
