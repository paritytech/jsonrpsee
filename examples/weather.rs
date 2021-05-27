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

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use restson::{Error as RestsonError, RestPath};
use serde::{Deserialize, Serialize};
use jsonrpsee::{
	ws_client::{traits::SubscriptionClient, v2::params::JsonRpcParams, WsClientBuilder},
	ws_server::WsServer,
	ws_server::RpcContextModule,
};

//API response:
// ```
//	{
//	"coord": {
//	  "lon": 145.77,
//	  "lat": -16.92
//	},
//	"weather": [
//	  {
//		"id": 802,
//		"main": "Clouds",
//		"description": "scattered clouds",
//		"icon": "03n"
//	  }
//	],
//	"base": "stations",
//	"main": {
//	  "temp": 300.15,
//	  "pressure": 1007,
//	  "humidity": 74,
//	  "temp_min": 300.15,
//	  "temp_max": 300.15
//	},
//	"visibility": 10000,
//	"wind": {
//	  "speed": 3.6,
//	  "deg": 160
//	},
//	"clouds": {
//	  "all": 40
//	},
//	"dt": 1485790200,
//	"sys": {
//	  "type": 1,
//	  "id": 8166,
//	  "message": 0.2064,
//	  "country": "AU",
//	  "sunrise": 1485720272,
//	  "sunset": 1485766550
//	},
//	"id": 2172797,
//	"name": "Cairns",
//	"cod": 200
//	}
//```

#[derive(Deserialize, Serialize, Debug)]
struct Weather {
	base: String,
	id: usize,
	name: String,
	wind: Wind,
	clouds: Clouds,
	main: Main,
}
#[derive(Deserialize, Serialize, Debug)]
struct Clouds {
	all: usize,
}
#[derive(Deserialize, Serialize, Debug)]
struct Main {
	temp: f64,
	pressure: usize,
	humidity: usize,
}
#[derive(Deserialize, Serialize, Debug)]
struct Wind {
	speed: f64,
	deg: usize,
}


impl RestPath<(&str, &str)> for Weather {
	fn get_path(params: (&str, &str)) -> Result< String, RestsonError> {
		let city = params.0;
		let units = params.1;
		Ok(String::from(format!("data/2.5/weather?q={}&units={}&appid=f6ba475df300d5f91135550da0f4a867", city, units)))
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();
	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;

	// Subscription to the london weather
	let params = JsonRpcParams::Array(vec!["London,uk".into(), "metric".into()]);
	let mut weather_sub = client.subscribe::<String>("weather_sub", params, "weather_unsub").await?;
	// NOTE: this is never printed.
	println!("[client] London weather: {:?}", weather_sub.next().await);


	Ok(())
}
async fn run_server() -> anyhow::Result<SocketAddr> {
	let mut server = WsServer::new("127.0.0.1:0").await?;
	let api_client = restson::RestClient::new("http://api.openweathermap.org").unwrap();
	let mut module = RpcContextModule::new(Mutex::new(api_client));
	module
		.register_subscription_with_context("weather_sub", "weather_unsub", |params, sink, api_client| {
			println!("[server] raw params={:?}", params);
			let params: (String, String) = params.parse()?;
			println!("[server] subscribed with params={:?}", params);
			std::thread::spawn(move || loop {
					// println!("[server] taking lock");
					let mut api = api_client.lock().unwrap();
					// println!("[server] took lock");
					let out: Weather = api.get(("London,uk", "imperial")).unwrap();
					println!("[server] got london weather: {:?}, sending", out);
					sink.send(&out).expect("Sending should work yes?");
					drop(api);
					// println!("[server] released lock; sleeping");
					std::thread::sleep(std::time::Duration::from_millis(150));
					// println!("[server] slept");
			});
			Ok(())
		})
		.unwrap();

	server.register_module(module.into_module());

	let addr = server.local_addr()?;
	tokio::spawn(async move { server.start().await });
	Ok(addr)
}
