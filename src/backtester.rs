use std::error::Error;
use std::time::Instant;
use crate::broker;
use crate::broker::Broker;
use crate::config::{Config, CONFIG};
use crate::metrics::Metrics;
use crate::strategies::{Strategy};
use crate::trades::trade_indices_from_broker;
use crate::utilities::{SerializeAsCsv, write_combined_csv};
pub struct Backtest {
    pub strategy: Strategy,
    pub(crate) broker: Broker,
    pub metrics: Metrics,
    pub local_config:Option<Config>,
}

impl Backtest {
    ///Create a Backtest and calculates, including Metrics
    pub fn new(strategy: Strategy, initial_account: f64) -> Self {
        let local_config = {
            //let guard = CONFIG.read().unwrap().clone().unwrap_or_else(Config::default);
            let guard = CONFIG.read().unwrap().clone().unwrap_or_default();
            guard.clone()
        };
        let start = Instant::now();
        let broker = broker::calculate(&strategy, initial_account);
        let duration = start.elapsed();
        println!("\x1b[34mBacktesting for {} calculated in {:?}\x1b[0m", strategy.name, duration);
        let mut metrics = Metrics::default();
        broker.calculate_metrics(&mut metrics);
        let trades = trade_indices_from_broker(&broker);
        trades.calculate_metrics(&mut metrics, strategy.clone());
        //adds TradesIndices to Metrics
        broker.trade_indices(&mut metrics);
        let duration = start.elapsed();
        println!("\x1b[34mBacktesting and metrics for { } calculated in {:?}\x1b[0m", strategy.name, duration);
        Backtest {
            strategy,
            broker,
            metrics,
            local_config: Some(local_config),
        }
    }
    ///save Backtest to csv
    pub fn to_csv(&self, filepath:&str) ->Result<(),Box<dyn Error>>{
        let datasets: Vec<&dyn SerializeAsCsv> = vec![&self.strategy.data, &self.strategy, &self.broker];
        write_combined_csv(filepath, &datasets[..])?;
        Ok(())
    }
    ///print configuration used in a Backtest (at the time of initialization)
    pub fn print_config(&self, index:usize){
        print!("\x1b[34m{}){} - config:",index,self.strategy.name);
        print!(" commission rate {:.2}%,",self.local_config.clone().unwrap().commission_rate*100.);
        println!(" execution time = {:?}\x1b[0m",self.local_config.clone().unwrap().execution_time);
    }
    ///outputs the list of all trades in a Backtest
    pub fn print_all_trades(&self){
        self.metrics.trades_indices.clone().unwrap().print_all_trades(self.strategy.clone(), self.local_config.clone().unwrap());
    }
    ///outputs details of a single trade (identified by index in the trade list)
    pub fn print_single_trade(&self, position: usize){
        //todo! check position (-1: starting from 1 instead of 0: change?)
        self.metrics.trades_indices.clone().unwrap().indices[position-1].print(self.strategy.clone(), self.local_config.clone().unwrap());
    }
}
