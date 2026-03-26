pub mod macd;
pub mod rsi;

pub use macd::{MacdConfig, MacdResult, calculate_macd};
pub use rsi::calculate_rsi;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Indicator {
    RSI {
        period: usize,
    },
    MACD {
        fast: usize,
        slow: usize,
        signal: usize,
    },
}

impl Default for Indicator {
    fn default() -> Self {
        Self::RSI { period: 14 }
    }
}

pub trait IndicatorCalculator: Send + Sync {
    fn name(&self) -> &'static str;
    fn calculate(&self, prices: &[f64]) -> Option<f64>;
}

#[derive(Debug, Clone)]
pub struct RsiCalculator {
    period: usize,
}

impl RsiCalculator {
    pub fn new(period: usize) -> Self {
        Self { period }
    }
}

impl IndicatorCalculator for RsiCalculator {
    fn name(&self) -> &'static str {
        "RSI"
    }

    fn calculate(&self, prices: &[f64]) -> Option<f64> {
        Some(rsi::calculate_rsi(prices, self.period))
    }
}

#[derive(Debug, Clone)]
pub struct MacdCalculator {
    config: MacdConfig,
}

impl MacdCalculator {
    pub fn new(fast: usize, slow: usize, signal: usize) -> Self {
        Self {
            config: MacdConfig { fast, slow, signal },
        }
    }
}

impl IndicatorCalculator for MacdCalculator {
    fn name(&self) -> &'static str {
        "MACD"
    }

    fn calculate(&self, prices: &[f64]) -> Option<f64> {
        let result = macd::calculate_macd(prices, &self.config)?;
        Some(result.histogram)
    }
}

pub struct IndicatorRegistry {
    calculators: Vec<Box<dyn IndicatorCalculator>>,
}

impl Default for IndicatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl IndicatorRegistry {
    pub fn new() -> Self {
        Self {
            calculators: vec![
                Box::new(RsiCalculator::new(14)),
                Box::new(MacdCalculator::new(12, 26, 9)),
            ],
        }
    }

    pub fn get(&self, name: &str) -> Option<&dyn IndicatorCalculator> {
        self.calculators
            .iter()
            .find(|c| c.name() == name)
            .map(|b| b.as_ref())
    }

    pub fn calculate(&self, name: &str, prices: &[f64]) -> Option<f64> {
        self.get(name).and_then(|c| c.calculate(prices))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_calculator() {
        let calc = RsiCalculator::new(14);
        let prices = vec![
            44.0, 44.34, 44.09, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.76, 46.27,
        ];
        let rsi = calc.calculate(&prices);
        assert!(rsi.is_some());
        let rsi_val = rsi.unwrap();
        assert!(rsi_val > 0.0 && rsi_val <= 100.0);
    }

    #[test]
    fn test_macd_calculator() {
        let calc = MacdCalculator::new(12, 26, 9);
        let prices: Vec<f64> = (0..50).map(|i| 100.0 + (i as f64) * 0.5).collect();
        let macd = calc.calculate(&prices);
        assert!(macd.is_some());
    }

    #[test]
    fn test_registry() {
        let registry = IndicatorRegistry::new();
        let prices: Vec<f64> = (0..50).map(|i| 100.0 + (i as f64) * 0.5).collect();

        assert!(registry.calculate("RSI", &prices).is_some());
        assert!(registry.calculate("MACD", &prices).is_some());
        assert!(registry.calculate("UNKNOWN", &prices).is_none());
    }
}
