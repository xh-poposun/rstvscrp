use reqwest::{Client, header};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::Instant;

/// SEC EDGAR API client with rate limiting
/// Enforces 10 requests per second as required by SEC
#[derive(Clone)]
pub struct SecEdgarClient {
    client: Client,
    rate_limiter: Arc<RateLimiter>,
    user_agent: String,
}

/// Rate limiter using token bucket algorithm
/// SEC requires max 10 requests per second
struct RateLimiter {
    semaphore: Semaphore,
    last_request: tokio::sync::Mutex<Instant>,
    min_interval: Duration,
}

impl RateLimiter {
    fn new(requests_per_second: u32) -> Self {
        Self {
            semaphore: Semaphore::new(requests_per_second as usize),
            last_request: tokio::sync::Mutex::new(Instant::now()),
            min_interval: Duration::from_millis(100), // 100ms = 10 req/sec
        }
    }

    async fn acquire(&self) {
        let _permit = self.semaphore.acquire().await.unwrap();
        
        let mut last = self.last_request.lock().await;
        let now = Instant::now();
        let elapsed = now.duration_since(*last);
        
        if elapsed < self.min_interval {
            tokio::time::sleep(self.min_interval - elapsed).await;
        }
        
        *last = Instant::now();
    }
}

impl SecEdgarClient {
    /// Create a new SEC EDGAR client
    ///
    /// # Arguments
    /// * `user_agent` - Required by SEC, format: "CompanyName ContactEmail"
    pub fn new(user_agent: String) -> Result<Self, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&user_agent).unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            rate_limiter: Arc::new(RateLimiter::new(10)),
            user_agent,
        })
    }

    /// Get the underlying HTTP client
    pub fn inner(&self) -> &Client {
        &self.client
    }

    /// Get the user agent string
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Make a rate-limited GET request
    pub async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.rate_limiter.acquire().await;
        self.client.get(url).send().await
    }

    /// Make a rate-limited GET request with query parameters
    pub async fn get_with_params<T: serde::Serialize + ?Sized>(
        &self,
        url: &str,
        params: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.rate_limiter.acquire().await;
        self.client.get(url).query(params).send().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = SecEdgarClient::new("TestCompany test@example.com".to_string());
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let client = SecEdgarClient::new("TestCompany test@example.com".to_string()).unwrap();
        let start = Instant::now();
        
        // Make 11 requests - should take at least 1 second due to rate limiting
        for _ in 0..11 {
            let _ = client.get("https://httpbin.org/get").await;
        }
        
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(1000), "Rate limiting not enforced");
    }
}
