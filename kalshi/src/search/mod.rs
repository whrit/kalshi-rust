use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Kalshi {
    /// Retrieves tags organized by series categories.
    ///
    /// This method returns a mapping of series categories to their associated tags,
    /// which can be used for filtering and search functionality.
    ///
    /// # Returns
    ///
    /// - `Ok(HashMap<String, Vec<String>>)`: A map of categories to their tags on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let tags = kalshi_instance.get_tags_by_categories().await.unwrap();
    /// for (category, tag_list) in tags {
    ///     println!("Category {}: {:?}", category, tag_list);
    /// }
    /// ```
    ///
    pub async fn get_tags_by_categories(&self) -> Result<HashMap<String, Vec<String>>, KalshiError> {
        let path = "/search/tags_by_categories";
        let res: TagsResponse = self.signed_get(path).await?;
        Ok(res.tags_by_category)
    }

    /// Retrieves available filters for sports markets.
    ///
    /// This method returns filter options that can be used to search
    /// and filter sports-related markets.
    ///
    /// # Returns
    ///
    /// - `Ok(SportsFilters)`: The available sports filters on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let filters = kalshi_instance.get_sports_filters().await.unwrap();
    /// ```
    ///
    pub async fn get_sports_filters(&self) -> Result<SportsFilters, KalshiError> {
        let path = "/search/sports_filters";
        self.signed_get(path).await
    }
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct TagsResponse {
    tags_by_category: HashMap<String, Vec<String>>,
}

// -------- Public models --------

/// Represents available filters for sports markets.
#[derive(Debug, Deserialize, Serialize)]
pub struct SportsFilters {
    /// List of available sports.
    pub sports: Vec<String>,
    /// List of available leagues.
    pub leagues: Vec<String>,
    /// List of available teams.
    pub teams: Option<Vec<String>>,
    /// Additional filter fields.
    #[serde(flatten)]
    pub additional_filters: HashMap<String, serde_json::Value>,
}

