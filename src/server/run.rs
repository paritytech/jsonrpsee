use crate::{server::Server, types::Error, types::JsonValue};
use futures::prelude::*;

/// Runs the given server using the given handler.
///
/// Whenever the server receives an RPC request, the handler is invoked in order to determine how
/// to respond to it.
pub async fn run<S, H, F>(server: &S, mut handler: H)
where
    S: Server,
    H: FnMut(&str, &JsonValue) -> F,
    F: Future<Output = Result<JsonValue, Error>>
{
    let mut send_back = stream::FuturesUnordered::new();

    loop {
        // TODO: don't use maybe_done
        let mut next_request = future::maybe_done(server.next_request());

        // Wait for either the next request to arrive, or for one of the
        // `send_back` items to complete.
        future::select(&mut next_request, send_back.next()).await;

        let request = match next_request {
            future::MaybeDone::Done(Err(())) => break,
            future::MaybeDone::Done(Ok(request)) => request,
            _ => continue     // One of the `send_back` futures has finished.
        };

        let future = handler(request.method(), request.params());
        send_back.push(async move {
            let response = future.await;
            let _ = request.respond(response).await;
        });
    }

    // Drain the rest of `send_back` before returning.
    while let Some(_) = send_back.next().await {}
}
