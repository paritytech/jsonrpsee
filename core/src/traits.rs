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

use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use serde_json::value::RawValue;

use jsonrpsee_types::SubscriptionId;

/// Marker trait for types that can be serialized as JSON array/sequence.
///
/// If your type isn't a sequence, for example `String`, `usize` or similar
/// you must insert it in a tuple, slice, array or Vec for it to work.
pub trait ToRpcParams: Serialize {
	/// Serialize the type as a JSON array.
	fn to_rpc_params(&self) -> Result<Box<RawValue>, serde_json::Error> {
		serde_json::to_string(&self).map(|json| RawValue::from_string(json).expect("JSON String; qed"))
	}
}

impl<P: Serialize> ToRpcParams for &[P] {}
impl<P: Serialize> ToRpcParams for Vec<P> {}
impl<P, const N: usize> ToRpcParams for [P; N] where [P; N]: Serialize {}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name: Serialize),+> ToRpcParams for ($($name,)+) {}
        )+
    }
}

tuple_impls! {
	1 => (0 T0)
	2 => (0 T0 1 T1)
	3 => (0 T0 1 T1 2 T2)
	4 => (0 T0 1 T1 2 T2 3 T3)
	5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
	6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
	7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
	8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
	9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
	10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
	11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
	12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
	13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
	14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
	15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
	16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

/// Trait used to provide unique subscription IDs.
pub trait IdProvider: Send + Sync {
	/// Returns the next ID for the subscription.
	fn next_id(&self) -> SubscriptionId<'static>;
}

/// Generates random integers as subscription ID.
#[derive(Debug)]
pub struct RandomIntegerIdProvider;

impl IdProvider for RandomIntegerIdProvider {
	fn next_id(&self) -> SubscriptionId<'static> {
		const JS_NUM_MASK: u64 = !0 >> 11;
		(rand::random::<u64>() & JS_NUM_MASK).into()
	}
}

/// Generates random strings of length `len` as subscription ID.
#[derive(Debug)]
pub struct RandomStringIdProvider {
	len: usize,
}

impl RandomStringIdProvider {
	/// Create a new random string provider.
	pub fn new(len: usize) -> Self {
		Self { len }
	}
}

impl IdProvider for RandomStringIdProvider {
	fn next_id(&self) -> SubscriptionId<'static> {
		let mut rng = rand::thread_rng();
		(&mut rng).sample_iter(Alphanumeric).take(self.len).map(char::from).collect::<String>().into()
	}
}

/// No-op implementation to be used for servers that doesn't support subscriptions.
#[derive(Debug)]
pub struct NoopIdProvider;

impl IdProvider for NoopIdProvider {
	fn next_id(&self) -> SubscriptionId<'static> {
		0.into()
	}
}
