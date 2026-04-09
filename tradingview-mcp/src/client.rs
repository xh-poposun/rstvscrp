use std::env;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, Semaphore};
use tokio::time::{Instant, interval, sleep_until};
use tracing::{debug, warn};

use tvdata_rs::{
    AuthConfig, TradingViewClient, TradingViewClientConfig,
    calendar::{CalendarWindowRequest, DividendCalendarRequest, DividendDateKind},
    history::{HistoryRequest, Interval},
    scanner::{ScanQuery, Ticker, fields::{core, price}},
    search::SearchRequest,
};

/// Configuration for the TradingView API client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// TradingView username (optional, for authenticated requests)
    pub username: Option<String>,
    /// TradingView password (optional, for authenticated requests)
    pub password: Option<String>,
    /// Session ID for authentication (alternative to username/password)
    pub session_id: Option<String>,
    /// Auth token for authentication
    pub auth_token: Option<String>,
    /// Rate limit: maximum requests per second (default: 10)
    pub rate_limit_per_sec: u32,
    /// Maximum number of concurrent connections
    pub max_concurrent_connections: usize,
    /// Maximum retry attempts for failed requests
    pub max_retries: u32,
    /// Initial retry delay in milliseconds
    pub retry_initial_delay_ms: u64,
    /// Maximum retry delay in milliseconds
    pub retry_max_delay_ms: u64,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// HTTP proxy URL (optional)
    pub proxy_url: Option<String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            username: None,
            password: None,
            session_id: None,
            auth_token: None,
            rate_limit_per_sec: 10,
            max_concurrent_connections: 8,
            max_retries: 3,
            retry_initial_delay_ms: 250,
            retry_max_delay_ms: 2000,
            timeout_secs: 30,
            proxy_url: None,
        }
    }
}

impl ClientConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(username) = env::var("TVDATA_USERNAME") {
            config.username = Some(username);
        }

        if let Ok(password) = env::var("TVDATA_PASSWORD") {
            config.password = Some(password);
        }

        if let Ok(session_id) = env::var("TVDATA_SESSION_ID") {
            config.session_id = Some(session_id);
        }

        if let Ok(auth_token) = env::var("TVDATA_AUTH_TOKEN") {
            config.auth_token = Some(auth_token);
        }

        if let Ok(rate_limit) = env::var("TVDATA_RATE_LIMIT_PER_SEC") {
            if let Ok(rate) = rate_limit.parse() {
                config.rate_limit_per_sec = rate;
            }
        }

        if let Ok(max_conn) = env::var("TVDATA_MAX_CONNECTIONS") {
            if let Ok(conn) = max_conn.parse() {
                config.max_concurrent_connections = conn;
            }
        }

        if let Ok(retries) = env::var("TVDATA_MAX_RETRIES") {
            if let Ok(r) = retries.parse() {
                config.max_retries = r;
            }
        }

        if let Ok(timeout) = env::var("TVDATA_TIMEOUT_SECS") {
            if let Ok(t) = timeout.parse() {
                config.timeout_secs = t;
            }
        }

        if let Ok(proxy) = env::var("TVDATA_PROXY_URL") {
            config.proxy_url = Some(proxy);
        }

        // Also check standard proxy environment variables
        if config.proxy_url.is_none() {
            if let Ok(https_proxy) = env::var("HTTPS_PROXY") {
                config.proxy_url = Some(https_proxy);
            } else if let Ok(http_proxy) = env::var("HTTP_PROXY") {
                config.proxy_url = Some(http_proxy);
            }
        }

        config
    }

    /// Build authentication configuration
    fn build_auth_config(&self) -> AuthConfig {
        if let Some(session_id) = &self.session_id {
            if let Some(auth_token) = &self.auth_token {
                AuthConfig::session_and_token(session_id.clone(), auth_token.clone())
            } else {
                AuthConfig::session(session_id.clone())
            }
        } else if let Some(auth_token) = &self.auth_token {
            AuthConfig::token(auth_token.clone())
        } else {
            AuthConfig::anonymous()
        }
    }
}

/// Rate limiter for controlling request rate
#[derive(Debug, Clone)]
struct RateLimiter {
    /// Minimum interval between requests
    min_interval: Duration,
    /// Last request timestamp
    last_request: Arc<Mutex<Instant>>,
    /// Semaphore for limiting concurrent requests
    semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    fn new(requests_per_second: u32, max_concurrent: usize) -> Self {
        let min_interval = Duration::from_secs_f64(1.0 / requests_per_second as f64);
        Self {
            min_interval,
            last_request: Arc::new(Mutex::new(Instant::now() - min_interval)),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    /// Acquire permission to make a request, respecting rate limits
    async fn acquire(&self) -> tokio::sync::OwnedSemaphorePermit {
        // Acquire semaphore permit for concurrency control
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore should not be closed");

        // Enforce rate limiting
        let mut last_request = self.last_request.lock().await;
        let now = Instant::now();
        let next_allowed = *last_request + self.min_interval;

        if next_allowed > now {
            let wait_duration = next_allowed - now;
            debug!("Rate limiting: waiting {:?} before next request", wait_duration);
            sleep_until(next_allowed).await;
        }

        *last_request = Instant::now();
        drop(last_request);

        permit
    }
}

/// Quote data structure
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

/// Fundamentals data structure
#[derive(Debug, Clone)]
pub struct Fundamentals {
    pub symbol: String,
    pub market_cap: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub eps: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub beta: Option<f64>,
    pub price_to_book: Option<f64>,
    pub debt_to_equity: Option<f64>,
    pub current_ratio: Option<f64>,
    pub quick_ratio: Option<f64>,
    pub roe: Option<f64>,
    pub roa: Option<f64>,
    pub revenue: Option<f64>,
    pub gross_profit: Option<f64>,
    pub operating_income: Option<f64>,
    pub net_income: Option<f64>,
    pub buyback_yield: Option<f64>,
    pub share_buyback_ratio_fq: Option<f64>,
    pub share_buyback_ratio_fy: Option<f64>,
    pub total_shares_outstanding: Option<f64>,
    pub total_shares_outstanding_current: Option<f64>,
    pub diluted_shares_outstanding_fq: Option<f64>,
    pub float_shares_outstanding: Option<f64>,
    pub shares_outstanding: Option<f64>,
    pub total_shares_outstanding_calculated: Option<f64>,
}

/// Search result structure
#[derive(Debug, Clone)]
pub struct SymbolSearchResult {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub asset_class: String,
}

/// Scanned stock result from the scanner API
#[derive(Debug, Clone)]
pub struct ScannedStock {
    pub symbol: String,
    pub name: Option<String>,
    pub exchange: Option<String>,
    pub price: Option<f64>,
}

/// Earnings calendar entry
#[derive(Debug, Clone)]
pub struct EarningsCalendarEntry {
    pub symbol: String,
    pub date: time::OffsetDateTime,
    pub eps_estimate: Option<f64>,
}

/// Dividend calendar entry
#[derive(Debug, Clone)]
pub struct DividendCalendarEntry {
    pub symbol: String,
    pub date: time::OffsetDateTime,
    pub amount: Option<f64>,
}

/// TradingView API client wrapper with rate limiting and retry logic
#[derive(Debug, Clone)]
pub struct TradingViewMcpClient {
    inner: Arc<TradingViewClient>,
    rate_limiter: RateLimiter,
    config: ClientConfig,
}

impl TradingViewMcpClient {
    /// Create a new client with default configuration
    pub async fn new() -> Result<Self, ClientError> {
        Self::with_config(ClientConfig::default()).await
    }

    /// Create a new client with configuration loaded from environment variables
    pub async fn from_env() -> Result<Self, ClientError> {
        Self::with_config(ClientConfig::from_env()).await
    }

    /// Create a new client with custom configuration
    pub async fn with_config(config: ClientConfig) -> Result<Self, ClientError> {
        let rate_limiter = RateLimiter::new(
            config.rate_limit_per_sec,
            config.max_concurrent_connections,
        );

        let tv_config = TradingViewClientConfig::backend_history();

        let client = TradingViewClient::from_config(tv_config)
            .map_err(|e| ClientError::Initialization(format!("Failed to create TradingView client: {}", e)))?;

        Ok(Self {
            inner: Arc::new(client),
            rate_limiter,
            config,
        })
    }

    /// Get a quote for a single symbol
    pub async fn get_quote(&self, symbol: &str) -> Result<Quote, ClientError> {
        self.execute_with_retry(|| async {
            let _permit = self.rate_limiter.acquire().await;

            let ticker: Ticker = symbol.to_string().into();
            let request = HistoryRequest::max(ticker, Interval::Day1);

            let series = self
                .inner
                .history(&request)
                .await
                .map_err(|e| ClientError::Api(format!("Failed to fetch quote: {}", e)))?;

            let bar = series
                .bars
                .last()
                .ok_or_else(|| ClientError::Api("No data available for symbol".to_string()))?;

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

            Ok(Quote {
                symbol: symbol.to_string(),
                price,
                change,
                change_percent,
                volume: bar.volume.unwrap_or(0.0) as u64,
                high: bar.high,
                low: bar.low,
                open: bar.open,
                previous_close,
            })
        })
        .await
    }

    /// Get quotes for multiple symbols
    pub async fn get_quotes(&self, symbols: &[String]) -> Result<Vec<Quote>, ClientError> {
        let mut quotes = Vec::with_capacity(symbols.len());

        for symbol in symbols {
            match self.get_quote(symbol).await {
                Ok(quote) => quotes.push(quote),
                Err(e) => {
                    warn!("Failed to get quote for {}: {}", symbol, e);
                    // Continue with other symbols even if one fails
                }
            }
        }

        Ok(quotes)
    }

    /// Get fundamentals for a single symbol
    pub async fn get_fundamentals(&self, symbol: &str) -> Result<Fundamentals, ClientError> {
        let symbol = symbol.to_string();
        let inner = Arc::clone(&self.inner);
        let rate_limiter = self.rate_limiter.clone();
        self.execute_with_retry(move || {
            let symbol = symbol.clone();
            let inner = Arc::clone(&inner);
            let rate_limiter = rate_limiter.clone();
            async move {
                let _permit = rate_limiter.acquire().await;

                let equity = inner.equity();
                let overview = equity
                    .overview(symbol.clone())
                    .await
                    .map_err(|e| ClientError::Api(format!("Failed to fetch fundamentals: {}", e)))?;

                Ok(Fundamentals {
                    symbol: symbol.clone(),
                    market_cap: overview.fundamentals.market_cap,
                    pe_ratio: overview.fundamentals.price_earnings_ttm,
                    eps: overview.fundamentals.eps_ttm,
                    dividend_yield: overview.fundamentals.dividend_yield_recent,
                    beta: None,
                    price_to_book: overview.fundamentals.price_to_book_fq,
                    debt_to_equity: overview.fundamentals.debt_to_equity_mrq,
                    current_ratio: overview.fundamentals.current_ratio_mrq,
                    quick_ratio: None,
                    roe: overview.fundamentals.return_on_equity_ttm,
                    roa: overview.fundamentals.return_on_assets_ttm,
                    revenue: overview.fundamentals.total_revenue_ttm,
                    gross_profit: None,
                    operating_income: None,
                    net_income: overview.fundamentals.net_income_ttm,
                    buyback_yield: overview.buyback.buyback_yield,
                    share_buyback_ratio_fq: overview.buyback.share_buyback_ratio_fq,
                    share_buyback_ratio_fy: overview.buyback.share_buyback_ratio_fy,
                    total_shares_outstanding: overview.buyback.total_shares_outstanding,
                    total_shares_outstanding_current: overview.buyback.total_shares_outstanding_current,
                    diluted_shares_outstanding_fq: overview.buyback.diluted_shares_outstanding_fq,
                    float_shares_outstanding: overview.buyback.float_shares_outstanding,
                    shares_outstanding: overview.buyback.shares_outstanding,
                    total_shares_outstanding_calculated: overview.buyback.total_shares_outstanding_calculated,
                })
            }
        })
        .await
    }

    /// Search for symbols
    pub async fn search_symbols(&self, query: &str) -> Result<Vec<SymbolSearchResult>, ClientError> {
        self.execute_with_retry(|| async {
            let _permit = self.rate_limiter.acquire().await;

            let request = SearchRequest::builder()
                .text(query)
                .build();

            let hits = self
                .inner
                .search(&request)
                .await
                .map_err(|e| ClientError::Api(format!("Failed to search symbols: {}", e)))?;

        let results = hits
            .into_iter()
            .map(|hit| SymbolSearchResult {
                symbol: hit.symbol,
                name: hit.description.unwrap_or_default(),
                exchange: hit.exchange.unwrap_or_default(),
                asset_class: hit.instrument_type.unwrap_or_default(),
            })
            .collect();

        Ok(results)
    })
    .await
}

/// Search for equities specifically
pub async fn search_equities(&self, query: &str) -> Result<Vec<SymbolSearchResult>, ClientError> {
    self.execute_with_retry(|| async {
        let _permit = self.rate_limiter.acquire().await;

        let hits = self
            .inner
            .search_equities(query)
            .await
            .map_err(|e| ClientError::Api(format!("Failed to search equities: {}", e)))?;

        let results = hits
            .into_iter()
            .map(|hit| SymbolSearchResult {
                symbol: hit.symbol,
                name: hit.description.unwrap_or_default(),
                exchange: hit.exchange.unwrap_or_default(),
                asset_class: hit.instrument_type.unwrap_or_default(),
            })
            .collect();

            Ok(results)
        })
        .await
    }

    pub async fn scan_stocks(
        &self,
        filters: Option<serde_json::Value>,
        limit: u32,
    ) -> Result<Vec<ScannedStock>, ClientError> {
        let market = filters
            .as_ref()
            .and_then(|f| f.get("market"))
            .and_then(|v| v.as_str())
            .unwrap_or("america")
            .to_string();

        self.execute_with_retry(move || {
            let market = market.clone();
            async move {
                let _permit = self.rate_limiter.acquire().await;

                let columns = vec![
                    core::NAME,
                    core::EXCHANGE,
                    price::CLOSE,
                ];

                let query = ScanQuery::new()
                    .market(market)
                    .select(columns)
                    .page(0, limit as usize)
                    .map_err(|e| ClientError::Api(format!("Invalid page parameters: {}", e)))?;

                let response = self
                    .inner
                    .scan(&query)
                    .await
                    .map_err(|e| ClientError::Api(format!("Failed to scan stocks: {}", e)))?;

                let results = response
                    .rows
                    .into_iter()
                    .map(|row| {
                        let name = row.values.get(0).and_then(|v| v.as_str()).map(String::from);
                        let exchange = row.values.get(1).and_then(|v| v.as_str()).map(String::from);
                        let price = row.values.get(2).and_then(|v| v.as_f64());

                        ScannedStock {
                            symbol: row.symbol,
                            name,
                            exchange,
                            price,
                        }
                    })
                    .collect();

                Ok(results)
            }
        })
        .await
    }

    /// Execute an operation with retry logic and exponential backoff
    async fn execute_with_retry<F, Fut, T>(&self, mut operation: F) -> Result<T, ClientError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, ClientError>>,
    {
        let mut attempt = 0;
        let mut delay = Duration::from_millis(self.config.retry_initial_delay_ms);
        let max_delay = Duration::from_millis(self.config.retry_max_delay_ms);

        loop {
            attempt += 1;

            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    // Don't retry on certain errors
                    if let ClientError::Api(msg) = &e {
                        if msg.contains("not found") || msg.contains("invalid") {
                            return Err(e);
                        }
                    }

                    if attempt >= self.config.max_retries {
                        return Err(e);
                    }

                    warn!(
                        "Request failed (attempt {}/{}): {}. Retrying in {:?}...",
                        attempt, self.config.max_retries, e, delay
                    );

                    tokio::time::sleep(delay).await;

                    // Exponential backoff with jitter
                    delay = std::cmp::min(delay * 2, max_delay);
                    // Add small random jitter (0-100ms) to prevent thundering herd
                    let jitter = Duration::from_millis(rand::random::<u64>() % 100);
                    delay += jitter;
                }
            }
        }
    }

    /// Get detailed financial statements for a symbol
    pub async fn get_financial_statements_detail(
        &self,
        symbol: &str,
    ) -> Result<tvdata_rs::equity::FinancialStatementsDetail, ClientError> {
        let symbol = symbol.to_string();
        self.execute_with_retry(|| async {
            let _permit = self.rate_limiter.acquire().await;

            let equity = self.inner.equity();
            let detail = equity
                .financial_statements_detail(symbol.clone())
                .await
                .map_err(|e| ClientError::Api(format!("Failed to fetch financial statements: {}", e)))?;

            Ok(detail)
        })
        .await
    }

    /// Get credit ratings for a symbol
    pub async fn get_credit_ratings(
        &self,
        symbol: &str,
    ) -> Result<tvdata_rs::equity::CreditRatingSnapshot, ClientError> {
        let symbol = symbol.to_string();
        self.execute_with_retry(|| async {
            let _permit = self.rate_limiter.acquire().await;

            let equity = self.inner.equity();
            let ratings = equity
                .credit_ratings(symbol.clone())
                .await
                .map_err(|e| ClientError::Api(format!("Failed to fetch credit ratings: {}", e)))?;

            Ok(ratings)
        })
        .await
    }

    /// Get company profile overview for a symbol
    pub async fn get_company_profile(
        &self,
        symbol: &str,
    ) -> Result<tvdata_rs::equity::EquityOverview, ClientError> {
        let symbol = symbol.to_string();
        self.execute_with_retry(|| async {
            let _permit = self.rate_limiter.acquire().await;

            let equity = self.inner.equity();
            let profile = equity
                .overview(symbol.clone())
                .await
                .map_err(|e| ClientError::Api(format!("Failed to fetch company profile: {}", e)))?;

            Ok(profile)
        })
        .await
    }

    /// Get debt maturity/summary for a symbol
    pub async fn get_debt_maturity(
        &self,
        symbol: &str,
    ) -> Result<tvdata_rs::equity::DebtDetail, ClientError> {
        let symbol = symbol.to_string();
        self.execute_with_retry(|| async {
            let _permit = self.rate_limiter.acquire().await;

            let equity = self.inner.equity();
            let debt = equity
                .debt_detail(symbol.clone())
                .await
                .map_err(|e| ClientError::Api(format!("Failed to fetch debt maturity: {}", e)))?;

            Ok(debt)
        })
        .await
    }

    /// Get earnings calendar for upcoming earnings announcements
    pub async fn get_earnings_calendar(
        &self,
        days_ahead: i64,
    ) -> Result<Vec<EarningsCalendarEntry>, ClientError> {
        let days_ahead = days_ahead;
        self.execute_with_retry(move || async move {
            let _permit = self.rate_limiter.acquire().await;

            let request = CalendarWindowRequest::upcoming("US", days_ahead);
            let entries = self
                .inner
                .earnings_calendar(&request)
                .await
                .map_err(|e| ClientError::Api(format!("Failed to fetch earnings calendar: {}", e)))?;

        let results = entries
            .into_iter()
            .map(|entry| EarningsCalendarEntry {
                symbol: entry.instrument.ticker.to_string(),
                date: entry.release_at,
                eps_estimate: entry.eps_forecast_next_fq,
            })
            .collect();

        Ok(results)
    })
    .await
}

    /// Get dividend calendar for upcoming dividend dates
    pub async fn get_dividend_calendar(
        &self,
        exchange: &str,
        days_ahead: i64,
    ) -> Result<Vec<DividendCalendarEntry>, ClientError> {
        let exchange = exchange.to_string();
        let days_ahead = days_ahead;
        self.execute_with_retry(|| {
            let exchange = exchange.clone();
            async move {
                let _permit = self.rate_limiter.acquire().await;

                let request = DividendCalendarRequest::upcoming(exchange, days_ahead)
                    .date_kind(DividendDateKind::ExDate);
                let entries = self
                    .inner
                    .dividend_calendar(&request)
                    .await
                    .map_err(|e| ClientError::Api(format!("Failed to fetch dividend calendar: {}", e)))?;

                let results = entries
                    .into_iter()
                    .map(|entry| DividendCalendarEntry {
                        symbol: entry.instrument.ticker.to_string(),
                        date: entry.effective_date,
                        amount: entry.amount,
                    })
                    .collect();

                Ok(results)
            }
        })
        .await
    }

/// Get historical OHLCV data for a symbol
    pub async fn get_historical(
        &self,
        symbol: &str,
        interval: Interval,
        bars: u32,
    ) -> Result<tvdata_rs::history::HistorySeries, ClientError> {
        self.execute_with_retry(|| async {
            let _permit = self.rate_limiter.acquire().await;

            let ticker: Ticker = symbol.to_string().into();
            let request = HistoryRequest::new(ticker, interval, bars);

            let series = self
                .inner
                .history(&request)
                .await
                .map_err(|e| ClientError::Api(format!("Failed to fetch historical data: {}", e)))?;

            Ok(series)
        })
        .await
    }

    /// Get the inner TradingView client for advanced usage
    pub fn inner(&self) -> &TradingViewClient {
        &self.inner
    }

    /// Get the client configuration
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Calculate RSI for a symbol
    pub async fn calculate_rsi(&self, symbol: &str, period: Option<u32>) -> Result<Option<f64>, ClientError> {
        let _ = symbol;
        let _ = period;
        Ok(None)
    }

    /// Calculate MACD for a symbol
    pub async fn calculate_macd(
        &self,
        symbol: &str,
        fast_period: Option<u32>,
        slow_period: Option<u32>,
        signal_period: Option<u32>,
    ) -> Result<MacdResult, ClientError> {
        let _ = symbol;
        let _ = fast_period;
        let _ = slow_period;
        let _ = signal_period;
        Ok(MacdResult {
            macd_line: None,
            signal_line: None,
            histogram: None,
        })
    }

    /// Compute MACD signal from MACD values
    pub async fn compute_macd_signal(&self, macd_values: &[f64]) -> Result<Option<Vec<f64>>, ClientError> {
        let _ = macd_values;
        Ok(None)
    }
}

/// MACD calculation result
#[derive(Debug, Clone)]
pub struct MacdResult {
    pub macd_line: Option<Vec<f64>>,
    pub signal_line: Option<Vec<f64>>,
    pub histogram: Option<Vec<f64>>,
}

/// Client error types
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Initialization error: {0}")]
    Initialization(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Timeout error")]
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert_eq!(config.rate_limit_per_sec, 10);
        assert_eq!(config.max_concurrent_connections, 8);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_rate_limiter_creation() {
        let limiter = RateLimiter::new(10, 8);
        assert_eq!(limiter.min_interval, Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_client_config_from_env() {
        // This test just ensures the function doesn't panic
        let _config = ClientConfig::from_env();
    }
}
