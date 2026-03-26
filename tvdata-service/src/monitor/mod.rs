pub mod calendar;
pub mod indicator;
pub mod price;

use chrono::Timelike;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::alert::AlertEngine;
use crate::config::MonitorConfig;
use crate::error::Result;
use crate::models::{Alert, AlertRule, Monitor, QuoteInfo, Severity};
use crate::tvclient::{Bar, TvClient};

fn format_sign(value: f64) -> String {
    let sign = if value >= 0.0 { "+" } else { "" };
    format!("{}{:.2}", sign, value)
}

pub struct MonitorEngine {
    db: SqlitePool,
    #[allow(dead_code)]
    tv_client: TvClient,
    #[allow(dead_code)]
    alert_engine: Arc<RwLock<AlertEngine>>,
    #[allow(dead_code)]
    config: MonitorConfig,
}

impl MonitorEngine {
    #[allow(dead_code)]
    pub async fn new(
        db: SqlitePool,
        tv_client: TvClient,
        alert_engine: Arc<RwLock<AlertEngine>>,
        config: MonitorConfig,
    ) -> Self {
        Self {
            db,
            tv_client,
            alert_engine,
            config,
        }
    }

    pub async fn run_check(&self) -> Result<Vec<(Alert, Option<QuoteInfo>)>> {
        let monitors = self.get_enabled_monitors().await?;
        let mut triggered_alerts = Vec::new();

        for monitor in monitors {
            let rules = self.get_rules_for_monitor(&monitor.id).await?;
            for rule in rules {
                if let Some((alert, quote_info)) = self.check_rule(&monitor, &rule).await? {
                    triggered_alerts.push((alert, quote_info));
                }
            }
        }

        Ok(triggered_alerts)
    }

    async fn get_enabled_monitors(&self) -> Result<Vec<Monitor>> {
        let monitors = sqlx::query_as::<_, Monitor>(
            "SELECT id, symbol, name, enabled, created_at, updated_at FROM monitors WHERE enabled = 1",
        )
        .fetch_all(&self.db)
        .await?;
        Ok(monitors)
    }

    async fn get_rules_for_monitor(&self, monitor_id: &str) -> Result<Vec<AlertRule>> {
        let rules = sqlx::query_as::<_, AlertRule>(
            "SELECT id, monitor_id, rule_type, name, condition, severity, cooldown_secs, enabled, created_at, last_triggered_date, daily_reset_hour_utc FROM alert_rules WHERE monitor_id = ? AND enabled = 1",
        )
        .bind(monitor_id)
        .fetch_all(&self.db)
        .await?;
        Ok(rules)
    }

    async fn check_rule(&self, monitor: &Monitor, rule: &AlertRule) -> Result<Option<(Alert, Option<QuoteInfo>)>> {
        if self.is_in_cooldown(rule).await? {
            return Ok(None);
        }

        match rule.rule_type.as_str() {
            "price" => self.check_price_rule(monitor, rule).await.map(|opt| opt.map(|(a, q)| (a, Some(q)))),
            "indicator" => self.check_indicator_rule(monitor, rule).await.map(|opt| opt.map(|a| (a, None::<QuoteInfo>))),
            "calendar" => self.check_calendar_rule(monitor, rule).await.map(|opt| opt.map(|a| (a, None::<QuoteInfo>))),
            _ => Ok(None),
        }
    }

    async fn is_in_cooldown(&self, rule: &AlertRule) -> Result<bool> {
        let now = chrono::Utc::now();
        let today = now.format("%Y-%m-%d").to_string();
        let current_hour = now.hour();

        // Check daily reset logic if configured
        if let Some(reset_hour) = rule.daily_reset_hour_utc {
            if let Some(ref last_date) = rule.last_triggered_date {
                if last_date == &today {
                    // Already triggered today
                    tracing::debug!("Rule {} already triggered today ({})", rule.id, today);
                    return Ok(true);
                }

                // Check if we haven't crossed the reset hour yet (new day but before reset time)
                // This handles the case: yesterday triggered, today before reset_hour
                if current_hour < reset_hour as u32 {
                    // Still before today's reset time, count yesterday's trigger
                    let yesterday = (now - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
                    if last_date == &yesterday {
                        tracing::debug!("Rule {} triggered yesterday, before reset hour {}", rule.id, reset_hour);
                        return Ok(true);
                    }
                }
            }
        }

        // Fallback: time-based cooldown
        let system_cooldown = rule.cooldown_secs as i64;

        let recent_alert: Option<Alert> = sqlx::query_as(
            "SELECT id, rule_id, symbol, message, severity, triggered_at, acknowledged, acknowledged_at, ack_by FROM alerts ORDER BY triggered_at DESC LIMIT 1",
        )
        .fetch_optional(&self.db)
        .await?;

        if let Some(alert) = recent_alert {
            let elapsed = chrono::Utc::now()
                .signed_duration_since(alert.triggered_at)
                .num_seconds();
            return Ok(elapsed < system_cooldown);
        }

        Ok(false)
    }

    #[allow(dead_code)]
    async fn check_price_rule(&self, monitor: &Monitor, rule: &AlertRule) -> Result<Option<(Alert, QuoteInfo)>> {
        let quotes = self
            .tv_client
            .get_quotes(&[&monitor.symbol])
            .await
            .map_err(|e| crate::error::Error::TradingView(e.to_string()))?;

        if quotes.is_empty() {
            return Ok(None);
        }

        let quote = &quotes[0];
        let condition: serde_json::Value =
            serde_json::from_str(&rule.condition).unwrap_or(serde_json::Value::Null);

        let should_alert = self.evaluate_price_condition(quote.change_percent, &condition);

        if should_alert {
            let severity = match rule.severity.as_str() {
                "critical" => Severity::Critical,
                "warning" => Severity::Warning,
                _ => Severity::Info,
            };

            let threshold = condition
                .get("threshold")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);

            let quote_info = QuoteInfo {
                price: quote.price,
                change: quote.change,
                change_percent: quote.change_percent,
                previous_close: quote.previous_close,
                open: quote.open,
                high: quote.high,
                low: quote.low,
                volume: quote.volume,
            };

            let message = format!(
                "Price change {}% (threshold: {}%)\n当前价: {} | 涨跌: {} | 昨收: {}",
                format_sign(quote.change_percent),
                threshold,
                quote_info.format_price(),
                quote_info.format_change(),
                quote_info.format_price()
            );

            let alert = Alert::new(
                rule.id.clone(),
                monitor.symbol.clone(),
                message,
                severity,
            );

            self.save_alert(&alert).await?;
            self.update_rule_trigger_date(&rule.id).await?;
            return Ok(Some((alert, quote_info)));
        }

        Ok(None)
    }

    fn evaluate_price_condition(&self, value: f64, condition: &serde_json::Value) -> bool {
        let threshold = condition
            .get("threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let op = condition.get("op").and_then(|v| v.as_str()).unwrap_or(">");

        match op {
            ">" => value > threshold,
            ">=" => value >= threshold,
            "<" => value < threshold,
            "<=" => value <= threshold,
            "==" => (value - threshold).abs() < f64::EPSILON,
            "!=" => (value - threshold).abs() >= f64::EPSILON,
            _ => false,
        }
    }

    #[allow(dead_code)]
    async fn check_indicator_rule(
        &self,
        monitor: &Monitor,
        rule: &AlertRule,
    ) -> Result<Option<Alert>> {
        let history: Vec<Bar> = self
            .tv_client
            .get_history(&monitor.symbol, "D")
            .await
            .map_err(|e| crate::error::Error::TradingView(e.to_string()))?;

        if history.is_empty() {
            return Ok(None);
        }

        let prices: Vec<f64> = history.iter().map(|bar| bar.close).collect();
        let condition: serde_json::Value =
            serde_json::from_str(&rule.condition).unwrap_or(serde_json::Value::Null);

        let indicator_type = condition
            .get("indicator")
            .and_then(|v| v.as_str())
            .unwrap_or("RSI");
        let threshold = condition
            .get("threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let op = condition.get("op").and_then(|v| v.as_str()).unwrap_or(">");

        let indicator_value = match indicator_type {
            "RSI" => {
                let period = condition
                    .get("period")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(14) as usize;
                crate::indicators::calculate_rsi(&prices, period)
            }
            "MACD" => {
                let config = crate::indicators::MacdConfig {
                    fast: condition.get("fast").and_then(|v| v.as_u64()).unwrap_or(12) as usize,
                    slow: condition.get("slow").and_then(|v| v.as_u64()).unwrap_or(26) as usize,
                    signal: condition
                        .get("signal")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(9) as usize,
                };
                crate::indicators::calculate_macd(&prices, &config)
                    .map(|r| r.histogram)
                    .unwrap_or(0.0)
            }
            _ => 0.0,
        };

        let should_alert = match op {
            ">" => indicator_value > threshold,
            ">=" => indicator_value >= threshold,
            "<" => indicator_value < threshold,
            "<=" => indicator_value <= threshold,
            _ => false,
        };

        if should_alert {
            let severity = match rule.severity.as_str() {
                "critical" => Severity::Critical,
                "warning" => Severity::Warning,
                _ => Severity::Info,
            };

            let alert = Alert::new(
                rule.id.clone(),
                monitor.symbol.clone(),
                format!(
                    "{}: {} {} {} (current: {:.2})",
                    monitor.symbol, indicator_type, op, threshold, indicator_value
                ),
                severity,
            );

            self.save_alert(&alert).await?;
            return Ok(Some(alert));
        }

        Ok(None)
    }

    #[allow(dead_code)]
    async fn check_calendar_rule(
        &self,
        _monitor: &Monitor,
        _rule: &AlertRule,
    ) -> Result<Option<Alert>> {
        Ok(None)
    }

    async fn save_alert(&self, alert: &Alert) -> Result<()> {
        sqlx::query(
            "INSERT INTO alerts (id, rule_id, symbol, message, severity, triggered_at, acknowledged, acknowledged_at, ack_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&alert.id)
        .bind(&alert.rule_id)
        .bind(&alert.symbol)
        .bind(&alert.message)
        .bind(&alert.severity)
        .bind(alert.triggered_at)
        .bind(alert.acknowledged)
        .bind(alert.acknowledged_at.as_ref())
        .bind(&alert.ack_by)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn update_rule_trigger_date(&self, rule_id: &str) -> Result<()> {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        sqlx::query("UPDATE alert_rules SET last_triggered_date = ? WHERE id = ?")
            .bind(&today)
            .bind(rule_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_evaluate_price_condition_greater_than() {
        let condition = serde_json::json!({"op": ">", "threshold": 5.0});
        assert!(evaluate_test(7.0, &condition));
        assert!(!evaluate_test(3.0, &condition));
    }

    #[test]
    fn test_evaluate_price_condition_less_than() {
        let condition = serde_json::json!({"op": "<", "threshold": 50.0});
        assert!(evaluate_test(30.0, &condition));
        assert!(!evaluate_test(60.0, &condition));
    }

    fn evaluate_test(value: f64, condition: &serde_json::Value) -> bool {
        let threshold = condition
            .get("threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let op = condition.get("op").and_then(|v| v.as_str()).unwrap_or(">");
        match op {
            ">" => value > threshold,
            ">=" => value >= threshold,
            "<" => value < threshold,
            "<=" => value <= threshold,
            "==" => (value - threshold).abs() < f64::EPSILON,
            "!=" => (value - threshold).abs() >= f64::EPSILON,
            _ => false,
        }
    }
}
