use crate::client::SecEdgarClient;
use crate::cik_lookup::{CikLookup, CompanyInfo};
use chrono::{Datelike, NaiveDate};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during debt maturity extraction
#[derive(Error, Debug)]
pub enum DebtMaturityError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("CIK lookup failed: {0}")]
    CikLookupError(#[from] crate::cik_lookup::CikLookupError),
    #[error("XBRL parsing failed: {0}")]
    XbrlParseError(String),
    #[error("No 10-K filing found for CIK: {0}")]
    NoFilingFound(String),
    #[error("No debt maturity data found")]
    NoDebtMaturityData,
}

/// Debt maturity entry for a specific year
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtMaturityEntry {
    /// Year of maturity
    pub year: i32,
    /// Amount due in that year (in thousands)
    pub amount: f64,
    /// Currency (usually USD)
    pub currency: String,
}

/// Complete debt maturity schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtMaturitySchedule {
    /// Company information
    pub company: CompanyInfo,
    /// Filing date of the 10-K
    pub filing_date: Option<NaiveDate>,
    /// Debt maturity entries by year
    pub maturities: Vec<DebtMaturityEntry>,
    /// Total long-term debt
    pub total_debt: Option<f64>,
    /// Thereafter amount (debt maturing after the scheduled years)
    pub thereafter: Option<f64>,
}

/// Debt maturity extractor from SEC 10-K filings
pub struct DebtMaturityExtractor {
    client: SecEdgarClient,
    cik_lookup: CikLookup,
}

impl DebtMaturityExtractor {
    /// Create a new debt maturity extractor
    pub fn new(client: SecEdgarClient) -> Self {
        let cik_lookup = CikLookup::new(client.clone());
        Self { client, cik_lookup }
    }

    /// Extract debt maturity schedule for a ticker symbol
    /// 
    /// # Arguments
    /// * `ticker` - Stock ticker symbol
    /// 
    /// # Returns
    /// * `Ok(DebtMaturitySchedule)` - Debt maturity schedule
    /// * `Err(DebtMaturityError)` - If extraction fails
    pub async fn extract(&self, ticker: &str) -> Result<DebtMaturitySchedule, DebtMaturityError> {
        // Step 1: Lookup CIK
        let company_info = self.cik_lookup.lookup_from_tickers_json(ticker).await?;
        
        // Step 2: Get latest 10-K filing
        let filing = self.get_latest_10k(&company_info.cik).await?;
        
        // Step 3: Extract XBRL data
        let xbrl_data = self.fetch_xbrl(&filing.xbrl_url).await?;
        
        // Step 4: Parse debt maturity from XBRL
        let maturities = self.parse_debt_maturity(&xbrl_data)?;
        
        Ok(DebtMaturitySchedule {
            company: company_info,
            filing_date: filing.filing_date,
            maturities,
            total_debt: None, // Could be extracted from XBRL
            thereafter: None, // Could be extracted from XBRL
        })
    }

    /// Get latest 10-K filing information
    async fn get_latest_10k(&self, cik: &str) -> Result<FilingInfo, DebtMaturityError> {
        let formatted_cik = CikLookup::format_cik(cik);
        
        // SEC submissions endpoint
        let url = format!(
            "https://data.sec.gov/submissions/CIK{}.json",
            formatted_cik
        );

        let response = self.client.get(&url).await?;
        
        if !response.status().is_success() {
            return Err(DebtMaturityError::NoFilingFound(cik.to_string()));
        }

        let submissions: serde_json::Value = response.json().await?;
        
        // Find latest 10-K
        if let Some(recent) = submissions.get("filings").and_then(|f| f.get("recent")) {
            if let Some(forms) = recent.get("form").and_then(|f| f.as_array()) {
                for (i, form) in forms.iter().enumerate() {
                    if form.as_str() == Some("10-K") {
                    let accession_number = recent["accessionNumber"][i].as_str()
                        .ok_or_else(|| DebtMaturityError::XbrlParseError("Missing accession number".to_string()))?;
                        
                        let filing_date = recent["filingDate"][i].as_str()
                            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
                        
                        // Build XBRL URL
                        let xbrl_url = format!(
                            "https://www.sec.gov/Archives/edgar/data/{}/{}/{}-index.htm",
                            formatted_cik.trim_start_matches('0'),
                            accession_number.replace("-", ""),
                            accession_number
                        );
                        
                        return Ok(FilingInfo {
                            accession_number: accession_number.to_string(),
                            filing_date,
                            xbrl_url,
                        });
                    }
                }
            }
        }
        
        Err(DebtMaturityError::NoFilingFound(cik.to_string()))
    }

    /// Fetch XBRL document
    async fn fetch_xbrl(&self, url: &str) -> Result<String, DebtMaturityError> {
        let response = self.client.get(url).await?;
        
        if !response.status().is_success() {
            return Err(DebtMaturityError::XbrlParseError(
                format!("Failed to fetch XBRL: {}", url)
            ));
        }

        Ok(response.text().await?)
    }

    /// Parse debt maturity data from XBRL XML
    fn parse_debt_maturity(&self, xbrl_data: &str) -> Result<Vec<DebtMaturityEntry>, DebtMaturityError> {
        let mut reader = Reader::from_str(xbrl_data);
        reader.trim_text(true);
        
        let mut maturities = Vec::new();
        let mut buf = Vec::new();
        let mut current_year: Option<i32> = None;
        let mut current_amount: Option<f64> = None;
        
        // XBRL tags to look for
        let debt_tags = [
            "LongTermDebtMaturities",
            "DebtMaturitySchedule",
            "LongTermDebtByMaturityYear",
            "LongTermDebtMaturitiesFiveYears",
            "LongTermDebtMaturitiesAfterFiveYears",
            "LongTermDebtMaturitiesNextTwelveMonths",
            "LongTermDebtMaturitiesYearOne",
            "LongTermDebtMaturitiesYearTwo",
            "LongTermDebtMaturitiesYearThree",
            "LongTermDebtMaturitiesYearFour",
            "LongTermDebtMaturitiesYearFive",
        ];
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = std::str::from_utf8(e.name().as_ref())
                        .unwrap_or("")
                        .to_string();
                    
                    // Check if this is a debt maturity tag
                    if debt_tags.iter().any(|tag| name.contains(tag)) {
                        // Try to extract year from tag name
                        current_year = self.extract_year_from_tag(&name);
                    }
                }
                Ok(Event::Text(e)) => {
                    if current_year.is_some() {
                        let text = e.unescape().unwrap_or_default();
                        if let Ok(amount) = text.parse::<f64>() {
                            current_amount = Some(amount);
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = std::str::from_utf8(e.name().as_ref())
                        .unwrap_or("")
                        .to_string();
                    
                    if debt_tags.iter().any(|tag| name.contains(tag)) {
                        if let (Some(year), Some(amount)) = (current_year, current_amount) {
                            maturities.push(DebtMaturityEntry {
                                year,
                                amount,
                                currency: "USD".to_string(),
                            });
                            current_year = None;
                            current_amount = None;
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(DebtMaturityError::XbrlParseError(format!("XML parse error: {}", e)));
                }
                _ => {}
            }
            buf.clear();
        }
        
        if maturities.is_empty() {
            return Err(DebtMaturityError::NoDebtMaturityData);
        }
        
        // Sort by year
        maturities.sort_by(|a, b| a.year.cmp(&b.year));
        
        Ok(maturities)
    }

    /// Extract year from XBRL tag name
    fn extract_year_from_tag(&self, tag: &str) -> Option<i32> {
        // Try to extract year from common patterns
        if tag.contains("YearOne") || tag.contains("NextTwelveMonths") {
            return Some(chrono::Local::now().year() + 1);
        }
        if tag.contains("YearTwo") {
            return Some(chrono::Local::now().year() + 2);
        }
        if tag.contains("YearThree") {
            return Some(chrono::Local::now().year() + 3);
        }
        if tag.contains("YearFour") {
            return Some(chrono::Local::now().year() + 4);
        }
        if tag.contains("YearFive") {
            return Some(chrono::Local::now().year() + 5);
        }
        if tag.contains("AfterFiveYears") {
            return Some(chrono::Local::now().year() + 6);
        }
        
        // Try to find 4-digit year in tag
        for i in 0..tag.len().saturating_sub(3) {
            if let Ok(year) = tag[i..i+4].parse::<i32>() {
                if year >= 2000 && year <= 2100 {
                    return Some(year);
                }
            }
        }
        
        None
    }

    /// Parse debt maturity from table in HTML (fallback method)
    pub fn parse_from_html_table(&self, html: &str) -> Result<Vec<DebtMaturityEntry>, DebtMaturityError> {
        let mut maturities = Vec::new();
        
        // Simple regex-like parsing for debt maturity tables
        // Look for patterns like "2024", "$1,234" or "2025", "2,345 million"
        let lines: Vec<&str> = html.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            // Look for year patterns
            if let Some(year) = self.extract_year_from_line(line) {
                // First try to find amount on the same line
                if let Some(amount) = self.extract_amount_from_line(line) {
                    maturities.push(DebtMaturityEntry {
                        year,
                        amount,
                        currency: "USD".to_string(),
                    });
                } else {
                    // Look for amount in next few lines
                    for j in i+1..(i+5).min(lines.len()) {
                        if let Some(amount) = self.extract_amount_from_line(lines[j]) {
                            maturities.push(DebtMaturityEntry {
                                year,
                                amount,
                                currency: "USD".to_string(),
                            });
                            break;
                        }
                    }
                }
            }
        }
        
        if maturities.is_empty() {
            return Err(DebtMaturityError::NoDebtMaturityData);
        }
        
        maturities.sort_by(|a, b| a.year.cmp(&b.year));
        Ok(maturities)
    }

    /// Extract year from a line of text
    fn extract_year_from_line(&self, line: &str) -> Option<i32> {
        // Look for 4-digit years between 2020-2035
        for i in 0..line.len().saturating_sub(3) {
            if let Ok(year) = line[i..i+4].parse::<i32>() {
                if year >= 2020 && year <= 2035 {
                    return Some(year);
                }
            }
        }
        None
    }

    /// Extract amount from a line of text
    fn extract_amount_from_line(&self, line: &str) -> Option<f64> {
        // Look for patterns like $1,234 or 1,234 or 1234
        // Skip 4-digit years (2020-2035)
        let re = regex::Regex::new(r"[$]?([\d,]+(?:\.\d+)?)").ok()?;
        
        for cap in re.captures_iter(line) {
            if let Some(matched) = cap.get(1) {
                let num_str = matched.as_str().replace(",", "");
                if let Ok(num) = num_str.parse::<f64>() {
                    // Skip if it looks like a year
                    if num < 2020.0 || num > 2035.0 {
                        return Some(num);
                    }
                }
            }
        }
        None
    }
}

/// Filing information
#[derive(Debug, Clone)]
struct FilingInfo {
    accession_number: String,
    filing_date: Option<NaiveDate>,
    xbrl_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_year_from_tag() {
        let client = SecEdgarClient::new("Test test@example.com".to_string()).unwrap();
        let extractor = DebtMaturityExtractor::new(client);
        
        let current_year = chrono::Local::now().year();
        
        assert_eq!(extractor.extract_year_from_tag("LongTermDebtMaturitiesYearOne"), Some(current_year + 1));
        assert_eq!(extractor.extract_year_from_tag("LongTermDebtMaturitiesYearTwo"), Some(current_year + 2));
        assert_eq!(extractor.extract_year_from_tag("Debt2025"), Some(2025));
        assert_eq!(extractor.extract_year_from_tag("DebtMaturity2026"), Some(2026));
    }

    #[test]
    fn test_extract_year_from_line() {
        let client = SecEdgarClient::new("Test test@example.com".to_string()).unwrap();
        let extractor = DebtMaturityExtractor::new(client);
        
        assert_eq!(extractor.extract_year_from_line("2024"), Some(2024));
        assert_eq!(extractor.extract_year_from_line("Fiscal Year 2025"), Some(2025));
        assert_eq!(extractor.extract_year_from_line("Year 2030"), Some(2030));
    }

    #[test]
    fn test_extract_amount_from_line() {
        let client = SecEdgarClient::new("Test test@example.com".to_string()).unwrap();
        let extractor = DebtMaturityExtractor::new(client);
        
        assert_eq!(extractor.extract_amount_from_line("$1,234"), Some(1234.0));
        assert_eq!(extractor.extract_amount_from_line("2,345.67"), Some(2345.67));
    }

    #[test]
    fn test_parse_from_html_table() {
        let html = r#"
        <table>
            <tr><td>2024</td><td>$1,000</td></tr>
            <tr><td>2025</td><td>$2,500</td></tr>
            <tr><td>2026</td><td>$3,000</td></tr>
        </table>
        "#;
        
        let client = SecEdgarClient::new("Test test@example.com".to_string()).unwrap();
        let extractor = DebtMaturityExtractor::new(client);
        
        let result = extractor.parse_from_html_table(html);
        assert!(result.is_ok());
        
        let maturities = result.unwrap();
        assert_eq!(maturities.len(), 3);
        assert_eq!(maturities[0].year, 2024);
        assert_eq!(maturities[0].amount, 1000.0);
    }
}
