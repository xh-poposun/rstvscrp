//! WSS debug logging module.
//!
//! Records all sent/received WebSocket frames to a file for debugging.
//! Controlled by the `WSS_DEBUG_LOG` environment variable (path to log file).
//! Auth tokens are redacted; heartbeat frames are condensed.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use std::sync::OnceLock;

use serde_json::Value;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024;

static LOGGER: OnceLock<Mutex<DebugLogger>> = OnceLock::new();

#[derive(Debug)]
struct DebugLogger {
    file: File,
    path: String,
}

/// Returns `true` if the `WSS_DEBUG_LOG` environment variable is set.
#[must_use]
pub fn is_enabled() -> bool {
    std::env::var("WSS_DEBUG_LOG").is_ok_and(|v| !v.is_empty())
}

/// Opens or creates the debug log file at `path`.
///
/// Truncates the file if it already exceeds [`MAX_LOG_SIZE`].
pub fn init(path: &str) -> Result<(), std::io::Error> {
    if let Ok(meta) = std::fs::metadata(path) {
        if meta.len() > MAX_LOG_SIZE {
            File::create(path)?;
        }
    }

    let file = OpenOptions::new().create(true).append(true).open(path)?;

    LOGGER
        .set(Mutex::new(DebugLogger {
            file,
            path: path.to_owned(),
        }))
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "logger already initialized",
            )
        })
}

/// Logs an outgoing WebSocket frame.
pub fn log_send(session_id: &str, symbol: &str, payload: &str) {
    log_frame("SEND", session_id, symbol, payload);
}

/// Logs an incoming WebSocket frame.
pub fn log_recv(session_id: &str, symbol: &str, payload: &str) {
    log_frame("RECV", session_id, symbol, payload);
}

fn log_frame(direction: &str, session_id: &str, symbol: &str, payload: &str) {
    let Some(lock) = LOGGER.get() else {
        return;
    };

    let formatted = format_payload(payload);
    let timestamp = format_timestamp();

    let line = format!("[{timestamp}] [{direction}] [{session_id}] [{symbol}] {formatted}\n");

    let Ok(mut guard) = lock.lock() else {
        return;
    };

    if let Ok(meta) = guard.file.metadata() {
        if meta.len() > MAX_LOG_SIZE {
            if let Ok(new_file) = File::create(&guard.path) {
                guard.file = new_file;
            }
        }
    }

    let _ = guard.file.write_all(line.as_bytes());
}

fn format_timestamp() -> String {
    let now = OffsetDateTime::now_utc();
    now.format(&Rfc3339).unwrap_or_else(|_| String::from("???"))
}

fn format_payload(payload: &str) -> String {
    if let Some(hb) = payload.strip_prefix("~h~") {
        return format!("[HEARTBEAT] {hb}");
    }

    match serde_json::from_str::<Value>(payload) {
        Ok(mut val) => {
            if is_auth_message(&val) {
                redact_auth_token(&mut val);
                return val.to_string();
            }
            payload.to_owned()
        }
        Err(_) => payload.to_owned(),
    }
}

fn is_auth_message(val: &Value) -> bool {
    val.get("m")
        .and_then(|m| m.as_str())
        .is_some_and(|m| m == "set_auth_token")
}

fn redact_auth_token(val: &mut Value) {
    if let Some(params) = val.get_mut("p").and_then(|p| p.as_array_mut()) {
        if let Some(first) = params.first_mut() {
            *first = Value::String(String::from("[REDACTED]"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_detection() {
        assert_eq!(format_payload("~h~30"), "[HEARTBEAT] 30");
        assert_eq!(format_payload("~h~"), "[HEARTBEAT] ");
    }

    #[test]
    fn auth_token_redaction() {
        let raw = r#"{"m":"set_auth_token","p":["super-secret-token-123"]}"#;
        let formatted = format_payload(raw);
        assert!(formatted.contains("[REDACTED]"));
        assert!(!formatted.contains("super-secret-token-123"));
    }

    #[test]
    fn non_auth_json_unchanged() {
        let raw = r#"{"m":"quote_add_symbols","p":["NASDAQ:AAPL"]}"#;
        assert_eq!(format_payload(raw), raw);
    }

    #[test]
    fn non_json_passthrough() {
        let raw = "arbitrary-binary-data";
        assert_eq!(format_payload(raw), raw);
    }

    #[test]
    fn format_includes_all_fields() {
        let payload = r#"{"m":"quote_add_symbols","p":["NASDAQ:AAPL"]}"#;
        let formatted = format_payload(payload);
        let timestamp = format_timestamp();
        let line = format!("[{timestamp}] [SEND] [qs_0000000000000001] [SSE:600941] {formatted}");

        assert!(line.contains("[SEND]"));
        assert!(line.contains("[qs_0000000000000001]"));
        assert!(line.contains("[SSE:600941]"));
        assert!(line.contains(&timestamp));
    }
}
