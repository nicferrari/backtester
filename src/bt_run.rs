use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub backtests: Vec<BacktestConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BacktestConfig {
    pub symbol: String,
    pub interval: String,
    pub range: String,
    pub initial_capital: f64,
    pub commission: f64,
    pub strategy: StrategyConfig,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum StrategyConfig {
    MovingAverage {
        period: u32,
    },
    Rsi {
        period: u32,
        overbought: f64,
        oversold: f64,
    },
}

use std::fs;
use crate::backtester::Backtest;
use crate::data::Data;
use crate::metrics::report_vertical;
use crate::strategies::{buy_n_hold, rsi_strategy, sma_cross, sma_strategy};


pub fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path).expect("Failed to read config file");
    toml::from_str(&content).expect("Failed to parse TOML")
}

pub fn run(btrun: Config)->Result<(),Box<dyn std::error::Error>>{
    let mut results : Vec<Backtest> = vec![];
    for bt in &btrun.backtests{
        println!("Running {:}",bt.symbol);
        let quotes = Data::new_from_yahoo(&bt.symbol, &bt.interval, &bt.range)?;
        let sma = sma_strategy(quotes.clone(), 10);
        let sma_bt = Backtest::new(sma.clone(), bt.initial_capital);
        results.push(sma_bt);
    }
    let refs : Vec<&Backtest> = results.iter().collect();
    report_vertical(&refs);
    Ok(())
}