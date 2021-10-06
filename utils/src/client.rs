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

//! Shared utilities for `jsonrpsee` clients.

#[doc(hidden)]
pub mod __reexports {
	pub use jsonrpsee_types::{to_json_value, v2::ParamsSer};
}

#[macro_export]
/// Convert the given values to a [`ParamsSer`] as expected by a jsonrpsee Client (http or websocket).
macro_rules! rpc_params {
	($($param:expr),*) => {
		{
			let mut __params = vec![];
			$(
				__params.push($crate::client::__reexports::to_json_value($param).expect("json serialization is infallible; qed."));
			)*
			$crate::client::__reexports::ParamsSer::Array(__params)
		}
	};
	() => {
		$crate::client::__reexports::ParamsSer::Array(vec![])
	}
}
