use rs_backtester::backtester::Backtest;
use rs_backtester::data::Data;
use rs_backtester::orders::Order::{BUY, NULL};
use rs_backtester::strategies::sma_strategy;
use std::error::Error;
use std::sync::Arc;

fn load_data() -> Result<Arc<Data>, Box<dyn Error>> {
    let filename = "test_data//NVDA.csv";
    Ok(Data::load(filename, "NVDA")?)
}
#[test]
fn ta_tests() -> Result<(), Box<dyn std::error::Error>> {
    let quotes = load_data().unwrap();
    let sma = sma_strategy(quotes, 10);
    let sma_bt = Backtest::new(sma, 100_000.);
    assert_eq!(sma_bt.strategy.indicator.unwrap()[0][9], 13.54124994277954);
    assert_eq!(sma_bt.strategy.choices[8], NULL);
    assert_eq!(sma_bt.strategy.choices[9], BUY);
    Ok(())
}

#[cfg(feature = "fin-primitives")]
#[test]
fn atr_wilder_tests() -> Result<(), Box<dyn std::error::Error>> {
    let q = load_data()?;
    let atr = rs_backtester::ta::atr(&q, 14);
    let tr = |i: usize| {
        (q.high[i] - q.low[i])
            .max((q.high[i] - q.close[i - 1]).abs())
            .max((q.low[i] - q.close[i - 1]).abs())
    };
    let seed: f64 = (1..=14).map(tr).sum::<f64>() / 14.;
    assert!(atr[13].is_nan());
    assert!((atr[14] - seed).abs() < 1e-9);
    assert!((atr[15] - (seed * 13. + tr(15)) / 14.).abs() < 1e-9);
    let (line, signal, hist) = rs_backtester::ta::macd(&q, 12, 26, 9);
    assert!(line[24].is_nan() && !line[25].is_nan() && !signal[33].is_nan());
    assert!((hist[40] - (line[40] - signal[40])).abs() < 1e-9);
    Ok(())
}
