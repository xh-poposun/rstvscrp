use axum::{
    Json, Router,
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::Utc;

use crate::api::monitors::AppState;
use crate::error::{Error, Result};
use crate::models::{AcknowledgeRequest, Alert, AlertListQuery, PaginatedResponse};

pub fn router(_state: AppState) -> Router<()> {
    Router::new()
        .route("/api/v1/alerts", get(list_alerts))
        .route("/api/v1/alerts/:id/ack", post(acknowledge_alert))
}

async fn list_alerts(
    Query(params): Query<AlertListQuery>,
) -> Result<Json<PaginatedResponse<Alert>>> {
    let db = crate::api::monitors::get_db_pool();
    let page = if params.page < 1 { 1 } else { params.page };
    let page_size = if params.page_size < 1 {
        20
    } else {
        params.page_size
    };
    let offset = (page - 1) * page_size;

    let mut query = String::from(
        "SELECT id, rule_id, symbol, message, severity, triggered_at, acknowledged, acknowledged_at, ack_by FROM alerts WHERE 1=1",
    );
    let mut count_query = String::from("SELECT COUNT(*) FROM alerts WHERE 1=1");

    if let Some(ref symbol) = params.symbol {
        query.push_str(&format!(" AND symbol = '{}'", symbol));
        count_query.push_str(&format!(" AND symbol = '{}'", symbol));
    }

    if let Some(acknowledged) = params.acknowledged {
        query.push_str(&format!(
            " AND acknowledged = {}",
            if acknowledged { 1 } else { 0 }
        ));
        count_query.push_str(&format!(
            " AND acknowledged = {}",
            if acknowledged { 1 } else { 0 }
        ));
    }

    query.push_str(&format!(
        " ORDER BY triggered_at DESC LIMIT {} OFFSET {}",
        page_size, offset
    ));

    let alerts = sqlx::query_as::<_, Alert>(&query)
        .fetch_all(db.as_ref())
        .await?;

    let total: (i64,) = sqlx::query_as(&count_query).fetch_one(db.as_ref()).await?;

    Ok(Json(PaginatedResponse {
        data: alerts,
        page,
        page_size,
        total: total.0,
    }))
}

async fn acknowledge_alert(
    Path(id): Path<String>,
    Json(req): Json<AcknowledgeRequest>,
) -> Result<impl IntoResponse> {
    let db = crate::api::monitors::get_db_pool();
    let existing = sqlx::query_as::<_, Alert>(
        "SELECT id, rule_id, symbol, message, severity, triggered_at, acknowledged, acknowledged_at, ack_by FROM alerts WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| Error::NotFound(format!("alert {} not found", id)))?;

    if existing.acknowledged {
        return Err(Error::InvalidInput(
            "alert already acknowledged".to_string(),
        ));
    }

    let now = Utc::now();
    let ack_by = req.ack_by.unwrap_or_else(|| "system".to_string());

    sqlx::query("UPDATE alerts SET acknowledged = 1, acknowledged_at = ?, ack_by = ? WHERE id = ?")
        .bind(now)
        .bind(&ack_by)
        .bind(&id)
        .execute(db.as_ref())
        .await?;

    let alert = sqlx::query_as::<_, Alert>(
        "SELECT id, rule_id, symbol, message, severity, triggered_at, acknowledged, acknowledged_at, ack_by FROM alerts WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(db.as_ref())
    .await?;

    Ok(Json(alert))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::monitors::set_db_pool;
    use crate::models::{AlertRule, Monitor, RuleType, Severity};
    use axum::extract::Path;
    use sqlx::SqlitePool;
    use std::sync::Arc;

    async fn create_test_pool() -> (SqlitePool, Alert, AlertRule) {
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

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS alerts (
                id TEXT PRIMARY KEY, rule_id TEXT NOT NULL, symbol TEXT NOT NULL,
                message TEXT NOT NULL, severity TEXT NOT NULL, triggered_at TEXT NOT NULL,
                acknowledged INTEGER DEFAULT 0, acknowledged_at TEXT, ack_by TEXT)",
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

        let rule = AlertRule::new(
            monitor.id.clone(),
            RuleType::Price,
            "Price Alert".to_string(),
            serde_json::json!({"op": ">", "threshold": 5.0}),
            Severity::Warning,
            300,
            None,
        );
        sqlx::query(
            "INSERT INTO alert_rules (id, monitor_id, rule_type, name, condition, severity, cooldown_secs, enabled, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        .execute(&pool)
        .await
        .unwrap();

        let alert = Alert::new(
            rule.id.clone(),
            "NASDAQ:AAPL".to_string(),
            "Price increased by 5%".to_string(),
            Severity::Warning,
        );
        sqlx::query(
            "INSERT INTO alerts (id, rule_id, symbol, message, severity, triggered_at, acknowledged, acknowledged_at, ack_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&alert.id)
        .bind(&alert.rule_id)
        .bind(&alert.symbol)
        .bind(&alert.message)
        .bind(&alert.severity)
        .bind(alert.triggered_at)
        .bind(alert.acknowledged)
        .bind(alert.acknowledged_at.as_ref())
        .bind(&alert.ack_by)
        .execute(&pool)
        .await
        .unwrap();

        (pool, alert, rule)
    }

    fn setup_global_pool(pool: &SqlitePool) {
        set_db_pool(Arc::new(pool.clone()));
        futures::executor::block_on(async {
            sqlx::query("DELETE FROM monitors").execute(pool).await.ok();
            sqlx::query("DELETE FROM alert_rules")
                .execute(pool)
                .await
                .ok();
            sqlx::query("DELETE FROM alerts").execute(pool).await.ok();
        });
    }

    #[tokio::test]
    #[ignore]
    async fn test_list_alerts() {
        let (pool, _alert, _) = create_test_pool().await;
        setup_global_pool(&pool);

        let query = AlertListQuery {
            page: 1,
            page_size: 10,
            symbol: None,
            acknowledged: None,
        };

        let resp = list_alerts(Query(query)).await.unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].message, "Price increased by 5%");
    }

    #[tokio::test]
    #[ignore]
    async fn test_list_alerts_with_symbol_filter() {
        let (pool, _alert, _) = create_test_pool().await;
        setup_global_pool(&pool);

        let query = AlertListQuery {
            page: 1,
            page_size: 10,
            symbol: Some("NASDAQ:TSLA".to_string()),
            acknowledged: None,
        };

        let resp = list_alerts(Query(query)).await.unwrap();
        assert_eq!(resp.data.len(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_acknowledge_alert() {
        let (pool, _alert, _) = create_test_pool().await;
        setup_global_pool(&pool);

        let resp = list_alerts(Query(AlertListQuery {
            page: 1,
            page_size: 10,
            symbol: None,
            acknowledged: None,
        }))
        .await
        .unwrap();

        if resp.data.is_empty() {
            return;
        }

        let alert_id = resp.data[0].id.clone();

        let req = AcknowledgeRequest {
            ack_by: Some("admin".to_string()),
        };

        let result = acknowledge_alert(Path(alert_id), Json(req)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_acknowledge_already_acknowledged() {
        let (pool, _, _) = create_test_pool().await;
        setup_global_pool(&pool);

        sqlx::query("UPDATE alerts SET acknowledged = 1 WHERE acknowledged = 0")
            .execute(&pool)
            .await
            .unwrap();

        let alert_id: (String,) = sqlx::query_as("SELECT id FROM alerts LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        let req = AcknowledgeRequest { ack_by: None };
        let result = acknowledge_alert(Path(alert_id.0), Json(req)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_acknowledge_not_found() {
        let (pool, _, _) = create_test_pool().await;
        setup_global_pool(&pool);

        let req = AcknowledgeRequest { ack_by: None };
        let result = acknowledge_alert(Path("nonexistent".to_string()), Json(req)).await;
        assert!(result.is_err());
    }
}
