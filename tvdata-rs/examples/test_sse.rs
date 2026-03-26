use tvdata_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = TradingViewClient::builder().build()?;
    let history = client.history(&HistoryRequest::new("SSE:600941", Interval::Day1, 5)).await?;
    println!("Got {} bars", history.bars.len());
    Ok(())
}
