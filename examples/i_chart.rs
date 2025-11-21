use std::error::Error;
use rs_backtester::backtester::Backtest;
use rs_backtester::chart_test::{backtest_i_chart};
use rs_backtester::data::Data;
use rs_backtester::strategies::sma_cross;

pub fn main() -> Result<(), Box<dyn Error>> {
    let data = Data::new_from_yahoo("GOOG", "1d", "1y")?;
    let strategy = sma_cross(data.clone(), 5, 15);
    let backtest = Backtest::new(strategy, 100000.);
    backtest_i_chart(&backtest);
    Ok(())
}