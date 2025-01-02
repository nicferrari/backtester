use std::error::Error;
use backtester::backtester::{Backtest, Commission};
use backtester::datas::Data;
use backtester::strategies::{sma_cross};
use backtester::report::{report};

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

    let sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64, commission);
    println!("---------------------------");
    println!("Now with 1% commission rate");
    println!("---------------------------");
    report(sma_cross_tester);
    Ok(())
}