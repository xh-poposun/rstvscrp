use std::time::Duration;
use tvdata_rs::{TradingViewClient, TradingViewClientConfig, history::{HistoryRequest, Interval}, scanner::Ticker};

#[tokio::main]
async fn main() {
    println!("Starting TV client test - WITHOUT fetch_all...");
    
    println!("Environment check:");
    println!("  http_proxy: {:?}", std::env::var("http_proxy").ok());
    println!("  https_proxy: {:?}", std::env::var("https_proxy").ok());
    
    let config = TradingViewClientConfig::default();
    
    match TradingViewClient::from_config(config) {
        Ok(client) => {
            println!("Client created successfully!");
            
            // Test WITHOUT fetch_all - just get 100 bars
            let ticker: Ticker = "NASDAQ:AAPL".into();
            let request = HistoryRequest::new(ticker.clone(), Interval::Day1, 100);
            
            println!("\n=== Test 1: Request 100 bars (NO fetch_all) ===");
            println!("Request: symbol={}, interval={}, bars={}, fetch_all={}", 
                request.symbol.as_str(), request.interval.as_code(), request.bars, request.fetch_all);
            
            match tokio::time::timeout(Duration::from_secs(30), client.history(&request)).await {
                Ok(result) => {
                    match result {
                        Ok(series) => {
                            println!("Success! Got {} bars", series.bars.len());
                            println!("\nFirst 5 bars (earliest?):");
                            for bar in series.bars.iter().take(5) {
                                println!("  ts={}: O={:.2} H={:.2} L={:.2} C={:.2}", 
                                    bar.time.unix_timestamp(), bar.open, bar.high, bar.low, bar.close);
                            }
                            println!("\nLast 5 bars (latest?):");
                            for bar in series.bars.iter().rev().take(5) {
                                println!("  ts={}: O={:.2} H={:.2} L={:.2} C={:.2}", 
                                    bar.time.unix_timestamp(), bar.open, bar.high, bar.low, bar.close);
                            }
                        }
                        Err(e) => println!("Error getting history: {}", e),
                    }
                }
                Err(e) => println!("Timeout waiting for history: {}", e),
            }
            
            // Test WITH fetch_all for comparison
            println!("\n=== Test 2: Request with fetch_all=true ===");
            let request_max = HistoryRequest::max(ticker, Interval::Day1);
            println!("Request: symbol={}, interval={}, bars={}, fetch_all={}", 
                request_max.symbol.as_str(), request_max.interval.as_code(), request_max.bars, request_max.fetch_all);
            
            match tokio::time::timeout(Duration::from_secs(60), client.history(&request_max)).await {
                Ok(result) => {
                    match result {
                        Ok(series) => {
                            println!("Success! Got {} bars", series.bars.len());
                            println!("\nFirst 5 bars:");
                            for bar in series.bars.iter().take(5) {
                                println!("  ts={}: O={:.2} H={:.2} L={:.2} C={:.2}", 
                                    bar.time.unix_timestamp(), bar.open, bar.high, bar.low, bar.close);
                            }
                            println!("\nLast 5 bars:");
                            for bar in series.bars.iter().rev().take(5) {
                                println!("  ts={}: O={:.2} H={:.2} L={:.2} C={:.2}", 
                                    bar.time.unix_timestamp(), bar.open, bar.high, bar.low, bar.close);
                            }
                        }
                        Err(e) => println!("Error getting history: {}", e),
                    }
                }
                Err(e) => println!("Timeout waiting for history: {}", e),
            }
        }
        Err(e) => println!("Failed to create client: {}", e),
    }
}
