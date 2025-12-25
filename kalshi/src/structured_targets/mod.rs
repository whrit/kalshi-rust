use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves all structured targets.
    ///
    /// This method lists all available structured target markets.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<StructuredTarget>)`: A vector of structured targets on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let targets = kalshi_instance.get_structured_targets().await.unwrap();
    /// ```
    ///
    pub async fn get_structured_targets(&self) -> Result<Vec<StructuredTarget>, KalshiError> {
        let path = "/structured_targets";
        let res: StructuredTargetsResponse = self.signed_get(path).await?;
        Ok(res.targets)
    }

    /// Retrieves a specific structured target by ID.
    ///
    /// This method fetches detailed information about a specific structured target market.
    ///
    /// # Arguments
    ///
    /// * `target_id` - The ID of the structured target to retrieve.
    ///
    /// # Returns
    ///
    /// - `Ok(StructuredTarget)`: The structured target details on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let target = kalshi_instance.get_structured_target("target-123").await.unwrap();
    /// ```
    ///
    pub async fn get_structured_target(
        &self,
        target_id: &str,
    ) -> Result<StructuredTarget, KalshiError> {
        let path = format!("/structured_targets/{}", target_id);
        let res: StructuredTargetResponse = self.signed_get(&path).await?;
        Ok(res.target)
    }
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct StructuredTargetsResponse {
    targets: Vec<StructuredTarget>,
}

#[derive(Debug, Deserialize)]
struct StructuredTargetResponse {
    target: StructuredTarget,
}

// -------- Public models --------

/// Represents a structured target market.
#[derive(Debug, Deserialize, Serialize)]
pub struct StructuredTarget {
    /// The unique identifier for the structured target.
    pub id: String,
    /// The title or description of the target.
    pub title: String,
    /// Additional target details.
    #[serde(flatten)]
    pub details: std::collections::HashMap<String, serde_json::Value>,
}
