use std::error::Error;
use rs_backtester::backtester_new::Backtest_arc;
use rs_backtester::datas::Data;
use rs_backtester::metrics::{report_horizontal_arc, report_vertical_arc};
use rs_backtester::strategies::{sma_cross, sma_cross_arc};

fn main() ->Result<(),Box<dyn Error>> {
    let quotes = Data::new_from_yahoo_arc("NVDA","1d","6mo")?;
    let sma_cross_strategy = sma_cross_arc(quotes,10,20);
    let sma_cross_bt = Backtest_arc::new(sma_cross_strategy.clone(),100_000.);
    report_horizontal_arc(&[&sma_cross_bt]);
    //if a strategy doesn't work one side, let's try the opposite!
    let sma_cross_inverted = sma_cross_strategy.clone().invert();
    let sma_cross_inverted_bt = Backtest_arc::new(sma_cross_inverted,100_000.);
    report_horizontal_arc(&[&sma_cross_inverted_bt]);
    //let's try a long only
    let sma_cross_long = sma_cross_strategy.clone().long_only();
    let sma_cross_long_bt = Backtest_arc::new(sma_cross_long,100_000.);
    report_horizontal_arc(&[&sma_cross_long_bt]);
    //let's try a short only
    let sma_cross_short = sma_cross_strategy.short_only();
    let sma_cross_short_bt = Backtest_arc::new(sma_cross_short,100_000.);
    report_horizontal_arc(&[&sma_cross_short_bt]);
    //let's compare them simultaneously
    report_vertical_arc(&[&sma_cross_bt,&sma_cross_inverted_bt,&sma_cross_long_bt,&sma_cross_short_bt]);
    Ok(())
}