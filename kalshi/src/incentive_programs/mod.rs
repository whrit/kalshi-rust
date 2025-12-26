use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves volume incentive programs.
    ///
    /// This method fetches information about active volume incentive programs,
    /// including trading rebates and market maker incentives.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<VolumeIncentive>)`: A vector of volume incentive programs on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let incentives = kalshi_instance.get_volume_incentives().await.unwrap();
    /// for incentive in incentives {
    ///     println!("Program: {}, Rate: {}", incentive.program_name, incentive.rebate_rate);
    /// }
    /// ```
    ///
    pub async fn get_volume_incentives(&self) -> Result<Vec<VolumeIncentive>, KalshiError> {
        let path = "/incentive_programs";
        let res: IncentiveProgramsResponse = self.signed_get(path).await?;
        Ok(res.programs)
    }
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct IncentiveProgramsResponse {
    programs: Vec<VolumeIncentive>,
}

// -------- Public models --------

/// Represents a volume incentive program.
#[derive(Debug, Deserialize, Serialize)]
pub struct VolumeIncentive {
    /// The name of the incentive program.
    pub program_name: String,
    /// The rebate rate for this program.
    pub rebate_rate: f64,
    /// Program start date.
    pub start_date: Option<String>,
    /// Program end date.
    pub end_date: Option<String>,
    /// Eligibility criteria and additional details.
    #[serde(flatten)]
    pub details: std::collections::HashMap<String, serde_json::Value>,
}
