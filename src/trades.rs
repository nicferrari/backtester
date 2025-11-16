use crate::broker::{Broker, Status};
use crate::config::{get_config, Config};
use crate::data::Data;
use crate::metrics::Metrics;
use crate::strategies::Strategy;
use std::sync::Arc;
///Trade: a single trade with indices of moment of open, close and position (= index of trade within the Backtest list of trades)
#[derive(Clone)]
pub struct Trade {
    pub(crate) position: usize,
    pub(crate) open_index: usize,
    pub(crate) close_index: usize,
}
///A vector of all Trades in a Backtest
#[derive(Clone)]
pub struct TradeList {
    pub indices: Vec<Trade>,
}
///produces TradeList in a Backtest
pub fn trade_indices_from_broker(broker: &Broker) -> TradeList {
    let indices: Vec<usize> = broker
        .status
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if *v == Status::Executed {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let mut pairs: Vec<(usize, usize)> = indices.windows(2).map(|w| (w[0], w[1])).collect();
    //force close last trade if open
    if let Some(&last_match) = indices.last() {
        pairs.push((last_match, broker.status.len() - 1))
    };
    let indices = pairs
        .into_iter()
        .enumerate()
        .map(|(i, (a, b))| Trade {
            position: i,
            open_index: a,
            close_index: b,
        })
        .collect();
    TradeList { indices }
}

impl Trade {
    ///print single trade
    pub fn print(&self, strategy: Strategy, cfg: Config) {
        print!(
            "Trade {} - {:?}/{:?}",
            self.position + 1,
            strategy.data.datetime[self.open_index].date_naive(),
            strategy.data.datetime[self.close_index].date_naive()
        );
        print!(" ({} days)", self.calc_duration(strategy.data.clone()));
        print!(", {:?}", strategy.choices[self.open_index - 1]); //todo! should be based on broker not on strategy and correct timing
        print!(
            " {:.2}/{:.2}",
            strategy.data.open[self.open_index], strategy.data.open[self.close_index]
        );
        print!(", p&l {:.2}%", self.calc_pl(strategy.clone(), cfg.clone()));
        let max_pl = strategy.choices[self.open_index - 1].sign() as f64
            * ((strategy.data.high[self.open_index..=self.close_index]
                .iter()
                .copied()
                .reduce(f64::max)
                .unwrap())
                / strategy.data.open[self.open_index]
                / (1. + cfg.commission_rate)
                    .powi(2 * strategy.choices[self.open_index - 1].sign() as i32))
            .ln()
            * 100.;
        let min_pl = strategy.choices[self.open_index - 1].sign() as f64
            * ((strategy.data.low[self.open_index..=self.close_index]
                .iter()
                .copied()
                .reduce(f64::min)
                .unwrap())
                / strategy.data.open[self.open_index]
                / (1. + cfg.commission_rate)
                    .powi(2 * strategy.choices[self.open_index - 1].sign() as i32))
            .ln()
            * 100.;
        print!(
            " (max {:.2}% min {:.2}%)",
            max_pl.max(min_pl),
            max_pl.min(min_pl)
        );
        println!();
    }
    ///calc p&l of a trade
    fn calc_pl(&self, strategy: Strategy, cfg: Config) -> f64 {
        strategy.choices[self.open_index - 1].sign() as f64
            * (cfg
                .execution_time
                .to_quotes(strategy.data.clone(), self.close_index)
                / cfg
                    .execution_time
                    .to_quotes(strategy.data.clone(), self.open_index)
                / (1. + cfg.commission_rate)
                    .powi(2 * strategy.choices[self.open_index - 1].sign() as i32))
            .ln()
            * 100.
    }
}

impl TradeList {
    ///print all Trades in a Strategy
    pub fn print_all_trades(&self, strategy: Strategy, cfg: Config) {
        println!("\nTrade list");
        for i in &self.indices {
            i.print(strategy.clone(), cfg.clone());
        }
    }
    ///adds to Metrics the ones from TradeList
    pub fn calculate_metrics(&self, metrics: &mut Metrics, strategy: Strategy) {
        let cfg = get_config();
        metrics.ticker = Some(strategy.data.ticker.to_string());
        metrics.strategy_name = Some(strategy.clone().name);
        metrics.trades_nr = Some(self.indices.len());
        let max_pl = self
            .indices
            .iter()
            .map(|t| t.calc_pl(strategy.clone(), cfg.clone()))
            .reduce(f64::max)
            .unwrap();
        let min_pl = self
            .indices
            .iter()
            .map(|t| t.calc_pl(strategy.clone(), cfg.clone()))
            .reduce(f64::min)
            .unwrap();
        let win_rate = self
            .indices
            .iter()
            .map(|t| t.calc_pl(strategy.clone(), cfg.clone()))
            .filter(|&i| i > 0.)
            .count() as f64
            / self.indices.len() as f64;
        let average_pl = self
            .indices
            .iter()
            .map(|t| t.calc_pl(strategy.clone(), cfg.clone()))
            .sum::<f64>()
            / self.indices.len() as f64;
        let average_duration = self.calc_duration(strategy.data.clone());
        metrics.max_pl = Some(max_pl);
        metrics.min_pl = Some(min_pl);
        metrics.win_rate = Some(win_rate);
        metrics.average_pl = Some(average_pl);
        metrics.avg_duration = Some(average_duration);
    }
}

pub trait TradeStats {
    fn calc_duration(&self, data: Arc<Data>) -> f64;
}

impl TradeStats for Trade {
    fn calc_duration(&self, data: Arc<Data>) -> f64 {
        (data.datetime[self.close_index] - data.datetime[self.open_index]).num_days() as f64
    }
}
impl TradeStats for TradeList {
    fn calc_duration(&self, data: Arc<Data>) -> f64 {
        self.indices
            .iter()
            .map(|i| i.calc_duration(data.clone()))
            .sum::<f64>()
            / self.indices.len() as f64
    }
}
