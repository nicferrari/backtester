use csv::Writer;
use crate::datas::Data;
use crate::orders::Order;
use crate::orders::Order::{BUY,SHORTSELL,NULL};
use std::error::Error;
use crate::ta::{Indicator,sma};

#[derive(Clone)]
pub struct Strategy{
    //name:String,
    name:&'static str,
    choices:Vec<Order>,
}

impl Strategy{
    pub fn choices(&self)->Vec<Order>{
        return self.choices.clone();
    }
    pub fn name(&self)->&'static str{ return self.name;}
    pub fn revert(&self)->Self{
        let length = self.choices.len();
        let mut rev_choices = self.choices.clone();
        for i in 0..length{
            if self.choices[i]==BUY {rev_choices[i]=SHORTSELL} else if self.choices[i]==SHORTSELL {rev_choices[i]=BUY}
        }
        Strategy{
            name:"revert",
            choices:rev_choices,
        }
    }
    pub fn to_csv(&self)->Result<(),Box<dyn Error>>{
        let mut wrt = Writer::from_path("strategies.csv")?;
        wrt.write_record(self.choices().iter().map(|e|e.to_string()))?;
        wrt.flush()?;
        Ok(())
    }
}

/// specific strategies should only implement function to define choices: but why not returning directly Strategy?
pub fn buy_and_hold(quotes:Data) ->Vec<Order>{
    let length = quotes.timestamps().len();
    return vec![BUY;length];
}

pub fn buy_n_hold(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![BUY;length];
    let name = "buy and hold";
    Strategy{
        name:name,
        choices:choices,
    }
}
pub fn short_n_hold(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![SHORTSELL;length];
    let name = "short and hold";
    Strategy{
        name:name,
        choices:choices,
    }
}
pub fn do_nothing(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![NULL;length];
    let name = "do nothing";
    Strategy{
        name:name,
        choices:choices,
    }
}
pub fn sma_cross(quotes:Data, period:usize)->Strategy{
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
    Strategy{
        name:name,
        choices:choices,
    }
}