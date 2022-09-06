use crate::tests::helpers::{init_logger, server_with_handles};
use futures_util::future::join;
use jsonrpsee_core::Error;
use jsonrpsee_test_utils::TimeoutFutureExt;
use std::time::Duration;

#[tokio::test]
async fn stop_works() {
	init_logger();
	let (_addr, server_handle) = server_with_handles().with_default_timeout().await.unwrap();

	server_handle.clone().stop().unwrap().await;

	// After that we should be able to wait for task handle to finish.
	// First `unwrap` is timeout, second is `JoinHandle`'s one.

	// After server was stopped, attempt to stop it again should result in an error.
	assert!(matches!(server_handle.stop(), Err(Error::AlreadyStopped)));
}

#[tokio::test]
async fn run_forever() {
	const TIMEOUT: Duration = Duration::from_millis(200);

	init_logger();
	let (_addr, server_handle) = server_with_handles().with_default_timeout().await.unwrap();

	assert!(matches!(server_handle.with_timeout(TIMEOUT).await, Err(_timeout_err)));

	let (_addr, server_handle) = server_with_handles().with_default_timeout().await.unwrap();

	// Send the shutdown request from one handle and await the server on the second one.
	join(server_handle.clone().stop().unwrap(), server_handle).with_timeout(TIMEOUT).await.unwrap();
}
