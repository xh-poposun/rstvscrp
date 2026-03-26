use crate::indicators::{MacdConfig, calculate_macd, calculate_rsi};
use crate::models::{Alert, AlertRule, Monitor, Severity};

#[allow(dead_code)]
pub async fn check_indicator(
    _monitor: &Monitor,
    _rule: &AlertRule,
    prices: &[f64],
) -> Option<Alert> {
    let condition: serde_json::Value = serde_json::from_str(&_rule.condition).ok()?;
    let indicator = condition.get("indicator")?.as_str()?;
    let threshold = condition.get("threshold")?.as_f64()?;
    let op = condition.get("op")?.as_str()?;

    let value = match indicator {
        "RSI" => {
            let period = condition
                .get("period")
                .and_then(|v| v.as_u64())
                .unwrap_or(14) as usize;
            calculate_rsi(prices, period)
        }
        "MACD" => {
            let config = MacdConfig {
                fast: condition.get("fast").and_then(|v| v.as_u64()).unwrap_or(12) as usize,
                slow: condition.get("slow").and_then(|v| v.as_u64()).unwrap_or(26) as usize,
                signal: condition
                    .get("signal")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(9) as usize,
            };
            calculate_macd(prices, &config)?.histogram
        }
        _ => return None,
    };

    let triggered = match op {
        ">" => value > threshold,
        ">=" => value >= threshold,
        "<" => value < threshold,
        "<=" => value <= threshold,
        _ => false,
    };

    if triggered {
        Some(Alert::new(
            _rule.id.clone(),
            _monitor.symbol.clone(),
            format!(
                "{} indicator {} {} {} (current: {:.2})",
                _monitor.symbol, indicator, op, threshold, value
            ),
            Severity::Info,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rsi_indicator_trigger() {
        let monitor = Monitor::new("NASDAQ:AAPL".to_string(), Some("Apple".to_string()));
        let rule = AlertRule::new(
            monitor.id.clone(),
            crate::models::RuleType::Indicator,
            "RSI Alert".to_string(),
            serde_json::json!({"indicator": "RSI", "op": ">", "threshold": 70.0, "period": 14}),
            Severity::Warning,
            300,
            None,
        );

        let prices: Vec<f64> = (0..30).map(|i| 100.0 + (i as f64)).collect();
        let alert = check_indicator(&monitor, &rule, &prices).await;
        assert!(alert.is_some());
    }

    #[tokio::test]
    async fn test_macd_indicator_no_trigger() {
        let monitor = Monitor::new("NASDAQ:TSLA".to_string(), Some("Tesla".to_string()));
        let rule = AlertRule::new(
            monitor.id.clone(),
            crate::models::RuleType::Indicator,
            "MACD Alert".to_string(),
            serde_json::json!({"indicator": "MACD", "op": ">", "threshold": 1000.0}),
            Severity::Critical,
            300,
            None,
        );

        let prices: Vec<f64> = (0..50).map(|i| 100.0 + (i as f64) * 0.5).collect();
        let alert = check_indicator(&monitor, &rule, &prices).await;
        assert!(alert.is_none());
    }
}
