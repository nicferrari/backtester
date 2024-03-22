use backtester::datas::Data;
use backtester::Result;
use backtester::strategies::simple_sma;

#[test]
fn strategies_tests()->Result<()>{
    let quotes = Data::new_from_yahoo("AAPL")?;
    let sma_cross_strategy = simple_sma(quotes.clone(), 5);
    sma_cross_strategy.to_csv();
    Ok(())
}