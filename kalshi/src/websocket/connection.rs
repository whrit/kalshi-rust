use crate::kalshi_error::KalshiError;
use crate::TradingEnvironment;
use futures_util::{stream::SplitSink, stream::SplitStream, SinkExt, StreamExt};
use openssl::pkey::{PKey, Private};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSink = SplitSink<WsStream, Message>;
type WsReader = SplitStream<WsStream>;

/// WebSocket client for real-time Kalshi data.
pub struct KalshiWebSocket {
    url: String,
    key_id: String,
    private_key: PKey<Private>,
    writer: Option<Arc<Mutex<WsSink>>>,
    reader: Option<WsReader>,
    next_id: i32,
    pub(crate) subscriptions: std::collections::HashMap<i32, super::Subscription>,
}

impl KalshiWebSocket {
    /// Creates a new WebSocket client (does not connect yet).
    pub fn new(trading_env: TradingEnvironment, key_id: &str, private_key: PKey<Private>) -> Self {
        let url = match trading_env {
            TradingEnvironment::DemoMode => "wss://demo-api.kalshi.co/trade-api/ws/v2",
            TradingEnvironment::ProdMode => "wss://api.elections.kalshi.com/trade-api/ws/v2",
        };

        Self {
            url: url.to_string(),
            key_id: key_id.to_string(),
            private_key,
            writer: None,
            reader: None,
            next_id: 1,
            subscriptions: std::collections::HashMap::new(),
        }
    }

    /// Connects to the WebSocket server with authentication.
    pub async fn connect(&mut self) -> Result<(), KalshiError> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let method = "GET";
        let path = "/trade-api/ws/v2";

        let message = format!("{}{}{}", timestamp, method, path);
        let signature = self.sign_message(&message)?;

        let auth_url = format!(
            "{}?api-key={}&timestamp={}&signature={}",
            self.url, self.key_id, timestamp, signature
        );

        let (ws_stream, _response) = connect_async(&auth_url)
            .await
            .map_err(|e| KalshiError::InternalError(format!("WebSocket connect failed: {}", e)))?;

        let (write, read) = ws_stream.split();
        self.writer = Some(Arc::new(Mutex::new(write)));
        self.reader = Some(read);

        Ok(())
    }

    /// Disconnects from the WebSocket server.
    pub async fn disconnect(&mut self) -> Result<(), KalshiError> {
        if let Some(writer) = &self.writer {
            let mut w = writer.lock().await;
            w.close()
                .await
                .map_err(|e| KalshiError::InternalError(format!("Close failed: {}", e)))?;
        }
        self.writer = None;
        self.reader = None;
        self.subscriptions.clear();
        Ok(())
    }

    /// Returns true if connected.
    pub fn is_connected(&self) -> bool {
        self.writer.is_some()
    }

    fn sign_message(&self, message: &str) -> Result<String, KalshiError> {
        use openssl::hash::MessageDigest;
        use openssl::rsa::Padding;
        use openssl::sign::Signer;

        let mut signer = Signer::new(MessageDigest::sha256(), &self.private_key)?;
        signer.set_rsa_padding(Padding::PKCS1_PSS)?;
        signer.set_rsa_pss_saltlen(openssl::sign::RsaPssSaltlen::DIGEST_LENGTH)?;
        signer.update(message.as_bytes())?;
        let signature = signer.sign_to_vec()?;
        Ok(base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &signature,
        ))
    }

    pub(crate) fn get_next_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Sends a command to the WebSocket server.
    pub(crate) async fn send_command(&mut self, cmd: serde_json::Value) -> Result<(), KalshiError> {
        let writer = self
            .writer
            .as_ref()
            .ok_or_else(|| KalshiError::InternalError("Not connected".to_string()))?;

        let msg = Message::Text(serde_json::to_string(&cmd)?);
        let mut w = writer.lock().await;
        w.send(msg)
            .await
            .map_err(|e| KalshiError::InternalError(format!("Send failed: {}", e)))?;
        Ok(())
    }
}

// Stream interface (Task 4.7)
use futures_util::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

impl KalshiWebSocket {
    /// Returns a stream of WebSocket messages.
    pub fn messages(&mut self) -> impl Stream<Item = super::WebSocketMessage> + '_ {
        MessageStream { ws: self }
    }
}

struct MessageStream<'a> {
    ws: &'a mut KalshiWebSocket,
}

impl<'a> Stream for MessageStream<'a> {
    type Item = super::WebSocketMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let reader = match self.ws.reader.as_mut() {
            Some(r) => r,
            None => return Poll::Ready(None),
        };

        match Pin::new(reader).poll_next(cx) {
            Poll::Ready(Some(Ok(Message::Text(text)))) => {
                match super::WebSocketMessage::parse(&text) {
                    Ok(msg) => Poll::Ready(Some(msg)),
                    Err(_) => {
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
            }
            Poll::Ready(Some(Ok(Message::Ping(_)))) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Poll::Ready(Some(Ok(_))) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Poll::Ready(Some(Err(_))) => Poll::Ready(None),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
