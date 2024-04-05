use std::error::Error;
use backtester::backtester::Backtest;
use backtester::datas::Data;
use backtester::strategies::{rsi_strategy, simple_sma, sma_cross};
use backtester::report::compare;
use std::env::{args};

pub fn main()->Result<(),Box<dyn Error>>{
    //call with optional --file="xxx.csv"
    let args:Vec<String> = args().collect();
    let fallback_file = "GOOGLE.csv";
    let mut filename = fallback_file;
    for arg in &args{
        if arg.starts_with("--filename="){
            filename = &arg[11..];
        }
    }
    println!("Trying to load filename = {}",filename);
    let quotes = Data::load(filename,"GOOG")?;
    let sma_cross = sma_cross(quotes.clone(),10,20);
    let sma = simple_sma(quotes.clone(),10);
    let rsi_strategy = rsi_strategy(quotes.clone(),15);
    let mut sma_cross_backt = Backtest::new(quotes.clone(),sma_cross,100000.);
    let mut sma_backt = Backtest::new(quotes.clone(),sma,100000.);
    let mut rsi_backt = Backtest::new(quotes.clone(),rsi_strategy,100000.);
    sma_cross_backt.calculate();
    sma_backt.calculate();
    rsi_backt.calculate();
    let mut cmp_backt=Vec::new();
    cmp_backt.push(sma_backt);
    cmp_backt.push(sma_cross_backt);
    cmp_backt.push(rsi_backt.clone());
    compare(&cmp_backt);
    rsi_backt.to_csv("rsi_backtest.csv");
    Ok(())
}