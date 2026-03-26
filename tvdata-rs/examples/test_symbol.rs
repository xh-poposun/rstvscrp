use tvdata_rs::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let symbol = env::args().nth(1).unwrap_or_else(|| "SSE:600941".to_string());
    println!("Testing symbol: {}", symbol);
    
    let client = TradingViewClient::builder().build()?;
    let history = client.history(&HistoryRequest::new(symbol.clone(), Interval::Day1, 5)).await?;
    
    println!("Got {} bars", history.bars.len());
    for bar in &history.bars {
        println!("  {} o={} h={} l={} c={}", bar.time, bar.open, bar.high, bar.low, bar.close);
    }
    
    Ok(())
}
