use rs_backtester::datas::Data;
use std::error::Error;
use rs_backtester::strategies::simple_sma;

#[test]
fn strategies_tests()->Result<(), Box<dyn Error>>{
    let quotes = Data::new_from_yahoo("AAPL","1d","1mo")?;
    let sma_cross_strategy = simple_sma(quotes.clone(), 5);
    sma_cross_strategy.to_csv("strategies.csv");
    Ok(())
}