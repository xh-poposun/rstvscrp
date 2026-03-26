use axum::{Json, Router, extract::Path, routing::get};

use crate::api::monitors::AppState;
use crate::error::Result;
use crate::tvclient::TvClient;

pub fn router(_state: AppState) -> Router<()> {
    Router::new().route("/api/v1/quotes/:symbol", get(get_quote))
}

async fn get_quote(Path(symbol): Path<String>) -> Result<Json<serde_json::Value>> {
    let client = TvClient::new()
        .await
        .map_err(|e| crate::error::Error::TradingView(e.to_string()))?;

    let quotes = client
        .get_quotes(&[&symbol])
        .await
        .map_err(|e| crate::error::Error::TradingView(e.to_string()))?;

    let quote = quotes
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::Error::NotFound(format!("quote for {} not found", symbol)))?;

    Ok(Json(serde_json::json!({
        "symbol": quote.symbol,
        "price": quote.price,
        "change": quote.change,
        "change_percent": quote.change_percent,
        "volume": quote.volume,
        "high": quote.high,
        "low": quote.low,
        "open": quote.open,
        "previous_close": quote.previous_close,
    })))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[ignore]
    async fn test_get_quote_live() {}
}
