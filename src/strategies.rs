use yahoo_finance_api::Quote;
use crate::datas::Data;
use crate::orders::Order;
use crate::orders::Order::{BUY,SHORTSELL,NULL};
use crate::Result;

#[derive(Clone)]
pub struct Strategy{
    //name:String,
    name:&'static str,
    choices:Vec<Order>,
}

impl Strategy{
    // below not really needed anymore but useful example of passing func as parameter
    fn apply(quotes:Data, func:&dyn Fn(Data)->Vec<Order>)->Result<Self>{
        let length = quotes.timestamps().len();
        Ok(Strategy{
            //name:"Buy_and_Hold".to_string(),
            name:stringify!(func),
            choices:func(quotes),
        })
    }
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