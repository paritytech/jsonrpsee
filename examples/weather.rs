// Copyright 2019 Parity Technologies (UK) Ltd.
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

//! Example of setting up a subscription that polls a remote API, in this case the api.openweathermap.org/weather, and
//! sends the data back to the subscriber whenever the weather in London changes. The openweathermap API client is
//! passed at registration as part of the "context" object. We only want to send data on the subscription when the
//! weather actually changes, so we store the current weather in the context, hence the need for a `Mutex` to allow
//! mutation.

use jsonrpsee::{
	ws_client::{traits::SubscriptionClient, v2::params::JsonRpcParams, WsClientBuilder},
	ws_server::RpcContextModule,
	ws_server::WsServer,
};
use restson::{Error as RestsonError, RestPath};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Mutex;

// Set up the types to deserialize the weather data.
// See https://openweathermap.org/current for the details about the API used in this example.
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
struct Weather {
	name: String,
	wind: Wind,
	clouds: Clouds,
	main: Main,
}
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
struct Clouds {
	all: usize,
}
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
struct Main {
	temp: f64,
	pressure: usize,
	humidity: usize,
}
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
struct Wind {
	speed: f64,
	deg: usize,
}

impl RestPath<&(String, String)> for Weather {
	fn get_path(params: &(String, String)) -> Result<String, RestsonError> {
		// Set up your own API key at https://openweathermap.org/current
		const API_KEY: &'static str = "f6ba475df300d5f91135550da0f4a867";
		Ok(String::from(format!("data/2.5/weather?q={}&units={}&appid={}", params.0, params.1, API_KEY,)))
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();
	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;

	// Subscription to the London weather
	let params = JsonRpcParams::Array(vec!["London,uk".into(), "metric".into()]);
	let mut weather_sub = client.subscribe::<Weather>("weather_sub", params, "weather_unsub").await?;
	while let Some(w) = weather_sub.next().await {
		println!("[client] London weather: {:?}", w);
	}

	Ok(())
}

/// The context passed on registration, used to store a REST client to query for the current weather and the current
/// "state".
struct WeatherApiCx {
	api_client: restson::RestClient,
	last_weather: Weather,
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let mut server = WsServer::new("127.0.0.1:0").await?;

	let api_client = restson::RestClient::new("http://api.openweathermap.org").unwrap();
	let last_weather = Weather::default();
	let cx = Mutex::new(WeatherApiCx { api_client, last_weather });
	let mut module = RpcContextModule::new(cx);
	module
		.register_subscription_with_context("weather_sub", "weather_unsub", |params, sink, cx| {
			let params: (String, String) = params.parse()?;
			log::debug!(target: "server", "Subscribed with params={:?}", params);
			std::thread::spawn(move || loop {
				let mut cx = cx.lock().unwrap();
				let current_weather: Weather = cx.api_client.get(&params).unwrap();
				if current_weather != cx.last_weather {
					log::debug!(target: "server", "Fetched London weather: {:?}, sending", current_weather);
					sink.send(&current_weather).unwrap();
					cx.last_weather = current_weather;
				} else {
					log::trace!(target: "server", "Same weather as before. Not sending.")
				}
				std::thread::sleep(std::time::Duration::from_millis(500));
			});
			Ok(())
		})
		.unwrap();

	server.register_module(module.into_module()).unwrap();

	let addr = server.local_addr()?;
	tokio::spawn(async move { server.start().await });
	Ok(addr)
}
