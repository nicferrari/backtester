use std::error::Error;
use backtester::backtester::Backtest;
use backtester::datas::Data;
use backtester::strategies::{sma_cross};
use backtester::report::{report};

fn main()->Result<(),Box<dyn Error>>{
    //example to calculate backtesting results
    let quotes = Data::new_from_yahoo("NVDA","1d","6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64);
    report(sma_cross_tester);
    Ok(())
}