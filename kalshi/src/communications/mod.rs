use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves a communication by ID.
    ///
    /// This method fetches a specific communication message or thread.
    ///
    /// # Arguments
    ///
    /// * `comm_id` - The communication ID to retrieve.
    ///
    /// # Returns
    ///
    /// - `Ok(Communication)`: The communication details on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let comm = kalshi_instance.get_communication("comm-123").await.unwrap();
    /// ```
    ///
    pub async fn get_communication(&self, comm_id: &str) -> Result<Communication, KalshiError> {
        let path = format!("/communications/{}", comm_id);
        self.signed_get(&path).await
    }

    /// Retrieves all RFQs (Requests for Quote) for the authenticated user.
    ///
    /// This method lists all RFQs that the user has created or received.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Rfq>)`: A vector of RFQs on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let rfqs = kalshi_instance.get_rfqs().await.unwrap();
    /// ```
    ///
    pub async fn get_rfqs(&self) -> Result<Vec<Rfq>, KalshiError> {
        let path = "/communications/rfqs";
        let res: RfqsResponse = self.signed_get(path).await?;
        Ok(res.rfqs)
    }

    /// Creates a new RFQ (Request for Quote).
    ///
    /// This method submits a new request for quote to market makers or other traders.
    ///
    /// # Arguments
    ///
    /// * `ticker` - The market ticker to request a quote for.
    /// * `quantity` - The desired quantity of contracts.
    /// * `side` - The side of the trade ("yes" or "no").
    /// * `message` - Optional message to include with the RFQ.
    ///
    /// # Returns
    ///
    /// - `Ok(Rfq)`: The created RFQ on successful creation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let rfq = kalshi_instance.create_rfq(
    ///     "MARKET-TICKER",
    ///     100,
    ///     "yes",
    ///     Some("Looking for best price")
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn create_rfq(
        &self,
        ticker: &str,
        quantity: i32,
        side: &str,
        message: Option<&str>,
    ) -> Result<Rfq, KalshiError> {
        let path = "/communications/rfqs";
        let body = CreateRfqRequest {
            ticker: ticker.to_string(),
            quantity,
            side: side.to_string(),
            message: message.map(|s| s.to_string()),
        };
        self.signed_post(path, &body).await
    }

    /// Retrieves a specific RFQ by ID.
    ///
    /// This method fetches detailed information about a specific RFQ.
    ///
    /// # Arguments
    ///
    /// * `rfq_id` - The RFQ ID to retrieve.
    ///
    /// # Returns
    ///
    /// - `Ok(Rfq)`: The RFQ details on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let rfq = kalshi_instance.get_rfq("rfq-123").await.unwrap();
    /// ```
    ///
    pub async fn get_rfq(&self, rfq_id: &str) -> Result<Rfq, KalshiError> {
        let path = format!("/communications/rfqs/{}", rfq_id);
        let res: RfqResponse = self.signed_get(&path).await?;
        Ok(res.rfq)
    }

    /// Deletes an RFQ.
    ///
    /// This method cancels and removes an RFQ. Only the creator can delete an RFQ.
    ///
    /// # Arguments
    ///
    /// * `rfq_id` - The RFQ ID to delete.
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
    /// kalshi_instance.delete_rfq("rfq-123").await.unwrap();
    /// ```
    ///
    pub async fn delete_rfq(&self, rfq_id: &str) -> Result<(), KalshiError> {
        let path = format!("/communications/rfqs/{}", rfq_id);
        let _res: DeleteRfqResponse = self.signed_delete(&path).await?;
        Ok(())
    }

    /// Retrieves all quotes for the authenticated user.
    ///
    /// This method lists all quotes that the user has created or received.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Quote>)`: A vector of quotes on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let quotes = kalshi_instance.get_quotes().await.unwrap();
    /// ```
    ///
    pub async fn get_quotes(&self) -> Result<Vec<Quote>, KalshiError> {
        let path = "/communications/quotes";
        let res: QuotesResponse = self.signed_get(path).await?;
        Ok(res.quotes)
    }

    /// Creates a new quote in response to an RFQ.
    ///
    /// This method submits a quote offer to an RFQ requestor.
    ///
    /// # Arguments
    ///
    /// * `rfq_id` - The RFQ ID this quote is responding to.
    /// * `price` - The quoted price in cents.
    /// * `quantity` - The quoted quantity of contracts.
    ///
    /// # Returns
    ///
    /// - `Ok(Quote)`: The created quote on successful creation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let quote = kalshi_instance.create_quote("rfq-123", 50, 100).await.unwrap();
    /// ```
    ///
    pub async fn create_quote(
        &self,
        rfq_id: &str,
        price: i32,
        quantity: i32,
    ) -> Result<Quote, KalshiError> {
        let path = "/communications/quotes";
        let body = CreateQuoteRequest {
            rfq_id: rfq_id.to_string(),
            price,
            quantity,
        };
        self.signed_post(path, &body).await
    }

    /// Retrieves a specific quote by ID.
    ///
    /// This method fetches detailed information about a specific quote.
    ///
    /// # Arguments
    ///
    /// * `quote_id` - The quote ID to retrieve.
    ///
    /// # Returns
    ///
    /// - `Ok(Quote)`: The quote details on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let quote = kalshi_instance.get_quote("quote-123").await.unwrap();
    /// ```
    ///
    pub async fn get_quote(&self, quote_id: &str) -> Result<Quote, KalshiError> {
        let path = format!("/communications/quotes/{}", quote_id);
        let res: QuoteResponse = self.signed_get(&path).await?;
        Ok(res.quote)
    }

    /// Deletes a quote.
    ///
    /// This method cancels and removes a quote. Only the creator can delete a quote.
    ///
    /// # Arguments
    ///
    /// * `quote_id` - The quote ID to delete.
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
    /// kalshi_instance.delete_quote("quote-123").await.unwrap();
    /// ```
    ///
    pub async fn delete_quote(&self, quote_id: &str) -> Result<(), KalshiError> {
        let path = format!("/communications/quotes/{}", quote_id);
        let _res: DeleteQuoteResponse = self.signed_delete(&path).await?;
        Ok(())
    }

    /// Accepts a quote.
    ///
    /// This method accepts a quote offer, which will execute the trade.
    ///
    /// # Arguments
    ///
    /// * `quote_id` - The quote ID to accept.
    ///
    /// # Returns
    ///
    /// - `Ok(QuoteAccepted)`: The accepted quote details on successful acceptance.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let result = kalshi_instance.accept_quote("quote-123").await.unwrap();
    /// ```
    ///
    pub async fn accept_quote(&self, quote_id: &str) -> Result<QuoteAccepted, KalshiError> {
        let path = format!("/communications/quotes/{}/accept", quote_id);
        self.signed_put(&path, None::<&()>).await
    }

    /// Confirms a quote.
    ///
    /// This method confirms a quote after acceptance, finalizing the transaction.
    ///
    /// # Arguments
    ///
    /// * `quote_id` - The quote ID to confirm.
    ///
    /// # Returns
    ///
    /// - `Ok(QuoteConfirmed)`: The confirmed quote details on successful confirmation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let result = kalshi_instance.confirm_quote("quote-123").await.unwrap();
    /// ```
    ///
    pub async fn confirm_quote(&self, quote_id: &str) -> Result<QuoteConfirmed, KalshiError> {
        let path = format!("/communications/quotes/{}/confirm", quote_id);
        self.signed_put(&path, None::<&()>).await
    }
}

// -------- Request bodies --------

#[derive(Debug, Serialize)]
struct CreateRfqRequest {
    ticker: String,
    quantity: i32,
    side: String,
    message: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreateQuoteRequest {
    rfq_id: String,
    price: i32,
    quantity: i32,
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct RfqsResponse {
    rfqs: Vec<Rfq>,
}

#[derive(Debug, Deserialize)]
struct RfqResponse {
    rfq: Rfq,
}

#[derive(Debug, Deserialize)]
struct DeleteRfqResponse {}

#[derive(Debug, Deserialize)]
struct QuotesResponse {
    quotes: Vec<Quote>,
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    quote: Quote,
}

#[derive(Debug, Deserialize)]
struct DeleteQuoteResponse {}

// -------- Public models --------

/// Represents a communication message or thread.
#[derive(Debug, Deserialize, Serialize)]
pub struct Communication {
    /// The communication ID.
    pub id: String,
    /// The communication type.
    #[serde(rename = "type")]
    pub comm_type: String,
    /// The message content.
    pub message: Option<String>,
    /// Timestamp when created.
    pub created_time: String,
    /// Additional fields.
    #[serde(flatten)]
    pub details: std::collections::HashMap<String, serde_json::Value>,
}

/// Represents an RFQ (Request for Quote).
#[derive(Debug, Deserialize, Serialize)]
pub struct Rfq {
    /// The RFQ ID.
    pub id: String,
    /// The market ticker requested.
    pub ticker: String,
    /// The desired quantity.
    pub quantity: i32,
    /// The side of the trade ("yes" or "no").
    pub side: String,
    /// Optional message with the RFQ.
    pub message: Option<String>,
    /// The status of the RFQ.
    pub status: String,
    /// Timestamp when created.
    pub created_time: String,
    /// Timestamp when expires.
    pub expires_time: Option<String>,
}

/// Represents a quote offer.
#[derive(Debug, Deserialize, Serialize)]
pub struct Quote {
    /// The quote ID.
    pub id: String,
    /// The RFQ this quote responds to.
    pub rfq_id: String,
    /// The quoted price in cents.
    pub price: i32,
    /// The quoted quantity.
    pub quantity: i32,
    /// The status of the quote.
    pub status: String,
    /// Timestamp when created.
    pub created_time: String,
    /// Timestamp when expires.
    pub expires_time: Option<String>,
}

/// Represents an accepted quote.
#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteAccepted {
    /// The quote ID.
    pub quote_id: String,
    /// The status after acceptance.
    pub status: String,
    /// The order ID if trade was executed.
    pub order_id: Option<String>,
}

/// Represents a confirmed quote.
#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteConfirmed {
    /// The quote ID.
    pub quote_id: String,
    /// The status after confirmation.
    pub status: String,
    /// The fill ID if trade was completed.
    pub fill_id: Option<String>,
}

