use crate::client::SecEdgarClient;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during CIK lookup
#[derive(Error, Debug)]
pub enum CikLookupError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Ticker symbol not found: {0}")]
    TickerNotFound(String),
    #[error("Invalid CIK format")]
    InvalidCikFormat,
}

/// Company information from SEC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyInfo {
    /// Central Index Key (CIK) - 10 digit number
    pub cik: String,
    /// Company name
    pub name: String,
    /// Ticker symbol
    pub ticker: String,
    /// Exchange (e.g., NASDAQ, NYSE)
    pub exchange: Option<String>,
}

/// CIK lookup service
pub struct CikLookup {
    client: SecEdgarClient,
}

impl CikLookup {
    /// Create a new CIK lookup service
    pub fn new(client: SecEdgarClient) -> Self {
        Self { client }
    }

    /// Lookup CIK by ticker symbol
    /// 
    /// Queries the SEC API to get company information from a ticker symbol.
    /// 
    /// # Arguments
    /// * `ticker` - Stock ticker symbol (e.g., "AAPL", "MSFT")
    /// 
    /// # Returns
    /// * `Ok(CompanyInfo)` - Company information including CIK
    /// * `Err(CikLookupError)` - If ticker not found or request fails
    pub async fn lookup(&self, ticker: &str) -> Result<CompanyInfo, CikLookupError> {
        let ticker_upper = ticker.to_uppercase();
        
        // SEC ticker lookup endpoint
        let url = format!(
            "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK={}&type=10-K&output=atom",
            ticker_upper
        );

        let response = self.client.get(&url).await?;
        
        if !response.status().is_success() {
            return Err(CikLookupError::TickerNotFound(ticker_upper));
        }

        let text = response.text().await?;
        
        // Parse the Atom feed response to extract CIK
        self.parse_cik_from_feed(&text, &ticker_upper)
    }

    /// Lookup CIK using the SEC company tickers JSON file
    /// This is more reliable than parsing the Atom feed
    pub async fn lookup_from_tickers_json(&self, ticker: &str) -> Result<CompanyInfo, CikLookupError> {
        let ticker_upper = ticker.to_uppercase();
        
        // SEC provides a JSON file with all company tickers
        let url = "https://www.sec.gov/files/company_tickers.json";
        
        let response = self.client.get(url).await?;
        
        if !response.status().is_success() {
            return Err(CikLookupError::ParseError(
                "Failed to fetch company tickers".to_string()
            ));
        }

        let tickers_map: serde_json::Value = response.json().await?;
        
        // Search through the JSON for matching ticker
        if let Some(obj) = tickers_map.as_object() {
            for (_, value) in obj {
                if let Some(ticker_str) = value.get("ticker").and_then(|t| t.as_str()) {
                    if ticker_str == ticker_upper {
                        let cik = value.get("cik_str")
                            .and_then(|c| c.as_str())
                            .ok_or_else(|| CikLookupError::ParseError("Missing CIK".to_string()))?;
                        
                        let name = value.get("title")
                            .and_then(|n| n.as_str())
                            .unwrap_or("Unknown");
                        
                        return Ok(CompanyInfo {
                            cik: cik.to_string(),
                            name: name.to_string(),
                            ticker: ticker_upper,
                            exchange: None,
                        });
                    }
                }
            }
        }
        
        Err(CikLookupError::TickerNotFound(ticker_upper))
    }

    /// Parse CIK from SEC Atom feed response
    fn parse_cik_from_feed(&self, feed: &str, ticker: &str) -> Result<CompanyInfo, CikLookupError> {
        // Extract CIK from the feed using simple string parsing
        // The CIK appears in the company-info section
        for line in feed.lines() {
            if line.contains("<cik>") {
                let start = line.find("<cik>").unwrap_or(0) + 5;
                let end = line.find("</cik>").unwrap_or(line.len());
                if start < end {
                    let cik = line[start..end].trim().to_string();
                    
                    // Extract company name if available
                    let name = self.extract_company_name(feed).unwrap_or_default();
                    
                    return Ok(CompanyInfo {
                        cik,
                        name,
                        ticker: ticker.to_string(),
                        exchange: None,
                    });
                }
            }
        }
        
        Err(CikLookupError::ParseError("CIK not found in feed".to_string()))
    }

    /// Extract company name from Atom feed
    fn extract_company_name(&self, feed: &str) -> Option<String> {
        for line in feed.lines() {
            if line.contains("<name>") {
                let start = line.find("<name>").unwrap_or(0) + 6;
                let end = line.find("</name>").unwrap_or(line.len());
                if start < end {
                    return Some(line[start..end].trim().to_string());
                }
            }
        }
        None
    }

    /// Format CIK to 10 digits with leading zeros
    pub fn format_cik(cik: &str) -> String {
        format!("{:0>10}", cik)
    }

    /// Validate CIK format (should be 10 digits)
    pub fn validate_cik(cik: &str) -> Result<String, CikLookupError> {
        let formatted = Self::format_cik(cik);
        if formatted.len() == 10 && formatted.chars().all(|c| c.is_ascii_digit()) {
            Ok(formatted)
        } else {
            Err(CikLookupError::InvalidCikFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_cik() {
        assert_eq!(CikLookup::format_cik("320193"), "0000320193");
        assert_eq!(CikLookup::format_cik("0000320193"), "0000320193");
        assert_eq!(CikLookup::format_cik("789019"), "0000789019");
    }

    #[test]
    fn test_validate_cik() {
        assert!(CikLookup::validate_cik("320193").is_ok());
        assert!(CikLookup::validate_cik("0000320193").is_ok());
        assert!(CikLookup::validate_cik("invalid").is_err());
        assert!(CikLookup::validate_cik("123").is_ok()); // Gets padded
    }

    #[test]
    fn test_parse_cik_from_feed() {
        let feed = r#"<?xml version="1.0" encoding="ISO-8859-1"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
            <company-info>
                <cik>0000320193</cik>
                <name>Apple Inc.</name>
            </company-info>
        </feed>"#;
        
        let client = SecEdgarClient::new("Test test@example.com".to_string()).unwrap();
        let lookup = CikLookup::new(client);
        
        let result = lookup.parse_cik_from_feed(feed, "AAPL");
        assert!(result.is_ok());
        
        let info = result.unwrap();
        assert_eq!(info.cik, "0000320193");
        assert_eq!(info.name, "Apple Inc.");
        assert_eq!(info.ticker, "AAPL");
    }
}
