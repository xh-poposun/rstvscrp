use axum::{
    Json, Router, 
    extract::{Path, Query},
    routing::{get, post},
};
use chrono::NaiveDate;
use sqlx::Row;

use crate::api::monitors::{get_db_pool, AppState};
use crate::error::{Error, Result};
use crate::models::HistoricalBar;

#[derive(Debug, serde::Deserialize)]
pub struct HistoryQuery {
    pub from: String,
    pub to: String,
}

pub fn router(_state: AppState) -> Router<()> {
    Router::new()
        .route("/api/v1/history/:symbol", get(query_history))
}

fn parse_date(s: &str) -> Result<i64> {
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| Error::InvalidInput(format!("invalid date format: {}", s)))?;
    Ok(date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())
}

async fn query_history(
    Path(symbol): Path<String>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<HistoricalBar>>> {
    let from_ts = parse_date(&query.from)?;
    let to_ts = parse_date(&query.to)?;
    
    let pool = get_db_pool();
    
    let rows = sqlx::query(
        "SELECT timestamp, open, high, low, close, volume FROM price_history 
         WHERE symbol = ? AND timestamp >= ? AND timestamp <= ?
         ORDER BY timestamp ASC"
    )
    .bind(&symbol)
    .bind(from_ts)
    .bind(to_ts)
    .fetch_all(pool.as_ref())
    .await?;
    
    let result: Vec<HistoricalBar> = rows
        .into_iter()
        .map(|row| HistoricalBar {
            timestamp: row.get(0),
            open: row.get(1),
            high: row.get(2),
            low: row.get(3),
            close: row.get(4),
            volume: row.get::<i64, _>(5) as u64,
        })
        .collect();
    
    Ok(Json(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_valid() {
        let ts = parse_date("2024-01-01").unwrap();
        assert!(ts > 0);
    }

    #[test]
    fn test_parse_date_invalid() {
        assert!(parse_date("invalid").is_err());
    }

    #[test]
    fn test_parse_date_edge_cases() {
        assert!(parse_date("2024-12-31").is_ok());
        assert!(parse_date("2020-02-29").is_ok());
        assert!(parse_date("not-a-date").is_err());
    }

    #[test]
    fn test_parse_date_wrong_format() {
        assert!(parse_date("01-01-2024").is_err());
        assert!(parse_date("2024/01/01").is_err());
    }
}
