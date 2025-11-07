use std::error::Error;
use std::time::Instant;
use crate::broker;
use crate::broker::Broker;
use crate::metrics::Metrics;
use crate::strategies::Strategy;
use crate::datas::Data;
use crate::trades::trade_indices_from_broker;
use crate::utilities::{SerializeAsCsv, write_combined_csv};
//use std::sync::Arc;

pub struct Backtest{
    quotes: Data,
    strategy: Strategy,
    broker: Broker,
    pub metrics: Metrics,
}
///Create a Backtest and calculates, including Metrics
impl Backtest{
    pub fn new(data: Data, strategy: Strategy, initial_account:f64)->Self{
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
        println!("\x1b[34mBacktesting and metrics for { }calculated in {:?}\x1b[0m", strategy.name,duration);
        Backtest{
            quotes:data,
            strategy:strategy,
            broker:broker,
            metrics:metrics,
        }
    }
    ///Save Backtest to CSV
    pub fn to_csv(&self, filepath:&str) ->Result<(),Box<dyn Error>>{
        let datasets: Vec<&dyn SerializeAsCsv> = vec![&self.quotes, &self.strategy, &self.broker];
        write_combined_csv(filepath, &datasets[..])?;
        Ok(())
    }
    ///Show trade list for a Backtest
    pub fn trade_list(&self){
        self.metrics.trades_indices.clone().unwrap().print_all_trades(self.quotes.clone(),self.strategy.clone());
    }
}