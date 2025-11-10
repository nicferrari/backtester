use std::error::Error;
use rs_backtester::backtester::{Backtest, Commission};
use rs_backtester::datas::Data;
use rs_backtester::strategies::{buy_n_hold, rsi_strategy, simple_sma, sma_cross};
use rs_backtester::report::{report};
use std::env::{args};

pub fn main()->Result<(),Box<dyn Error>>{
    //example to compare different strategies
    //call with optional --filename="xxx.csv"
    //fallback to "GOOGLE.csv" which should be in directory
    let args:Vec<String> = args().collect();
    let fallback_file = "test_data//NVDA.csv";
    let mut filename = fallback_file;
    for arg in &args{
        if arg.starts_with("--filename="){
            filename = &arg[11..];
        }
    }
    //let path = env::current_dir()?;
    //println!("Loading filename = {:?}",path.into_os_string().into_string().unwrap()+"\\"+filename);
    let quotes = Data::load(filename,"test_data/NVDA.csv")?;
    let sma_cross = sma_cross(quotes.clone(),10,20);
    let sma = simple_sma(quotes.clone(),10);
    let rsi_strategy = rsi_strategy(quotes.clone(),15);
    let sma_cross_backt = Backtest::new(quotes.clone(),sma_cross,100000., Commission::default());
    let sma_backt = Backtest::new(quotes.clone(),sma,100000., Commission::default());
    let rsi_backt = Backtest::new(quotes.clone(),rsi_strategy,100000., Commission::default());
    let mut cmp_backt=Vec::new();
    let buynhold = Backtest::new(quotes.clone(),buy_n_hold(quotes.clone()),100000., Commission::default());
    cmp_backt.push(buynhold);
    cmp_backt.push(sma_backt);
    cmp_backt.push(sma_cross_backt);
    cmp_backt.push(rsi_backt.clone());
    report(cmp_backt);
    Ok(())
}