pub use crate::raw_client::RawClientRef;

/// Wraps around a "raw client" and analyzes everything correctly.
pub struct Client<R> {
    inner: R,
}

impl<R> Client<R>
where
    for<'r> &'r R: RawClientRef<'r>,
{

}
