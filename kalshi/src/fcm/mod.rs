use super::Kalshi;
use crate::kalshi_error::*;
use crate::portfolio::Order;
use serde::Deserialize;

impl Kalshi {
    /// Retrieves FCM (Futures Commission Merchant) orders.
    ///
    /// This method is intended for use by FCM members only.
    /// It retrieves orders through the FCM interface.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Order>)`: A vector of FCM orders on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi` with FCM access
    /// let fcm_orders = kalshi_instance.get_fcm_orders().await.unwrap();
    /// ```
    ///
    /// # Note
    ///
    /// This endpoint is only available to users with FCM account access.
    ///
    pub async fn get_fcm_orders(&self) -> Result<Vec<Order>, KalshiError> {
        let path = "/fcm/orders";
        let res: FcmOrdersResponse = self.signed_get(path).await?;
        Ok(res.orders)
    }

    /// Retrieves FCM (Futures Commission Merchant) positions.
    ///
    /// This method is intended for use by FCM members only.
    /// It retrieves positions through the FCM interface.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<FcmPosition>)`: A vector of FCM positions on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi` with FCM access
    /// let fcm_positions = kalshi_instance.get_fcm_positions().await.unwrap();
    /// ```
    ///
    /// # Note
    ///
    /// This endpoint is only available to users with FCM account access.
    ///
    pub async fn get_fcm_positions(&self) -> Result<Vec<FcmPosition>, KalshiError> {
        let path = "/fcm/positions";
        let res: FcmPositionsResponse = self.signed_get(path).await?;
        Ok(res.positions)
    }
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct FcmOrdersResponse {
    orders: Vec<Order>,
}

#[derive(Debug, Deserialize)]
struct FcmPositionsResponse {
    positions: Vec<FcmPosition>,
}

// -------- Public models --------

use serde::Serialize;

/// Represents an FCM position (simplified version for FCM interface).
#[derive(Debug, Deserialize, Serialize)]
pub struct FcmPosition {
    /// The market ticker for this position.
    pub ticker: String,
    /// The number of contracts held.
    pub position: i32,
    /// Additional position details.
    #[serde(flatten)]
    pub details: std::collections::HashMap<String, serde_json::Value>,
}
