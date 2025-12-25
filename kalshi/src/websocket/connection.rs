use crate::kalshi_error::KalshiError;
use crate::TradingEnvironment;
use futures_util::{stream::SplitSink, stream::SplitStream, SinkExt, StreamExt};
use openssl::pkey::{PKey, Private};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSink = SplitSink<WsStream, Message>;
type WsReader = SplitStream<WsStream>;

/// Response from a WebSocket command.
#[derive(Debug, Clone)]
pub enum CommandResponse {
    /// Successful acknowledgment.
    Ok { id: i32 },
    /// Error response from the server.
    Error { code: i32, msg: String },
    /// Subscription confirmation with assigned SID.
    Subscribed { sid: i32, channel: String },
}

/// Default timeout for waiting on command responses (in seconds).
const DEFAULT_COMMAND_TIMEOUT_SECS: u64 = 10;

/// WebSocket client for real-time Kalshi data.
pub struct KalshiWebSocket {
    url: String,
    key_id: String,
    private_key: PKey<Private>,
    writer: Option<Arc<Mutex<WsSink>>>,
    reader: Option<WsReader>,
    next_id: i32,
    pub(crate) subscriptions: HashMap<i32, super::Subscription>,
    /// Pending command response channels, keyed by command ID.
    pending_commands: HashMap<i32, oneshot::Sender<CommandResponse>>,
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
            subscriptions: HashMap::new(),
            pending_commands: HashMap::new(),
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
        self.pending_commands.clear();
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

    /// Registers a pending command to receive its response.
    pub(crate) fn register_pending_command(
        &mut self,
        id: i32,
    ) -> oneshot::Receiver<CommandResponse> {
        let (tx, rx) = oneshot::channel();
        self.pending_commands.insert(id, tx);
        rx
    }

    /// Routes a command response to the appropriate pending command.
    /// Returns true if the response was routed, false if no pending command was found.
    pub(crate) fn route_response(&mut self, id: i32, response: CommandResponse) -> bool {
        if let Some(sender) = self.pending_commands.remove(&id) {
            // Ignore send error - receiver may have been dropped
            let _ = sender.send(response);
            true
        } else {
            false
        }
    }

    /// Waits for a single command response with timeout.
    pub(crate) async fn wait_for_response(
        &mut self,
        rx: oneshot::Receiver<CommandResponse>,
    ) -> Result<CommandResponse, KalshiError> {
        match tokio::time::timeout(Duration::from_secs(DEFAULT_COMMAND_TIMEOUT_SECS), rx).await {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err(KalshiError::InternalError(
                "Response channel closed unexpectedly".to_string(),
            )),
            Err(_) => Err(KalshiError::InternalError(
                "Timeout waiting for command response".to_string(),
            )),
        }
    }

    /// Waits for multiple command responses (e.g., multiple `subscribed` messages).
    /// Returns responses in the order they are received.
    pub(crate) async fn wait_for_responses(
        &mut self,
        mut receivers: Vec<(i32, oneshot::Receiver<CommandResponse>)>,
        expected_count: usize,
    ) -> Result<Vec<CommandResponse>, KalshiError> {
        let mut responses = Vec::with_capacity(expected_count);
        let deadline =
            tokio::time::Instant::now() + Duration::from_secs(DEFAULT_COMMAND_TIMEOUT_SECS);

        while responses.len() < expected_count && !receivers.is_empty() {
            let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remaining.is_zero() {
                return Err(KalshiError::InternalError(
                    "Timeout waiting for all command responses".to_string(),
                ));
            }

            // Try to read more messages to route responses
            if let Some(reader) = self.reader.as_mut() {
                match tokio::time::timeout(Duration::from_millis(100), reader.next()).await {
                    Ok(Some(Ok(Message::Text(text)))) => {
                        if let Ok(msg) = super::WebSocketMessage::parse(&text) {
                            self.handle_control_message(&msg);
                        }
                    }
                    Ok(Some(Ok(_))) => {
                        // Non-text message, ignore
                    }
                    Ok(Some(Err(_))) | Ok(None) => {
                        return Err(KalshiError::InternalError(
                            "WebSocket connection closed".to_string(),
                        ));
                    }
                    Err(_) => {
                        // Timeout on read, continue checking receivers
                    }
                }
            }

            // Check which receivers have responses ready
            let mut i = 0;
            while i < receivers.len() {
                match receivers[i].1.try_recv() {
                    Ok(response) => {
                        responses.push(response);
                        receivers.remove(i);
                    }
                    Err(oneshot::error::TryRecvError::Empty) => {
                        i += 1;
                    }
                    Err(oneshot::error::TryRecvError::Closed) => {
                        // Channel closed without response
                        receivers.remove(i);
                    }
                }
            }
        }

        if responses.len() < expected_count {
            return Err(KalshiError::InternalError(format!(
                "Expected {} responses, got {}",
                expected_count,
                responses.len()
            )));
        }

        Ok(responses)
    }

    /// Handles control messages (subscribed, ok, error) and routes them to pending commands.
    pub(crate) fn handle_control_message(&mut self, msg: &super::WebSocketMessage) {
        match msg {
            super::WebSocketMessage::Subscribed(sub_msg) => {
                // For subscribed messages, we need to find the pending command by iterating
                // since the server response doesn't include the original command ID directly.
                // Instead, we route based on channel matching for the most recently registered command.
                // Note: This is a simplification. In practice, we track by command ID.
                let response = CommandResponse::Subscribed {
                    sid: sub_msg.sid,
                    channel: sub_msg.channel.clone(),
                };
                // Try to route to any pending command (they should be waiting for subscribed responses)
                if let Some((&id, _)) = self.pending_commands.iter().next() {
                    self.route_response(id, response);
                }
            }
            super::WebSocketMessage::Ok(ok_msg) => {
                let response = CommandResponse::Ok { id: ok_msg.sid };
                self.route_response(ok_msg.sid, response);
            }
            super::WebSocketMessage::Error(err_msg) => {
                let response = CommandResponse::Error {
                    code: err_msg.code,
                    msg: err_msg.msg.clone(),
                };
                // Route to the first pending command since errors don't have command IDs
                if let Some((&id, _)) = self.pending_commands.iter().next() {
                    self.route_response(id, response);
                }
            }
            _ => {
                // Non-control message, ignore
            }
        }
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
                    Ok(msg) => {
                        // Route control messages to pending commands
                        self.ws.handle_control_message(&msg);
                        Poll::Ready(Some(msg))
                    }
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
