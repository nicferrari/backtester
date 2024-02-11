use chrono::{DateTime, FixedOffset};
use crate::datas::Data;
use crate::strategies::Strategy;

pub fn print_report(quotes:Data, strategy: Strategy){
    for i in 1..quotes.timestamps().len(){
        println!("date = {} - close = {:.2} - strategy was = {:?}",quotes.timestamps()[i].format("%Y-%m-%d"),quotes.close()[i],strategy.choices()[i]);
    }
}