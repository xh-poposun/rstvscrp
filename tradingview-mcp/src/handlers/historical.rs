use crate::client::{TradingViewMcpClient, ClientError};
use crate::tools::{GetHistoricalParams, GetHistoricalResponse, HistoricalPoint};
use tvdata_rs::history::Interval;

fn parse_interval(interval_str: &str) -> Result<Interval, ClientError> {
    match interval_str {
        "1m" => Ok(Interval::Min1),
        "3m" => Ok(Interval::Min3),
        "5m" => Ok(Interval::Min5),
        "15m" => Ok(Interval::Min15),
        "30m" => Ok(Interval::Min30),
        "45m" => Ok(Interval::Min45),
        "1h" => Ok(Interval::Hour1),
        "2h" => Ok(Interval::Hour2),
        "3h" => Ok(Interval::Hour3),
        "4h" => Ok(Interval::Hour4),
        "1d" => Ok(Interval::Day1),
        "1w" => Ok(Interval::Week1),
        "1M" => Ok(Interval::Month1),
        _ => Err(ClientError::Api(format!(
            "Invalid interval: {}. Supported intervals: 1m, 3m, 5m, 15m, 30m, 45m, 1h, 2h, 3h, 4h, 1d, 1w, 1M",
            interval_str
        ))),
    }
}

pub async fn handle_get_historical(
    client: &TradingViewMcpClient,
    params: GetHistoricalParams,
) -> Result<GetHistoricalResponse, ClientError> {
    let interval = parse_interval(&params.interval)?;
    let count = params.count.unwrap_or(100);

    let series = client.get_historical(&params.symbol, interval, count).await?;

    let data: Vec<HistoricalPoint> = series
        .bars
        .into_iter()
        .map(|bar| HistoricalPoint {
            timestamp: bar.time.to_string(),
            open: bar.open,
            high: bar.high,
            low: bar.low,
            close: bar.close,
            volume: bar.volume,
        })
        .collect();

    Ok(GetHistoricalResponse {
        symbol: params.symbol,
        interval: params.interval,
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_interval_valid() {
        assert!(matches!(parse_interval("1m"), Ok(Interval::Min1)));
        assert!(matches!(parse_interval("5m"), Ok(Interval::Min5)));
        assert!(matches!(parse_interval("1h"), Ok(Interval::Hour1)));
        assert!(matches!(parse_interval("1d"), Ok(Interval::Day1)));
        assert!(matches!(parse_interval("1w"), Ok(Interval::Week1)));
        assert!(matches!(parse_interval("1M"), Ok(Interval::Month1)));
    }

    #[test]
    fn test_parse_interval_invalid() {
        assert!(parse_interval("invalid").is_err());
        assert!(parse_interval("10m").is_err());
        assert!(parse_interval("").is_err());
    }
}
