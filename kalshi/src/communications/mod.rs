use super::Kalshi;
use crate::kalshi_error::*;
use crate::Side;
use serde::{Deserialize, Serialize};

impl Kalshi {
    // ========== Task 2.3: get_communications_id() ==========

    /// Retrieves the user's public communications ID.
    ///
    /// This ID is used to identify the user in communications with other traders.
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: The user's communications ID on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let comm_id = kalshi_instance.get_communications_id().await.unwrap();
    /// println!("Communications ID: {}", comm_id);
    /// ```
    ///
    pub async fn get_communications_id(&self) -> Result<String, KalshiError> {
        let path = "/communications/id";
        let res: CommunicationsIdResponse = self.signed_get(path).await?;
        Ok(res.communications_id)
    }

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

    // ========== Task 3.3: get_rfqs() with pagination ==========

    /// Retrieves RFQs (Requests for Quote) with optional filtering and pagination.
    ///
    /// This method lists RFQs that the user has created or received, with support
    /// for filtering by various parameters and pagination.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor from previous request.
    /// * `event_ticker` - Filter by event ticker.
    /// * `market_ticker` - Filter by market ticker.
    /// * `limit` - Number of results per page (default 100, max 100).
    /// * `status` - Filter by RFQ status.
    /// * `creator_user_id` - Filter by creator user ID.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Rfq>))`: A tuple containing the next cursor (if any) and a vector of RFQs.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, rfqs) = kalshi_instance.get_rfqs(
    ///     None,        // cursor
    ///     None,        // event_ticker
    ///     None,        // market_ticker
    ///     Some(10),    // limit
    ///     None,        // status
    ///     None,        // creator_user_id
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_rfqs(
        &self,
        cursor: Option<String>,
        event_ticker: Option<String>,
        market_ticker: Option<String>,
        limit: Option<i32>,
        status: Option<String>,
        creator_user_id: Option<String>,
    ) -> Result<(Option<String>, Vec<Rfq>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6);
        add_param!(params, "cursor", cursor);
        add_param!(params, "event_ticker", event_ticker);
        add_param!(params, "market_ticker", market_ticker);
        add_param!(params, "limit", limit);
        add_param!(params, "status", status);
        add_param!(params, "creator_user_id", creator_user_id);

        let path = if params.is_empty() {
            "/communications/rfqs".to_string()
        } else {
            let qs = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("/communications/rfqs?{}", qs)
        };

        let res: RfqsResponse = self.signed_get(&path).await?;
        Ok((res.cursor, res.rfqs))
    }

    // ========== Task 3.1: create_rfq() ==========

    /// Creates a new RFQ (Request for Quote).
    ///
    /// This method submits a new request for quote to market makers or other traders.
    ///
    /// # Arguments
    ///
    /// * `market_ticker` - The market ticker to request a quote for.
    /// * `rest_remainder` - Whether to rest the remainder after execution.
    /// * `contracts` - Number of contracts for the RFQ (optional).
    /// * `target_cost_centi_cents` - Target cost in centi-cents (optional).
    /// * `replace_existing` - Whether to delete existing RFQs (default: false).
    /// * `subtrader_id` - Subtrader ID (FCM members only).
    ///
    /// # Returns
    ///
    /// - `Ok(CreateRfqResponse)`: The created RFQ response containing the new RFQ ID.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let response = kalshi_instance.create_rfq(
    ///     "MARKET-TICKER",
    ///     false,
    ///     Some(100),
    ///     None,
    ///     None,
    ///     None,
    /// ).await.unwrap();
    /// println!("Created RFQ with ID: {}", response.id);
    /// ```
    ///
    pub async fn create_rfq(
        &self,
        market_ticker: &str,
        rest_remainder: bool,
        contracts: Option<i32>,
        target_cost_centi_cents: Option<i64>,
        replace_existing: Option<bool>,
        subtrader_id: Option<String>,
    ) -> Result<CreateRfqResponse, KalshiError> {
        let path = "/communications/rfqs";
        let body = CreateRfqRequest {
            market_ticker: market_ticker.to_string(),
            rest_remainder,
            contracts,
            target_cost_centi_cents,
            replace_existing,
            subtrader_id,
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

    // ========== Task 3.3: get_quotes() with pagination ==========

    /// Retrieves quotes with optional filtering and pagination.
    ///
    /// This method lists quotes that the user has created or received, with support
    /// for filtering by various parameters and pagination.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor from previous request.
    /// * `event_ticker` - Filter by event ticker.
    /// * `market_ticker` - Filter by market ticker.
    /// * `limit` - Number of results per page (default 500, max 500).
    /// * `status` - Filter by quote status.
    /// * `quote_creator_user_id` - Filter by quote creator user ID.
    /// * `rfq_creator_user_id` - Filter by RFQ creator user ID.
    /// * `rfq_id` - Filter by RFQ ID.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Quote>))`: A tuple containing the next cursor (if any) and a vector of quotes.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, quotes) = kalshi_instance.get_quotes(
    ///     None,        // cursor
    ///     None,        // event_ticker
    ///     None,        // market_ticker
    ///     Some(10),    // limit
    ///     None,        // status
    ///     None,        // quote_creator_user_id
    ///     None,        // rfq_creator_user_id
    ///     None,        // rfq_id
    /// ).await.unwrap();
    /// ```
    ///
    #[allow(clippy::too_many_arguments)]
    pub async fn get_quotes(
        &self,
        cursor: Option<String>,
        event_ticker: Option<String>,
        market_ticker: Option<String>,
        limit: Option<i32>,
        status: Option<String>,
        quote_creator_user_id: Option<String>,
        rfq_creator_user_id: Option<String>,
        rfq_id: Option<String>,
    ) -> Result<(Option<String>, Vec<Quote>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8);
        add_param!(params, "cursor", cursor);
        add_param!(params, "event_ticker", event_ticker);
        add_param!(params, "market_ticker", market_ticker);
        add_param!(params, "limit", limit);
        add_param!(params, "status", status);
        add_param!(params, "quote_creator_user_id", quote_creator_user_id);
        add_param!(params, "rfq_creator_user_id", rfq_creator_user_id);
        add_param!(params, "rfq_id", rfq_id);

        let path = if params.is_empty() {
            "/communications/quotes".to_string()
        } else {
            let qs = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("/communications/quotes?{}", qs)
        };

        let res: QuotesResponse = self.signed_get(&path).await?;
        Ok((res.cursor, res.quotes))
    }

    // ========== Task 3.2: create_quote() ==========

    /// Creates a new quote in response to an RFQ.
    ///
    /// This method submits a quote offer to an RFQ requestor.
    ///
    /// # Arguments
    ///
    /// * `rfq_id` - The RFQ ID this quote responds to.
    /// * `yes_bid` - Bid price for YES contracts in dollars ("0.5600" format).
    /// * `no_bid` - Bid price for NO contracts in dollars ("0.5600" format).
    /// * `rest_remainder` - Whether to rest the remainder after execution.
    ///
    /// # Returns
    ///
    /// - `Ok(CreateQuoteResponse)`: The created quote response containing the new quote ID.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let response = kalshi_instance.create_quote(
    ///     "rfq-123",
    ///     "0.5000",
    ///     "0.5000",
    ///     false,
    /// ).await.unwrap();
    /// println!("Created quote with ID: {}", response.id);
    /// ```
    ///
    pub async fn create_quote(
        &self,
        rfq_id: &str,
        yes_bid: &str,
        no_bid: &str,
        rest_remainder: bool,
    ) -> Result<CreateQuoteResponse, KalshiError> {
        let path = "/communications/quotes";
        let body = CreateQuoteRequest {
            rfq_id: rfq_id.to_string(),
            yes_bid: yes_bid.to_string(),
            no_bid: no_bid.to_string(),
            rest_remainder,
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

    // ========== Task 3.4: accept_quote() with accepted_side ==========

    /// Accepts a quote.
    ///
    /// This method accepts a quote offer, which will execute the trade.
    ///
    /// # Arguments
    ///
    /// * `quote_id` - The quote ID to accept.
    /// * `accepted_side` - Which side to accept (Yes or No).
    ///
    /// # Returns
    ///
    /// - `Ok(())`: Success confirmation (API returns 204 No Content).
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// use kalshi::Side;
    ///
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// kalshi_instance.accept_quote("quote-123", Side::Yes).await.unwrap();
    /// ```
    ///
    pub async fn accept_quote(
        &self,
        quote_id: &str,
        accepted_side: Side,
    ) -> Result<(), KalshiError> {
        let path = format!("/communications/quotes/{}/accept", quote_id);
        let body = AcceptQuoteRequest { accepted_side };
        let _: serde_json::Value = self.signed_put(&path, Some(&body)).await?;
        Ok(())
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

#[derive(Debug, Deserialize)]
struct CommunicationsIdResponse {
    communications_id: String,
}

#[derive(Debug, Serialize)]
struct CreateRfqRequest {
    market_ticker: String,
    rest_remainder: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    contracts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_cost_centi_cents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_existing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtrader_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreateQuoteRequest {
    rfq_id: String,
    yes_bid: String,
    no_bid: String,
    rest_remainder: bool,
}

#[derive(Debug, Serialize)]
struct AcceptQuoteRequest {
    accepted_side: Side,
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct RfqsResponse {
    rfqs: Vec<Rfq>,
    cursor: Option<String>,
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
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    quote: Quote,
}

#[derive(Debug, Deserialize)]
struct DeleteQuoteResponse {}

// -------- Public models --------

/// Response from creating a new RFQ.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRfqResponse {
    /// The ID of the newly created RFQ.
    pub id: String,
}

/// Response from creating a new quote.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateQuoteResponse {
    /// The ID of the newly created quote.
    pub id: String,
}

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
    #[serde(alias = "ticker")]
    pub market_ticker: Option<String>,
    /// The desired quantity.
    #[serde(default)]
    pub contracts: Option<i32>,
    /// The side of the trade ("yes" or "no").
    pub side: Option<String>,
    /// Optional message with the RFQ.
    pub message: Option<String>,
    /// The status of the RFQ.
    pub status: Option<String>,
    /// Timestamp when created.
    pub created_time: Option<String>,
    /// Timestamp when expires.
    pub expires_time: Option<String>,
    /// Whether to rest the remainder.
    pub rest_remainder: Option<bool>,
    /// Target cost in centi-cents.
    pub target_cost_centi_cents: Option<i64>,
    /// Creator user ID.
    pub creator_user_id: Option<String>,
    /// Additional fields that may be returned by the API.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Represents a quote offer.
#[derive(Debug, Deserialize, Serialize)]
pub struct Quote {
    /// The quote ID.
    pub id: String,
    /// The RFQ this quote responds to.
    pub rfq_id: Option<String>,
    /// The quoted yes bid price in dollars.
    pub yes_bid: Option<String>,
    /// The quoted no bid price in dollars.
    pub no_bid: Option<String>,
    /// The quoted price in cents (legacy).
    pub price: Option<i32>,
    /// The quoted quantity (legacy).
    pub quantity: Option<i32>,
    /// The status of the quote.
    pub status: Option<String>,
    /// Timestamp when created.
    pub created_time: Option<String>,
    /// Timestamp when expires.
    pub expires_time: Option<String>,
    /// Whether to rest the remainder.
    pub rest_remainder: Option<bool>,
    /// Quote creator user ID.
    pub quote_creator_user_id: Option<String>,
    /// RFQ creator user ID.
    pub rfq_creator_user_id: Option<String>,
    /// Additional fields that may be returned by the API.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
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
