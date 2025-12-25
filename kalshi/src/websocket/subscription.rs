use super::{Channel, CommandResponse, KalshiWebSocket};
use crate::kalshi_error::KalshiError;
use serde::{Deserialize, Serialize};

/// Represents an active subscription.
#[derive(Debug, Clone)]
pub struct Subscription {
    pub sid: i32,
    pub channel: Channel,
    pub market_tickers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubscribeResponse {
    pub sid: i32,
    pub channel: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAction {
    AddMarkets,
    DeleteMarkets,
}

/// Parses a channel string back to a Channel enum.
fn parse_channel(channel_str: &str) -> Option<Channel> {
    match channel_str {
        "orderbook_delta" => Some(Channel::OrderbookDelta),
        "ticker" => Some(Channel::Ticker),
        "trade" => Some(Channel::Trade),
        "fill" => Some(Channel::Fill),
        "market_position" => Some(Channel::MarketPosition),
        "market_lifecycle_v2" => Some(Channel::MarketLifecycleV2),
        "event_lifecycle" => Some(Channel::EventLifecycle),
        "multivariate" => Some(Channel::Multivariate),
        "communications" => Some(Channel::Communications),
        _ => None,
    }
}

impl KalshiWebSocket {
    /// Subscribe to one or more channels for specified markets.
    ///
    /// This method sends a subscribe command and waits for the server to confirm
    /// each channel subscription. The returned `SubscribeResponse` contains the
    /// assigned subscription IDs (SIDs) which can be used for unsubscribing.
    ///
    /// # Arguments
    ///
    /// * `channels` - List of channels to subscribe to
    /// * `market_ticker` - Optional single market ticker to subscribe to
    /// * `market_tickers` - Optional list of market tickers to subscribe to
    ///
    /// # Returns
    ///
    /// A vector of `SubscribeResponse` containing the SID and channel name for each subscription.
    pub async fn subscribe(
        &mut self,
        channels: Vec<Channel>,
        market_ticker: Option<String>,
        market_tickers: Option<Vec<String>>,
    ) -> Result<Vec<SubscribeResponse>, KalshiError> {
        if channels.is_empty() {
            return Ok(vec![]);
        }

        let id = self.get_next_id();
        let expected_responses = channels.len();

        // Collect the market tickers for storing in subscriptions
        let tickers: Vec<String> = match (&market_ticker, &market_tickers) {
            (Some(ticker), _) => vec![ticker.clone()],
            (_, Some(tickers)) => tickers.clone(),
            (None, None) => vec![],
        };

        // Register pending commands for each expected response
        let mut receivers = Vec::with_capacity(expected_responses);
        for i in 0..expected_responses {
            let response_id = id + i as i32;
            let rx = self.register_pending_command(response_id);
            receivers.push((response_id, rx));
        }

        // Build and send the subscribe command
        let mut cmd = serde_json::json!({
            "id": id,
            "cmd": "subscribe",
            "params": {
                "channels": channels.iter().map(|c| c.to_string()).collect::<Vec<_>>()
            }
        });

        if let Some(ticker) = market_ticker {
            cmd["params"]["market_ticker"] = serde_json::Value::String(ticker);
        }
        if let Some(tickers_list) = market_tickers {
            cmd["params"]["market_tickers"] = serde_json::Value::Array(
                tickers_list
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            );
        }

        self.send_command(cmd).await?;

        // Wait for all subscription confirmations
        let responses = self
            .wait_for_responses(receivers, expected_responses)
            .await?;

        // Process responses and build result
        let mut result = Vec::with_capacity(responses.len());
        for response in responses {
            match response {
                CommandResponse::Subscribed { sid, channel } => {
                    // Store the subscription
                    if let Some(channel_enum) = parse_channel(&channel) {
                        self.subscriptions.insert(
                            sid,
                            Subscription {
                                sid,
                                channel: channel_enum,
                                market_tickers: tickers.clone(),
                            },
                        );
                    }
                    result.push(SubscribeResponse { sid, channel });
                }
                CommandResponse::Error { code, msg } => {
                    return Err(KalshiError::InternalError(format!(
                        "Subscribe failed with code {}: {}",
                        code, msg
                    )));
                }
                CommandResponse::Ok { .. } => {
                    // Unexpected Ok response for subscribe, but not an error
                }
            }
        }

        Ok(result)
    }

    /// Unsubscribe from one or more subscriptions.
    ///
    /// Removes the specified subscriptions and waits for server confirmation.
    /// Also removes the subscriptions from the local tracking.
    ///
    /// # Arguments
    ///
    /// * `sids` - List of subscription IDs to unsubscribe from
    pub async fn unsubscribe(&mut self, sids: Vec<i32>) -> Result<(), KalshiError> {
        if sids.is_empty() {
            return Ok(());
        }

        let id = self.get_next_id();

        // Register for response
        let rx = self.register_pending_command(id);

        let cmd = serde_json::json!({
            "id": id,
            "cmd": "unsubscribe",
            "params": {
                "sids": sids
            }
        });

        self.send_command(cmd).await?;

        // Wait for confirmation
        let response = self.wait_for_response(rx).await?;

        match response {
            CommandResponse::Ok { .. } => {
                // Remove subscriptions from local tracking
                for sid in &sids {
                    self.subscriptions.remove(sid);
                }
                Ok(())
            }
            CommandResponse::Error { code, msg } => Err(KalshiError::InternalError(format!(
                "Unsubscribe failed with code {}: {}",
                code, msg
            ))),
            CommandResponse::Subscribed { .. } => {
                // Unexpected subscribed response, but not an error
                for sid in &sids {
                    self.subscriptions.remove(sid);
                }
                Ok(())
            }
        }
    }

    /// List all active subscriptions.
    ///
    /// Returns the locally tracked subscriptions. Note that this returns
    /// the subscriptions that have been tracked by this client instance.
    pub fn list_subscriptions(&self) -> Vec<Subscription> {
        self.subscriptions.values().cloned().collect()
    }

    /// Get a subscription by its SID.
    pub fn get_subscription(&self, sid: i32) -> Option<&Subscription> {
        self.subscriptions.get(&sid)
    }

    /// Update an existing subscription by adding or removing markets.
    ///
    /// # Arguments
    ///
    /// * `sids` - List of subscription IDs to update
    /// * `market_tickers` - List of market tickers to add or remove
    /// * `action` - Whether to add or remove the market tickers
    pub async fn update_subscription(
        &mut self,
        sids: Vec<i32>,
        market_tickers: Vec<String>,
        action: UpdateAction,
    ) -> Result<(), KalshiError> {
        if sids.is_empty() || market_tickers.is_empty() {
            return Ok(());
        }

        let id = self.get_next_id();

        // Register for response
        let rx = self.register_pending_command(id);

        let action_str = match action {
            UpdateAction::AddMarkets => "add_markets",
            UpdateAction::DeleteMarkets => "delete_markets",
        };

        let cmd = serde_json::json!({
            "id": id,
            "cmd": "update_subscription",
            "params": {
                "sids": sids,
                "market_tickers": market_tickers,
                "action": action_str
            }
        });

        self.send_command(cmd).await?;

        // Wait for confirmation
        let response = self.wait_for_response(rx).await?;

        match response {
            CommandResponse::Ok { .. } => {
                // Update local subscription tracking
                for sid in &sids {
                    if let Some(sub) = self.subscriptions.get_mut(sid) {
                        match action {
                            UpdateAction::AddMarkets => {
                                for ticker in &market_tickers {
                                    if !sub.market_tickers.contains(ticker) {
                                        sub.market_tickers.push(ticker.clone());
                                    }
                                }
                            }
                            UpdateAction::DeleteMarkets => {
                                sub.market_tickers.retain(|t| !market_tickers.contains(t));
                            }
                        }
                    }
                }
                Ok(())
            }
            CommandResponse::Error { code, msg } => Err(KalshiError::InternalError(format!(
                "Update subscription failed with code {}: {}",
                code, msg
            ))),
            CommandResponse::Subscribed { .. } => {
                // Unexpected subscribed response, but not an error
                Ok(())
            }
        }
    }
}
