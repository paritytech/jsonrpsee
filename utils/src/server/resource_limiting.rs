use std::convert::TryFrom;
use std::sync::{
	atomic::{AtomicU32, Ordering},
	Arc,
};

pub type ResourceMap<T> = [T; 8];

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
pub struct ResourceId(u8);

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
			.map(|(id, resource)| (ResourceId(id as u8), resource))
	}

	pub fn register_resource(&mut self, label: &'static str, capacity: u32, default: u32) -> Result<ResourceId, Error> {
		if self.get(label).is_some() {
			return Err(Error::ResourceNameAlreadyTaken(label));
		}

		let id = ResourceId(u8::try_from(self.table.len()).map_err(|_| Error::MaxResourcesReached)?);

		self.table.push(Resource { label, capacity, default });

		Ok(id)
	}

	pub fn build(self) -> ResourcesInternal {
		ResourcesInternal {
			table: self
				.table
				.into_iter()
				.map(|Resource { label, capacity, .. }| ResourceInternal { label, capacity, used: AtomicU32::new(0) })
				.collect(),
		}
	}
}

#[derive(Debug)]
struct ResourceInternal {
	/// We store the label, although we only use it for verbose errors
	label: &'static str,
	/// Max capacity of arbitrary units of the resource
	capacity: u32,
	/// Currently used units
	used: AtomicU32,
}

#[derive(Clone, Debug)]
pub struct ResourcesInternal {
	table: Arc<[ResourceInternal]>,
}

impl ResourcesInternal {
	/// Attempt to claim a given amount of units of a resource using the `id`.
	pub fn claim(&self, id: ResourceId, units: u32) -> Result<ClaimedResource, Error> {
		let resource = self.table.get(id.0 as usize).ok_or_else(|| Error::InvalidResourceId(id))?;

		resource.claim(units)?;

		Ok(ClaimedResource { resource, units })
	}
}

impl ResourceInternal {
	fn claim(&self, units: u32) -> Result<(), Error> {
		let previous = self.used.fetch_add(units, Ordering::SeqCst);

		if previous + units > self.capacity {
			self.used.fetch_sub(units, Ordering::SeqCst);
			Err(Error::ResourceAtCapacity(self.label))
		} else {
			Ok(())
		}
	}

	fn free(&self, units: u32) {
		let previous = self.used.fetch_sub(units, Ordering::SeqCst);

		// Overflow below 0 should never happen!
		if previous < units {
			panic!("Trying to free more units than where used for resource: {}", self.label);
		}
	}
}

/// RAII style "lock" for claimed resources, will automatically release them once dropped.
pub struct ClaimedResource<'a> {
	resource: &'a ResourceInternal,
	units: u32,
}

impl Drop for ClaimedResource<'_> {
	fn drop(&mut self) {
		self.resource.free(self.units)
	}
}
