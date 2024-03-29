use backtester::datas::Data;
use backtester::Result;
use backtester::ta::{sma, Indicator, rsi};
#[test]
fn indicator_tests()->Result<()>{
    let quotes = &Data::new_from_yahoo("AAPL")?;
    //let indicator = sma(quotes,5);
    let indicator = rsi(quotes,5);
    println!("{:?}",indicator);
    println!("{:?}",quotes.close());
    let a = Indicator{indicator:indicator, quotes:quotes.clone()};
    a.to_csv();
    Ok(())
}