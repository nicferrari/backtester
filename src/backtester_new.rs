use std::error::Error;
use std::time::Instant;
use crate::broker;
use crate::broker::Broker;
use crate::config::{Config, CONFIG};
use crate::metrics::Metrics;
use crate::strategies::{Strategy_arc};
use crate::trades::trade_indices_from_broker;
use crate::utilities::{SerializeAsCsv, write_combined_csv};
/*
pub struct Backtest{
    pub(super) quotes: Data,
    strategy: Strategy,
    broker: Broker,
    pub metrics: Metrics,
    pub local_config:Option<Config>,
}
///Create a Backtest and calculates, including Metrics
impl Backtest{
    pub fn new(data: Data, strategy: Strategy, initial_account:f64)->Self{
        let local_config = {let guard = CONFIG.read().unwrap().clone().unwrap_or_else(Config::default); guard.clone()};
        let start = Instant::now();
        let broker = broker::calculate(strategy.clone(), data.clone(), initial_account);
        let duration = start.elapsed();
        println!("\x1b[34mBacktesting for {} calculated in {:?}\x1b[0m", strategy.name,duration);
        let mut metrics=Metrics::default();
        broker.calculate_metrics(&mut metrics);
        let trades = trade_indices_from_broker(&broker);
        trades.calculate_metrics(&mut metrics, data.clone(), strategy.clone());
        broker.clone().trade_indices(&mut metrics);
        let duration = start.elapsed();
        println!("\x1b[34mBacktesting and metrics for { } calculated in {:?}\x1b[0m", strategy.name,duration);
        Backtest{
            quotes:data,
            strategy:strategy,
            broker:broker,
            metrics:metrics,
            local_config:Some(local_config),
        }
    }
    ///Save Backtest to CSV
    /// Example:
    /// backtest.to_csv("backtest.csv")?;
    pub fn to_csv(&self, filepath:&str) ->Result<(),Box<dyn Error>>{
        let datasets: Vec<&dyn SerializeAsCsv> = vec![&self.quotes, &self.strategy, &self.broker];
        write_combined_csv(filepath, &datasets[..])?;
        Ok(())
    }
    ///Show trade list for a Backtest
    pub fn trade_list(&self){
        self.metrics.trades_indices.clone().unwrap().print_all_trades(self.quotes.clone(),self.strategy.clone(),self.local_config.clone().unwrap());
    }
    pub fn print_config(&self, index:usize){
        print!("\x1b[34m{}){} - config:",index,self.strategy.name);
        print!(" commission rate {:.2}%,",self.local_config.clone().unwrap().commission_rate*100.);
        println!(" execution time = {:?}\x1b[0m",self.local_config.clone().unwrap().execution_time);
    }
}
*/
pub struct Backtest_arc{
    pub strategy: Strategy_arc,
    pub(crate) broker: Broker,
    pub metrics: Metrics,
    pub local_config:Option<Config>,
}
///Create a Backtest and calculates, including Metrics
impl crate::backtester_new::Backtest_arc {
    pub fn new(strategy: Strategy_arc, initial_account: f64) -> Self {
        let local_config = {
            //let guard = CONFIG.read().unwrap().clone().unwrap_or_else(Config::default);
            let guard = CONFIG.read().unwrap().clone().unwrap_or_default();
            guard.clone()
        };
        let start = Instant::now();
        let broker = broker::calculate_arc(&strategy, initial_account);
        let duration = start.elapsed();
        println!("\x1b[34mBacktesting for {} calculated in {:?}\x1b[0m", strategy.name, duration);
        let mut metrics = Metrics::default();
        broker.calculate_metrics(&mut metrics);
        let trades = trade_indices_from_broker(&broker);
        trades.calculate_metrics_arc(&mut metrics,  strategy.clone());
        //adds TradesIndices to Metrics
        broker.trade_indices(&mut metrics);
        let duration = start.elapsed();
        println!("\x1b[34mBacktesting and metrics for { } calculated in {:?}\x1b[0m", strategy.name, duration);
        crate::backtester_new::Backtest_arc {
            strategy,
            broker,
            metrics,
            local_config: Some(local_config),
        }
    }
    pub fn to_csv_arc(&self, filepath:&str) ->Result<(),Box<dyn Error>>{
        let datasets: Vec<&dyn SerializeAsCsv> = vec![&self.strategy.data, &self.strategy, &self.broker];
        write_combined_csv(filepath, &datasets[..])?;
        Ok(())
    }
    pub fn print_config(&self, index:usize){
        print!("\x1b[34m{}){} - config:",index,self.strategy.name);
        print!(" commission rate {:.2}%,",self.local_config.clone().unwrap().commission_rate*100.);
        println!(" execution time = {:?}\x1b[0m",self.local_config.clone().unwrap().execution_time);
    }
    pub fn print_all_trades(&self){
        self.metrics.trades_indices.clone().unwrap().print_all_trades_arc(self.strategy.clone(),self.local_config.clone().unwrap());
    }
    pub fn print_single_trade(&self, position: usize){
        //todo! check position (-1: starting from 1 instead of 0: change?)
        self.metrics.trades_indices.clone().unwrap().indices[position-1].print_arc(self.strategy.clone(),self.local_config.clone().unwrap());
    }
}
