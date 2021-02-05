#[cfg(all(feature = "tokio1", feature = "tokio02"))]
compile_error!("feature \"tokio1\" and \"tokio02\" are mutably exclusive");

#[cfg(feature = "hyper13")]
extern crate hyper13 as hyper;

#[cfg(feature = "hyper14")]
extern crate hyper14 as hyper;

pub mod http;
