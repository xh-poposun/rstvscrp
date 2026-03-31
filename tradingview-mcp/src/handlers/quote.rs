use crate::client::{TradingViewMcpClient, ClientError};
use crate::tools::{GetQuoteParams, GetQuoteResponse, SearchSymbolsParams, SearchSymbolsResponse};

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
        price: Some(quote.price),
        change: Some(quote.change),
        change_percent: Some(quote.change_percent),
        volume: Some(quote.volume as f64),
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
            price: Some(quote.price),
            change: Some(quote.change),
            change_percent: Some(quote.change_percent),
            volume: Some(quote.volume as f64),
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
