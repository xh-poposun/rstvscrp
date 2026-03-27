use std::sync::Arc;
use tvdata_rs::{
    TradingViewClient, TradingViewClientConfig,
    history::{HistoryRequest, Interval},
    scanner::Ticker,
    search::{SearchHit, SearchRequest},
};

#[derive(Debug)]
pub struct TvClient {
    client: Arc<TradingViewClient>,
}

#[derive(Debug, Clone)]
pub struct Quote {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: u64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub previous_close: f64,
}

impl TvClient {
    pub async fn new() -> Result<Self, String> {
        tracing::info!(
            "Initializing TvClient - using default reqwest client which respects HTTP_PROXY/HTTPS_PROXY environment variables"
        );

        let config = TradingViewClientConfig::backend_history();

        let client = TradingViewClient::from_config(config)
            .map_err(|e| format!("failed to create client: {}", e))?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn get_quotes(&self, symbols: &[&str]) -> Result<Vec<Quote>, String> {
        let mut quotes = Vec::new();

        for symbol in symbols {
            // Use max request like history refresh - works reliably for Chinese stocks
            let quote = self.get_quote_with_retry(symbol, 3).await;
            if let Some(q) = quote {
                quotes.push(q);
            }
        }

        Ok(quotes)
    }

    async fn get_quote_with_retry(&self, symbol: &str, max_retries: u32) -> Option<Quote> {
        for attempt in 1..=max_retries {
            let ticker: Ticker = symbol.to_string().into();
            let request = HistoryRequest::max(ticker, Interval::Day1);

            match self.client.history(&request).await {
                Ok(series) => {
                    if let Some(bar) = series.bars.last() {
                        let previous_close = series
                            .bars
                            .get(series.bars.len().saturating_sub(2))
                            .map(|b| b.close)
                            .unwrap_or(bar.close);

                        let price = bar.close;
                        let change = price - previous_close;
                        let change_percent = if previous_close > 0.0 {
                            (change / previous_close) * 100.0
                        } else {
                            0.0
                        };

                        return Some(Quote {
                            symbol: symbol.to_string(),
                            price,
                            change,
                            change_percent,
                            volume: bar.volume.unwrap_or(0.0) as u64,
                            high: bar.high,
                            low: bar.low,
                            open: bar.open,
                            previous_close,
                        });
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        "failed to get quote for {} (attempt {}/{}): {}",
                        symbol,
                        attempt,
                        max_retries,
                        e
                    );
                    if attempt < max_retries {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    }
                }
            }
        }
        None
    }

    pub async fn get_history(&self, symbol: &str, interval: &str) -> Result<Vec<Bar>, String> {
        let parsed_interval = parse_interval(interval)?;

        let ticker: Ticker = symbol.to_string().into();
        let request = HistoryRequest::max(ticker, parsed_interval);

        let series = self
            .client
            .history(&request)
            .await
            .map_err(|e| format!("failed to get history: {}", e))?;

        Ok(series
            .bars
            .into_iter()
            .map(|b| Bar {
                timestamp: b.time.unix_timestamp(),
                open: b.open,
                high: b.high,
                low: b.low,
                close: b.close,
                volume: b.volume.unwrap_or(0.0) as u64,
            })
            .collect())
    }

    pub async fn search_equities(&self, text: &str) -> Result<Vec<SearchHit>, String> {
        self.client
            .search_equities(text)
            .await
            .map_err(|e| format!("failed to search equities: {}", e))
    }

    pub async fn search_forex(&self, text: &str) -> Result<Vec<SearchHit>, String> {
        self.client
            .search_forex(text)
            .await
            .map_err(|e| format!("failed to search forex: {}", e))
    }

    pub async fn search_crypto(&self, text: &str) -> Result<Vec<SearchHit>, String> {
        self.client
            .search_crypto(text)
            .await
            .map_err(|e| format!("failed to search crypto: {}", e))
    }

    pub async fn search(&self, request: &SearchRequest) -> Result<Vec<SearchHit>, String> {
        self.client
            .search(request)
            .await
            .map_err(|e| format!("failed to search: {}", e))
    }
}

fn parse_interval(interval: &str) -> Result<Interval, String> {
    match interval {
        "1" | "1m" => Ok(Interval::Min1),
        "5" | "5m" => Ok(Interval::Min5),
        "15" | "15m" => Ok(Interval::Min15),
        "30" | "30m" => Ok(Interval::Min30),
        "60" | "1h" => Ok(Interval::Hour1),
        "240" | "4h" => Ok(Interval::Hour4),
        "D" | "1d" => Ok(Interval::Day1),
        "W" | "1w" => Ok(Interval::Week1),
        _ => Err(format!("unknown interval: {}", interval)),
    }
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_interval() {
        assert!(parse_interval("1").is_ok());
        assert!(parse_interval("1m").is_ok());
        assert!(parse_interval("D").is_ok());
        assert!(parse_interval("W").is_ok());
        assert!(parse_interval("invalid").is_err());
    }

    #[tokio::test]
    async fn test_client_new() {
        match TvClient::new().await {
            Ok(_) => {}
            Err(err) => {
                assert!(err.contains("failed to") || err.contains("connection"));
            }
        }
    }
}
