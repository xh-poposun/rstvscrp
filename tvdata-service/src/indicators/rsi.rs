pub fn calculate_rsi(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period + 1 {
        return 50.0;
    }

    let mut gains = Vec::with_capacity(prices.len() - 1);
    let mut losses = Vec::with_capacity(prices.len() - 1);

    for window in prices.windows(2) {
        let change = window[1] - window[0];
        if change > 0.0 {
            gains.push(change);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-change);
        }
    }

    let initial_avg_gain: f64 = gains.iter().take(period).sum::<f64>() / period as f64;
    let initial_avg_loss: f64 = losses.iter().take(period).sum::<f64>() / period as f64;

    let mut avg_gain = initial_avg_gain;
    let mut avg_loss = initial_avg_loss;

    for i in period..gains.len() {
        avg_gain = (avg_gain * (period - 1) as f64 + gains[i]) / period as f64;
        avg_loss = (avg_loss * (period - 1) as f64 + losses[i]) / period as f64;
    }

    if avg_loss == 0.0 {
        return 100.0;
    }

    let rs = avg_gain / avg_loss;
    let rsi = 100.0 - (100.0 / (1.0 + rs));

    rsi.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_with_standard_data() {
        let prices = vec![
            44.0, 44.34, 44.09, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.76, 46.27, 45.78, 45.38, 45.82, 46.08, 45.99, 46.35, 46.66, 46.66, 46.52, 46.75,
        ];
        let rsi = calculate_rsi(&prices, 14);
        assert!((0.0..=100.0).contains(&rsi));
    }

    #[test]
    fn test_rsi_uptrend() {
        let prices: Vec<f64> = (0..30).map(|i| 100.0 + (i as f64)).collect();
        let rsi = calculate_rsi(&prices, 14);
        assert!(rsi > 50.0);
    }

    #[test]
    fn test_rsi_downtrend() {
        let prices: Vec<f64> = (0..30).map(|i| 130.0 - (i as f64)).collect();
        let rsi = calculate_rsi(&prices, 14);
        assert!(rsi < 50.0);
    }

    #[test]
    fn test_rsi_insufficient_data() {
        let prices = vec![100.0, 101.0, 102.0];
        let rsi = calculate_rsi(&prices, 14);
        assert_eq!(rsi, 50.0);
    }

    #[test]
    fn test_rsi_stable_price() {
        let prices: Vec<f64> = (0..30).map(|i| 100.0 + (i as f64) * 0.1).collect();
        let rsi = calculate_rsi(&prices, 14);
        assert!((0.0..=100.0).contains(&rsi));
    }

    #[test]
    fn test_rsi_bounds() {
        let prices: Vec<f64> = (0..100)
            .map(|i| if i % 2 == 0 { 100.0 } else { 200.0 })
            .collect();
        let rsi = calculate_rsi(&prices, 14);
        assert!((0.0..=100.0).contains(&rsi));
    }
}
