use std::error::Error;
use crate::broker;
use crate::broker::Broker;
use crate::metrics::Metrics;
use crate::strategies::Strategy;
use crate::datas::Data;
use crate::trades::trade_indices_from_broker;
use crate::utilities::{SerializeAsCsv, write_combined_csv};

pub struct Backtest{
    quotes: Data,
    strategy: Strategy,
    broker: Broker,
    pub metrics: Metrics,
}

impl Backtest{
    pub fn new(data: Data, strategy: Strategy, initial_account:f64)->Self{
        let broker = broker::calculate(strategy.clone(), data.clone(), initial_account);
        let mut metrics=Metrics::default();
        broker.calculate_metrics(&mut metrics);
        let trades = trade_indices_from_broker(broker.clone());
        trades.calculate_metrics(&mut metrics, data.clone(), strategy.clone());
        Backtest{
            quotes:data,
            strategy:strategy,
            broker:broker,
            metrics:metrics,
        }
    }
    pub fn save(&self, filepath:&str)->Result<(),Box<dyn Error>>{
        let datasets: Vec<&dyn SerializeAsCsv> = vec![&self.quotes, &self.strategy, &self.broker];
        write_combined_csv(filepath, &datasets[..])?;
        Ok(())
    }
}