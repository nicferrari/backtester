use std::error::Error;
use std::time::Instant;
use crate::broker;
use crate::broker::Broker;
use crate::config::{Config, CONFIG};
use crate::metrics::Metrics;
use crate::strategies::Strategy;
use crate::datas::Data;
use crate::trades::trade_indices_from_broker;
use crate::utilities::{SerializeAsCsv, write_combined_csv};
//use std::sync::Arc;

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
        let trades = trade_indices_from_broker(broker.clone());
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