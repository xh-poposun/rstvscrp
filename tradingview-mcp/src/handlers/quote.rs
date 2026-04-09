use crate::client::{TradingViewMcpClient, ClientError};
use crate::tools::{GetQuoteParams, GetQuoteResponse, SearchSymbolsParams, SearchSymbolsResponse};
use chrono::Utc;

/// Parameters for batch quote requests
#[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
pub struct GetBatchQuotesParams {
    /// List of stock symbols with exchange prefix (e.g., ["NASDAQ:AAPL", "NYSE:MSFT"])
    pub symbols: Vec<String>,
}

/// Response for batch quote requests
#[derive(Debug, Clone, serde::Serialize, schemars::JsonSchema)]
pub struct GetBatchQuotesResponse {
    pub quotes: Vec<GetQuoteResponse>,
}

/// Get current quote for a single symbol
pub async fn handle_get_quote(
    client: &TradingViewMcpClient,
    params: GetQuoteParams,
) -> Result<GetQuoteResponse, ClientError> {
    let quote = client.get_quote(&params.symbol).await?;

    Ok(GetQuoteResponse {
        symbol: quote.symbol,
        price: quote.price,
        change: quote.change,
        change_percent: quote.change_percent,
        volume: quote.volume,
        high: quote.high,
        low: quote.low,
        open: quote.open,
        previous_close: quote.previous_close,
        currency: "USD".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

/// Search for symbols by query string
pub async fn handle_search_symbols(
    client: &TradingViewMcpClient,
    params: SearchSymbolsParams,
) -> Result<SearchSymbolsResponse, ClientError> {
    let results = client.search_symbols(&params.query).await?;

    let symbols = results
        .into_iter()
        .map(|result| result.symbol)
        .collect();

    Ok(SearchSymbolsResponse { symbols })
}

/// Get quotes for multiple symbols in a batch
pub async fn handle_get_batch_quotes(
    client: &TradingViewMcpClient,
    params: GetBatchQuotesParams,
) -> Result<GetBatchQuotesResponse, ClientError> {
    let quotes = client.get_quotes(&params.symbols).await?;

    let quote_responses: Vec<GetQuoteResponse> = quotes
        .into_iter()
        .map(|quote| GetQuoteResponse {
            symbol: quote.symbol,
            price: quote.price,
            change: quote.change,
            change_percent: quote.change_percent,
            volume: quote.volume,
            high: quote.high,
            low: quote.low,
            open: quote.open,
            previous_close: quote.previous_close,
            currency: "USD".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
        .collect();

    Ok(GetBatchQuotesResponse {
        quotes: quote_responses,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_batch_quotes_params_schema() {
        let schema = schemars::schema_for!(GetBatchQuotesParams);
        assert!(schema.schema.properties.contains_key("symbols"));
    }

    #[test]
    fn test_get_batch_quotes_response_schema() {
        let schema = schemars::schema_for!(GetBatchQuotesResponse);
        assert!(schema.schema.properties.contains_key("quotes"));
    }
}
