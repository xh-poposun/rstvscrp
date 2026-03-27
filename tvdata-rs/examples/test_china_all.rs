use tvdata_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = TradingViewClient::builder().build()?;

    // Test multiple Chinese stocks
    let symbols = ["SSE:600941", "SSE:600519", "HKEX:700", "HKEX:9988"];

    for symbol in symbols {
        match client
            .history(&HistoryRequest::new(symbol, Interval::Day1, 5))
            .await
        {
            Ok(h) => println!("✓ {} - {} bars", symbol, h.bars.len()),
            Err(e) => println!("✗ {} - Error: {}", symbol, e),
        }
    }

    Ok(())
}
