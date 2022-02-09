use std::time::Duration;

use anyhow::anyhow;
use futures_channel::{mpsc, oneshot};
use futures_timer::Delay;
use futures_util::future::{self, Either};
use futures_util::StreamExt;
use jsonrpsee_core::client::{TransportReceiverT, TransportSenderT};
use jsonrpsee_core::{async_trait, Error};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CloseEvent, MessageEvent, WebSocket};

#[derive(Debug)]
enum WebSocketMessage {
	Data(String),
	Close,
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
		self.0.unbounded_send(WebSocketMessage::Data(msg)).map_err(|e| Error::Transport(anyhow!("{:?}", e)))
	}

	async fn close(&mut self) -> Result<(), Error> {
		self.0.unbounded_send(WebSocketMessage::Close).map_err(|e| Error::Transport(anyhow!("{:?}", e)))
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
			None => Err(Error::Transport(anyhow!("Connection closed"))),
		}
	}
}

/// Create a transport sender & receiver pair.
pub async fn connect(url: impl AsRef<str>, connection_timeout: Duration) -> Result<(Sender, Receiver), Error> {
	let (from_back, rx) = mpsc::unbounded();
	let (tx, mut to_back) = mpsc::unbounded();

	let websocket =
		WebSocket::new(url.as_ref()).map_err(|e| Error::Transport(anyhow!("Connection failed: {:?}", e)))?;
	websocket.set_binary_type(web_sys::BinaryType::Arraybuffer);

	let tx1 = tx.clone();

	let from_back1 = from_back.clone();
	let on_msg_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
		// Binary message.
		if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
			let msg = abuf.to_string();
			let _ = from_back1.unbounded_send(msg.into());
		// Text message.
		} else if let Some(txt) = e.data().as_string() {
			let _ = from_back1.unbounded_send(txt);
		} else {
			tracing::warn!("Received unsupported message");
		}
	}) as Box<dyn FnMut(MessageEvent)>);

	// Close event.
	let on_close_callback = Closure::once(move |_e: CloseEvent| {
		tracing::info!("Connection closed");
		tx1.close_channel();
		from_back.close_channel();
	});

	let (conn_tx, conn_rx) = oneshot::channel();

	let on_open_callback = Closure::once(move |_: JsValue| {
		tracing::info!("Connection established");
		conn_tx.send(()).expect("rx still alive; qed");
	});

	websocket.set_onopen(Some(on_open_callback.as_ref().unchecked_ref()));
	websocket.set_onmessage(Some(on_msg_callback.as_ref().unchecked_ref()));
	websocket.set_onclose(Some(on_close_callback.as_ref().unchecked_ref()));

	// Prevent for being dropped (this will be leaked intentionally).
	on_msg_callback.forget();
	on_open_callback.forget();
	on_close_callback.forget();

	match future::select(conn_rx, Delay::new(connection_timeout)).await {
		Either::Left((_, _)) => (),
		Either::Right((_, _)) => return Err(Error::Transport(anyhow!("Connection timeout exceeded"))),
	};

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
