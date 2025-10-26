use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves all API keys for the authenticated user.
    ///
    /// This method lists all API keys associated with your account,
    /// including their metadata but not the secret values.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<ApiKey>)`: A vector of API key information on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let keys = kalshi_instance.get_api_keys().await.unwrap();
    /// ```
    ///
    pub async fn get_api_keys(&self) -> Result<Vec<ApiKey>, KalshiError> {
        let path = "/api_keys";
        let res: ApiKeysResponse = self.signed_get(path).await?;
        Ok(res.keys)
    }

    /// Creates a new API key.
    ///
    /// This method generates a new API key for programmatic access.
    /// The secret will only be shown once during creation.
    ///
    /// # Arguments
    ///
    /// * `label` - A descriptive label for the API key.
    ///
    /// # Returns
    ///
    /// - `Ok(ApiKeyCreated)`: The created API key with its secret on successful creation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let new_key = kalshi_instance.create_api_key("My Trading Bot").await.unwrap();
    /// println!("Save this secret: {}", new_key.secret);
    /// ```
    ///
    pub async fn create_api_key(&self, label: &str) -> Result<ApiKeyCreated, KalshiError> {
        let path = "/api_keys";
        let body = CreateApiKeyRequest {
            label: label.to_string(),
        };
        self.signed_post(path, &body).await
    }

    /// Generates a new secret for an existing API key.
    ///
    /// This method rotates the secret for an existing API key.
    /// The old secret will be invalidated and a new one generated.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The UUID of the API key to regenerate.
    ///
    /// # Returns
    ///
    /// - `Ok(ApiKeySecret)`: The new API key secret on successful generation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let new_secret = kalshi_instance.generate_api_key("key-uuid").await.unwrap();
    /// ```
    ///
    pub async fn generate_api_key(&self, key_id: &str) -> Result<ApiKeySecret, KalshiError> {
        let path = format!("/api_keys/{}/generate", key_id);
        self.signed_post(&path, &()).await
    }

    /// Deletes an API key.
    ///
    /// This method permanently deletes an API key. The key will immediately
    /// stop working and cannot be recovered.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The UUID of the API key to delete.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: Success confirmation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// kalshi_instance.delete_api_key("key-uuid").await.unwrap();
    /// ```
    ///
    pub async fn delete_api_key(&self, key_id: &str) -> Result<(), KalshiError> {
        let path = format!("/api_keys/{}", key_id);
        let _res: DeleteApiKeyResponse = self.signed_delete(&path).await?;
        Ok(())
    }
}

// -------- Request bodies --------

#[derive(Debug, Serialize)]
struct CreateApiKeyRequest {
    label: String,
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct ApiKeysResponse {
    keys: Vec<ApiKey>,
}

#[derive(Debug, Deserialize)]
struct DeleteApiKeyResponse {
    // Empty response or success message
}

// -------- Public models --------

/// Represents an API key (without the secret).
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiKey {
    /// The unique identifier for the API key.
    pub key_id: String,
    /// The descriptive label for the API key.
    pub label: String,
    /// The creation timestamp.
    pub created_time: String,
    /// Whether the key is currently active.
    pub is_active: bool,
}

/// Represents a newly created API key with its secret.
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiKeyCreated {
    /// The unique identifier for the API key.
    pub key_id: String,
    /// The descriptive label for the API key.
    pub label: String,
    /// The API key secret (only shown once).
    pub secret: String,
    /// The creation timestamp.
    pub created_time: String,
}

/// Represents a regenerated API key secret.
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiKeySecret {
    /// The API key secret.
    pub secret: String,
}

