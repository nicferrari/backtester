//use std::cell::RefCell;
//use csv::Writer;
use crate::datas::Data;
use crate::orders::Order;
use crate::orders::Order::{BUY,SHORTSELL,NULL};
//use std::error::Error;
use crate::ta::{sma,rsi, Indicator_arc};
use serde::{Serialize};
//use serde::de::Unexpected::Str;
use std::sync::Arc;
/*
/// Struct to hold vector of choices and indicators<BR>
/// There is no specific constructor<BR>
/// Need to be created via a user-defined function which return a Strategy
#[derive(Clone, Serialize)]
pub struct Strategy{
    pub name:String,
    pub choices:Vec<Order>,
    pub indicator:Option<Vec<Vec<f64>>>,
}*/
/// Struct to hold vector of choices and indicators<BR>
/// There is no specific constructor<BR>
/// Need to be created via a user-defined function which return a Strategy
#[derive(Clone, Serialize)]
pub struct Strategy_arc{
    pub name:String,
    pub choices:Vec<Order>,
    pub indicator:Option<Vec<Vec<f64>>>,
    pub data:Arc<Data>,
}

/*
impl Strategy{
    pub fn choices(&self)->Vec<Order>{
        return self.choices.clone();
    }
    pub fn name(&self)->&String{ return &self.name;}
    pub fn indicator(&self)->Option<Vec<Vec<f64>>>{ return self.indicator.clone();}
    ///invert strategy (LONG->SHORT and viceversa)
    pub fn invert(&self) ->Self{
        let length = self.choices.len();
        let mut inv_choices = self.choices.clone();
        for i in 0..length{
            if self.choices[i]==BUY { inv_choices[i]=SHORTSELL} else if self.choices[i]==SHORTSELL { inv_choices[i]=BUY}
        }
        let indicator = self.indicator.clone();
        Strategy{
            name:self.name.clone()+"_inv",
            choices: inv_choices,
            indicator,
        }
    }
    pub fn long_only(&self) ->Self{
        let length = self.choices.len();
        let mut long_choices = self.choices.clone();
        for i in 0..length{
            if self.choices[i]==SHORTSELL { long_choices[i]=NULL}
        }
        let indicator = self.indicator.clone();
        Strategy{
            name:self.name.clone()+"_long",
            choices: long_choices,
            indicator,
        }
    }
    pub fn short_only(&self) ->Self{
        let length = self.choices.len();
        let mut short_choices = self.choices.clone();
        for i in 0..length{
            if self.choices[i]==BUY { short_choices[i]=NULL}
        }
        let indicator = self.indicator.clone();
        Strategy{
            name:self.name.clone()+"_short",
            choices: short_choices,
            indicator,
        }
    }
    ///skip first signal
    pub fn skipfirst(&self)->Strategy{
        let mut change_count = 0;
        let mut new_choices = self.choices.clone();
        let indicator=self.indicator.clone();
        for i in 1..self.choices.len(){
            if self.choices()[i]!=self.choices()[i-1]{
                change_count += 1;
            }
            if change_count<2{
                new_choices[i]=Order::NULL;
            }
            else { new_choices[i]=self.choices[i] }
        }
        Strategy{
            name:self.name.clone()+"_skip",
            choices:new_choices,
            indicator,
        }
    }
}*/

impl Strategy_arc{
    ///invert strategy (LONG->SHORT and viceversa)
    pub fn invert(&self) ->Self{
        let mut inv_choices = self.choices.clone();
        /*for i in 0..length{
            if self.choices[i]==BUY { inv_choices[i]=SHORTSELL} else if self.choices[i]==SHORTSELL { inv_choices[i]=BUY}
        }*/
        for (choices, inv_choices) in self.choices.iter().zip(inv_choices.iter_mut()){
            if *choices==BUY {*inv_choices=SHORTSELL} else if *choices==SHORTSELL{ *inv_choices=BUY };
        }
        let indicator = self.indicator.clone();
        Strategy_arc{
            name:self.name.clone()+"_inv",
            choices: inv_choices,
            indicator,
            data:self.data.clone(),
        }
    }
    ///transform current strategy in long only
    pub fn long_only(&self) ->Self{
        let mut long_choices = self.choices.clone();
        /*
        for i in 0..length{
            if self.choices[i]==SHORTSELL { long_choices[i]=NULL}
        }*/
        for (choices,long_choices) in self.choices.iter().zip(long_choices.iter_mut()){
            if *choices==SHORTSELL{*long_choices=NULL}
        }
        let indicator = self.indicator.clone();
        Strategy_arc{
            name:self.name.clone()+"_long",
            choices: long_choices,
            indicator,
            data:self.data.clone(),
        }
    }///
    ///transform current strategy in short only
    pub fn short_only(&self) ->Self{
        //let length = self.choices.len();
        let mut short_choices = self.choices.clone();
        /*
        for i in 0..length{
            if self.choices[i]==BUY { short_choices[i]=NULL}
        }*/
        for (choice,short_choices) in self.choices.iter().zip(short_choices.iter_mut()){
            if *choice==BUY{*short_choices=NULL}
        }
        let indicator = self.indicator.clone();
        Strategy_arc{
            name:self.name.clone()+"_short",
            choices: short_choices,
            indicator,
            data:self.data.clone(),
        }
    }
    ///skip first signal
    pub fn skipfirst(&self)->Self{
        let mut change_count = 0;
        let mut new_choices = self.choices.clone();
        let indicator=self.indicator.clone();
        /*
        for i in 1..self.choices.len(){
            if self.choices[i]!=self.choices[i-1]{
                change_count += 1;
            }
            if change_count<2{
                new_choices[i]=Order::NULL;
            }
            else { new_choices[i]=self.choices[i] }
        }*/
        for (i, window) in self.choices.windows(2).enumerate() {
            if window[0] != window[1] {
                change_count += 1;
            }

            new_choices[i + 1] = if change_count < 2 {
                Order::NULL
            } else {
                window[1]
            };
        }

        Strategy_arc{
            name:self.name.clone()+"_skip",
            choices:new_choices,
            indicator,
            data:self.data.clone(),
        }
    }
}
/*
///Returns Buy and Hold Strategy
pub fn buy_n_hold(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![BUY;length];
    let name = "buy_and_hold".to_string();
    let indicator = Some(vec![vec![-1.;length]]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}*/
///Returns Buy and Hold Strategy
/// todo! buy and hold start from 3rd+ period (1 to see the data (order is on change of stance), 1 to send order, 1+ to execute)
pub fn buy_n_hold_arc(quotes:Arc<Data>)->Strategy_arc{
    let length = quotes.datetime.len();
    let mut choices = vec![BUY;length];
    let name = "buy_and_hold".to_string();
    let indicator = Some(vec![vec![-1.;length]]);
    choices[0]=NULL;
    Strategy_arc{
        name,
        choices,
        indicator,
        data:quotes.clone(),
    }
}

///Returns the opposite of a Buy and Hold Strategy:
/// start by shortselling and keep the short position open to the end
pub fn short_n_hold_arc(quotes:Arc<Data>)->Strategy_arc{
    let length = quotes.datetime.len();
    let choices = vec![SHORTSELL;length];
    let name = "short and hold".to_string();
    let indicator = Some(vec![vec![-1.;length]]);
    Strategy_arc{
        name,
        choices,
        indicator,
        data:quotes.clone(),
    }
}
///Returns a Strategy which does exactly nothing (i.e. always stays out of the market)
pub fn do_nothing_arc(quotes:Arc<Data>)->Strategy_arc{
    let length = quotes.datetime.len();
    let choices = vec![NULL;length];
    let name = "do nothing".to_string();
    let indicator = Some(vec![vec![-1.;length]]);
    Strategy_arc{
        name,
        choices,
        indicator,
        data:quotes.clone(),
    }
}
/*
///Returns a Simple Moving Average Strategy with a user specified time-period
pub fn simple_sma(quotes:Data, period:usize) ->Strategy{
    let sma = sma(&quotes,period);
    let indicator = Indicator{indicator:sma,quotes:quotes};
    let length = indicator.quotes.timestamps().len();
    let mut choices = vec![NULL;length];
    for i in 0..length{
        if indicator.indicator[i]!=-1.{
            if indicator.indicator[i]<=indicator.quotes.close()[i]{
                choices[i] = BUY;
            }else if indicator.indicator[i]>indicator.quotes.close()[i]{
               choices[i] = SHORTSELL}
        }
    }
    let name = format!("simple_sma_{}",period);
    let indicator = Some(vec![indicator.indicator()]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}*/
///Returns a Simple Moving Average Strategy with a user specified time-period
pub fn sma_arc(quotes:Arc<Data>, period:usize) ->Strategy_arc{
    let sma = sma(&quotes,period);
    let indicator = Indicator_arc{indicator:sma,quotes:quotes.clone()};
    let length = indicator.quotes.datetime.len();
    let mut choices = vec![NULL;length];
    /*for i in 0..length{
        if indicator.indicator[i]!=-1.{
            if indicator.indicator[i]<=indicator.quotes.close[i]{
                choices[i] = BUY;
            }else if indicator.indicator[i]>indicator.quotes.close[i]{
                choices[i] = SHORTSELL}
        }
    }*/
    for ((ind, close), choice) in indicator.indicator.iter().zip(&indicator.quotes.close).zip(choices.iter_mut())
    {
        if *ind != -1. {
            *choice = if *ind <= *close {BUY} else {SHORTSELL};
        }
    }
    let name = format!("sma_{}",period);
    let indicator = Some(vec![indicator.indicator]);
    Strategy_arc{
        name,
        choices,
        indicator,
        data:quotes.clone(),
    }
}
/*
///Returns a Simple Moving Average Crossing Strategy (i.e. goes long when SMA short crosses SMA long and shortsells otherwise)<BR>
///User can specify both time-periods (short and long, with short first)
pub fn sma_cross(quotes:Data, short_period:usize, long_period:usize)->Strategy{
    if short_period >= long_period {panic!("Error: short SMA parameter should be shorter than long SMA parameter");}
    let sma_short = sma(&quotes, short_period);
    let sma_long = sma(&quotes, long_period);
    let ind_short = Indicator{indicator:sma_short,quotes:quotes.clone()};
    let ind_long = Indicator{indicator:sma_long, quotes:quotes.clone()};
    let length = ind_short.quotes().timestamps().len();
    let mut choices = vec![NULL;length];
    for i in 0..length{
        if ind_long.indicator()[i]!=-1.{
            if ind_short.indicator()[i]>ind_long.indicator()[i]{choices[i]=BUY}
            else {choices[i]=SHORTSELL};
        }
    }
    let name=format!("sma_cross_{}_{}",short_period,long_period);
    let indicator = Some(vec![ind_short.indicator(),ind_long.indicator()]);
    Strategy{
        name:name,
        choices:choices,
        indicator:indicator,
    }
}*/
///Returns a Simple Moving Average Crossing Strategy (i.e. goes long when SMA short crosses SMA long and shortsells otherwise)<BR>
///User can specify both time-periods (short and long, with short first)
pub fn sma_cross_arc(quotes:Arc<Data>, short_period:usize, long_period:usize)->Strategy_arc{
    if short_period >= long_period {panic!("Error: short SMA parameter should be shorter than long SMA parameter");}
    let sma_short = sma(&quotes, short_period);
    let sma_long = sma(&quotes, long_period);
    let ind_short = Indicator_arc{indicator:sma_short,quotes:quotes.clone()};
    let ind_long = Indicator_arc{indicator:sma_long, quotes:quotes.clone()};
    let length = ind_short.quotes.datetime.len();
    let mut choices = vec![NULL;length];
    for ((ind_long,ind_short),choices) in ind_long.indicator.iter().zip(ind_short.indicator.iter()).zip(choices.iter_mut()){
        if *ind_long!=-1.{ *choices = if *ind_short>*ind_long {BUY} else { SHORTSELL } }
    }
    /*
    for i in 0..length{
        if ind_long.indicator[i]!=-1.{
            if ind_short.indicator[i]>ind_long.indicator[i]{choices[i]=BUY}
            else {choices[i]=SHORTSELL};
        }
    }*/
    let name=format!("sma_cross_{}_{}",short_period,long_period);
    let indicator = Some(vec![ind_short.indicator,ind_long.indicator]);
    Strategy_arc{
        name,
        choices,
        indicator,
        data:quotes,
    }
}
/*
///Returns a Relative Strength Index Strategy (i.e. goes short if RSI > 70, long when RSI < 30, and stay out of market elsewhere)
pub fn rsi_strategy(quotes:Data, period:usize)->Strategy{
    let rsi = rsi(&quotes,period);
    let indicator = Indicator{indicator:rsi,quotes};
    let length = indicator.quotes().timestamps().len();
    let mut choices = vec![NULL;length];
    for i in 0..length{
        if indicator.indicator()[i]!=-1.{
            if indicator.indicator()[i]>70.{choices[i]=SHORTSELL}
            else if indicator.indicator()[i]<30. {choices[i]=BUY}
        }
    }
    let name = format!("rsi_{}",period);
    let indicator=Some(vec![indicator.indicator()]);
    Strategy{
        name,
        choices,
        indicator,
    }
}*/
///Returns a Relative Strength Index Strategy (i.e. goes short if RSI > 70, long when RSI < 30, and stay out of market elsewhere)
pub fn rsi_strategy_arc(quotes:Arc<Data>, period:usize)->Strategy_arc{
    let rsi = rsi(&quotes,period);
    let indicator = Indicator_arc{indicator:rsi,quotes:quotes.clone()};
    let length = indicator.quotes.datetime.len();
    let mut choices = vec![NULL;length];
    /*for i in 0..length{
        if indicator.indicator[i]!=-1.{
            if indicator.indicator[i]>70.{choices[i]=SHORTSELL}
            else if indicator.indicator[i]<30. {choices[i]=BUY}
        }
    }*/
    for (ind,choice) in indicator.indicator.iter().zip(choices.iter_mut()){
        if *ind !=-1.{if *ind > 70.{*choice = SHORTSELL} else if *ind < 30. {*choice = BUY}}
    }
    let name = format!("rsi_{}",period);
    let indicator=Some(vec![indicator.indicator]);
    Strategy_arc{
        name,
        choices,
        indicator,
        data:quotes.clone(),
    }
}