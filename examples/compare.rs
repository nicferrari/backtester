use std::error::Error;
use backtester::backtester::Backtest;
use backtester::datas::Data;
use backtester::strategies::{simple_sma, sma_cross};
use backtester::report::compare;

pub fn main()->Result<(),Box<dyn Error>>{
    let quotes = Data::load("GOOG.csv","GOOG")?;
    let sma_cross = sma_cross(quotes.clone(),10,20);
    let sma = simple_sma(quotes.clone(),10);
    let mut sma_cross_backt = Backtest::new(quotes.clone(),sma_cross,100000.);
    let mut sma_backt = Backtest::new(quotes.clone(),sma,100000.);
    sma_cross_backt.calculate();
    sma_backt.calculate();
    let mut cmp_backt=Vec::new();
    cmp_backt.push(sma_backt);
    cmp_backt.push(sma_cross_backt);
    compare(cmp_backt);
    Ok(())
}