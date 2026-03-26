use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub monitor: MonitorConfig,
    pub alert: AlertConfig,
    pub search: SearchConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MonitorConfig {
    pub check_interval_secs: u64,
    pub max_concurrent_checks: usize,
    pub cooldown_secs: u64,
    pub max_monitors: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlertConfig {
    pub rate_limit_per_hour: u32,
    pub webhook: WebhookConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub msg_type: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SearchConfig {
    pub language: String,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            language: "cn".to_string(),
        }
    }
}


impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            check_interval_secs: 60,
            max_concurrent_checks: 10,
            cooldown_secs: 300,
            max_monitors: 500,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: "./data/tvmonitor.db".to_string(),
        }
    }
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            msg_type: "interactive".to_string(),
        }
    }
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            rate_limit_per_hour: 10,
            webhook: WebhookConfig::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            monitor: MonitorConfig::default(),
            alert: AlertConfig::default(),
            search: SearchConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let monitor = MonitorConfig::default();
        assert_eq!(monitor.check_interval_secs, 60);
        assert_eq!(monitor.max_monitors, 500);
        assert_eq!(monitor.cooldown_secs, 300);
    }

    #[test]
    fn test_config_load_yaml() {
        let yaml = r#"
server:
  host: "127.0.0.1"
  port: 9000
database:
  path: "/tmp/test.db"
monitor:
  check_interval_secs: 30
  max_concurrent_checks: 5
  cooldown_secs: 600
  max_monitors: 100
alert:
  rate_limit_per_hour: 20
  webhook:
    url: "https://example.com/hook"
    msg_type: "text"
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.server.port, 9000);
        assert_eq!(config.monitor.check_interval_secs, 30);
        assert_eq!(config.alert.webhook.url, "https://example.com/hook");
    }
}
