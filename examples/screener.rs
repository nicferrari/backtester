use std::error::Error;
use backtester::datas::Data;
use chrono::{DateTime};

fn main() ->Result<(),Box<dyn Error>> {
    let quotes = Data::new_from_yahoo("NVDA","1d","1mo")?;
    println!("{:.2}%",quotes.ret());
    let start_date = DateTime::parse_from_rfc3339("2024-07-13T13:30:00+00:00").unwrap();
    println!("{:.2}",quotes.ret_from_date(start_date));
    println!("{:.?}%",quotes.ret_from_period(&["2w","5d"]));
    Ok(())
}