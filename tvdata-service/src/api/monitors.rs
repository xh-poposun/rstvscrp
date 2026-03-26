use once_cell::sync::OnceCell;
use std::sync::Arc;

static DB_POOL: OnceCell<Arc<sqlx::SqlitePool>> = OnceCell::new();
static CONFIG: OnceCell<crate::config::Config> = OnceCell::new();

pub fn set_db_pool(pool: Arc<sqlx::SqlitePool>) {
    DB_POOL.set(pool).ok();
}

pub fn get_db_pool() -> &'static Arc<sqlx::SqlitePool> {
    DB_POOL.get().expect("DB pool not initialized")
}

pub fn set_config(config: crate::config::Config) {
    CONFIG.set(config).ok();
}

pub fn get_config() -> &'static crate::config::Config {
    CONFIG.get().expect("Config not initialized")
}

#[allow(unused_imports)]
use axum::routing::{delete, post, put};
use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use chrono::Utc;

use crate::error::{Error, Result};
use crate::models::{CreateMonitorRequest, Monitor, PaginatedResponse, UpdateMonitorRequest};
use crate::tvclient::TvClient;

#[derive(Clone)]
pub struct AppState;

pub fn router(_state: AppState) -> Router<()> {
    Router::new()
        .route("/api/v1/monitors", get(list_monitors).post(create_monitor))
        .route(
            "/api/v1/monitors/:id",
            get(get_monitor).put(update_monitor).delete(delete_monitor),
        )
}

async fn list_monitors() -> Result<Json<PaginatedResponse<Monitor>>> {
    let db = get_db_pool();
    let monitors = sqlx::query_as::<_, Monitor>(
        "SELECT id, symbol, name, enabled, created_at, updated_at FROM monitors ORDER BY created_at DESC",
    )
    .fetch_all(db.as_ref())
    .await?;

    let total = monitors.len() as i64;
    Ok(Json(PaginatedResponse {
        data: monitors,
        page: 1,
        page_size: total,
        total,
    }))
}

async fn create_monitor(Json(req): Json<CreateMonitorRequest>) -> Result<impl IntoResponse> {
    if req.symbol.is_empty() {
        return Err(Error::InvalidInput("symbol cannot be empty".to_string()));
    }

    let config = get_config();
    let db = get_db_pool();

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM monitors")
        .fetch_one(db.as_ref())
        .await?;

    if count.0 >= config.monitor.max_monitors as i64 {
        return Err(Error::InvalidInput(format!(
            "monitor limit reached (max: {})",
            config.monitor.max_monitors
        )));
    }

    let client = TvClient::new().await.map_err(|e| Error::TradingView(e.to_string()))?;
    let history = client.get_history(&req.symbol, "D")
        .await
        .map_err(|e| Error::TradingView(e.to_string()))?;

    if history.is_empty() {
        return Err(Error::InvalidInput(format!(
            "symbol {} has no data on TradingView",
            req.symbol
        )));
    }

    // Check if symbol already exists
    let existing: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM monitors WHERE symbol = ?"
    )
    .bind(&req.symbol)
    .fetch_optional(db.as_ref())
    .await?;
    tracing::debug!("duplicate check for {}: {:?}", req.symbol, existing);

    if existing.is_some() {
        return Err(Error::InvalidInput(format!(
            "monitor for symbol {} already exists",
            req.symbol
        )));
    }

    let monitor = Monitor::new(req.symbol.clone(), req.name);
    sqlx::query(
        "INSERT INTO monitors (id, symbol, name, enabled, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&monitor.id)
    .bind(&monitor.symbol)
    .bind(&monitor.name)
    .bind(monitor.enabled)
    .bind(monitor.created_at)
    .bind(monitor.updated_at)
    .execute(db.as_ref())
    .await?;

    for bar in &history {
        sqlx::query(
            "INSERT OR IGNORE INTO price_history (symbol, timestamp, open, high, low, close, volume) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&req.symbol)
        .bind(bar.timestamp)
        .bind(bar.open)
        .bind(bar.high)
        .bind(bar.low)
        .bind(bar.close)
        .bind(bar.volume as i64)
        .execute(db.as_ref())
        .await?;
    }

    Ok((StatusCode::CREATED, Json(monitor)))
}

async fn get_monitor(Path(id): Path<String>) -> Result<impl IntoResponse> {
    let db = get_db_pool();
    let monitor = sqlx::query_as::<_, Monitor>(
        "SELECT id, symbol, name, enabled, created_at, updated_at FROM monitors WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| Error::NotFound(format!("monitor {} not found", id)))?;

    Ok(Json(monitor))
}

async fn update_monitor(
    Path(id): Path<String>,
    Json(req): Json<UpdateMonitorRequest>,
) -> Result<impl IntoResponse> {
    let db = get_db_pool();
    let existing = sqlx::query_as::<_, Monitor>(
        "SELECT id, symbol, name, enabled, created_at, updated_at FROM monitors WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| Error::NotFound(format!("monitor {} not found", id)))?;

    let name = req.name.or(existing.name);
    let enabled = req.enabled.unwrap_or(existing.enabled);
    let updated_at = Utc::now();

    sqlx::query("UPDATE monitors SET name = ?, enabled = ?, updated_at = ? WHERE id = ?")
        .bind(&name)
        .bind(enabled)
        .bind(updated_at)
        .bind(&id)
        .execute(db.as_ref())
        .await?;

    let monitor = sqlx::query_as::<_, Monitor>(
        "SELECT id, symbol, name, enabled, created_at, updated_at FROM monitors WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(db.as_ref())
    .await?;

    Ok(Json(monitor))
}

async fn delete_monitor(Path(id): Path<String>) -> Result<impl IntoResponse> {
    let db = get_db_pool();
    let result = sqlx::query("DELETE FROM monitors WHERE id = ?")
        .bind(&id)
        .execute(db.as_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(Error::NotFound(format!("monitor {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS monitors (
                id TEXT PRIMARY KEY,
                symbol TEXT NOT NULL,
                name TEXT,
                enabled INTEGER DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL)",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    fn setup_global_pool(pool: &SqlitePool) {
        set_db_pool(Arc::new(pool.clone()));
        set_config(crate::config::Config {
            server: crate::config::ServerConfig::default(),
            database: crate::config::DatabaseConfig::default(),
            monitor: crate::config::MonitorConfig::default(),
            alert: crate::config::AlertConfig::default(),
        });
        futures::executor::block_on(async {
            sqlx::query("DELETE FROM monitors").execute(pool).await.ok();
        });
    }

    #[tokio::test]
    async fn test_create_monitor() {
        let pool = create_test_pool().await;
        setup_global_pool(&pool);

        let req = CreateMonitorRequest {
            symbol: "NASDAQ:AAPL".to_string(),
            name: Some("Apple".to_string()),
        };
        let result = create_monitor(Json(req)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_monitor_empty_symbol() {
        let pool = create_test_pool().await;
        setup_global_pool(&pool);

        let req = CreateMonitorRequest {
            symbol: "".to_string(),
            name: None,
        };
        let result = create_monitor(Json(req)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_list_monitors() {
        let pool = create_test_pool().await;
        setup_global_pool(&pool);

        let req = CreateMonitorRequest {
            symbol: "NASDAQ:AAPL".to_string(),
            name: None,
        };
        create_monitor(Json(req)).await.unwrap();

        let resp = list_monitors().await.unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].symbol, "NASDAQ:AAPL");
    }

    #[tokio::test]
    async fn test_get_monitor_not_found() {
        let pool = create_test_pool().await;
        setup_global_pool(&pool);

        let result = get_monitor(Path("nonexistent".to_string())).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_monitor() {
        let pool = create_test_pool().await;
        setup_global_pool(&pool);

        let req = CreateMonitorRequest {
            symbol: "NASDAQ:TSLA".to_string(),
            name: None,
        };
        create_monitor(Json(req)).await.unwrap();

        let resp = list_monitors().await.unwrap();
        let monitor_id = resp.data[0].id.clone();

        let result = delete_monitor(Path(monitor_id.clone())).await;
        assert!(result.is_ok());

        let result = get_monitor(Path(monitor_id)).await;
        assert!(result.is_err());
    }
}
