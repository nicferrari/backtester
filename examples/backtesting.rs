use std::error::Error;
use rs_backtester::backtester::{Backtest, Commission};
use rs_backtester::datas::Data;
use rs_backtester::strategies::{sma_cross};
use rs_backtester::report::{report};

fn main()->Result<(),Box<dyn Error>>{
    //example to calculate backtesting results
    let quotes = Data::new_from_yahoo("NVDA","1d","6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let sma_cross_tester_zero_comm = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64, Commission::default());
    println!("---------------------------");
    println!("With zero commission rate");
    println!("---------------------------");
    report(sma_cross_tester_zero_comm);

    //now let's see if strategy is still profitable with a custom commission rate of 1% on every trade
    let mut commission = Commission::default();
    commission.rate = 0.01;

    let sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64, commission.clone());
    println!("---------------------------");
    println!("Now with {:.2}% commission rate",commission.rate*100.);
    println!("---------------------------");
    report(sma_cross_tester);
    Ok(())
}