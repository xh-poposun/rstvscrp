use std::time::Duration;
use tokio::time::interval;

use crate::api::monitors::get_db_pool;
use crate::tvclient::TvClient;

const MAX_RETRIES: u32 = 3;
const BASE_DELAY_SECS: u64 = 2;

pub async fn start_history_refresh_task() {
    tracing::info!("Starting history refresh background task");
    
    let mut interval = interval(Duration::from_secs(86400)); // 24 hours
    
    loop {
        interval.tick().await;
        
        if let Err(e) = refresh_all_histories().await {
            tracing::error!("history refresh failed: {}", e);
        }
    }
}

async fn refresh_all_histories() -> Result<(), String> {
    let pool = get_db_pool();
    
    let monitors: Vec<(String,)> = sqlx::query_as(
        "SELECT symbol FROM monitors WHERE enabled = 1"
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| format!("failed to fetch monitors: {}", e))?;
    
    tracing::info!("refreshing history for {} monitors", monitors.len());
    
    for (symbol,) in monitors {
        let result = refresh_with_retry(&symbol, MAX_RETRIES, BASE_DELAY_SECS).await;
        
        match result {
            Ok(count) => {
                tracing::info!("refreshed {} bars for {}", count, symbol);
            }
            Err(e) => {
                tracing::error!("failed to refresh {} after {} retries: {}", 
                    symbol, MAX_RETRIES, e);
            }
        }
        
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    
    Ok(())
}

async fn refresh_with_retry(symbol: &str, max_retries: u32, base_delay: u64) -> Result<usize, String> {
    let mut last_error = String::new();
    
    for attempt in 1..=max_retries {
        match refresh_symbol_history(symbol).await {
            Ok(count) => return Ok(count),
            Err(e) => {
                last_error = e;
                if attempt < max_retries {
                    let delay = base_delay * 2u64.pow(attempt - 1);
                    tracing::warn!("retry {}/{} for {} in {}s: {}", 
                        attempt, max_retries, symbol, delay, last_error);
                    tokio::time::sleep(Duration::from_secs(delay)).await;
                }
            }
        }
    }
    
    Err(last_error)
}

async fn refresh_symbol_history(symbol: &str) -> Result<usize, String> {
    let client = TvClient::new().await
        .map_err(|e| format!("failed to create client: {}", e))?;
    
    let history = client.get_history(symbol, "D").await
        .map_err(|e| format!("failed to get history: {}", e))?;
    
    if history.is_empty() {
        return Ok(0);
    }
    
    let pool = get_db_pool();
    let mut count = 0;
    
    for bar in history {
        let result = sqlx::query(
            "INSERT OR IGNORE INTO price_history (symbol, timestamp, open, high, low, close, volume) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(symbol)
        .bind(bar.timestamp)
        .bind(bar.open)
        .bind(bar.high)
        .bind(bar.low)
        .bind(bar.close)
        .bind(bar.volume as i64)
        .execute(pool.as_ref())
        .await;
        
        if result.is_ok() {
            count += 1;
        }
    }
    
    Ok(count)
}