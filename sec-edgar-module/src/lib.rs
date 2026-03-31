//! SEC EDGAR Debt Maturity Extraction Module
//!
//! This module provides functionality to extract debt maturity schedules
//! from SEC EDGAR 10-K filings using XBRL parsing.
//!
//! # Example
//!
//! ```rust
//! use sec_edgar_module::{SecEdgarClient, DebtMaturityExtractor};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = SecEdgarClient::new("YourCompany contact@example.com".to_string())?;
//!     let extractor = DebtMaturityExtractor::new(client);
//!     
//!     let schedule = extractor.extract("AAPL").await?;
//!     println!("Debt maturity for {}: {:?}", schedule.company.name, schedule.maturities);
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod cik_lookup;
pub mod debt_maturity;

pub use client::SecEdgarClient;
pub use cik_lookup::{CikLookup, CompanyInfo, CikLookupError};
pub use debt_maturity::{DebtMaturityExtractor, DebtMaturitySchedule, DebtMaturityEntry, DebtMaturityError};

use thiserror::Error;

/// Combined error type for the module
#[derive(Error, Debug)]
pub enum SecEdgarError {
    #[error("Client error: {0}")]
    ClientError(#[from] reqwest::Error),
    #[error("CIK lookup error: {0}")]
    CikLookupError(#[from] CikLookupError),
    #[error("Debt maturity error: {0}")]
    DebtMaturityError(#[from] DebtMaturityError),
}

/// Convenience function to extract debt maturity for a ticker
/// 
/// # Arguments
/// * `ticker` - Stock ticker symbol (e.g., "AAPL", "MSFT")
/// * `user_agent` - Required by SEC, format: "CompanyName ContactEmail"
/// 
/// # Returns
/// * `Ok(DebtMaturitySchedule)` - Debt maturity schedule
/// * `Err(SecEdgarError)` - If extraction fails
/// 
/// # Example
/// ```rust
/// use sec_edgar_module::get_debt_maturity;
///
/// #[tokio::main]
/// async fn main() {
///     match get_debt_maturity("AAPL", "MyCompany contact@example.com").await {
///         Ok(schedule) => println!("Found {} debt entries", schedule.maturities.len()),
///         Err(e) => eprintln!("Error: {}", e),
///     }
/// }
/// ```
pub async fn get_debt_maturity(
    ticker: &str,
    user_agent: &str,
) -> Result<DebtMaturitySchedule, SecEdgarError> {
    let client = SecEdgarClient::new(user_agent.to_string())?;
    let extractor = DebtMaturityExtractor::new(client);
    
    Ok(extractor.extract(ticker).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_module_integration() {
        // This test verifies the module compiles and basic types work
        let client = SecEdgarClient::new("Test test@example.com".to_string());
        assert!(client.is_ok());
    }
}
