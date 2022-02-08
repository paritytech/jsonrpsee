use futures_channel::mpsc;
use futures_util::StreamExt;
use jsonrpsee_core::client::{TransportReceiverT, TransportSenderT};
use jsonrpsee_core::{async_trait, Error};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CloseEvent, MessageEvent, WebSocket};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/// Sender.
#[derive(Debug)]
pub struct Sender(WebSocket);

// TODO: safety.
unsafe impl Send for Sender {}

/// Receiver.
#[derive(Debug)]
pub struct Receiver(mpsc::UnboundedReceiver<String>);

#[async_trait]
impl TransportSenderT for Sender {
	/// Sends out a request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	async fn send(&mut self, msg: String) -> Result<(), Error> {
		log!("tx: {:?}", msg);
		self.0.send_with_str(&msg).map_err(|e| Error::Custom(e.as_string().unwrap()))
	}

	/// Send a close message and close the connection.
	async fn close(&mut self) -> Result<(), Error> {
		self.0.close().map_err(|e| Error::Custom(e.as_string().unwrap()))
	}
}

#[async_trait]
impl TransportReceiverT for Receiver {
	/// Returns a `Future` resolving when the server sent us something back.
	async fn receive(&mut self) -> Result<String, Error> {
		match self.0.next().await {
			Some(msg) => {
				log!("rx: {:?}", msg);
				Ok(msg)
			}
			None => Err(Error::Custom("channel closed".into())),
		}
	}
}

/// Create a transport sender & receiver pair.
pub async fn build_transport(url: impl AsRef<str>) -> Result<(Sender, Receiver), ()> {
	let (tx, rx) = mpsc::unbounded();

	let websocket = WebSocket::new(url.as_ref()).map_err(|_| ())?;
	websocket.set_binary_type(web_sys::BinaryType::Arraybuffer);

	let tx1 = tx.clone();

	let on_msg_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
		// Binary message.
		if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
			let msg = abuf.to_string();
			let _ = tx.unbounded_send(msg.into());
		// Text message.
		} else if let Some(txt) = e.data().as_string() {
			let _ = tx.unbounded_send(txt);
		} else if let Ok(_blob) = e.data().dyn_into::<web_sys::Blob>() {
			log!("Received blob message; not supported");
		} else {
			log!("Received unsupported message");
		}
	}) as Box<dyn FnMut(MessageEvent)>);

	// Close event.
	let on_close_callback = Closure::wrap(Box::new(move |_e: CloseEvent| {
		log!("channel closed");
		tx1.close_channel();
	}) as Box<dyn FnMut(web_sys::CloseEvent)>);

	let (conn_tx, mut conn_rx) = mpsc::unbounded();

	let on_open_callback = Closure::wrap(Box::new(move |_| {
		conn_tx.unbounded_send(()).expect("rx still alive; qed");
	}) as Box<dyn FnMut(JsValue)>);

	websocket.set_onopen(Some(on_open_callback.as_ref().unchecked_ref()));
	websocket.set_onmessage(Some(on_msg_callback.as_ref().unchecked_ref()));
	websocket.set_onclose(Some(on_close_callback.as_ref().unchecked_ref()));

	// Prevent for being dropped (this will be leaked intentionally).
	on_msg_callback.forget();
	on_open_callback.forget();
	on_close_callback.forget();

	conn_rx.next().await;

	Ok((Sender(websocket), Receiver(rx)))
}
