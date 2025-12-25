use super::{Channel, KalshiWebSocket};
use crate::kalshi_error::KalshiError;
use serde::{Deserialize, Serialize};

/// Represents an active subscription.
#[derive(Debug, Clone)]
pub struct Subscription {
    pub sid: i32,
    pub channel: Channel,
    pub market_tickers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
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

impl KalshiWebSocket {
    /// Subscribe to one or more channels for specified markets.
    pub async fn subscribe(
        &mut self,
        channels: Vec<Channel>,
        market_ticker: Option<String>,
        market_tickers: Option<Vec<String>>,
    ) -> Result<Vec<SubscribeResponse>, KalshiError> {
        let id = self.get_next_id();

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
        if let Some(tickers) = market_tickers {
            cmd["params"]["market_tickers"] = serde_json::Value::Array(
                tickers.into_iter().map(serde_json::Value::String).collect(),
            );
        }

        self.send_command(cmd).await?;
        Ok(vec![])
    }

    /// Unsubscribe from one or more subscriptions.
    pub async fn unsubscribe(&mut self, sids: Vec<i32>) -> Result<(), KalshiError> {
        let id = self.get_next_id();

        let cmd = serde_json::json!({
            "id": id,
            "cmd": "unsubscribe",
            "params": {
                "sids": sids
            }
        });

        self.send_command(cmd).await
    }

    /// List all active subscriptions.
    pub async fn list_subscriptions(&mut self) -> Result<Vec<Subscription>, KalshiError> {
        let id = self.get_next_id();

        let cmd = serde_json::json!({
            "id": id,
            "cmd": "list_subscriptions"
        });

        self.send_command(cmd).await?;
        Ok(self.subscriptions.values().cloned().collect())
    }

    /// Update an existing subscription by adding or removing markets.
    pub async fn update_subscription(
        &mut self,
        sids: Vec<i32>,
        market_tickers: Vec<String>,
        action: UpdateAction,
    ) -> Result<(), KalshiError> {
        let id = self.get_next_id();

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

        self.send_command(cmd).await
    }
}
