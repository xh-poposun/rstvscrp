use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::models::QuoteInfo;

pub struct AlertEngine {
    client: Client,
    webhook_url: String,
    rate_limiter: tokio::sync::Mutex<RateLimiter>,
}

#[derive(Debug, Clone)]
struct RateLimiter {
    per_hour: u32,
    sent_count: u32,
    window_start: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
struct FeishuInteractiveCard {
    #[serde(rename = "msg_type")]
    msg_type: String,
    card: FeishuCard,
}

#[derive(Debug, Serialize)]
struct FeishuCard {
    pub template: String,
    pub header: Vec<FeishuCardHeader>,
    pub elements: Vec<FeishuCardElement>,
}

#[derive(Debug, Serialize)]
struct FeishuCardHeader {
    pub title: FeishuCardTitle,
}

#[derive(Debug, Serialize)]
struct FeishuCardTitle {
    pub tag: String,
    pub content: String,
    pub color: String,
}

#[derive(Debug, Serialize)]
struct FeishuCardElement {
    pub tag: String,
    pub text: Option<FeishuText>,
    pub fields: Option<Vec<FeishuField>>,
}

#[derive(Debug, Serialize)]
struct FeishuText {
    pub tag: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct FeishuField {
    pub is_short: bool,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FeishuResponse {
    code: i32,
    msg: String,
}

impl AlertEngine {
    pub fn new(webhook_url: String) -> Self {
        let client = Client::builder()
            .use_rustls_tls()
            .build()
            .expect("failed to build reqwest client");

        Self {
            client,
            webhook_url,
            rate_limiter: tokio::sync::Mutex::new(RateLimiter {
                per_hour: 10,
                sent_count: 0,
                window_start: chrono::Utc::now(),
            }),
        }
    }

    pub async fn send_alert(
        &self,
        symbol: &str,
        rule_type: &str,
        message: &str,
        severity: &str,
        quote_info: Option<&QuoteInfo>,
    ) -> Result<(), String> {
        let mut limiter = self.rate_limiter.lock().await;
        let now = chrono::Utc::now();

        if now.signed_duration_since(limiter.window_start).num_hours() >= 1 {
            limiter.sent_count = 0;
            limiter.window_start = now;
        }

        if limiter.sent_count >= limiter.per_hour {
            return Err("rate limit exceeded".to_string());
        }

        limiter.sent_count += 1;
        drop(limiter);

        let full_message = if let Some(quote) = quote_info {
            format!(
                "📈 {} {} Alert\n\
                 \n\
                 💰 价格信息:\n\
                 ├─ 当前价: {}\n\
                 ├─ 涨跌额: {}\n\
                 ├─ 涨跌幅: {}\n\
                 ├─ 昨收价: ${:.2}\n\
                 ├─ 开盘价: ${:.2}\n\
                 ├─ 最高价: ${:.2}\n\
                 ├─ 最低价: ${:.2}\n\
                 └─ 成交量: {}\n\
                 \n\
                 📋 规则: {} | 告警级别: {}",
                get_severity_emoji(severity),
                symbol,
                quote.format_price(),
                quote.format_change(),
                quote.format_change_percent(),
                quote.previous_close,
                quote.open,
                quote.high,
                quote.low,
                quote.format_volume(),
                message,
                severity
            )
        } else {
            format!(
                "📈 {} {} Alert\n{}\n规则: {} | 告警级别: {}",
                get_severity_emoji(severity),
                symbol,
                message,
                rule_type,
                severity
            )
        };

        let payload = serde_json::json!({
            "msg_type": "text",
            "content": {
                "text": full_message
            }
        });

        tracing::info!("Sending webhook with payload: {}", payload);

        let response = self
            .client
            .post(&self.webhook_url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("failed to send webhook: {}", e))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        tracing::info!("Webhook response status: {}, body: {}", status, body);

        if !status.is_success() {
            return Err(format!("webhook returned status: {} body: {}", status, body));
        }

        Ok(())
    }
}

fn get_severity_emoji(severity: &str) -> &'static str {
    match severity {
        "critical" => "🔴",
        "warning" => "🟡",
        _ => "🟢",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_engine_creation() {
        let engine = AlertEngine::new("https://example.com/hook".to_string());
        assert_eq!(engine.webhook_url, "https://example.com/hook");
    }
}
