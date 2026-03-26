use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::Path;
use std::time::Duration;

use crate::error::Result;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn init_database(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS monitors (
            id TEXT PRIMARY KEY,
            symbol TEXT NOT NULL,
            name TEXT,
            enabled INTEGER DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create unique index to prevent duplicate symbols
    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_monitors_symbol ON monitors(symbol)"
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS alert_rules (
            id TEXT PRIMARY KEY,
            monitor_id TEXT NOT NULL,
            rule_type TEXT NOT NULL,
            name TEXT NOT NULL,
            condition TEXT NOT NULL,
            severity TEXT NOT NULL,
            cooldown_secs INTEGER DEFAULT 300,
            enabled INTEGER DEFAULT 1,
            created_at TEXT NOT NULL,
            last_triggered_date TEXT,
            daily_reset_hour_utc INTEGER,
            FOREIGN KEY (monitor_id) REFERENCES monitors(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Add new columns for backwards compatibility with existing databases
    let columns: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM pragma_table_info('alert_rules') WHERE name = 'last_triggered_date'"
    )
    .fetch_all(pool)
    .await?;

    if columns.is_empty() {
        sqlx::query("ALTER TABLE alert_rules ADD COLUMN last_triggered_date TEXT")
            .execute(pool)
            .await?;
    }

    let columns: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM pragma_table_info('alert_rules') WHERE name = 'daily_reset_hour_utc'"
    )
    .fetch_all(pool)
    .await?;

    if columns.is_empty() {
        sqlx::query("ALTER TABLE alert_rules ADD COLUMN daily_reset_hour_utc INTEGER")
            .execute(pool)
            .await?;
    }

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS alerts (
            id TEXT PRIMARY KEY,
            rule_id TEXT NOT NULL,
            symbol TEXT NOT NULL,
            message TEXT NOT NULL,
            severity TEXT NOT NULL,
            triggered_at TEXT NOT NULL,
            acknowledged INTEGER DEFAULT 0,
            acknowledged_at TEXT,
            ack_by TEXT,
            FOREIGN KEY (rule_id) REFERENCES alert_rules(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS price_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            symbol TEXT NOT NULL,
            timestamp INTEGER NOT NULL CHECK(typeof(timestamp) = 'integer'),
            open REAL,
            high REAL,
            low REAL,
            close REAL,
            volume INTEGER CHECK(typeof(volume) = 'integer'),
            UNIQUE(symbol, timestamp)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Enable WAL mode for better concurrent read/write performance
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(pool)
        .await?;

    // Optimize for write performance
    sqlx::query("PRAGMA synchronous=NORMAL")
        .execute(pool)
        .await?;

    // Increase cache size (negative value = KB)
    sqlx::query("PRAGMA cache_size=-64000")  // 64MB cache
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_alerts_triggered_at ON alerts(triggered_at)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_price_history_symbol ON price_history(symbol)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_price_history_symbol_time ON price_history(symbol, timestamp)
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub fn ensure_data_dir(path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn build_database_url(path: &str) -> String {
    format!("sqlite:{}", path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_connect_pool() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        init_database(&pool).await.unwrap();

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(count.0 >= 4);
    }

    #[test]
    fn test_build_database_url() {
        let url = build_database_url("./data/test.db");
        assert_eq!(url, "sqlite://./data/test.db");

        let url = build_database_url("/absolute/path.db");
        assert_eq!(url, "sqlite:///absolute/path.db");
    }

    #[test]
    fn test_ensure_data_dir() {
        ensure_data_dir("./test_data/subdir/nested.db").unwrap();
        assert!(Path::new("./test_data/subdir").exists());

        std::fs::remove_dir_all("./test_data").ok();
    }
}
