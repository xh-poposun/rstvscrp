use crate::client::TradingViewMcpClient;
use crate::tools::{
    CalculateMacdParams, CalculateMacdResponse, CalculateRsiParams, CalculateRsiResponse,
    ComputeMacdSignalParams, ComputeMacdSignalResponse,
};

/// Calculate RSI for a symbol
pub async fn handle_calculate_rsi(
    client: &TradingViewMcpClient,
    params: CalculateRsiParams,
) -> Result<CalculateRsiResponse, Box<dyn std::error::Error>> {
    let rsi = client.calculate_rsi(&params.symbol, params.period).await?;
    Ok(CalculateRsiResponse { symbol: params.symbol, rsi })
}

/// Calculate MACD for a symbol
pub async fn handle_calculate_macd(
    client: &TradingViewMcpClient,
    params: CalculateMacdParams,
) -> Result<CalculateMacdResponse, Box<dyn std::error::Error>> {
    let macd = client
        .calculate_macd(
            &params.symbol,
            params.fast_period,
            params.slow_period,
            params.signal_period,
        )
        .await?;
    Ok(CalculateMacdResponse {
        symbol: params.symbol,
        macd: macd.macd_line,
    })
}

/// Compute MACD signal from MACD values
pub async fn handle_compute_macd_signal(
    client: &TradingViewMcpClient,
    params: ComputeMacdSignalParams,
) -> Result<ComputeMacdSignalResponse, Box<dyn std::error::Error>> {
    let signal = client.compute_macd_signal(&params.macd_values).await?;
    Ok(ComputeMacdSignalResponse { signal })
}
