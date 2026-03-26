use axum::{Json, Router, extract::Query, routing::get};

use serde::Deserialize;

use crate::api::monitors::{get_config, AppState};
use crate::error::{Error, Result};
use crate::tvclient::TvClient;
use tvdata_rs::SearchRequest;

pub fn router(_state: AppState) -> Router<()> {
    Router::new().route("/api/v1/search", get(search))
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String,
    #[serde(default = "default_instrument_type")]
    #[serde(rename = "type")]
    instrument_type: String,
}

fn default_instrument_type() -> String {
    "equity".to_string()
}

#[derive(Debug, serde::Serialize)]
struct SearchResult {
    symbol: String,
    description: Option<String>,
    exchange: Option<String>,
    #[serde(rename = "instrument_type")]
    instrument_type: Option<String>,
}

async fn search(Query(query): Query<SearchQuery>) -> Result<Json<Vec<SearchResult>>> {
    let query_text = query.q.trim();
    if query_text.is_empty() {
        return Err(Error::InvalidInput(
            "search query cannot be empty".to_string(),
        ));
    }

    let config = get_config();
    let language = config.search.language.clone();

    let client = TvClient::new()
        .await
        .map_err(|e| Error::TradingView(e.to_string()))?;

    // Build search request with language and instrument type
    let instrument_type = match query.instrument_type.as_str() {
        "forex" => "forex",
        "crypto" => "crypto",
        _ => "stock",  // equity maps to stock in TV API
    };

    let request = SearchRequest::builder()
        .text(query_text)
        .language(language)
        .instrument_type(instrument_type)
        .build();

    let hits = client
        .search(&request)
        .await
        .map_err(|e| Error::TradingView(e.to_string()))?;

    let results: Vec<SearchResult> = hits
        .into_iter()
        .map(|hit| SearchResult {
            symbol: hit.symbol,
            description: hit.description,
            exchange: hit.exchange,
            instrument_type: hit.instrument_type,
        })
        .collect();

    Ok(Json(results))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Query;

    #[test]
    fn test_search_query_with_type() {
        let query = SearchQuery {
            q: "aapl".to_string(),
            instrument_type: "forex".to_string(),
        };
        assert_eq!(query.q, "aapl");
        assert_eq!(query.instrument_type, "forex");
    }

    #[test]
    fn test_search_query_without_type_uses_default() {
        let query = SearchQuery {
            q: "aapl".to_string(),
            instrument_type: default_instrument_type(),
        };
        assert_eq!(query.q, "aapl");
        assert_eq!(query.instrument_type, "equity");
    }

    #[test]
    fn test_search_query_invalid_type_preserved() {
        let query = SearchQuery {
            q: "aapl".to_string(),
            instrument_type: "invalid".to_string(),
        };
        assert_eq!(query.q, "aapl");
        assert_eq!(query.instrument_type, "invalid");
    }

    #[test]
    fn test_search_query_empty_query() {
        let query = SearchQuery {
            q: "".to_string(),
            instrument_type: "equity".to_string(),
        };
        assert!(query.q.is_empty());
    }

    #[test]
    fn test_search_query_crypto_type() {
        let query = SearchQuery {
            q: "btc".to_string(),
            instrument_type: "crypto".to_string(),
        };
        assert_eq!(query.q, "btc");
        assert_eq!(query.instrument_type, "crypto");
    }

    #[test]
    fn test_search_query_equity_type() {
        let query = SearchQuery {
            q: "tesla".to_string(),
            instrument_type: "equity".to_string(),
        };
        assert_eq!(query.q, "tesla");
        assert_eq!(query.instrument_type, "equity");
    }

    #[tokio::test]
    async fn test_search_empty_query_returns_error() {
        let query = SearchQuery {
            q: "".to_string(),
            instrument_type: "equity".to_string(),
        };
        let result = search(Query(query)).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[tokio::test]
    async fn test_search_whitespace_query_returns_error() {
        let query = SearchQuery {
            q: "   ".to_string(),
            instrument_type: "equity".to_string(),
        };
        let result = search(Query(query)).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_search_live() {}
}
