use backtester::datas::Data;
use backtester::Result;
use backtester::strategies::sma_cross;

#[test]
fn strategies_tests()->Result<()>{
    let quotes = Data::new_from_yahoo("AAPL".to_string())?;
    let sma_cross_strategy = sma_cross(quotes.clone(),5);
    sma_cross_strategy.to_csv();
    Ok(())
}