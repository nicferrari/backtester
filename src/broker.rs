use crate::broker::Execution::AtOpen;
use crate::datas::Data;
use crate::metrics::Metrics;
use crate::strategies::Strategy;
#[derive(Clone)]
pub enum Execution{
    AtOpen(u32),
    No,
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
    pub account:Vec<f64>,
    pub position:Vec<i32>,
}

pub fn calculate(strategy:Strategy, quotes:Data, initial_account:f64) ->Broker{
    let orders:Vec<Execution> = std::iter::once(Execution::No).chain(
        strategy.choices.iter().zip(strategy.choices.iter().skip(1)).map(|(prev,curr)| if curr!=prev{Execution::AtOpen(1)} else {Execution::No})).collect();
    let mut carry: Option<u32> = None;
    let mut orders_delayed = Vec::with_capacity(orders.len());
    for val in orders {
        match val {
            AtOpen(n) => {
                carry = Some(n);
                orders_delayed.push(AtOpen(n));
            }
            NA => {
                if let Some(x) = carry {
                    if x >= 1 {
                        carry = Some(x - 1);
                        orders_delayed.push(AtOpen(x - 1));
                    } else {
                        carry = None;
                        orders_delayed.push(NA);
                    }
                } else {
                    orders_delayed.push(NA);
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
            NA => status.push(Status::No),
        }
    }
    //calculate accounts and positions
    let mut accounts = vec![initial_account;status.len()];
    let mut positions = vec![0;status.len()];
    let mut availables = vec![initial_account;status.len()];
    for i in 0..status.len(){
        availables[i..].fill(positions[i] as f64*quotes.open[i] + accounts[i]);
        if status[i]==Status::Executed{
            positions[i..].fill(((strategy.choices[i].sign() as f64)*availables[i]/quotes.open[i]) as i32);
            accounts[i..].fill(availables[i]-positions[i] as f64*quotes.open[i])
        }
    }
    Broker{execution:orders_delayed,status, available:availables, account:accounts, position:positions}
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
        metrics.bt_return = Some((self.available.last().unwrap()/self.available.first().unwrap()).ln()*100.);
        let exposure_time = self.position.iter().filter(|&&i|i!=0).count() as f64/self.position.len() as f64;
        metrics.exposure_time = Some(exposure_time);
        let mut peak = self.available.first().unwrap();
        let max_drawdown = self.available.iter().map(|v| {
                if v > peak { peak = v; }(peak - v) / peak }).fold(0.0, |max_dd, dd| dd.max(max_dd));
        metrics.max_drawd = Some(max_drawdown);
        //sharpe ratio
        let returns:Vec<f64> = self.available.windows(2).map(|w|(w[1]/w[0]).ln()).collect();
        let rf = 0.00;
        let excess: Vec<f64> = returns.iter().map(|r| r - rf).collect();
        let mean = excess.iter().sum::<f64>() / excess.len() as f64;
        let std = (excess.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (excess.len() as f64 - 1.0)).sqrt();
        let sharpe = mean / std;
        metrics.sharpe = Some(sharpe);
    }
}