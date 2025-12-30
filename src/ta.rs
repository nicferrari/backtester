use crate::data::Data;
use std::sync::Arc;
///container for checking calculation of indicator vs mktdata
pub struct Indicator {
    pub indicator: Vec<f64>,
    pub quotes: Arc<Data>,
}
///calculate simple moving average of period x
///use close data
/// todo! generalize to OHLC
pub fn sma(quotes: &Data, period: usize) -> Vec<f64> {
    let mut indicator: Vec<f64> = vec![-1.; period - 1];
    let length = quotes.datetime.len();
    for i in period..length + 1 {
        let slice = &quotes.close[i - period..i];
        let sum: f64 = Iterator::sum(slice.iter());
        let sma = sum / (period as f64);
        indicator.append(&mut vec![sma; 1]);
    }
    indicator
}
///calculate RSI
///Wilder version
pub fn rsi(quotes: &Data, period: usize) -> Vec<f64> {
    let close = &quotes.close;
    let mut out = vec![f64::NAN; period];

    // 1. Compute close-close diffs
    let diffs: Vec<f64> = close
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();

    // 2. First average gain/loss (simple average)
    let (sum_gain, sum_loss) = diffs[..period].iter().fold((0.0, 0.0), |(g, l), &d| {
        if d > 0.0 { (g + d, l) } else { (g, l - d) }
    });

    let mut avg_gain = sum_gain / period as f64;
    let mut avg_loss = sum_loss / period as f64;

    // 3. First RSI
    let first_rsi = if avg_loss == 0.0 {
        100.0
    } else {
        let rs = avg_gain / avg_loss;
        100.0 - 100.0 / (1.0 + rs)
    };
    out.push(first_rsi);

    // 4. Wilder smoothing for the rest
    diffs[period..].iter().for_each(|&d| {
        let gain = d.max(0.0);
        let loss = (-d).max(0.0);

        avg_gain = (avg_gain * (period as f64 - 1.0) + gain) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + loss) / period as f64;

        let rsi = if avg_loss == 0.0 {
            100.0
        } else {
            let rs = avg_gain / avg_loss;
            100.0 - 100.0 / (1.0 + rs)
        };

        out.push(rsi);
    });

    out
}
