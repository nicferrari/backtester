use std::error::Error;
use backtester::backtester::Backtest;
use backtester::charts::{plot, PlotConfig};
use backtester::datas::Data;
use backtester::strategies::{sma_cross};
use backtester::report::{report,compare,uniq_report};

fn main()->Result<(),Box<dyn Error>>{

    let quotes = Data::new_from_yahoo("CSSPX.MI","1d","6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64);

    sma_cross_tester.log(&["date","open","high","low","close","position","account","indicator"]);

    plot(sma_cross_tester.clone(), PlotConfig::default())?;

    sma_cross_tester.to_csv("sma_cross.csv")?;

    report(&sma_cross_tester);
    //uniq_report(sma_cross_tester);

    Ok(())
}