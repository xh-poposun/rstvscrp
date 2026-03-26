use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Monitor {
    pub id: String,
    pub symbol: String,
    pub name: Option<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMonitorRequest {
    pub symbol: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMonitorRequest {
    pub name: Option<String>,
    pub enabled: Option<bool>,
}

impl Monitor {
    pub fn new(symbol: String, name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            symbol,
            name,
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleType {
    #[serde(rename = "price")]
    Price,
    #[serde(rename = "indicator")]
    Indicator,
    #[serde(rename = "calendar")]
    Calendar,
}

impl std::fmt::Display for RuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Price => write!(f, "price"),
            Self::Indicator => write!(f, "indicator"),
            Self::Calendar => write!(f, "calendar"),
        }
    }
}

impl std::str::FromStr for RuleType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "price" => Ok(Self::Price),
            "indicator" => Ok(Self::Indicator),
            "calendar" => Ok(Self::Calendar),
            _ => Err(format!("unknown rule type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "critical")]
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warning => write!(f, "warning"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

impl std::str::FromStr for Severity {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "info" => Ok(Self::Info),
            "warning" => Ok(Self::Warning),
            "critical" => Ok(Self::Critical),
            _ => Err(format!("unknown severity: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AlertRule {
    pub id: String,
    pub monitor_id: String,
    pub rule_type: String,
    pub name: String,
    pub condition: String,
    pub severity: String,
    pub cooldown_secs: i32,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub last_triggered_date: Option<String>,
    #[serde(default)]
    pub daily_reset_hour_utc: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRuleRequest {
    pub monitor_id: String,
    pub rule_type: RuleType,
    pub name: String,
    pub condition: serde_json::Value,
    pub severity: Severity,
    #[serde(default = "default_cooldown")]
    pub cooldown_secs: i32,
    #[serde(default)]
    pub daily_reset_hour_utc: Option<i32>,
}

fn default_cooldown() -> i32 {
    300
}

impl AlertRule {
    pub fn new(
        monitor_id: String,
        rule_type: RuleType,
        name: String,
        condition: serde_json::Value,
        severity: Severity,
        cooldown_secs: i32,
        daily_reset_hour_utc: Option<i32>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            monitor_id,
            rule_type: rule_type.to_string(),
            name,
            condition: serde_json::to_string(&condition).unwrap_or_default(),
            severity: severity.to_string(),
            cooldown_secs,
            enabled: true,
            created_at: Utc::now(),
            last_triggered_date: None,
            daily_reset_hour_utc,
        }
    }

    pub fn condition_json(&self) -> serde_json::Value {
        serde_json::from_str(&self.condition).unwrap_or(serde_json::Value::Null)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Alert {
    pub id: String,
    pub rule_id: String,
    pub symbol: String,
    pub message: String,
    pub severity: String,
    pub triggered_at: DateTime<Utc>,
    pub acknowledged: bool,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub ack_by: Option<String>,
}

impl Alert {
    pub fn new(rule_id: String, symbol: String, message: String, severity: Severity) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            rule_id,
            symbol,
            message,
            severity: severity.to_string(),
            triggered_at: Utc::now(),
            acknowledged: false,
            acknowledged_at: None,
            ack_by: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcknowledgeRequest {
    pub ack_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertListQuery {
    #[serde(default)]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
    pub symbol: Option<String>,
    pub acknowledged: Option<bool>,
}

fn default_page_size() -> i64 {
    20
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteInfo {
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub previous_close: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,
}

impl QuoteInfo {
    pub fn format_price(&self) -> String {
        format!("${:.2}", self.price)
    }

    pub fn format_change(&self) -> String {
        let sign = if self.change >= 0.0 { "+" } else { "" };
        format!("{}{:.2}", sign, self.change)
    }

    pub fn format_change_percent(&self) -> String {
        let sign = if self.change_percent >= 0.0 { "+" } else { "" };
        format!("{}{:.2}%", sign, self.change_percent)
    }

    pub fn format_volume(&self) -> String {
        let vol = self.volume as f64;
        if vol >= 1_000_000_000.0 {
            format!("{:.1}B", vol / 1_000_000_000.0)
        } else if vol >= 1_000_000.0 {
            format!("{:.1}M", vol / 1_000_000.0)
        } else if vol >= 1_000.0 {
            format!("{:.1}K", vol / 1_000.0)
        } else {
            self.volume.to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryQuery {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalBar {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_new() {
        let monitor = Monitor::new("NASDAQ:AAPL".to_string(), Some("Apple".to_string()));
        assert!(!monitor.id.is_empty());
        assert_eq!(monitor.symbol, "NASDAQ:AAPL");
        assert!(monitor.enabled);
    }

    #[test]
    fn test_rule_type_parsing() {
        assert_eq!("price".parse::<RuleType>().unwrap(), RuleType::Price);
        assert_eq!(
            "indicator".parse::<RuleType>().unwrap(),
            RuleType::Indicator
        );
        assert_eq!("calendar".parse::<RuleType>().unwrap(), RuleType::Calendar);
        assert!("unknown".parse::<RuleType>().is_err());
    }

    #[test]
    fn test_severity_parsing() {
        assert_eq!("info".parse::<Severity>().unwrap(), Severity::Info);
        assert_eq!("warning".parse::<Severity>().unwrap(), Severity::Warning);
        assert_eq!("critical".parse::<Severity>().unwrap(), Severity::Critical);
        assert!("urgent".parse::<Severity>().is_err());
    }

    #[test]
    fn test_alert_rule_new() {
        let rule = AlertRule::new(
            "monitor-1".to_string(),
            RuleType::Price,
            "Price Alert".to_string(),
            serde_json::json!({"op": ">", "threshold": 5.0}),
            Severity::Warning,
            300,
            None,
        );
        assert!(!rule.id.is_empty());
        assert_eq!(rule.rule_type, "price");
        assert_eq!(rule.severity, "warning");
    }

    #[test]
    fn test_alert_new() {
        let alert = Alert::new(
            "rule-1".to_string(),
            "NASDAQ:AAPL".to_string(),
            "Price increased by 5%".to_string(),
            Severity::Critical,
        );
        assert!(!alert.id.is_empty());
        assert!(!alert.acknowledged);
    }
}
