use governor::clock::DefaultClock;
use governor::state::keyed::DefaultKeyedStateStore;
use governor::{Quota, RateLimiter};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

const SEC_BASE_URL: &str = "https://www.sec.gov";
const SEC_DATA_URL: &str = "https://data.sec.gov";
const USER_AGENT_STR: &str = "tradingview-mcp/1.0 (contact@example.com)";

#[derive(Error, Debug)]
pub enum SecEdgarError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("CIK not found for ticker: {0}")]
    CikNotFound(String),
    #[error("No 10-K filing found for CIK: {0}")]
    No10kFound(String),
    #[error("Data extraction failed: {0}")]
    ExtractionFailed(String),
}

pub type Result<T> = std::result::Result<T, SecEdgarError>;

/// Debt maturity schedule from 10-K footnotes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtMaturitySchedule {
    pub maturities: Vec<DebtMaturity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtMaturity {
    pub year: i32,
    pub amount: f64,
}

/// Segment revenue breakdown from 10-K
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentRevenue {
    pub segment_name: String,
    pub revenue: f64,
}

/// Filing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filing {
    pub accession_number: String,
    pub filing_date: String,
    pub form: String,
    pub primary_document: String,
}

/// SEC EDGAR API client with rate limiting
pub struct SecEdgarClient {
    client: reqwest::Client,
    rate_limiter: Arc<RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>>,
}

impl SecEdgarClient {
    /// Create a new SEC EDGAR client with rate limiting
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(USER_AGENT_STR),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        // Rate limit: 10 requests per second
        let quota = Quota::per_second(NonZeroU32::new(10).unwrap());
        let rate_limiter = Arc::new(RateLimiter::keyed(quota));

        Ok(Self {
            client,
            rate_limiter,
        })
    }

    /// Check rate limit before making request
    async fn check_rate_limit(&self, key: &str) -> Result<()> {
        match self.rate_limiter.check_key(&key.to_string()) {
            Ok(_) => Ok(()),
            Err(_) => Err(SecEdgarError::RateLimitExceeded),
        }
    }

    /// Lookup CIK from ticker symbol
    /// Uses SEC's company_tickers.json endpoint
    pub async fn lookup_cik(&self, ticker: &str) -> Result<Option<String>> {
        self.check_rate_limit("cik_lookup").await?;

        let url = format!("{}/files/company_tickers.json", SEC_BASE_URL);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SecEdgarError::HttpError(
                response.error_for_status().unwrap_err()
            ));
        }

        let data: serde_json::Value = response.json().await?;
        
        // company_tickers.json is an object with numeric keys
        if let Some(obj) = data.as_object() {
            for (_, value) in obj {
                if let Some(ticker_str) = value.get("ticker").and_then(|t| t.as_str()) {
                    if ticker_str.eq_ignore_ascii_case(ticker) {
                        if let Some(cik) = value.get("cik_str").and_then(|c| c.as_str()) {
                            return Ok(Some(cik.to_string()));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Get latest 10-K filing for a CIK
    pub async fn get_latest_10k(&self, cik: &str) -> Result<Option<Filing>> {
        self.check_rate_limit("submissions").await?;

        // Pad CIK to 10 digits
        let padded_cik = format!("{:0>10}", cik);
        let url = format!("{}/submissions/CIK{}.json", SEC_DATA_URL, padded_cik);
        
        let response = self.client.get(&url).send().await?;
        
        if response.status().as_u16() == 404 {
            return Ok(None);
        }
        
        if !response.status().is_success() {
            return Err(SecEdgarError::HttpError(
                response.error_for_status().unwrap_err()
            ));
        }

        let data: serde_json::Value = response.json().await?;
        
        // Find latest 10-K in recent filings
        if let Some(recent) = data.get("recent") {
            if let Some(forms) = recent.get("form").and_then(|f| f.as_array()) {
                if let Some(dates) = recent.get("filingDate").and_then(|d| d.as_array()) {
                    if let Some(accessions) = recent.get("accessionNumber").and_then(|a| a.as_array()) {
                        if let Some(docs) = recent.get("primaryDocument").and_then(|p| p.as_array()) {
                            for (i, form) in forms.iter().enumerate() {
                                if let Some(form_str) = form.as_str() {
                                    if form_str == "10-K" {
                                        if let (Some(date), Some(acc), Some(doc)) = (
                                            dates.get(i).and_then(|d| d.as_str()),
                                            accessions.get(i).and_then(|a| a.as_str()),
                                            docs.get(i).and_then(|d| d.as_str()),
                                        ) {
                                            return Ok(Some(Filing {
                                                accession_number: acc.to_string(),
                                                filing_date: date.to_string(),
                                                form: form_str.to_string(),
                                                primary_document: doc.to_string(),
                                            }));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Get company facts (XBRL data) for a CIK
    async fn get_company_facts(&self, cik: &str) -> Result<Option<serde_json::Value>> {
        self.check_rate_limit("companyfacts").await?;

        let padded_cik = format!("{:0>10}", cik);
        let url = format!("{}/api/xbrl/companyfacts/CIK{}.json", SEC_DATA_URL, padded_cik);
        
        let response = self.client.get(&url).send().await?;
        
        if response.status().as_u16() == 404 {
            return Ok(None);
        }
        
        if !response.status().is_success() {
            return Err(SecEdgarError::HttpError(
                response.error_for_status().unwrap_err()
            ));
        }

        let data: serde_json::Value = response.json().await?;
        Ok(Some(data))
    }

    /// Extract debt maturity schedule from 10-K
    /// Looks for LongTermDebtMaturities and DebtMaturitySchedule XBRL tags
    pub async fn get_debt_maturity(&self, ticker: &str) -> Result<Option<DebtMaturitySchedule>> {
        let cik = match self.lookup_cik(ticker).await? {
            Some(cik) => cik,
            None => return Err(SecEdgarError::CikNotFound(ticker.to_string())),
        };

        let facts = match self.get_company_facts(&cik).await? {
            Some(facts) => facts,
            None => return Ok(None),
        };

        let mut maturities: Vec<DebtMaturity> = Vec::new();

        // Look for debt maturity tags in facts
        if let Some(us_gaap) = facts.get("facts").and_then(|f| f.get("us-gaap")) {
            // Try LongTermDebtMaturities first
            if let Some(debt_data) = us_gaap.get("LongTermDebtMaturities") {
                if let Some(units) = debt_data.get("units") {
                    if let Some(usd) = units.get("USD") {
                        if let Some(arr) = usd.as_array() {
                            for item in arr {
                                if let (Some(year_str), Some(val)) = (
                                    item.get("fp").and_then(|y| y.as_str()),
                                    item.get("val").and_then(|v| v.as_f64()),
                                ) {
                                    // Extract year from fiscal period (e.g., "FY2023" or "2023")
                                    let year = Self::extract_year(year_str);
                                    if year > 0 {
                                        maturities.push(DebtMaturity { year, amount: val });
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Try DebtMaturitySchedule as fallback
            if maturities.is_empty() {
                if let Some(debt_data) = us_gaap.get("DebtMaturitySchedule") {
                    if let Some(units) = debt_data.get("units") {
                        if let Some(usd) = units.get("USD") {
                            if let Some(arr) = usd.as_array() {
                                for item in arr {
                                    if let (Some(year_str), Some(val)) = (
                                        item.get("fp").and_then(|y| y.as_str()),
                                        item.get("val").and_then(|v| v.as_f64()),
                                    ) {
                                        let year = Self::extract_year(year_str);
                                        if year > 0 {
                                            maturities.push(DebtMaturity { year, amount: val });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if maturities.is_empty() {
            return Ok(None);
        }

        // Sort by year
        maturities.sort_by(|a, b| a.year.cmp(&b.year));

        Ok(Some(DebtMaturitySchedule { maturities }))
    }

    /// Extract segment revenue breakdown from 10-K
    /// Looks for SegmentRevenue and RevenueFromContractWithCustomerBySegment tags
    pub async fn get_segment_revenue(&self, ticker: &str) -> Result<Option<Vec<SegmentRevenue>>> {
        let cik = match self.lookup_cik(ticker).await? {
            Some(cik) => cik,
            None => return Err(SecEdgarError::CikNotFound(ticker.to_string())),
        };

        let facts = match self.get_company_facts(&cik).await? {
            Some(facts) => facts,
            None => return Ok(None),
        };

        let mut segments: Vec<SegmentRevenue> = Vec::new();

        if let Some(us_gaap) = facts.get("facts").and_then(|f| f.get("us-gaap")) {
            // Try SegmentRevenue first
            if let Some(segment_data) = us_gaap.get("SegmentRevenue") {
                segments = Self::extract_segments_from_data(segment_data)?;
            }

            // Try RevenueFromContractWithCustomerBySegment as fallback
            if segments.is_empty() {
                if let Some(segment_data) = us_gaap.get("RevenueFromContractWithCustomerBySegment") {
                    segments = Self::extract_segments_from_data(segment_data)?;
                }
            }

            // Try RevenueBySegment as another fallback
            if segments.is_empty() {
                if let Some(segment_data) = us_gaap.get("RevenueBySegment") {
                    segments = Self::extract_segments_from_data(segment_data)?;
                }
            }
        }

        if segments.is_empty() {
            return Ok(None);
        }

        Ok(Some(segments))
    }

    /// Extract segments from XBRL data
    fn extract_segments_from_data(segment_data: &serde_json::Value) -> Result<Vec<SegmentRevenue>> {
        let mut segments: Vec<SegmentRevenue> = Vec::new();

        if let Some(units) = segment_data.get("units") {
            // Try USD first, then pure numbers
            let unit_key = if units.get("USD").is_some() {
                "USD"
            } else if units.get("pure").is_some() {
                "pure"
            } else {
                // Get first available unit
                units.as_object()
                    .and_then(|o| o.keys().next().map(|k| k.as_str()))
                    .unwrap_or("USD")
            };

            if let Some(data_arr) = units.get(unit_key).and_then(|u| u.as_array()) {
                for item in data_arr {
                    // Look for segment dimension
                    if let Some(segment_name) = item
                        .get("segment")
                        .and_then(|s| s.as_str())
                        .or_else(|| item.get("dim").and_then(|d| d.as_str()))
                    {
                        if let Some(val) = item.get("val").and_then(|v| v.as_f64()) {
                            // Only add if not already present (avoid duplicates)
                            if !segments.iter().any(|s| s.segment_name == segment_name) {
                                segments.push(SegmentRevenue {
                                    segment_name: segment_name.to_string(),
                                    revenue: val,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(segments)
    }

    /// Extract year from fiscal period string
    fn extract_year(fp: &str) -> i32 {
        // Handle formats like "FY2023", "2023", "2023Q1", etc.
        let cleaned = fp.trim_start_matches("FY");
        cleaned[..4.min(cleaned.len())]
            .parse::<i32>()
            .unwrap_or(0)
    }
}

impl Default for SecEdgarClient {
    fn default() -> Self {
        Self::new().expect("Failed to create SEC EDGAR client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_year() {
        assert_eq!(SecEdgarClient::extract_year("FY2023"), 2023);
        assert_eq!(SecEdgarClient::extract_year("2023"), 2023);
        assert_eq!(SecEdgarClient::extract_year("2023Q1"), 2023);
        assert_eq!(SecEdgarClient::extract_year("2024"), 2024);
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = SecEdgarClient::new();
        assert!(client.is_ok());
    }
}
