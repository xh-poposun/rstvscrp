use axum::{Router, routing::post};
use std::sync::Arc;
use tokio::sync::broadcast;

pub fn router(shutdown_tx: Arc<broadcast::Sender<()>>) -> Router<()> {
    Router::new().route(
        "/shutdown",
        post(move || {
            let tx = Arc::clone(&shutdown_tx);
            async move {
                let _ = tx.send(());
                "Shutting down..."
            }
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::broadcast;

    #[test]
    fn test_router_creates_successfully() {
        let (tx, _rx) = broadcast::channel::<()>(1);
        let _router = router(Arc::new(tx));
    }
}
