use axum::{Router, routing::get};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::RwLock;
use tokio::sync::broadcast;
use tokio::time::interval;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tvdata_service::{
    alert::AlertEngine,
    api::{
        self, AppState,
        monitors::{set_config, set_db_pool},
    },
    config::Config,
    db,
    history_refresh::start_history_refresh_task,
    monitor::MonitorEngine,
    tvclient::TvClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tvdata_service=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load("config.yaml").unwrap_or_else(|_| {
        tracing::warn!("config.yaml not found, using defaults");
        Config {
            server: tvdata_service::config::ServerConfig::default(),
            database: tvdata_service::config::DatabaseConfig::default(),
            monitor: tvdata_service::config::MonitorConfig::default(),
            alert: tvdata_service::config::AlertConfig::default(),
            search: tvdata_service::config::SearchConfig::default(),
        }
    });

    db::ensure_data_dir(&config.database.path)?;
    let db_url = db::build_database_url(&config.database.path);
    tracing::info!("Database URL: {}", db_url);
    let pool = db::create_pool(&db_url).await?;
    db::init_database(&pool).await?;

    tracing::info!("Database initialized at {}", config.database.path);

    set_db_pool(Arc::new(pool.clone()));
    set_config(config.clone());
    let alert_engine = Arc::new(RwLock::new(AlertEngine::new(
        config.alert.webhook.url.clone(),
    )));

    // Create and start the monitor engine
    tracing::info!("About to create TV client...");
    let tv_client_result = TvClient::new().await;
    tracing::info!("TV client result: {:?}", tv_client_result);
    if let Ok(tv_client) = tv_client_result {
        tracing::info!("Creating monitor engine...");
        let monitor_engine = MonitorEngine::new(
            pool,
            tv_client,
            alert_engine.clone(),
            config.monitor.clone(),
        ).await;
        
        tracing::info!("Spawning monitor task...");
        // Start background monitoring task
        tokio::spawn(async move {
            start_monitor_task(monitor_engine, alert_engine, config.monitor.clone()).await;
        });
        tracing::info!("Monitor task spawned");
    } else {
        tracing::warn!("Failed to create TV client for monitoring: {:?}", tv_client_result.err());
    }

    tokio::spawn(async { start_history_refresh_task().await });

    let (shutdown_tx, mut shutdown_rx) = broadcast::channel::<()>(1);

    let shutdown_tx_for_ctrlc = shutdown_tx.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        tracing::info!("Received Ctrl+C, initiating shutdown...");
        let _ = shutdown_tx_for_ctrlc.send(());
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(api::monitors::router(AppState))
        .merge(api::rules::router(AppState))
        .merge(api::alerts::router(AppState))
        .merge(api::quotes::router(AppState))
        .merge(api::search::router(AppState))
        .merge(api::history::router(AppState))
        .merge(api::shutdown::router(Arc::new(shutdown_tx)))
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    tracing::info!("Ctrl+C received in graceful shutdown");
                }
                _ = shutdown_rx.recv() => {
                    tracing::info!("Shutdown signal received via /shutdown endpoint");
                }
            }
        })
        .await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "ok"
}

async fn start_monitor_task(
    engine: MonitorEngine,
    alert_engine: Arc<RwLock<AlertEngine>>,
    config: tvdata_service::config::MonitorConfig,
) {
    tracing::info!("Starting price monitor background task (interval: {}s)", config.check_interval_secs);
    
    let mut check_interval = interval(Duration::from_secs(config.check_interval_secs));
    
    loop {
        check_interval.tick().await;
        
        tracing::debug!("Running price check...");
        
        match engine.run_check().await {
            Ok(alerts) => {
                if !alerts.is_empty() {
                    tracing::info!("Triggered {} alerts", alerts.len());
                    
                    for (alert, quote_info) in &alerts {
                        let engine = alert_engine.write().await;
                        if let Err(e) = engine.send_alert(&alert.symbol, "price", &alert.message, &alert.severity, quote_info.as_ref()).await {
                            tracing::error!("Failed to send webhook for alert {}: {}", alert.id, e);
                        } else {
                            tracing::info!("Webhook sent for alert {}", alert.id);
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("Price check failed: {}", e);
            }
        }
    }
}
