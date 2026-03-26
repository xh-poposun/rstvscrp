use std::time::Duration;
use tvdata_rs::{TradingViewClient, TradingViewClientConfig, history::{HistoryRequest, Interval}, scanner::Ticker};

#[tokio::main]
async fn main() {
    println!("Starting TV client test...");
    
    // Enable RUST_LOG for debug output
    println!("Environment check:");
    println!("  http_proxy: {:?}", std::env::var("http_proxy").ok());
    println!("  https_proxy: {:?}", std::env::var("https_proxy").ok());
    
    let config = TradingViewClientConfig::default();
    
    match TradingViewClient::from_config(config) {
        Ok(client) => {
            println!("Client created successfully!");
            
            let ticker: Ticker = "NASDAQ:AAPL".into();
            let request = HistoryRequest::max(ticker, Interval::Day1);
            
            println!("Fetching history for NASDAQ:AAPL (timeout: 30s)...");
            println!("Request: symbol={}, interval={}, bars={}", 
                request.symbol.as_str(), request.interval.as_code(), request.bars);
            
            // Try with a shorter timeout first to see faster feedback
            match tokio::time::timeout(Duration::from_secs(30), client.history(&request)).await {
                Ok(result) => {
                    match result {
                        Ok(series) => {
                            println!("Success! Got {} bars", series.bars.len());
                            for bar in series.bars.iter().take(5) {
                                println!("  {}: O={} H={} L={} C={}", 
                                    bar.time.unix_timestamp(), bar.open, bar.high, bar.low, bar.close);
                            }
                        }
                        Err(e) => println!("Error getting history: {}", e),
                    }
                }
                Err(e) => println!("Timeout waiting for history: {}", e),
            }
            
            // Also try a simple search to see if HTTP works
            println!("\nTrying search as alternative...");
            match tokio::time::timeout(Duration::from_secs(10), client.search_equities("Apple")).await {
                Ok(Ok(results)) => {
                    println!("Search returned {} results", results.len());
                }
                Ok(Err(e)) => println!("Search error: {}", e),
                Err(_) => println!("Search timeout"),
            }
        }
        Err(e) => println!("Failed to create client: {}", e),
    }
}