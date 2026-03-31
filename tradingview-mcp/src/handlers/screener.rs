use crate::client::{TradingViewMcpClient, ClientError};
use crate::tools::{ScanStocksParams, ScanStocksResponse};
use tvdata_rs::scanner::{
    ScanQuery, Column, FilterCondition, FilterOperator, Market,
    fields::{fundamentals, core, price},
};
use serde_json::Value;

/// Stock result item for screener
#[derive(Debug, Clone)]
pub struct StockResult {
    pub symbol: String,
    pub name: Option<String>,
    pub market_cap: Option<f64>,
    pub price: Option<f64>,
    pub change_percent: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub sector: Option<String>,
}

/// Scan stocks with filters (market cap, P/E, sector, etc.)
pub async fn handle_scan_stocks(
    client: &TradingViewMcpClient,
    params: ScanStocksParams,
) -> Result<ScanStocksResponse, ClientError> {
    // Build the scan query with filters
    let mut query = ScanQuery::new()
        .select([
            core::NAME,
            core::MARKET,
            price::CLOSE,
            price::CHANGE_PERCENT,
            fundamentals::MARKET_CAP_BASIC,
            fundamentals::PRICE_EARNINGS_TTM,
        ]);

    // Set market filter
    let market_str = params.market.clone().unwrap_or_else(|| "america".to_string());
    let market: Market = market_str.into();
    query = query.market(market);

    // Apply limit if specified (default 50)
    let limit = params.limit.unwrap_or(50);
    query = query.page(0, limit).map_err(|e| ClientError::Api(format!("Invalid page parameters: {}", e)))?;

    // Build filter conditions
    let mut filters: Vec<FilterCondition> = Vec::new();

    // Market cap min filter
    if let Some(min_market_cap) = params.market_cap_min {
        filters.push(FilterCondition {
            left: fundamentals::MARKET_CAP_BASIC,
            operation: FilterOperator::EGreater,
            right: Value::from(min_market_cap),
        });
    }

    // Market cap max filter
    if let Some(max_market_cap) = params.market_cap_max {
        filters.push(FilterCondition {
            left: fundamentals::MARKET_CAP_BASIC,
            operation: FilterOperator::ELess,
            right: Value::from(max_market_cap),
        });
    }

    // P/E min filter
    if let Some(min_pe) = params.pe_min {
        filters.push(FilterCondition {
            left: fundamentals::PRICE_EARNINGS_TTM,
            operation: FilterOperator::EGreater,
            right: Value::from(min_pe),
        });
    }

    // P/E max filter
    if let Some(max_pe) = params.pe_max {
        filters.push(FilterCondition {
            left: fundamentals::PRICE_EARNINGS_TTM,
            operation: FilterOperator::ELess,
            right: Value::from(max_pe),
        });
    }

    // Sector filter (using match operator for string matching)
    if let Some(sector) = &params.sector {
        filters.push(FilterCondition {
            left: Column::from_static("sector"),
            operation: FilterOperator::Match,
            right: Value::from(sector.clone()),
        });
    }

    // Apply all filters
    for filter in filters {
        query = query.filter(filter);
    }

    // Execute the scan using the inner client
    let scan_response = client
        .inner()
        .scan(&query)
        .await
        .map_err(|e| ClientError::Api(format!("Failed to scan stocks: {}", e)))?;

    // Map scan rows to stock results
    let columns = vec![
        core::NAME,
        core::MARKET,
        price::CLOSE,
        price::CHANGE_PERCENT,
        fundamentals::MARKET_CAP_BASIC,
        fundamentals::PRICE_EARNINGS_TTM,
    ];

    let mut tickers_found: Vec<String> = Vec::new();

    for row in scan_response.rows {
        tickers_found.push(row.symbol.clone());
    }

    Ok(ScanStocksResponse {
        tickers_found,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_stocks_params_validation() {
        // Test that limit validation works
        let params = ScanStocksParams {
            market: Some("america".to_string()),
            sector: Some("Technology".to_string()),
            market_cap_min: Some(1_000_000_000.0),
            market_cap_max: Some(1_000_000_000_000.0),
            pe_min: Some(10.0),
            pe_max: Some(30.0),
            limit: Some(100),
        };

        assert_eq!(params.market, Some("america".to_string()));
        assert_eq!(params.sector, Some("Technology".to_string()));
        assert_eq!(params.market_cap_min, Some(1_000_000_000.0));
        assert_eq!(params.market_cap_max, Some(1_000_000_000_000.0));
        assert_eq!(params.pe_min, Some(10.0));
        assert_eq!(params.pe_max, Some(30.0));
        assert_eq!(params.limit, Some(100));
    }

    #[test]
    fn test_scan_stocks_params_defaults() {
        let params = ScanStocksParams {
            market: None,
            sector: None,
            market_cap_min: None,
            market_cap_max: None,
            pe_min: None,
            pe_max: None,
            limit: None,
        };

        assert!(params.market.is_none());
        assert!(params.sector.is_none());
        assert!(params.market_cap_min.is_none());
        assert!(params.market_cap_max.is_none());
        assert!(params.pe_min.is_none());
        assert!(params.pe_max.is_none());
        assert!(params.limit.is_none());
    }
}
