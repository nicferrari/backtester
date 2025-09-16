use std::error::Error;
use rs_backtester::datas::{Data};
use rs_backtester::stateful::{sma, sma_cross, State};

fn main() ->Result<(),Box<dyn Error>> {
    let quotes = Data::new_from_yahoo("PLTR","1d","6mo")?;
    let quotes2 = quotes.slice(19);
    println!("{:?}",sma(quotes2.clone(),10).unwrap_or(-1.0));
    println!("{:?}",sma(quotes2.clone(),20).unwrap_or(-1.0));
    println!("{:?}",sma_cross(quotes2,10,20));
    let state = State{account:100000.,position:0};
    Ok(())
}