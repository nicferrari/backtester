use std::error::Error;
use backtester::backtester::Backtest;
use backtester::datas::Data;
use backtester::strategies::sma_cross;

fn main()->Result<(),Box<dyn Error>> {
    //example to log or debug backtesting
    let quotes = Data::new_from_yahoo("PLTR","1d","6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64);
    sma_cross_tester.log(&["date","open","high","low","close","position","account","indicator"]);
    sma_cross_tester.to_csv("sma_cross.csv")?;
    Ok(())
}