use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacdConfig {
    pub fast: usize,
    pub slow: usize,
    pub signal: usize,
}

impl Default for MacdConfig {
    fn default() -> Self {
        Self {
            fast: 12,
            slow: 26,
            signal: 9,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacdResult {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

pub fn calculate_macd(prices: &[f64], config: &MacdConfig) -> Option<MacdResult> {
    if prices.len() < config.slow {
        return None;
    }

    let fast_ema = calculate_ema(prices, config.fast)?;
    let slow_ema = calculate_ema(prices, config.slow)?;
    let macd_line = fast_ema - slow_ema;

    let macd_values = calculate_macd_line(prices, config)?;
    let signal_line = calculate_ema_of_series(&macd_values, config.signal)?;

    let histogram = macd_line - signal_line;

    Some(MacdResult {
        macd: macd_line,
        signal: signal_line,
        histogram,
    })
}

fn calculate_ema(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period {
        return None;
    }

    let multiplier = 2.0 / (period as f64 + 1.0);
    let mut ema = prices.iter().take(period).sum::<f64>() / period as f64;

    for price in prices.iter().skip(period) {
        ema = (*price - ema) * multiplier + ema;
    }

    Some(ema)
}

fn calculate_macd_line(prices: &[f64], config: &MacdConfig) -> Option<Vec<f64>> {
    if prices.len() < config.slow {
        return None;
    }

    let mut macd_line = Vec::with_capacity(prices.len());

    for i in (config.slow - 1)..prices.len() {
        let fast_ema = calculate_ema(&prices[..=i], config.fast)?;
        let slow_ema = calculate_ema(&prices[..=i], config.slow)?;
        macd_line.push(fast_ema - slow_ema);
    }

    Some(macd_line)
}

fn calculate_ema_of_series(values: &[f64], period: usize) -> Option<f64> {
    if values.len() < period {
        return None;
    }
    calculate_ema(values, period)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macd_config_default() {
        let config = MacdConfig::default();
        assert_eq!(config.fast, 12);
        assert_eq!(config.slow, 26);
        assert_eq!(config.signal, 9);
    }

    #[test]
    fn test_calculate_macd() {
        let prices: Vec<f64> = (0..50).map(|i| 100.0 + (i as f64) * 0.5).collect();
        let config = MacdConfig::default();
        let result = calculate_macd(&prices, &config);
        assert!(result.is_some());
        let macd = result.unwrap();
        assert!(macd.macd > 0.0);
    }

    #[test]
    fn test_macd_insufficient_data() {
        let prices = vec![100.0, 101.0, 102.0];
        let config = MacdConfig::default();
        let result = calculate_macd(&prices, &config);
        assert!(result.is_none());
    }

    #[test]
    fn test_macd_downtrend() {
        let prices: Vec<f64> = (0..50).map(|i| 150.0 - (i as f64)).collect();
        let config = MacdConfig::default();
        let result = calculate_macd(&prices, &config);
        assert!(result.is_some());
        let macd = result.unwrap();
        assert!(macd.macd < 0.0);
    }

    #[test]
    fn test_macd_cross() {
        let mut prices = Vec::new();
        for i in 0..30 {
            prices.push(100.0 + (i as f64));
        }
        for i in 0..30 {
            prices.push(130.0 - (i as f64));
        }
        let config = MacdConfig::default();
        let result = calculate_macd(&prices, &config);
        assert!(result.is_some());
    }
}
