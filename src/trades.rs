use std::cmp::min;
use chrono::{DateTime, Duration, FixedOffset};
use crate::backtester::Backtest;
use crate::broker::{Broker, Status};
use crate::datas::Data;
use crate::metrics::Metrics;
use crate::orders::Order;
use crate::strategies::Strategy;

pub struct TradeList{
    trades:Vec<Trade>,
}
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Trade{
    open_date:DateTime<FixedOffset>,
    close_date:DateTime<FixedOffset>,
    order: Order,
    open_price:f64,
    close_price:f64,
    pl:f64,
    pl_max:f64,
    pl_min:f64,
}

///Produce the list of trades executed by the strategy
pub fn trade_list(backtest: Backtest)->TradeList{
    let mut trades:Vec<Trade> = Vec::new();
    for i in 1..backtest.quotes().close.len()-1{
        if backtest.strategy().choices[i]!=backtest.strategy().choices[i-1] && backtest.strategy().choices[i]!=Order::NULL {
            for j in i+1..backtest.quotes().close.len()-1{
                if backtest.strategy().choices[j]!=backtest.strategy().choices[j-1]{
                    let pl = if backtest.strategy().choices[i]==Order::BUY {backtest.quotes().open[j+1]/backtest.quotes().open[i+1]}
                        else{backtest.quotes().open[i+1]/backtest.quotes().open[j+1]};
                    let newtrade = Trade{
                        open_date: backtest.quotes().datetime[i+1],
                        close_date: backtest.quotes().datetime[j+1],
                        order: backtest.strategy().choices[i],
                        open_price: backtest.quotes().open[i+1],
                        close_price: backtest.quotes().open[j+1],
                        pl:(pl-1.0)*100.,
                        //pl_net:(pl*(1.-backtest.commission_rate())/(1.+backtest.commission_rate())-1.0)*100.,
                        pl_max:0.,
                        pl_min:0.,
                    };
                    trades.push(newtrade);
                    break;
                }
                //automatically close last trade
                if j==backtest.strategy().choices.len()-2 && backtest.strategy().choices[i]!=Order::NULL{
                    let pl = if backtest.strategy().choices[i]==Order::BUY {backtest.quotes().open[backtest.quotes().close.len()-1]/backtest.quotes().open[i+1]}
                        else{backtest.quotes().open[i+1]/backtest.quotes().open[backtest.quotes().close.len()-1]};
                    let newtrade = Trade{
                        open_date: backtest.quotes().datetime[i+1],
                        close_date:backtest.quotes().datetime[backtest.quotes().close.len()-1],
                        order: backtest.strategy().choices[i],
                        open_price: backtest.quotes().open[i+1],
                        close_price: backtest.quotes().open[backtest.quotes().close.len()-1],
                        pl:(pl-1.0)*100.,
                        //pl_net:(pl*(1.-backtest.commission_rate())/(1.+backtest.commission_rate())-1.0)*100.,
                        pl_max:0.,
                        pl_min:0.,
                    };
                    trades.push(newtrade);
                };
            }
        };
    }
    //adding case of trade opened on last date (+ automatic close)
    if backtest.strategy().choices[backtest.strategy().choices.len()-2]!=backtest.strategy().choices[backtest.strategy().choices.len()-3]&&backtest.strategy().choices[backtest.strategy().choices.len()-2]!=Order::NULL{
        let pl = if backtest.strategy().choices[backtest.strategy().choices.len()-2]==Order::BUY {backtest.quotes().close[backtest.quotes().close.len()-1]/backtest.quotes().open[backtest.strategy().choices.len()-1]}
            else{backtest.quotes().close[backtest.strategy().choices.len()-1]/backtest.quotes().open[backtest.quotes().close.len()-1]};
        let newtrade = Trade{
            open_date: backtest.quotes().datetime[backtest.strategy().choices.len()-1],
            close_date: backtest.quotes().datetime[backtest.strategy().choices.len()-1],
            order: backtest.strategy().choices[backtest.strategy().choices.len()-2],
            open_price:backtest.quotes().open[backtest.strategy().choices.len()-1],
            close_price:backtest.quotes().close[backtest.strategy().choices.len()-1],
            pl:(pl-1.0)*100.,
            //pl_net:(pl*(1.-backtest.commission_rate())/(1.+backtest.commission_rate())-1.0)*100.,
            pl_max:0.,
            pl_min:0.,
        };
        trades.push(newtrade);
    }
    for i in 0..trades.len(){
        println!("Trade {:} opened at {:?} closed at {:?}, order = {:?} executed at {:.2} and closed at {:.2}, p&l = {:.2}%"
                 ,i+1,trades[i].open_date.date_naive(),trades[i].close_date.date_naive(), trades[i].order, trades[i].open_price, trades[i].close_price,
        trades[i].pl);
    }
    TradeList{trades}
}

pub fn trade_list_from_broker (broker: Broker, quotes: Data, strategy: Strategy)->TradeList{
    let indices:Vec<usize> = broker.status.iter().enumerate().filter_map(|(i,v)| if *v == Status::Executed {Some(i)} else {None}).collect();
    let mut pairs:Vec<(usize,usize)> = indices.windows(2).map(|w|(w[0],w[1])).collect();
    //force close last trade if open
    if let Some(&last_match) = indices.last(){pairs.push((last_match,broker.status.len()-1))};

    let trades:Vec<Trade> = pairs.iter().map(|&(i,j)|Trade{
        open_date:quotes.datetime[i],
        close_date:quotes.datetime[j],
        order:strategy.choices[i-1],// todo:adapt to slippage
        open_price:quotes.open[i],
        close_price:quotes.open[j],
        pl:strategy.choices[i-1].sign() as f64 *(quotes.open[j]/quotes.open[i]-1.)*100.,
        //pl:strategy.choices[i-1].sign() as f64 * (quotes.open[j]/quotes.open[i]).ln()*100.,
        pl_max:((quotes.high[i..=j].iter().copied().reduce(f64::max).unwrap())/quotes.open[i]-1.)*100.*strategy.choices[i-1].sign() as f64,
        pl_min:((quotes.low[i..=j].iter().copied().reduce(f64::min).unwrap())/quotes.open[i]-1.)*100.*strategy.choices[i-1].sign() as f64,
    }).collect();
    TradeList{trades}
}
#[deprecated(note="old, to be replaced")]
pub fn report_trade(trades_list: TradeList){
    for i in 0..trades_list.trades.len(){
        println!("Trade {:} opened at {:?} closed at {:?} ({:?} days), order = {:?} executed at {:.2} and closed at {:.2}, p&l = {:.2}%  max p&l = {:.2}% min p&l = {:.2}%"
                 ,i+1,trades_list.trades[i].open_date.date_naive(),trades_list.trades[i].close_date.date_naive(), (trades_list.trades[i].close_date-trades_list.trades[i].open_date).num_days(),trades_list.trades[i].order, trades_list.trades[i].open_price, trades_list.trades[i].close_price,
                 trades_list.trades[i].pl, trades_list.trades[i].pl_max.max(trades_list.trades[i].pl_min),trades_list.trades[i].pl_max.min(trades_list.trades[i].pl_min));
    }
    let nr_trades = trades_list.trades.len();
    let win_rate = (trades_list.trades.iter().filter(|&i|i.pl>0.).count() as f64)/(nr_trades as f64)*100.;
    let best_trade = trades_list.trades.iter().max_by(|a,b|a.pl.partial_cmp(&b.pl).unwrap());
    let worst_trade = trades_list.trades.iter().min_by(|a,b|a.pl.partial_cmp(&b.pl).unwrap());
    let average_pl = trades_list.trades.iter().map(|a|a.pl).sum::<f64>()/trades_list.trades.len() as f64;
    let tot_duration = trades_list.trades.iter().map(|a|a.close_date-a.open_date).fold(Duration::zero(),|acc, d|acc+d);
    let average_duration = (tot_duration/(trades_list.trades.len() as i32)).num_seconds() as f64/86_400.;
    println!("Trades # = {:}",nr_trades);
    println!("Win rate = {:.2}%",win_rate);
    println!("Best trade = {:.2}%",best_trade.unwrap().pl);
    println!("Worst trade = {:.2}%",worst_trade.unwrap().pl);
    println!("Average profit/loss = {:.2}%",average_pl);
    println!("Average trade duration = {:.5} days",format!("{average_duration}"));
}

pub struct TradeIndices{
    position:usize,
    open_index:usize,
    close_index: usize,
}
pub struct TradesIndices{
    pub indices:Vec<TradeIndices>,
}
pub fn trade_indices_from_broker(broker: Broker)->TradesIndices{
    let indices:Vec<usize> = broker.status.iter().enumerate().filter_map(|(i,v)| if *v == Status::Executed {Some(i)} else {None}).collect();
    let mut pairs:Vec<(usize,usize)> = indices.windows(2).map(|w|(w[0],w[1])).collect();
    //force close last trade if open
    if let Some(&last_match) = indices.last(){pairs.push((last_match,broker.status.len()-1))};
    let indices = pairs.into_iter().enumerate().map(|(i,(a,b))|TradeIndices{position:i,open_index:a,close_index:b}).collect();
    TradesIndices{indices}
}

impl TradeIndices{
    pub fn print(&self, data:Data, strategy: Strategy){
        print!("Trade {} - {:?}/{:?}",self.position+1,data.datetime[self.open_index].date_naive(),data.datetime[self.close_index].date_naive());
        print!(" ({} days)",self.calc_duration(data.clone()));
        print!(", {:?}",strategy.choices[self.open_index-1]);//todo! should be based on broker not on strategy and correct timing
        print!(" {:.2}/{:.2}",data.open[self.open_index],data.open[self.close_index]);
        print!(", p&l {:.2}%",self.calc_pl(data.clone(),strategy.clone()));
        let max_pl = ((data.high[self.open_index..=self.close_index].iter().copied().reduce(f64::max).unwrap())/data.open[self.open_index]-1.)*100.*strategy.choices[self.open_index-1].sign() as f64;
        let min_pl = ((data.low[self.open_index..=self.close_index].iter().copied().reduce(f64::min).unwrap())/data.open[self.open_index]-1.)*100.*strategy.choices[self.open_index-1].sign() as f64;
        print!(" (max {:.2}% min {:.2}%)",max_pl.max(min_pl),max_pl.min(min_pl));
        print!("\n");
    }
    fn calc_pl(&self, data: Data, strategy: Strategy) -> f64 {
        //strategy.choices[self.open_index-1].sign() as f64 *(data.open[self.close_index]/data.open[self.open_index]-1.)*100.
        strategy.choices[self.open_index-1].sign() as f64 *(data.open[self.close_index]/data.open[self.open_index]).ln()*100.
    }
}

impl TradesIndices{
    pub fn print(&self, data: Data, strategy: Strategy){
        println!("\nTrade stats");
        println!("Trades  = {}",self.indices.len());
        let max_pl = self.indices.iter().map(|t|t.calc_pl(data.clone(), strategy.clone())).reduce(f64::max).unwrap();
        let min_pl = self.indices.iter().map(|t|t.calc_pl(data.clone(), strategy.clone())).reduce(f64::min).unwrap();
        let win_rate = self.indices.iter().map(|t|t.calc_pl(data.clone(), strategy.clone())).filter(|&i|i>0.).count() as f64/self.indices.len() as f64 ;
        let average_pl = self.indices.iter().map(|t|t.calc_pl(data.clone(), strategy.clone())).sum::<f64>()/self.indices.len() as f64 ;
        println!("Win rate = {:.2}%",win_rate*100.);
        println!("Best trade = {:.2}%",max_pl);
        println!("Worst trade = {:.2}%",min_pl);
        println!("Average p&l = {:.2}%",average_pl);
        println!("Average trade duration = {:.2} days",self.calc_duration(data.clone()));
    }
    pub fn print_all_trades(&self, data:Data, strategy: Strategy){
        println!("\nTrade list");
        for i in &self.indices{
            i.print(data.clone(), strategy.clone());
        }
    }
    pub fn calculate_metrics(&self, metrics: &mut Metrics, data: Data, strategy: Strategy){
        //let mut metrics = Metrics::default();
        metrics.strategy_name = Some(strategy.clone().name);
        metrics.trades_nr = Some(self.indices.len());
        let max_pl = self.indices.iter().map(|t |t.calc_pl(data.clone(), strategy.clone())).reduce(f64::max).unwrap();
        let min_pl = self.indices.iter().map(|t|t.calc_pl(data.clone(), strategy.clone())).reduce(f64::min).unwrap();
        let win_rate = self.indices.iter().map(|t|t.calc_pl(data.clone(), strategy.clone())).filter(|&i|i>0.).count() as f64/self.indices.len() as f64 ;
        let average_pl = self.indices.iter().map(|t|t.calc_pl(data.clone(), strategy.clone())).sum::<f64>()/self.indices.len() as f64;
        let average_duration = self.calc_duration(data.clone());
        metrics.max_pl = Some(max_pl);
        metrics.min_pl = Some(min_pl);
        metrics.win_rate = Some(win_rate);
        metrics.average_pl = Some(average_pl);
        metrics.avg_duration = Some(average_duration);
    }
}

pub trait TradeStats {
    fn calc_duration(&self, data: Data)->f64;
}

impl TradeStats for TradeIndices{
    fn calc_duration(&self, data: Data) -> f64{
        (data.datetime[self.close_index]-data.datetime[self.open_index]).num_days() as f64
    }
}
impl TradeStats for TradesIndices{
    fn calc_duration(&self, data: Data) -> f64 {
        self.indices.iter().map(|i|i.calc_duration(data.clone())).sum::<f64>()/self.indices.len() as f64
    }
}