use crate::{raw_server::RawServerRef, server::Server, types::Error, types::JsonValue};
use futures::{prelude::*, pin_mut};

/// Runs the given server using the given handler.
///
/// Whenever the server receives an RPC request, the handler is invoked in order to determine how
/// to respond to it.
pub async fn run<H, F>(mut server: Server<crate::raw_server::HttpServer>, mut handler: H)
where
    //for<'r> &'r mut S: RawServerRef<'r>,
    H: FnMut(&str, &JsonValue) -> F,
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

        let future = handler(request.method(), &crate::types::JsonValue::Null);// TODO: request.params());
        //send_back.push(async move {
            let response = future.await;
            let _ = request.respond(response).await;
        //});
    }

    // Drain the rest of `send_back` before returning.
    //while let Some(_) = send_back.next().await {}
}

#[cfg(test)]
mod tests {
    use futures::prelude::*;

    #[test]
    fn is_send_static() {
        fn req<T: 'static>(_: T) {}     // TODO: + Send
        fn test() {
            let fut = super::run(unimplemented!(), |_, _| future::ready(panic!()));
            req(fut);
        }
    }
}
