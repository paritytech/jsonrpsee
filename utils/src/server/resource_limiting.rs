use std::convert::TryFrom;
use std::sync::{
	atomic::{AtomicU32, Ordering},
	Arc,
};
use parking_lot::Mutex;

const RESOURCE_COUNT: usize = 8;

pub type ResourceMap<T> = [T; RESOURCE_COUNT];

/// Resource definition
pub struct Resource {
	/// Human readable label for a resource, e.g.: "CPU", "Memory"...
	label: &'static str,
	/// Max capacity of arbitrary units of the resource
	capacity: u32,
	/// Default amount of units running a method costs
	default: u32,
}

/// Id referencing a resource for `O(1)` lookups
#[derive(Debug, Clone, Copy)]
pub struct ResourceId(usize);

pub struct ResourceBuilder {
	table: Vec<Resource>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Invalid resource id: {0:?}")]
	InvalidResourceId(ResourceId),
	#[error("Resource at capacity: {0}")]
	ResourceAtCapacity(&'static str),
	#[error("Resource name already taken: {0}")]
	ResourceNameAlreadyTaken(&'static str),
	#[error("Maximum number of resources reached")]
	MaxResourcesReached,
}

impl ResourceBuilder {
	pub fn new() -> Self {
		ResourceBuilder { table: Vec::new() }
	}

	pub fn get(&self, label: &str) -> Option<(ResourceId, &Resource)> {
		self.table
			.iter()
			.enumerate()
			.find(|(_, resource)| resource.label == label)
			.map(|(id, resource)| (ResourceId(id), resource))
	}

	pub fn register_resource(&mut self, label: &'static str, capacity: u32, default: u32) -> Result<ResourceId, Error> {
		if self.get(label).is_some() {
			return Err(Error::ResourceNameAlreadyTaken(label));
		}

		if self.table.len() >= RESOURCE_COUNT {
			return Err(Error::MaxResourcesReached);
		}

		let id = ResourceId(self.table.len());

		self.table.push(Resource { label, capacity, default });

		Ok(id)
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
	totals: Mutex<ResourceMap<u32>>,
	caps: ResourceMap<u32>,
	labels: ResourceMap<&'static str>,
}

impl ResourcesInternal {
	pub fn claim(&self, units: ResourceMap<u32>) -> Result<ClaimedResource, Error> {
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
	totals: &'a Mutex<ResourceMap<u32>>,
	units: ResourceMap<u32>,
}

impl Drop for ClaimedResource<'_> {
	fn drop(&mut self) {
		for (sum, claimed) in self.totals.lock().iter_mut().zip(self.units) {
			*sum -= claimed;
		}
	}
}
