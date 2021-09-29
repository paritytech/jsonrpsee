use arrayvec::ArrayVec;
use jsonrpsee_types::error::Error;
use parking_lot::Mutex;

const RESOURCE_COUNT: usize = 8;

pub type ResourceVec<T> = ArrayVec<T, RESOURCE_COUNT>;
pub type ResourceMap<T> = [T; RESOURCE_COUNT];

#[derive(Debug)]
pub struct Resources {
	totals: Mutex<ResourceMap<u16>>,
	pub caps: ResourceMap<u16>,
	pub defaults: ResourceMap<u16>,
	pub labels: ResourceVec<&'static str>,
}

impl Resources {
	pub fn new() -> Self {
		Resources {
			totals: Mutex::new([0; RESOURCE_COUNT]),
			caps: [0; RESOURCE_COUNT],
			defaults: [0; RESOURCE_COUNT],
			labels: ResourceVec::new(),
		}
	}

	pub fn register(&mut self, label: &'static str, capacity: u16, default: u16) -> Result<(), Error> {
		if self.labels.iter().any(|&l| l == label) {
			return Err(Error::ResourceNameAlreadyTaken(label));
		}

		let idx = self.labels.len();

		self.labels.try_push(label).map_err(|_| Error::MaxResourcesReached)?;


		self.caps[idx] = capacity;
		self.defaults[idx] = default;

		Ok(())
	}

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
