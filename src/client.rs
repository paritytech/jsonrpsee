pub use crate::raw_client::RawClientRef;


pub struct Client<R> {
    inner: R,
}

impl<R> Client<R>
where
    for<'r> &'r R: RawClientRef<'r>,
{

}

match_json! request {
    "test" => || {

    }
}
