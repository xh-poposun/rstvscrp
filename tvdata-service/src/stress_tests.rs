#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;
    
    #[tokio::test]
    async fn test_large_data_insert_performance() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS price_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                symbol TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                open REAL, high REAL, low REAL, close REAL, volume INTEGER,
                UNIQUE(symbol, timestamp)
            )"
        )
        .execute(&pool)
        .await
        .unwrap();
        
        let start = std::time::Instant::now();
        
        for i in 0..10000 {
            let ts = 1704067200i64 + (i as i64) * 86400;
            sqlx::query(
                "INSERT OR IGNORE INTO price_history VALUES (NULL, 'TEST', ?, 100.0, 101.0, 99.0, 100.0, 1000000)"
            )
            .bind(ts)
            .execute(&pool)
            .await
            .unwrap();
        }
        
        let elapsed = start.elapsed();
        println!("Inserted 10000 rows in {:?}", elapsed);
        
        assert!(elapsed.as_secs() < 30, "insert too slow");
    }

    #[tokio::test]
    async fn test_range_query_performance() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS price_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                symbol TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                open REAL, high REAL, low REAL, close REAL, volume INTEGER,
                UNIQUE(symbol, timestamp)
            )"
        )
        .execute(&pool)
        .await
        .unwrap();
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx ON price_history(symbol, timestamp)")
            .execute(&pool)
            .await
            .unwrap();
        
        for i in 0..5000 {
            let ts = 1704067200i64 + (i as i64) * 86400;
            sqlx::query(
                "INSERT OR IGNORE INTO price_history VALUES (NULL, 'AAPL', ?, 100.0, 101.0, 99.0, 100.0, 1000000)"
            )
            .bind(ts)
            .execute(&pool)
            .await
            .unwrap();
        }
        
        let start = std::time::Instant::now();
        
        let result: Vec<(i64,)> = sqlx::query_as(
            "SELECT timestamp FROM price_history WHERE symbol = 'AAPL' AND timestamp >= 1704067200 AND timestamp <= 1735689600"
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        
        let elapsed = start.elapsed();
        println!("Range query (1 year) returned {} rows in {:?}", result.len(), elapsed);
        
        assert!(elapsed.as_millis() < 100, "query too slow: {:?}", elapsed);
    }

    #[tokio::test]
    async fn test_concurrent_read_write() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS price_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                symbol TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                open REAL, high REAL, low REAL, close REAL, volume INTEGER,
                UNIQUE(symbol, timestamp)
            )"
        )
        .execute(&pool)
        .await
        .unwrap();
        
        let pool_clone = pool.clone();
        let writer = tokio::spawn(async move {
            for i in 0..1000 {
                let ts = 1704067200i64 + (i as i64) * 86400;
                sqlx::query(
                    "INSERT OR IGNORE INTO price_history VALUES (NULL, 'TEST', ?, 100.0, 101.0, 99.0, 100.0, 1000000)"
                )
                .bind(ts)
                .execute(&pool_clone)
                .await
                .unwrap();
            }
        });
        
        let pool_clone2 = pool.clone();
        let reader = tokio::spawn(async move {
            for _ in 0..100 {
                let _: Vec<(i64,)> = sqlx::query_as(
                    "SELECT timestamp FROM price_history WHERE symbol = 'TEST' LIMIT 10"
                )
                .fetch_all(&pool_clone2)
                .await
                .unwrap();
            }
        });
        
        writer.await.unwrap();
        reader.await.unwrap();
        
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM price_history")
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(count.0, 1000);
    }
}