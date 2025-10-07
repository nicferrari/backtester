use std::fmt::format;
use std::ptr::slice_from_raw_parts;
use crate::broker::Execution::AtOpen;
use crate::datas::Data;
use crate::orders::Order;
use crate::strategies::Strategy;

pub enum Execution {
    AtOpen(u32),
    No,
}
#[derive(PartialEq)]
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

pub struct Broker{
    pub execution: Vec<Execution>,
    pub status: Vec<Status>,
    pub available:Vec<f64>,
    pub account:Vec<f64>,
    pub position:Vec<i32>,
}

pub fn calculate(strategy:Strategy, quotes:Data) ->Broker{
    let mut orders:Vec<Execution> = std::iter::once(Execution::No).chain(
        strategy.choices.iter().zip(strategy.choices.iter().skip(1)).map(|(prev,curr)| if curr!=prev{Execution::AtOpen(2)} else {Execution::No})).collect();
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
    let mut accounts = vec![100000.;status.len()];
    let mut positions = vec![0;status.len()];
    let mut availables = vec![100000.;status.len()];
    for i in 0..status.len(){
        availables[i..].fill(positions[i] as f64*quotes.open[i] + accounts[i]);
        if status[i]==Status::Executed{
            positions[i..].fill(((strategy.choices[i].sign() as f64)*availables[i]/quotes.open[i]) as i32);
            accounts[i..].fill(availables[i]-positions[i] as f64*quotes.open[i])
        }
    }
    Broker{execution:orders_delayed,status, available:availables, account:accounts, position:positions}
}