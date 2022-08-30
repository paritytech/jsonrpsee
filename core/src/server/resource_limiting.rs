// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! # Resource Limiting
//!
//! This module handles limiting the capacity of the server to respond to requests.
//!
//! `jsonrpsee` is agnostic about the types of resources available on the server, and the units used are arbitrary.
//! The units are used to model the availability of a resource, be it something mundane like CPU or Memory,
//! or more exotic things like remote API access to a 3rd party service, or use of some external hardware
//! that's under the control of the server.
//!
//! To get the most out of this feature, we suggest benchmarking individual methods to see how many resources they
//! consume, in particular anything critical that is expected to result in a lot of stress on the server,
//! and then defining your units such that the limits (`capacity`) can be adjusted for different hardware configurations.
//!
//! Up to 8 resources can be defined using the [`ServerBuilder::register_resource`](../../../jsonrpsee_server/struct.ServerBuilder.html#method.register_resource)
//!
//!
//! Each method will claim the specified number of units (or the default) for the duration of its execution.
//! Any method execution that would cause the total sum of claimed resource units to exceed
//! the `capacity` of that resource will be denied execution, immediately returning JSON-RPC error object with code `-32604`.
//!
//! Setting the execution cost to `0` equates to the method effectively not being limited by a given resource. Likewise setting the
//! `capacity` to `0` disables any limiting for a given resource.
//!
//! To specify a different than default number of units a method should use, use the `resources` argument in the
//! `#[method]` attribute:
//!
//! ```
//! # use jsonrpsee::{core::RpcResult, proc_macros::rpc};
//! #
//! #[rpc(server)]
//! pub trait Rpc {
//!     #[method(name = "my_expensive_method", resources("cpu" = 5, "mem" = 2))]
//!     async fn my_expensive_method(&self) -> RpcResult<&'static str> {
//!         // Do work
//!         Ok("hello")
//!     }
//! }
//! ```
//!
//! Alternatively, you can use the `resource` method when creating a module manually without the help of the macro:
//!
//! ```
//! # use jsonrpsee::{RpcModule, core::RpcResult};
//! #
//! # fn main() -> RpcResult<()> {
//! #
//! let mut module = RpcModule::new(());
//!
//! module
//!     .register_async_method("my_expensive_method", |_, _| async move {
//!         // Do work
//!         Ok("hello")
//!     })?
//!     .resource("cpu", 5)?
//!     .resource("mem", 2)?;
//! # Ok(())
//! # }
//! ```
//!
//! Each resource needs to have a unique name, such as `"cpu"` or `"memory"`, which can then be used across all
//! [`RpcModule`s](crate::server::rpc_module::RpcModule). In case a module definition uses a resource label not
//! defined on the server, starting the server with such a module will result in a runtime error containing the
//! information about the offending method.

use std::sync::Arc;

use crate::Error;
use arrayvec::ArrayVec;
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
