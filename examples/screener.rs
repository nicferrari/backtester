use std::error::Error;
use backtester::datas::Data;
use backtester::screeners::screener_returns;
use chrono::{DateTime};

fn main() ->Result<(),Box<dyn Error>> {
    let quotes = Data::new_from_yahoo("NVDA","1d","1mo")?;
    println!("{:.2}%",quotes.ret());
    let start_date = DateTime::parse_from_rfc3339("2024-07-13T13:30:00+00:00").unwrap();
    println!("{:.2}",quotes.ret_from_date(start_date));
    let terms = ["1d","1w","2w","4w"];
    //println!("{:.?}%",quotes.ret_from_period(&terms));
    let rets = quotes.ret_from_period(&terms);
    terms.iter().zip(rets.iter()).for_each(|(term,val)|println!("return over {} is {:.2}% ",term,val));
    let tickers = &["AAPL","NVDA","FSLR"];
    let scr = screener_returns::new(tickers,&terms);
    println!("--------------------------");
    scr.report();
    Ok(())
}