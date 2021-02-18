/*#[cfg(all(feature = "hyper13", feature = "hyper14"))]
compile_error!("feature `hyper13` and `hyper14` are mutably exclusive");

#[cfg(not(any(feature = "hyper13", feature = "hyper14")))]
compile_error!("feature `hyper13` or `hyper14` must be enabled for this crate");

#[cfg(all(feature = "hyper13", not(feature = "hyper14")))]
extern crate hyper13 as hyper;

#[cfg(all(feature = "hyper14", not(feature = "hyper13")))]
extern crate hyper14 as hyper;*/

pub mod http;
