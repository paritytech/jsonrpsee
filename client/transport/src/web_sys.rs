use std::time::Duration;

use futures_channel::{mpsc, oneshot};
use futures_timer::Delay;
use futures_util::future::{self, Either};
use futures_util::StreamExt;
use jsonrpsee_core::async_trait;
use jsonrpsee_core::client::{TransportReceiverT, TransportSenderT};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CloseEvent, MessageEvent, WebSocket};

#[derive(Debug)]
enum WebSocketMessage {
	Data(String),
	Close,
}

/// Web-sys transport error that can occur.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Internal send error
	#[error("Could not send message: {0}")]
	SendError(#[from] mpsc::SendError),
	/// Connection got closed
	#[error("Connection is closed")]
	ConnectionClosed,
	/// Timeout while trying to connect.
	#[error("Connection timeout exceeded: {0:?}")]
	ConnectionTimeout(Duration),
	/// Error that occurred in `JS context`.
	#[error("JS Error: {0:?}")]
	JsError(String),
}

/// Sender.
#[derive(Debug)]
pub struct Sender(mpsc::UnboundedSender<WebSocketMessage>);

/// Receiver.
#[derive(Debug)]
pub struct Receiver(mpsc::UnboundedReceiver<String>);

#[async_trait]
impl TransportSenderT for Sender {
	type Error = Error;

	async fn send(&mut self, msg: String) -> Result<(), Self::Error> {
		tracing::trace!("tx: {:?}", msg);
		self.0.unbounded_send(WebSocketMessage::Data(msg)).map_err(|e| e.into_send_error())?;
		Ok(())
	}

	async fn close(&mut self) -> Result<(), Error> {
		self.0.unbounded_send(WebSocketMessage::Close).map_err(|e| e.into_send_error())?;
		Ok(())
	}
}

#[async_trait]
impl TransportReceiverT for Receiver {
	type Error = Error;

	async fn receive(&mut self) -> Result<String, Self::Error> {
		match self.0.next().await {
			Some(msg) => {
				tracing::trace!("rx: {:?}", msg);
				Ok(msg)
			}
			None => Err(Error::ConnectionClosed),
		}
	}
}

/// Create a transport sender & receiver pair.
pub async fn connect(url: impl AsRef<str>, conn_timeout: Duration) -> Result<(Sender, Receiver), Error> {
	let (from_back, rx) = mpsc::unbounded();
	let (tx, mut to_back) = mpsc::unbounded();

	let websocket = WebSocket::new(url.as_ref()).map_err(|e| Error::JsError(format!("{:?}", e)))?;
	// TODO: use `BinaryType::Blob` it's faster for larger objects.
	websocket.set_binary_type(web_sys::BinaryType::Arraybuffer);

	let tx1 = tx.clone();

	let from_back1 = from_back.clone();
	let on_msg_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
		// Supported formats: https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send
		let js_val = e.data();
		tracing::trace!("rx: {:?}", js_val);

		// Text message.
		if let Some(txt) = js_val.dyn_ref::<js_sys::JsString>() {
			let _ = from_back1.unbounded_send(String::from(txt));
		}
		// Binary message.
		else if let Some(abuf) = js_val.dyn_ref::<js_sys::ArrayBuffer>() {
			let array = js_sys::Uint8Array::new(abuf);
			let msg = String::from_utf8(array.to_vec()).expect("valid UTF-8 from WebSocket; qed");
			let _ = from_back1.unbounded_send(msg);
		} else {
			tracing::warn!("Received unsupported message: {:?}", js_val);
		}
	}) as Box<dyn FnMut(MessageEvent)>);

	// Close event.
	let on_close_callback = Closure::once(move |_e: CloseEvent| {
		tracing::info!("Connection closed");
		tx1.close_channel();
		from_back.close_channel();
	});

	websocket.set_onmessage(Some(on_msg_callback.as_ref().unchecked_ref()));
	websocket.set_onclose(Some(on_close_callback.as_ref().unchecked_ref()));

	// Prevent for being dropped (this will be leaked intentionally).
	on_msg_callback.forget();
	on_close_callback.forget();

	try_connect_until(&websocket, conn_timeout).await?;

	let tx3 = tx.clone();
	wasm_bindgen_futures::spawn_local(async move {
		while let Some(WebSocketMessage::Data(msg)) = to_back.next().await {
			if let Err(e) = websocket.send_with_str(&msg) {
				tracing::warn!("Failed to send: {:?}", e);
				break;
			}
		}

		let _ = websocket.close();
		tx3.close_channel();
	});

	Ok((Sender(tx), Receiver(rx)))
}

async fn try_connect_until(websocket: &WebSocket, conn_timeout: Duration) -> Result<(), Error> {
	let (tx, rx) = oneshot::channel();

	let on_open_callback = Closure::once(move |_: JsValue| {
		tracing::info!("Connection established");
		let _ = tx.send(());
	});

	websocket.set_onopen(Some(on_open_callback.as_ref().unchecked_ref()));

	let res = match future::select(rx, Delay::new(conn_timeout)).await {
		Either::Left((Ok(()), _)) => Ok(()),
		Either::Left((Err(_), _)) => unreachable!("A message is sent on this channel before close; qed"),
		Either::Right((_, _)) => Err(Error::ConnectionTimeout(conn_timeout)),
	};
	drop(on_open_callback);

	res
}
