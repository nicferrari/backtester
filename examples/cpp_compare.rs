use std::error::Error;
use rs_backtester::backtester::{Backtest, Commission};
use rs_backtester::datas::Data;
use rs_backtester::strategies::simple_sma;

fn main() -> Result<(),Box<dyn Error>>{
    let quotes = Data::load("GOOGLE.csv","GOOG")?;
    let sma = simple_sma(quotes.clone(),5);
    let sma_tester = Backtest::new(quotes.clone(),sma,100000.,Commission::default());
    sma_tester.to_csv("cpp_compare.csv")?;
    Ok(())
}