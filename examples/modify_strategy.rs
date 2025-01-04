use std::error::Error;
use rs_backtester::backtester::{Backtest, Commission};
use rs_backtester::datas::Data;
use rs_backtester::report::report;
use rs_backtester::strategies::sma_cross;

fn main() ->Result<(),Box<dyn Error>> {
    let quotes = Data::new_from_yahoo("NVDA", "1d", "6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10, 20);
    let sma_cross_tester = Backtest::new(quotes.clone(), sma_cross_strategy.clone(), 100000f64, Commission::default());
    println!("---------------------------");
    println!("SMA Cross Strategy");
    println!("---------------------------");
    report(sma_cross_tester.clone());
    //if a strategy doesn't work one side, let's try the opposite!
    let sma_cross_inverted = sma_cross_tester.strategy().invert();
    let sma_cross_inverted_tester = Backtest::new(quotes.clone(), sma_cross_inverted.clone(), 100000., Commission::default());
    println!("---------------------------");
    println!("SMA Cross Strategy - Inverted");
    println!("---------------------------");
    report(sma_cross_inverted_tester);
    //let's try a long only
    let sma_cross_long = sma_cross_strategy.long_only();
    let sma_cross_long_tester = Backtest::new(quotes.clone(), sma_cross_long.clone(), 100000., Commission::default());
    println!("---------------------------");
    println!("SMA Cross Strategy - Long Only");
    println!("---------------------------");
    report(sma_cross_long_tester);
    //let's try a short only
    let sma_cross_short = sma_cross_strategy.short_only();
    let sma_cross_short_tester = Backtest::new(quotes.clone(), sma_cross_short.clone(), 100000., Commission::default());
    println!("---------------------------");
    println!("SMA Cross Strategy - Short Only");
    println!("---------------------------");
    report(sma_cross_short_tester);
    Ok(())
}