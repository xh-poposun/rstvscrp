use std::env;
use std::time::Instant;

use tvdata_rs::prelude::*;

const DEFAULT_SYMBOLS: &[&str] = &["NASDAQ:AAPL", "SSE:600941", "HKEX:700"];

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("WSS_DEBUG_LOG").is_err() {
        #[allow(unsafe_code)]
        unsafe {
            env::set_var("WSS_DEBUG_LOG", "wss_debug.log");
        }
    }

    let symbols: Vec<String> = env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .if_empty_then(DEFAULT_SYMBOLS.iter().map(|s| s.to_string()).collect());

    let client = TradingViewClient::builder().build()?;

    println!("WSS Compare Test");
    println!("================");
    println!("Symbols: {:?}", symbols);
    println!();

    for symbol in symbols {
        println!("--- {} ---", symbol);

        let history_start = Instant::now();
        match client
            .history(&HistoryRequest::new(symbol.clone(), Interval::Day1, 5))
            .await
        {
            Ok(h) => {
                let elapsed = history_start.elapsed();
                println!(
                    "  [HISTORY]  OK    - {} bars ({:.3}s)",
                    h.bars.len(),
                    elapsed.as_secs_f64()
                );
            }
            Err(e) => {
                let elapsed = history_start.elapsed();
                println!("  [HISTORY]  FAIL  - {} ({:.3}s)", e, elapsed.as_secs_f64());
            }
        }

        let quote_start = Instant::now();
        match client.equity().quote(symbol.clone()).await {
            Ok(q) => {
                let elapsed = quote_start.elapsed();
                println!(
                    "  [QUOTE]    OK    - close={:?} ({:.3}s)",
                    q.close,
                    elapsed.as_secs_f64()
                );
            }
            Err(e) => {
                let elapsed = quote_start.elapsed();
                println!("  [QUOTE]    FAIL  - {} ({:.3}s)", e, elapsed.as_secs_f64());
            }
        }

        println!();
    }

    Ok(())
}

trait IfEmptyThen<T> {
    fn if_empty_then(self, default: Vec<T>) -> Vec<T>;
}

impl<T> IfEmptyThen<T> for Vec<T> {
    fn if_empty_then(self, default: Vec<T>) -> Vec<T> {
        if self.is_empty() { default } else { self }
    }
}
