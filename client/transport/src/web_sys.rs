use futures_channel::mpsc;
use futures_util::StreamExt;
use jsonrpsee_core::async_trait;
use jsonrpsee_core::client::{TransportReceiverT, TransportSenderT};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{CloseEvent, MessageEvent, WebSocket};

#[derive(Debug)]
enum WebSocketMessage {
	Data(String),
	Close,
}

/// Sender.
// TODO(niklasad1): this might be slow because extra channel but did like this to avoid `unsafe Send for Sender { .. }`
#[derive(Debug)]
pub struct Sender(mpsc::UnboundedSender<WebSocketMessage>);

/// Receiver.
// TODO(niklasad1): this might be slow because extra channel but did like this to avoid `unsafe Send for Receiver { .. }`
#[derive(Debug)]
pub struct Receiver(mpsc::UnboundedReceiver<String>);

#[async_trait]
impl TransportSenderT for Sender {
	type Error = ();

	/// Sends out a request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	async fn send(&mut self, msg: String) -> Result<(), Self::Error> {
		self.0.unbounded_send(WebSocketMessage::Data(msg)).map_err(|_| ())
	}

	/// Send a close message and close the connection.
	async fn close(&mut self) -> Result<(), Self::Error> {
		self.0.unbounded_send(WebSocketMessage::Close).map_err(|_| ())
	}
}

#[async_trait]
impl TransportReceiverT for Receiver {
	type Error = ();

	/// Returns a `Future` resolving when the server sent us something back.
	async fn receive(&mut self) -> Result<String, Self::Error> {
		match self.0.next().await {
			Some(msg) => Ok(msg),
			None => Err(()),
		}
	}
}

/// Create a transport sender & receiver pair.
pub async fn build_transport(url: impl AsRef<str>) -> Result<(Sender, Receiver), ()> {
	let (from_back, rx) = mpsc::unbounded();
	let (tx, mut to_back) = mpsc::unbounded();

	let websocket = WebSocket::new(url.as_ref()).map_err(|_| ())?;
	websocket.set_binary_type(web_sys::BinaryType::Arraybuffer);

	let from_back1 = from_back.clone();
	let on_msg_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
		// Binary message.
		if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
			let msg = abuf.to_string();
			let _ = from_back1.unbounded_send(msg.into());
		// Text message.
		} else if let Some(txt) = e.data().as_string() {
			let _ = from_back1.unbounded_send(txt);
		} else if let Ok(_blob) = e.data().dyn_into::<web_sys::Blob>() {
			// no supported yet..
		} else {
			// add logging...
		}
	}) as Box<dyn FnMut(MessageEvent)>);

	let websocket2 = websocket.clone();
	spawn_local(async move {
		while let Some(WebSocketMessage::Data(msg)) = to_back.next().await {
			let _ = websocket2.send_with_str(&msg);
		}
		let _ = websocket2.close();
	});

	let tx1 = tx.clone();
	// Close event.
	let on_close_callback = Closure::wrap(Box::new(move |_e: CloseEvent| {
		from_back.close_channel();
		tx1.close_channel();
	}) as Box<dyn FnMut(web_sys::CloseEvent)>);

	websocket.set_onopen(Some(on_msg_callback.as_ref().unchecked_ref()));
	websocket.set_onclose(Some(on_close_callback.as_ref().unchecked_ref()));

	// Prevent for being dropped (this will be leaked intentionally).
	on_msg_callback.forget();
	on_close_callback.forget();

	Ok((Sender(tx), Receiver(rx)))
}
