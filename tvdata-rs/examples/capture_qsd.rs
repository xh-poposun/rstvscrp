use tvdata_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let symbol = std::env::args().nth(1).unwrap_or_else(|| "NASDAQ:AAPL".to_string());
    let client = TradingViewClient::builder().build()?;
    let equity = client.equity();

    match equity.fundamentals_point_in_time(symbol.clone()).await {
        Ok(f) => {
            let first = f.quarterly.first();
            println!("FUNDAMENTALS_PIT OK: quarters={}", f.quarterly.len());
            if let Some(obs) = first {
                println!("  first_revenue={:?}", obs.value.total_revenue);
            }
        }
        Err(e) => println!("FUNDAMENTALS_PIT FAIL: {}", e),
    }

    match equity.estimate_history(symbol.clone()).await {
        Ok(h) => {
            let first = h.quarterly.first();
            println!("ESTIMATE_HISTORY OK: quarters={}", h.quarterly.len());
            if let Some(obs) = first {
                println!("  first_revenue={:?}", obs.value.revenue_forecast);
            }
        }
        Err(e) => println!("ESTIMATE_HISTORY FAIL: {}", e),
    }

    Ok(())
}
