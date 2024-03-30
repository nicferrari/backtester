//use backtester::Result;
use backtester::backtester::Backtest;
use backtester::datas::Data;
use backtester::strategies::buy_n_hold;
use std::error::Error;

#[test]
fn init_backtester()->Result<(),Box<dyn Error>>{
    let quotes = Data::new_from_yahoo("AAPL","1d","1mo")?;
    let strategy = buy_n_hold(quotes.clone());
    let tester = Backtest::new(quotes.clone(),strategy.clone(),100000f64);
    Ok(())
}