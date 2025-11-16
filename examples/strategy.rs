use std::error::Error;
use std::sync::Arc;
use rs_backtester::data::Data;
use rs_backtester::strategies::{Strategy};
use rs_backtester::orders::Order::{BUY,SHORTSELL,NULL};
extern crate rand;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rs_backtester::backtester::Backtest;
use rs_backtester::metrics::report_horizontal;


pub fn main() -> Result<(),Box<dyn Error>>{
    //example to show how to build a custom strategy
    let quotes = Data::new_from_yahoo("PLTR", "1d", "6mo")?;

    pub fn random_strategy(quotes:Arc<Data>)-> Strategy {
        let length = quotes.datetime.len();
        let mut choices = vec![NULL;length];
        let name = "random strategy".to_string();
        let indicator = Some(vec![vec![-1.;length]]);
        let rnd_orders= vec![BUY,SHORTSELL,NULL];
        for i in 0..length{
            let mut rng = thread_rng();
            choices[i] = *rnd_orders.choose(&mut rng).unwrap();
        }
        Strategy {
            name,
            choices,
            indicator,
            data:quotes.clone(),
        }
    }

    let rnd_strategy = random_strategy(quotes.clone());
    let random_backtester = Backtest::new(rnd_strategy, 1e5);
    report_horizontal(&[&random_backtester]);
    Ok(())
}