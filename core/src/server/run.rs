use crate::{
    common::Error, common::JsonValue, server::raw::RawServer, server::Server,
    server::ServerRequestParams,
};
use futures::{pin_mut, prelude::*};

/// Runs the given server using the given handler.
///
/// Whenever the server receives an RPC request, the handler is invoked in order to determine how
/// to respond to it.
pub async fn run<'a, S, H, F>(server: &'a mut Server<S>, mut handler: H)
where
    S: RawServer,
    H: FnMut(&str, ServerRequestParams) -> F,
    F: Future<Output = Result<JsonValue, Error>>,
{
    //let mut send_back = stream::FuturesUnordered::new();
    let mut send_back = stream::FuturesUnordered::<future::Pending<()>>::new();

    loop {
        // Wait for either the next request to arrive, or for one of the
        // `send_back` items to complete.
        let next_request = server.next_request();
        pin_mut!(next_request);
        let request = match future::select(next_request, send_back.next()).await {
            // We received a request from the server.
            future::Either::Left((Ok(rq), _)) => rq,
            // Server has shut down.
            future::Either::Left((Err(_), _)) => break,
            // One of the elements of `send_back` is finished.
            future::Either::Right(_) => continue,
        };

        let future = handler(request.method(), request.params());
        //send_back.push(async move {
        let response = future.await;
        let _ = request.respond(response).await;
        //});
    }

    // Drain the rest of `send_back` before returning.
    while let Some(_) = send_back.next().await {}
}

#[cfg(test)]
mod tests {
    use futures::prelude::*;

    #[test]
    fn is_send_static() {
        fn req<T: 'static>(_: T) {} // TODO: + Send; see https://github.com/rust-lang/rust/issues/64176
        #[allow(unused)]
        fn test() {
            let fut = super::run(unimplemented!(), |_, _| future::ready(panic!()));
            req(fut);
        }
    }
}
