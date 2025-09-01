use rs_backtester::backtester::{Backtest, Commission};
use rs_backtester::datas::Data;
use rs_backtester::strategies::{simple_sma, sma_cross};
use rs_backtester::report::report;
use rs_backtester::trades::trade_list;

fn main() {
    let quotes = Data::new_from_yahoo("NVDA", "1d", "6mo").unwrap();
    let backtest = Backtest::new(quotes.clone(), simple_sma(quotes.clone(), 10), 100000., Commission::default());
    println!("--------------------");
    report(backtest.clone());
    trade_list(backtest.clone());
    let backtest_long = Backtest::new(quotes.clone(), simple_sma(quotes.clone(), 10).long_only(), 100000., Commission::default());
    println!("--------------------");
    report(backtest_long.clone());
    trade_list(backtest_long.clone());
    let backtest_short = Backtest::new(quotes.clone(), simple_sma(quotes.clone(), 10).short_only(), 100000., Commission::default());
    println!("--------------------");
    report(backtest_short.clone());
    trade_list(backtest_short.clone());
    let mut commission = Commission::default();
    commission.rate=0.01;
    let backtest_cross = Backtest::new(quotes.clone(), sma_cross(quotes.clone(), 5, 20), 100000., commission);
    println!("--------------------");
    report(backtest_cross.clone());
    trade_list(backtest_cross.clone());
}