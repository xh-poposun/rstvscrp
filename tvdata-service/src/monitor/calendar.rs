use crate::models::{Alert, AlertRule, Monitor};
use time::Duration;
use tvdata_rs::{
    TradingViewClient, TradingViewClientConfig,
    calendar::{CalendarWindowRequest, DividendCalendarRequest},
    scanner::Market,
};

pub async fn check_calendar_events(monitor: &Monitor, rule: &AlertRule) -> Option<Alert> {
    let condition: serde_json::Value = serde_json::from_str(&rule.condition).ok()?;

    let event_type = condition.get("event_type").and_then(|v| v.as_str())?;

    let symbol = monitor.symbol.replace("NASDAQ:", "").replace("NYSE:", "");

    match event_type {
        "earnings" => check_earnings(&symbol, rule).await,
        "dividends" => check_dividends(&symbol, rule).await,
        "ipo" => check_ipo(&symbol, rule).await,
        _ => None,
    }
}

async fn check_earnings(symbol: &str, rule: &AlertRule) -> Option<Alert> {
    let config = TradingViewClientConfig::default();
    let client = match TradingViewClient::from_config(config).ok() {
        Some(c) => c,
        None => return None,
    };

    let now = time::OffsetDateTime::now_utc();
    let market = Market::new("america");
    let request =
        CalendarWindowRequest::new(market.clone(), now, now + Duration::days(7)).limit(100);

    let events = match client.earnings_calendar(&request).await {
        Ok(e) => e,
        Err(_) => return None,
    };

    for event in events {
        let ticker_str = event.instrument.ticker.to_string();
        if ticker_str.to_uppercase() == symbol.to_uppercase() {
            return Some(Alert::new(
                rule.id.clone(),
                ticker_str.clone(),
                format!(
                    "Earnings: {} - Date: {:?} - EPS Forecast: {:?}",
                    ticker_str, event.calendar_date, event.eps_forecast_next_fq
                ),
                serde_json::from_str(&rule.severity).unwrap_or(crate::models::Severity::Info),
            ));
        }
    }

    None
}

async fn check_dividends(symbol: &str, rule: &AlertRule) -> Option<Alert> {
    let config = TradingViewClientConfig::default();
    let client = match TradingViewClient::from_config(config).ok() {
        Some(c) => c,
        None => return None,
    };

    let now = time::OffsetDateTime::now_utc();
    let market = Market::new("america");
    let request =
        DividendCalendarRequest::new(market.clone(), now, now + Duration::days(30)).limit(100);

    let events = match client.dividend_calendar(&request).await {
        Ok(e) => e,
        Err(_) => return None,
    };

    for event in events {
        let ticker_str = event.instrument.ticker.to_string();
        if ticker_str.to_uppercase() == symbol.to_uppercase() {
            return Some(Alert::new(
                rule.id.clone(),
                ticker_str.clone(),
                format!(
                    "Dividend: {} - Ex-Date: {:?} - Amount: {:?}",
                    ticker_str, event.ex_date, event.amount
                ),
                serde_json::from_str(&rule.severity).unwrap_or(crate::models::Severity::Info),
            ));
        }
    }

    None
}

async fn check_ipo(symbol: &str, rule: &AlertRule) -> Option<Alert> {
    let config = TradingViewClientConfig::default();
    let client = match TradingViewClient::from_config(config).ok() {
        Some(c) => c,
        None => return None,
    };

    let now = time::OffsetDateTime::now_utc();
    let market = Market::new("america");
    let request =
        CalendarWindowRequest::new(market.clone(), now, now + Duration::days(30)).limit(100);

    let events = match client.ipo_calendar(&request).await {
        Ok(e) => e,
        Err(_) => return None,
    };

    for event in events {
        let ticker_str = event.instrument.ticker.to_string();
        if ticker_str.to_uppercase() == symbol.to_uppercase() {
            return Some(Alert::new(
                rule.id.clone(),
                ticker_str.clone(),
                format!(
                    "IPO: {} - Date: {:?} - Price: {:?}",
                    ticker_str, event.offer_date, event.offer_price_usd
                ),
                serde_json::from_str(&rule.severity).unwrap_or(crate::models::Severity::Info),
            ));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calendar_check_returns_none_for_unknown_type() {
        let monitor = Monitor::new("NASDAQ:AAPL".to_string(), Some("Apple".to_string()));
        let rule = AlertRule::new(
            monitor.id.clone(),
            crate::models::RuleType::Calendar,
            "Calendar Rule".to_string(),
            serde_json::json!({"event_type": "unknown"}),
            crate::models::Severity::Info,
            300,
            None,
        );

        let alert = check_calendar_events(&monitor, &rule).await;
        assert!(alert.is_none());
    }
}
