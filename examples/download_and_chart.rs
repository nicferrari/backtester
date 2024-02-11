use backtester::datas::{Data};
use backtester::charts::{plot};
use backtester::Result;
use backtester::reports::print_report;
use backtester::strategies::Strategy;

fn main()->Result<()>{
    let quotes = Data::new_from_yahoo("AAPL".to_string())?;
    println!("{:?}'s quotes of the last month: {:?}", quotes.ticker(),quotes.timestamps().len());
    _ = plot(&quotes);
    let strategy = Strategy::apply(quotes.clone())?;
    print_report(quotes,strategy);
    Ok(())
}