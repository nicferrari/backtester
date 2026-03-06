use crate::backtester::Backtest;
use crate::data::Data;
use crate::metrics::report_vertical;
use crate::strategies::{buy_n_hold, rsi_strategy, sma_cross, sma_strategy};
use serde::Deserialize;
use std::fs;
use crate::config::{get_config, update_config};

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

    #[serde(rename = "strategy")]
    pub strategies: Vec<StrategyConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum StrategyConfig {
    Sma {
        period: usize,
    },
    Rsi {
        period: usize,
    },
    SmaCross {
        short_period: usize,
        long_period: usize,
    },
}

pub fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path).expect("Failed to read config file");
    toml::from_str(&content).expect("Failed to parse TOML")
}

pub fn run(btrun: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut results: Vec<Backtest> = vec![];

    for bt in &btrun.backtests {
        println!("Running {}", bt.symbol);

        let quotes = Data::new_from_yahoo(&bt.symbol, &bt.interval, &bt.range)?;

        let cfg = get_config();
        update_config(|cfg|{cfg.commission_rate = bt.commission});

        for strategy in &bt.strategies {
            match strategy {
                StrategyConfig::Sma { period } => {
                    let sma = sma_strategy(quotes.clone(), *period);
                    let sma_bt = Backtest::new(sma.clone(), bt.initial_capital);
                    results.push(sma_bt);
                }

                StrategyConfig::Rsi { period } => {
                    let rsi = rsi_strategy(quotes.clone(), *period);
                    let rsi_bt = Backtest::new(rsi.clone(), bt.initial_capital);
                    results.push(rsi_bt);
                }
                StrategyConfig::SmaCross {
                    short_period,
                    long_period,
                } => {
                    let sma_cross = sma_cross(quotes.clone(), *short_period, *long_period);
                    let sma_cross_bt = Backtest::new(sma_cross.clone(), bt.initial_capital);
                    results.push(sma_cross_bt);
                }
            }
        }
    }

    let refs: Vec<&Backtest> = results.iter().collect();
    report_vertical(&refs);

    Ok(())
}
