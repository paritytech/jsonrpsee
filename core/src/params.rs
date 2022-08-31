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

//! RPC parameters.

use crate::Error;
use serde::Serialize;
use serde_json::value::RawValue;

/// Helper module for building parameters.
mod params_builder {
	use serde::Serialize;

	/// Initial number of bytes for a parameter length.
	const PARAM_BYTES_CAPACITY: usize = 128;

	/// Generic parameter builder that serializes parameters to bytes.
	/// This produces a JSON compatible String.
	///
	/// The implementation relies on `Vec<u8>` to hold the serialized
	/// parameters in memory for the following reasons:
	///   1. Other serialization methods than `serde_json::to_writer` would internally
	///      have an extra heap allocation for temporarily holding the value in memory.
	///   2. `io::Write` is not implemented for `String` required for serialization.
	#[derive(Debug)]
	pub(crate) struct ParamsBuilder {
		bytes: Vec<u8>,
		end: char,
	}

	impl ParamsBuilder {
		/// Construct a new [`ParamsBuilder`] with custom start and end tokens.
		/// The inserted values are wrapped by the _start_ and _end_ tokens.
		fn new(start: char, end: char) -> Self {
			let mut bytes = Vec::with_capacity(PARAM_BYTES_CAPACITY);
			bytes.push(start as u8);
			ParamsBuilder { bytes, end }
		}

		/// Construct a new [`ParamsBuilder`] for positional parameters equivalent to a JSON array object.
		pub(crate) fn positional() -> Self {
			Self::new('[', ']')
		}

		/// Construct a new [`ParamsBuilder`] for named parameters equivalent to a JSON map object.
		pub(crate) fn named() -> Self {
			Self::new('{', '}')
		}

		/// Insert a named value (key, value) pair into the builder.
		/// The _name_ and _value_ are delimited by the `:` token.
		pub(crate) fn insert_named<P: Serialize>(&mut self, name: &str, value: P) -> Result<(), serde_json::Error> {
			serde_json::to_writer(&mut self.bytes, name)?;
			self.bytes.push(b':');
			serde_json::to_writer(&mut self.bytes, &value)?;
			self.bytes.push(b',');

			Ok(())
		}

		/// Insert a plain value into the builder.
		pub(crate) fn insert<P: Serialize>(&mut self, value: P) -> Result<(), serde_json::Error> {
			serde_json::to_writer(&mut self.bytes, &value)?;
			self.bytes.push(b',');

			Ok(())
		}

		/// Finish the building process and return a JSON compatible string.
		pub(crate) fn build(mut self) -> String {
			let idx = self.bytes.len() - 1;
			if self.bytes[idx] == b',' {
				self.bytes[idx] = self.end as u8;
			} else {
				self.bytes.push(self.end as u8);
			}

			// Safety: This is safe because JSON does not emit invalid UTF-8.
			unsafe { String::from_utf8_unchecked(self.bytes) }
		}
	}
}

/// Parameter builder that serializes named value parameters to a JSON compatible string.
/// This is the equivalent of a JSON Map object `{ key: value }`.
///
/// # Examples
///
/// ```rust
///
/// use jsonrpsee_core::params::ObjectParamsBuilder;
///
/// let mut builder = ObjectParamsBuilder::new();
/// builder.insert("param1", 1);
/// builder.insert("param2", "abc");
/// let params = builder.build();
///
/// // Use RPC parameters...
/// ```
#[derive(Debug)]
pub struct ObjectParamsBuilder(params_builder::ParamsBuilder);

impl ObjectParamsBuilder {
	/// Construct a new [`ObjectParamsBuilder`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Insert a named value (key, value) pair into the builder.
	/// The _name_ and _value_ are delimited by the `:` token.
	pub fn insert<P: Serialize>(&mut self, name: &str, value: P) -> Result<(), serde_json::Error> {
		self.0.insert_named(name, value)
	}

	/// Finish the building process and return a JSON compatible string.
	pub fn build(self) -> ObjectParams {
		ObjectParams(self.0.build())
	}
}

impl Default for ObjectParamsBuilder {
	fn default() -> Self {
		Self(params_builder::ParamsBuilder::named())
	}
}

/// Object RPC parameters stored as a JSON Map object `{ key: value }`.
#[derive(Clone, Debug)]
pub struct ObjectParams(String);

impl ToRpcParams for ObjectParams {
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
		RawValue::from_string(self.0).map(Some).map_err(Error::ParseError)
	}
}

/// Parameter builder that serializes plain value parameters to a JSON compatible string.
/// This is the equivalent of a JSON Array object `[ value0, value1, .., valueN ]`.
///
/// # Examples
///
/// ```rust
///
/// use jsonrpsee_core::params::ArrayParamsBuilder;
///
/// let mut builder = ArrayParamsBuilder::new();
/// builder.insert("param1");
/// builder.insert(1);
/// let params = builder.build();
///
/// // Use RPC parameters...
/// ```
#[derive(Debug)]
pub struct ArrayParamsBuilder(params_builder::ParamsBuilder);

impl ArrayParamsBuilder {
	/// Construct a new [`ArrayParamsBuilder`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Insert a plain value into the builder.
	pub fn insert<P: Serialize>(&mut self, value: P) -> Result<(), serde_json::Error> {
		self.0.insert(value)
	}

	/// Finish the building process and return a JSON compatible string.
	pub fn build(self) -> ArrayParams {
		ArrayParams(self.0.build())
	}
}

impl Default for ArrayParamsBuilder {
	fn default() -> Self {
		Self(params_builder::ParamsBuilder::positional())
	}
}

/// Array RPC parameters stored as a JSON Array object `[ value0, value1, .., valueN ]`.
#[derive(Clone, Debug)]
pub struct ArrayParams(String);

impl ToRpcParams for ArrayParams {
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
		RawValue::from_string(self.0).map(Some).map_err(Error::ParseError)
	}
}

/// Marker trait for types that can be serialized as JSON compatible strings.
///
/// This trait ensures the correctness of the RPC parameters.
///
/// # Note
///
/// Please consider using the [`ArrayParamsBuilder`] and [`ObjectParamsBuilder`] than
/// implementing this trait.
///
/// # Examples
///
/// - Implementation for hard-coded strings
///
/// ```rust
///
/// use jsonrpsee_core::params::ToRpcParams;
/// use serde_json::value::RawValue;
/// use jsonrpsee_core::Error;
///
/// struct ManualParam;
///
/// impl ToRpcParams for ManualParam {
///     fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
///         // Manually define a valid JSONRPC parameter.
///         RawValue::from_string("[1, \"2\", 3]".to_string()).map(Some).map_err(Error::ParseError)
///     }
/// }
/// ```
///
/// - Implementation for JSON serializable structures
///
/// ```rust
/// use jsonrpsee_core::params::ToRpcParams;
/// use serde_json::value::RawValue;
/// use serde::Serialize;
/// use jsonrpsee_core::Error;
///
/// #[derive(Serialize)]
/// struct SerParam {
///     param_1: u8,
///     param_2: String,
/// };
///
/// impl ToRpcParams for SerParam {
///     fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
///         let s = String::from_utf8(serde_json::to_vec(&self)?).expect("Valid UTF8 format");
///         RawValue::from_string(s).map(Some).map_err(Error::ParseError)
///     }
/// }
/// ```
pub trait ToRpcParams {
	/// Consume and serialize the type as a JSON raw value.
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error>;
}

/// Initial number of parameters in a batch request.
const BATCH_PARAMS_NUM_CAPACITY: usize = 4;

/// Request builder that serializes RPC parameters to construct a valid batch parameter.
/// This is the equivalent of chaining multiple RPC requests.
#[derive(Clone, Debug, Default)]
pub struct BatchRequestBuilder<'a>(Vec<(&'a str, Option<Box<RawValue>>)>);

impl<'a> BatchRequestBuilder<'a> {
	/// Construct a new [`BatchRequestBuilder`].
	pub fn new() -> Self {
		Self(Vec::with_capacity(BATCH_PARAMS_NUM_CAPACITY))
	}

	/// Inserts the RPC method with provided parameters into the builder.
	pub fn insert<Params: ToRpcParams>(&mut self, method: &'a str, value: Params) -> Result<(), Error> {
		self.0.push((method, value.to_rpc_params()?));
		Ok(())
	}

	/// Finish the building process and return a valid batch parameter.
	pub fn build(self) -> Vec<(&'a str, Option<Box<RawValue>>)> {
		self.0
	}
}

/// Empty RPC parameters that perform no allocation.
#[derive(Clone, Debug)]
pub struct EmptyParams;

/// Custom implementation for empty RPC parameters.
impl ToRpcParams for EmptyParams {
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
		Ok(None)
	}
}
