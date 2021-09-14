use std::convert::TryFrom;
use std::sync::{
	atomic::{AtomicU32, Ordering},
	Arc,
};

pub struct Resource {
	label: &'static str,
	capacity: u32,
	default: u32,
}

pub struct ResourceId(u8);

pub struct Resources {
	table: Vec<Resource>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Resource at capacity: {0}")]
	ResourceAtCapacity(&'static str),
	#[error("Resource name already taken: {0}")]
	ResourceNameAlreadyTaken(&'static str),
	#[error("Maximum number of resources reached")]
	MaxResourcesReached,
}

impl Resources {
	pub fn new() -> Self {
		Resources { table: Vec::new() }
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

	pub fn into_internal(self) -> ResourcesInternal {
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
pub struct ResourceInternal {
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
	pub fn get(&self, id: ResourceId) -> Option<&ResourceInternal> {
		self.table.get(id.0 as usize)
	}
}

impl ResourceInternal {
	pub fn claim(&self, units: u32) -> Result<(), Error> {
		let previous = self.used.fetch_add(units, Ordering::SeqCst);

		if previous + units > self.capacity {
			self.used.fetch_sub(units, Ordering::SeqCst);
			Err(Error::ResourceAtCapacity(self.label))
		} else {
			Ok(())
		}
	}

	pub fn free(&self, units: u32) {
		let previous = self.used.fetch_sub(units, Ordering::SeqCst);

		// Overflow below 0 should never happen!
		if previous < units {
			panic!("Trying to free more units than where used for resource: {}", self.label);
		}
	}
}
