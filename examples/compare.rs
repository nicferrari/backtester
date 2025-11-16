use rs_backtester::backtester::Backtest;
use rs_backtester::data::Data;
use rs_backtester::metrics::report_vertical;
use rs_backtester::strategies::{buy_n_hold, rsi_strategy, sma_cross, sma_strategy};
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let filename = "test_data//NVDA.csv";
    let quotes_arc = Data::load(filename, "NVDA")?;
    let buynhold = buy_n_hold(quotes_arc.clone());
    let sma_cross = sma_cross(quotes_arc.clone(), 10, 20);
    let sma = sma_strategy(quotes_arc.clone(), 10);
    let rsi = rsi_strategy(quotes_arc.clone(), 15);
    let buynhold_bt = Backtest::new(buynhold, 100_000.);
    let sma_cross_bt = Backtest::new(sma_cross, 100_000.);
    let sma_bt = Backtest::new(sma, 100_000.);
    let rsi_bt = Backtest::new(rsi, 100_000.);
    report_vertical(&[&buynhold_bt, &sma_bt, &sma_cross_bt, &rsi_bt]);
    Ok(())
}
