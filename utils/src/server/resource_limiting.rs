use arrayvec::ArrayVec;
use parking_lot::Mutex;
use jsonrpsee_types::error::Error;

const RESOURCE_COUNT: usize = 8;

pub type ResourceVec<T> = ArrayVec<T, RESOURCE_COUNT>;
pub type ResourceMap<T> = [T; RESOURCE_COUNT];

/// Resource definition
#[derive(Debug)]
pub struct Resource {
	/// Human readable label for a resource, e.g.: "CPU", "Memory"...
	label: &'static str,
	/// Max capacity of arbitrary units of the resource
	capacity: u16,
	/// Default amount of units running a method costs
	default: u16,
}

#[derive(Debug)]
pub struct ResourceBuilder {
	table: ResourceVec<Resource>,
}

impl ResourceBuilder {
	pub fn new() -> Self {
		ResourceBuilder { table: ResourceVec::new() }
	}

	pub fn get(&self, label: &str) -> Option<(usize, &Resource)> {
		self.table
			.iter()
			.enumerate()
			.find(|(_, resource)| resource.label == label)
			.map(|(id, resource)| (id, resource))
	}

	pub fn register(&mut self, label: &'static str, capacity: u16, default: u16) -> Result<(), Error> {
		if self.get(label).is_some() {
			return Err(Error::ResourceNameAlreadyTaken(label));
		}

		self.table.try_push(Resource { label, capacity, default }).map_err(|_| Error::MaxResourcesReached)
	}

	pub fn build(self) -> ResourcesInternal {
		let mut caps = [0; RESOURCE_COUNT];
		let mut labels = [""; RESOURCE_COUNT];

		for (idx, Resource { label, capacity, .. }) in self.table.into_iter().enumerate() {
			caps[idx] = capacity;
			labels[idx] = label;
		}

		ResourcesInternal {
			totals: Mutex::new([0; RESOURCE_COUNT]),
			caps,
			labels,
		}
	}
}

#[derive(Debug)]
pub struct ResourcesInternal {
	totals: Mutex<ResourceMap<u16>>,
	caps: ResourceMap<u16>,
	labels: ResourceMap<&'static str>,
}

impl ResourcesInternal {
	pub fn claim(&self, units: ResourceMap<u16>) -> Result<ClaimedResource, Error> {
		let mut totals = self.totals.lock();
		let mut sum = *totals;

		for idx in 0..RESOURCE_COUNT {
			sum[idx] += units[idx];

			if sum[idx] > self.caps[idx] {
				return Err(Error::ResourceAtCapacity(self.labels[idx]));
			}
		}

		*totals = sum;

		Ok(ClaimedResource { totals: &self.totals, units })
	}
}

/// RAII style "lock" for claimed resources, will automatically release them once dropped.
pub struct ClaimedResource<'a> {
	totals: &'a Mutex<ResourceMap<u16>>,
	units: ResourceMap<u16>,
}

impl Drop for ClaimedResource<'_> {
	fn drop(&mut self) {
		for (sum, claimed) in self.totals.lock().iter_mut().zip(self.units) {
			*sum -= claimed;
		}
	}
}
