use beef::lean::Cow;
use rustc_hash::FxHashMap;
use serde::de::{self, Deserializer, Unexpected, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::fmt;
use tokio::sync::mpsc;

pub mod error;
pub mod traits;

pub type ConnectionId = usize;
pub type RpcSender<'a> = &'a mpsc::UnboundedSender<String>;
pub type RpcId<'a> = Option<&'a RawValue>;
pub type Method = Box<dyn Send + Sync + Fn(RpcId, RpcParams, RpcSender, ConnectionId) -> anyhow::Result<()>>;
pub type Methods = FxHashMap<&'static str, Method>;
pub use error::RpcError;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcRequest<'a> {
	pub jsonrpc: TwoPointZero,

	#[serde(borrow)]
	pub id: Option<&'a RawValue>,

	#[serde(borrow)]
	pub method: Cow<'a, str>,

	#[serde(borrow)]
	pub params: Option<&'a RawValue>,
}

#[derive(Deserialize, Debug)]
pub struct JsonRpcInvalidRequest<'a> {
	#[serde(borrow)]
	pub id: Option<&'a RawValue>,
}

#[derive(Serialize, Debug)]
pub struct JsonRpcNotification<'a> {
	pub jsonrpc: TwoPointZero,
	pub method: &'a str,
	pub params: JsonRpcNotificationParams<'a>,
}

#[derive(Serialize, Debug)]
pub struct JsonRpcNotificationParams<'a> {
	pub subscription: u64,
	pub result: &'a RawValue,
}

#[derive(Serialize, Debug)]
pub struct JsonRpcResponse<'a, T> {
	pub jsonrpc: TwoPointZero,
	pub result: T,
	pub id: Option<&'a RawValue>,
}

#[derive(Serialize, Debug)]
pub struct JsonRpcError<'a> {
	pub jsonrpc: TwoPointZero,
	pub error: JsonRpcErrorParams<'a>,
	pub id: Option<&'a RawValue>,
}

#[derive(Serialize, Debug)]
pub struct JsonRpcErrorParams<'a> {
	pub code: i32,
	pub message: &'a str,
}

#[derive(Debug, Default)]
pub struct TwoPointZero;

struct TwoPointZeroVisitor;

impl<'de> Visitor<'de> for TwoPointZeroVisitor {
	type Value = TwoPointZero;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(r#"a string "2.0""#)
	}

	fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		match s {
			"2.0" => Ok(TwoPointZero),
			_ => Err(de::Error::invalid_value(Unexpected::Str(s), &self)),
		}
	}
}

impl<'de> Deserialize<'de> for TwoPointZero {
	fn deserialize<D>(deserializer: D) -> Result<TwoPointZero, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_str(TwoPointZeroVisitor)
	}
}

impl Serialize for TwoPointZero {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str("2.0")
	}
}

/// Parameters sent with the RPC request
#[derive(Clone, Copy)]
pub struct RpcParams<'a>(Option<&'a str>);

impl<'a> RpcParams<'a> {
	/// Create params
	pub fn new(raw: Option<&'a str>) -> Self {
		Self(raw)
	}

	/// Attempt to parse all parameters as array or map into type T
	pub fn parse<T>(self) -> Result<T, RpcError>
	where
		T: Deserialize<'a>,
	{
		match self.0 {
			None => Err(RpcError::InvalidParams),
			Some(params) => serde_json::from_str(params).map_err(|_| RpcError::InvalidParams),
		}
	}

	/// Attempt to parse only the first parameter from an array into type T
	pub fn one<T>(self) -> Result<T, RpcError>
	where
		T: Deserialize<'a>,
	{
		self.parse::<[T; 1]>().map(|[res]| res)
	}
}
