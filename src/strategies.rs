use yahoo_finance_api::Quote;
use crate::datas::Data;
use crate::orders::Order;
use crate::orders::Order::BUY;
use crate::Result;


pub struct Strategy{
    name:String,
    choices:Vec<Order>,
}

impl Strategy{
    pub fn apply(quotes:Data)->Result<Self>{
        let length = quotes.timestamps().len();
        Ok(Strategy{
            name:"Buy_and_Hold".to_string(),
            choices:vec!(BUY;length),
        })
    }
    pub fn choices(&self)->Vec<Order>{
        return self.choices.clone();
    }
}