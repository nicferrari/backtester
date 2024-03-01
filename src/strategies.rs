use csv::Writer;
use crate::datas::Data;
use crate::orders::Order;
use crate::orders::Order::{BUY,SHORTSELL,NULL};
use std::error::Error;
use yahoo_finance_api::Quote;
use crate::ta::{Indicator,sma};
///struct to hold vector of choices and indicators.
/// there is no specific constructor.
/// need to be created via dedicated user-defined functions which return a Strategy
#[derive(Clone)]
pub struct Strategy{
    //name:String,
    name:&'static str,
    choices:Vec<Order>,
    indicator:Option<Vec<Vec<f64>>>,
}

impl Strategy{
    pub fn choices(&self)->Vec<Order>{
        return self.choices.clone();
    }
    pub fn name(&self)->&'static str{ return self.name;}
    pub fn indicator(&self)->Option<Vec<Vec<f64>>>{ return self.indicator.clone();}
    pub fn invert(&self) ->Self{
        let length = self.choices.len();
        let mut inv_choices = self.choices.clone();
        for i in 0..length{
            if self.choices[i]==BUY { inv_choices[i]=SHORTSELL} else if self.choices[i]==SHORTSELL { inv_choices[i]=BUY}
        }
        let indicator = self.indicator.clone();
        Strategy{
            name:"invert",
            choices: inv_choices,
            indicator,
        }
    }
    pub fn to_csv(&self)->Result<(),Box<dyn Error>>{
        let mut wrt = Writer::from_path("strategies.csv")?;
        wrt.write_record(self.choices().iter().map(|e|e.to_string()))?;
        wrt.flush()?;
        Ok(())
    }
}

// specific strategies should only implement function to define choices: but why not returning directly Strategy?
pub fn buy_and_hold(quotes:Data) ->Vec<Order>{
    let length = quotes.timestamps().len();
    return vec![BUY;length];
}

pub fn buy_n_hold(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![BUY;length];
    let name = "buy and hold";
    let indicator = Some(vec![vec![-1.;length]]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}
pub fn short_n_hold(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![SHORTSELL;length];
    let name = "short and hold";
    let indicator = Some(vec![vec![-1.;length]]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}
pub fn do_nothing(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![NULL;length];
    let name = "do nothing";
    let indicator = Some(vec![vec![-1.;length]]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}
pub fn simple_sma(quotes:Data, period:usize) ->Strategy{
    let sma = sma(&quotes,period);
    let indicator = Indicator{indicator:sma,quotes:quotes};
    let length = indicator.quotes.timestamps().len();
    let mut choices = vec![NULL;length];
    for i in 1..length{
        if indicator.indicator[i]!=-1.{
            if indicator.indicator[i]>=indicator.quotes.open()[i]{
                choices[i] = BUY;
            }else if indicator.indicator[i]<indicator.quotes.open()[i]{
               choices[i] = SHORTSELL}
        }
    }
    let name = "sma cross";
    let indicator = Some(vec![indicator.indicator()]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}
pub fn sma_cross(quotes:Data, short_period:usize, long_period:usize)->Strategy{
    let sma_short = sma(&quotes, short_period);
    let sma_long = sma(&quotes, long_period);
    todo!()
}