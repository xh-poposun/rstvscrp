#[allow(unused_imports)]
use axum::routing::{delete, post};
use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};

use crate::api::monitors::AppState;
use crate::error::{Error, Result};
use crate::models::{AlertRule, CreateRuleRequest, PaginatedResponse};

pub fn router(_state: AppState) -> Router<()> {
    Router::new()
        .route("/api/v1/rules", get(list_rules).post(create_rule))
        .route("/api/v1/rules/:id", get(get_rule).delete(delete_rule))
}

async fn list_rules() -> Result<Json<PaginatedResponse<AlertRule>>> {
    let db = crate::api::monitors::get_db_pool();
    let rules = sqlx::query_as::<_, AlertRule>(
        "SELECT id, monitor_id, rule_type, name, condition, severity, cooldown_secs, enabled, created_at, last_triggered_date, daily_reset_hour_utc FROM alert_rules ORDER BY created_at DESC",
    )
    .fetch_all(db.as_ref())
    .await?;

    let total = rules.len() as i64;
    Ok(Json(PaginatedResponse {
        data: rules,
        page: 1,
        page_size: total,
        total,
    }))
}

async fn create_rule(Json(req): Json<CreateRuleRequest>) -> Result<impl IntoResponse> {
    if req.name.is_empty() {
        return Err(Error::InvalidInput("name cannot be empty".to_string()));
    }

    let db = crate::api::monitors::get_db_pool();
    let monitor_exists = sqlx::query("SELECT id FROM monitors WHERE id = ?")
        .bind(&req.monitor_id)
        .fetch_optional(db.as_ref())
        .await?;

    if monitor_exists.is_none() {
        return Err(Error::InvalidInput(format!(
            "monitor {} not found",
            req.monitor_id
        )));
    }

    let rule = AlertRule::new(
        req.monitor_id,
        req.rule_type,
        req.name,
        req.condition,
        req.severity,
        req.cooldown_secs,
        req.daily_reset_hour_utc,
    );

    sqlx::query(
        "INSERT INTO alert_rules (id, monitor_id, rule_type, name, condition, severity, cooldown_secs, enabled, created_at, last_triggered_date, daily_reset_hour_utc) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&rule.id)
    .bind(&rule.monitor_id)
    .bind(&rule.rule_type)
    .bind(&rule.name)
    .bind(&rule.condition)
    .bind(&rule.severity)
    .bind(rule.cooldown_secs)
    .bind(rule.enabled)
    .bind(rule.created_at)
    .bind(&rule.last_triggered_date)
    .bind(rule.daily_reset_hour_utc)
    .execute(db.as_ref())
    .await?;

    Ok((StatusCode::CREATED, Json(rule)))
}

async fn get_rule(Path(id): Path<String>) -> Result<impl IntoResponse> {
    let db = crate::api::monitors::get_db_pool();
    let rule = sqlx::query_as::<_, AlertRule>(
        "SELECT id, monitor_id, rule_type, name, condition, severity, cooldown_secs, enabled, created_at, last_triggered_date, daily_reset_hour_utc FROM alert_rules WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| Error::NotFound(format!("rule {} not found", id)))?;

    Ok(Json(rule))
}

async fn delete_rule(Path(id): Path<String>) -> Result<impl IntoResponse> {
    let db = crate::api::monitors::get_db_pool();
    let result = sqlx::query("DELETE FROM alert_rules WHERE id = ?")
        .bind(&id)
        .execute(db.as_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(Error::NotFound(format!("rule {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::monitors::set_db_pool;
    use crate::models::{Monitor, RuleType, Severity};
    use axum::extract::Path;
    use sqlx::SqlitePool;
    use std::sync::Arc;

    async fn create_test_pool() -> (SqlitePool, Monitor) {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS monitors (
                id TEXT PRIMARY KEY, symbol TEXT NOT NULL, name TEXT,
                enabled INTEGER DEFAULT 1, created_at TEXT NOT NULL, updated_at TEXT NOT NULL)",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS alert_rules (
                id TEXT PRIMARY KEY, monitor_id TEXT NOT NULL, rule_type TEXT NOT NULL,
                name TEXT NOT NULL, condition TEXT NOT NULL, severity TEXT NOT NULL,
                cooldown_secs INTEGER DEFAULT 300, enabled INTEGER DEFAULT 1, created_at TEXT NOT NULL)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let monitor = Monitor::new("NASDAQ:AAPL".to_string(), Some("Apple".to_string()));
        sqlx::query("INSERT INTO monitors (id, symbol, name, enabled, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&monitor.id)
            .bind(&monitor.symbol)
            .bind(&monitor.name)
            .bind(monitor.enabled)
            .bind(monitor.created_at)
            .bind(monitor.updated_at)
            .execute(&pool)
            .await
            .unwrap();

        (pool, monitor)
    }

    fn setup_global_pool(pool: &SqlitePool) {
        set_db_pool(Arc::new(pool.clone()));
        futures::executor::block_on(async {
            sqlx::query("DELETE FROM monitors").execute(pool).await.ok();
            sqlx::query("DELETE FROM alert_rules")
                .execute(pool)
                .await
                .ok();
        });
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_rule() {
        let (pool, monitor) = create_test_pool().await;
        setup_global_pool(&pool);

        let req = CreateRuleRequest {
            monitor_id: monitor.id.clone(),
            rule_type: RuleType::Price,
            name: "Price Alert".to_string(),
            condition: serde_json::json!({"op": ">", "threshold": 5.0}),
            severity: Severity::Warning,
            cooldown_secs: 300,
            daily_reset_hour_utc: None,
        };

        let result = create_rule(Json(req)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_rule_invalid_monitor() {
        let (pool, _) = create_test_pool().await;
        setup_global_pool(&pool);

        let req = CreateRuleRequest {
            monitor_id: "nonexistent".to_string(),
            rule_type: RuleType::Price,
            name: "Test".to_string(),
            condition: serde_json::json!({"op": ">", "threshold": 5.0}),
            severity: Severity::Info,
            cooldown_secs: 300,
            daily_reset_hour_utc: None,
        };

        let result = create_rule(Json(req)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_rule_not_found() {
        let (pool, _) = create_test_pool().await;
        setup_global_pool(&pool);

        let result = get_rule(Path("nonexistent".to_string())).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_rule() {
        let (pool, monitor) = create_test_pool().await;
        setup_global_pool(&pool);

        let req = CreateRuleRequest {
            monitor_id: monitor.id,
            rule_type: RuleType::Price,
            name: "To Delete".to_string(),
            condition: serde_json::json!({"op": ">", "threshold": 5.0}),
            severity: Severity::Info,
            cooldown_secs: 300,
            daily_reset_hour_utc: None,
        };

        create_rule(Json(req)).await.unwrap();

        let resp = list_rules().await.unwrap();
        let rule_id = resp.data[0].id.clone();

        let result = delete_rule(Path(rule_id.clone())).await;
        assert!(result.is_ok());

        let result = get_rule(Path(rule_id)).await;
        assert!(result.is_err());
    }
}
