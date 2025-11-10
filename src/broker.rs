use crate::broker::Execution::AtOpen;
use crate::config::get_config;
use crate::datas::Data;
use crate::metrics::Metrics;
use crate::strategies::{Strategy, Strategy_arc};
use crate::trades::trade_indices_from_broker;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Execution{
    AtOpen(u32),
    No,
}

impl Execution{
    pub fn to_quotes(&self, data: Data, index:usize)->f64{
        match self {
            AtOpen(_) => data.open[index],
            _ => 0.,
        }
    }
    pub fn to_quotes_arc(&self, data: Arc<Data>, index:usize)->f64{
        match self {
            AtOpen(_) => data.open[index],
            _ => 0.,
        }
    }

}

#[derive(PartialEq, Clone)]
pub enum Status{
    Sent,
    Executed,
    No,
}


impl Execution{
    pub fn to_string(&self)->String{
        match self {
            Execution::AtOpen(u32)=>format!("At Open ({})",u32),
            Execution::No=>"".to_string(),
        }
    }
}
impl Status{
    pub fn to_string(&self)->String{
        match self {
            Status::Sent=>"sent".to_string(),
            Status::Executed=>"executed".to_string(),
            Status::No=>"".to_string(),
        }
    }
}
#[derive(Clone)]
pub struct Broker{
    pub execution: Vec<Execution>,
    pub status: Vec<Status>,
    pub available:Vec<f64>,
    pub position:Vec<i32>,
    pub invested:Vec<f64>,
    pub fees:Vec<f64>,
    pub account:Vec<f64>,
    pub cash:Vec<f64>,
    pub mtm:Vec<f64>,
    pub networth:Vec<f64>,
}

pub fn calculate(strategy:Strategy, quotes:Data, initial_account:f64) ->Broker{
    let cfg = get_config();
    let orders:Vec<Execution> = std::iter::once(Execution::No).chain(
        //strategy.choices.iter().zip(strategy.choices.iter().skip(1)).map(|(prev,curr)| if curr!=prev{Execution::AtOpen(1)} else {Execution::No})).collect();
        strategy.choices.iter().zip(strategy.choices.iter().skip(1)).map(|(prev,curr)| if curr!=prev{cfg.execution_time.clone()} else {Execution::No})).collect();
    let mut carry: Option<u32> = None;
    let mut orders_delayed = Vec::with_capacity(orders.len());
    for val in orders {
        match val {
            AtOpen(n) => {
                carry = Some(n);
                orders_delayed.push(AtOpen(n));
            }
            _ => {
                if let Some(x) = carry {
                    if x >= 1 {
                        carry = Some(x - 1);
                        orders_delayed.push(AtOpen(x - 1));
                    } else {
                        carry = None;
                        orders_delayed.push(Execution::No);
                    }
                } else {
                    orders_delayed.push(Execution::No);
                }
            }
        }
    }
    //calculate order status
    let mut status = Vec::new();
    for val in &orders_delayed {
        match val {
            AtOpen(0) => status.push(Status::Executed),
            AtOpen(_) => status.push(Status::Sent),
            _ => status.push(Status::No),
        }
    }
    //calculate accounts and positions
    let mut accounts = vec![initial_account;status.len()];
    let mut positions = vec![0;status.len()];
    let mut availables = vec![initial_account;status.len()];
    let mut fees = vec![0.;status.len()];
    let mut invested = vec![0.;status.len()];
    let mut mtm = vec![0.;status.len()];
    let mut cash = vec![0.;status.len()];
    let mut networth = vec![0.;status.len()];
    let cfg = get_config();
    for i in 0..status.len(){
        //availables[i..].fill(positions[i] as f64*cfg.execution_time.to_quotes(quotes.clone(),i) + accounts[i]);//
        availables[i..].fill(positions[i] as f64*cfg.execution_time.to_quotes(quotes.clone(),i)+accounts[i]);
        //availables[i..].fill(positions[i] as f64*quotes.open[i] + accounts[i]);
        /*
        if status[i]==Status::Executed{
            positions[i..].fill(((strategy.choices[i].sign() as f64)*availables[i]/quotes.open[i]) as i32);
            accounts[i..].fill(availables[i]-positions[i] as f64*quotes.open[i])
        }
        */
        if status[i]==Status::Executed{
            fees[i] = (positions[i] as f64).abs()*cfg.execution_time.to_quotes(quotes.clone(),i)*cfg.commission_rate;
            positions[i..].fill(((strategy.choices[i].sign() as f64)*(availables[i]-fees[i])/cfg.execution_time.to_quotes(quotes.clone(),i)/(1.+cfg.commission_rate)) as i32);
            invested[i..].fill(positions[i] as f64*cfg.execution_time.to_quotes(quotes.clone(),i));
            fees[i]+=invested[i].abs()*cfg.commission_rate;
            accounts[i..].fill(availables[i]-invested[i]-fees[i]);
            cash[i..].fill(availables[i]-invested[i].abs()-fees[i]);
            //accounts[i..].fill(availables[i]-positions[i] as f64*cfg.execution_time.to_quotes(quotes.clone(),i))
        }
        mtm[i]=positions[i] as f64 * cfg.execution_time.to_quotes(quotes.clone(),i) - invested[i];//todo! mtm is now calculated on execution_time: do on close?
        networth[i] = positions[i] as f64 * quotes.close[i] + accounts[i];
    }
    Broker{execution:orders_delayed,status, available:availables, position:positions,
        invested:invested, fees:fees, account:accounts, cash:cash,mtm:mtm, networth:networth}
}

impl Broker{
    pub fn print_stats(&self){
        println!("\nBroker stats");
        println!("Return = {:.2}%", (self.available.last().unwrap()/self.available.first().unwrap()).ln()*100.); //todo: now calculated on open not close
        let exposure_time = self.position.iter().filter(|&&i|i!=0).count() as f64/self.position.len() as f64;
        println!("Exposure time = {:.2}%",exposure_time*100.);

        //sharpe ratio
        let returns:Vec<f64> = self.available.windows(2).map(|w|(w[1]/w[0]).ln()).collect();
        let rf = 0.00;
        let excess: Vec<f64> = returns.iter().map(|r| r - rf).collect();
        let mean = excess.iter().sum::<f64>() / excess.len() as f64;
        let std = (excess.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (excess.len() as f64 - 1.0)).sqrt();
        let sharpe = mean / std;
        println!("Sharpe rate = {:.2}",sharpe*252f64.sqrt());

        //max drawdown calculations
        let mut peak = self.available.first().unwrap();
        let max_drawdown = self.available
            .iter()
            .map(|v| {
                if v > peak {
                    peak = v;
                }
                (peak - v) / peak
            })
            .fold(0.0, |max_dd, dd| dd.max(max_dd));
        println!("Max drawdown = {:.2}%",max_drawdown*100.);
    }
    pub fn calculate_metrics(&self, metrics: &mut Metrics){
        metrics.bt_return = Some((self.networth.last().unwrap()/self.networth.first().unwrap()).ln()*100.);
        let exposure_time = self.position.iter().filter(|&&i|i!=0).count() as f64/self.position.len() as f64;//todo!calculate on indices not on days
        metrics.exposure_time = Some(exposure_time);
        let mut peak = self.networth.first().unwrap();
        let max_drawdown = self.networth.iter().map(|v| {
                if v > peak { peak = v; }(peak - v) / peak }).fold(0.0, |max_dd, dd| dd.max(max_dd));
        metrics.max_drawd = Some(max_drawdown);
        //sharpe ratio
        let returns:Vec<f64> = self.networth.windows(2).map(|w|(w[1]/w[0]).ln()).collect();
        let rf = 0.00;
        let excess: Vec<f64> = returns.iter().map(|r| r - rf).collect();
        let mean = excess.iter().sum::<f64>() / excess.len() as f64;
        let std = (excess.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (excess.len() as f64 - 1.0)).sqrt();
        let sharpe = mean / std;
        metrics.sharpe = Some(sharpe);
    }
    pub fn trade_indices(&self, metrics: &mut Metrics){
        metrics.trades_indices = Some(trade_indices_from_broker(self));
    }
}

pub fn calculate_arc(strategy:&Strategy_arc, initial_account:f64) ->Broker{
    let cfg = get_config();
    let orders:Vec<Execution> = std::iter::once(Execution::No).chain(
        //strategy.choices.iter().zip(strategy.choices.iter().skip(1)).map(|(prev,curr)| if curr!=prev{Execution::AtOpen(1)} else {Execution::No})).collect();
        strategy.choices.iter().zip(strategy.choices.iter().skip(1)).map(|(prev,curr)| if curr!=prev{cfg.execution_time.clone()} else {Execution::No})).collect();
    let mut carry: Option<u32> = None;
    let mut orders_delayed = Vec::with_capacity(orders.len());
    for val in orders {
        match val {
            AtOpen(n) => {
                carry = Some(n);
                orders_delayed.push(AtOpen(n));
            }
            _ => {
                if let Some(x) = carry {
                    if x >= 1 {
                        carry = Some(x - 1);
                        orders_delayed.push(AtOpen(x - 1));
                    } else {
                        carry = None;
                        orders_delayed.push(Execution::No);
                    }
                } else {
                    orders_delayed.push(Execution::No);
                }
            }
        }
    }
    //calculate order status
    let mut status = Vec::new();
    for val in &orders_delayed {
        match val {
            AtOpen(0) => status.push(Status::Executed),
            AtOpen(_) => status.push(Status::Sent),
            _ => status.push(Status::No),
        }
    }
    //calculate accounts and positions
    let mut accounts = vec![initial_account;status.len()];
    let mut positions = vec![0;status.len()];
    let mut availables = vec![initial_account;status.len()];
    let mut fees = vec![0.;status.len()];
    let mut invested = vec![0.;status.len()];
    let mut mtm = vec![0.;status.len()];
    let mut cash = vec![0.;status.len()];
    let mut networth = vec![0.;status.len()];
    let cfg = get_config();
    for i in 0..status.len(){
        //availables[i..].fill(positions[i] as f64*cfg.execution_time.to_quotes(quotes.clone(),i) + accounts[i]);//
        availables[i..].fill(positions[i] as f64*cfg.execution_time.to_quotes_arc(strategy.data.clone(),i)+accounts[i]);
        //availables[i..].fill(positions[i] as f64*quotes.open[i] + accounts[i]);
        /*
        if status[i]==Status::Executed{
            positions[i..].fill(((strategy.choices[i].sign() as f64)*availables[i]/quotes.open[i]) as i32);
            accounts[i..].fill(availables[i]-positions[i] as f64*quotes.open[i])
        }
        */
        if status[i]==Status::Executed{
            fees[i] = (positions[i] as f64).abs()*cfg.execution_time.to_quotes_arc(strategy.data.clone(),i)*cfg.commission_rate;
            positions[i..].fill(((strategy.choices[i].sign() as f64)*(availables[i]-fees[i])/cfg.execution_time.to_quotes_arc(strategy.data.clone(),i)/(1.+cfg.commission_rate)) as i32);
            invested[i..].fill(positions[i] as f64*cfg.execution_time.to_quotes_arc(strategy.data.clone(),i));
            fees[i]+=invested[i].abs()*cfg.commission_rate;
            accounts[i..].fill(availables[i]-invested[i]-fees[i]);
            cash[i..].fill(availables[i]-invested[i].abs()-fees[i]);
            //accounts[i..].fill(availables[i]-positions[i] as f64*cfg.execution_time.to_quotes(quotes.clone(),i))
        }
        mtm[i]=positions[i] as f64 * cfg.execution_time.to_quotes_arc(strategy.data.clone(),i) - invested[i];//todo! mtm is now calculated on execution_time: do on close?
        networth[i] = positions[i] as f64 * strategy.data.close[i] + accounts[i];
    }
    Broker{execution:orders_delayed,status, available:availables, position:positions,
        invested:invested, fees:fees, account:accounts, cash:cash,mtm:mtm, networth:networth}
}
