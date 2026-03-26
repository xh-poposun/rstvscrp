use crate::models::{Alert, AlertRule, Monitor};

pub async fn check_price(
    _monitor: &Monitor,
    _rule: &AlertRule,
    current_price: f64,
    previous_price: f64,
) -> Option<Alert> {
    if previous_price == 0.0 {
        return None;
    }

    let change_percent = ((current_price - previous_price) / previous_price) * 100.0;
    let condition: serde_json::Value = serde_json::from_str(&_rule.condition).ok()?;
    let threshold = condition.get("threshold")?.as_f64()?;
    let op = condition.get("op")?.as_str()?;

    let triggered = match op {
        ">" => change_percent > threshold,
        ">=" => change_percent >= threshold,
        "<" => change_percent < threshold,
        "<=" => change_percent <= threshold,
        _ => false,
    };

    if triggered {
        Some(Alert::new(
            _rule.id.clone(),
            _monitor.symbol.clone(),
            format!(
                "{} price changed by {:.2}% (threshold: {} {})",
                _monitor.symbol, change_percent, op, threshold
            ),
            serde_json::from_str(&_rule.severity).unwrap_or(crate::models::Severity::Info),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Severity;

    #[tokio::test]
    async fn test_price_change_detection() {
        let monitor = Monitor::new("NASDAQ:AAPL".to_string(), Some("Apple".to_string()));
        let rule = AlertRule::new(
            monitor.id.clone(),
            crate::models::RuleType::Price,
            "Test Rule".to_string(),
            serde_json::json!({"op": ">", "threshold": 5.0}),
            Severity::Warning,
            300,
            None,
        );

        let alert = check_price(&monitor, &rule, 110.0, 100.0).await;
        assert!(alert.is_some());

        let alert = check_price(&monitor, &rule, 103.0, 100.0).await;
        assert!(alert.is_none());
    }
}
