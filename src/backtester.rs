use std::collections::HashMap;
use crate::strategies::Strategy;
use crate::{orders, Result};
use crate::datas::Data;
use crate::orders::Order;

pub struct Backtest{
    quotes:Data,
    strategy:Strategy,
    position:Vec<f64>,
    account:Vec<f64>,
}

#[derive(PartialEq)]
enum Stance{
    LONG,
    NULL,
    SHORT,
}

impl Backtest{
    pub fn new(quotes:Data,strategy: Strategy,account:f64)->Result<Self>{
        let length = quotes.timestamps().len();
        let position = vec![0.;length];
        let account = vec![account;length];
        Ok(Backtest{
            quotes:quotes,
            strategy:strategy,
            position:position,
            account:account,
        })
    }

    pub fn quotes(&self)->&Data{return &self.quotes}
    pub fn orders(&self)->Vec<Order>{return self.strategy.choices()}
    pub fn position(&self)->Vec<f64>{return self.position.clone()}
    pub fn account(&self)->Vec<f64>{return self.account.clone();}
    pub fn strategy(&self)->Strategy{return self.strategy.clone();}
    pub fn print_report_arg(&self, list:&[&str]){
        let mut data_functions: HashMap<&str, fn(&Data)->Vec<f64>>=HashMap::new();
        data_functions.insert("close", Data::close);
        data_functions.insert("open", Data::open);
        let mut backtest_functions: HashMap<&str, fn(&Backtest)->Vec<f64>>=HashMap::new();
        backtest_functions.insert("position",Backtest::position);
        backtest_functions.insert("account",Backtest::account);
        for i in 1..self.quotes.timestamps().len(){
            print!("Date = {:} - ",&self.quotes.timestamps()[i].format("%Y-%m-%d"));
            for j in list{
                if let Some(func) = data_functions.get(j){
                    let value = func(&self.quotes)[i];
                    print!("{} = {:.2}  ",j,value)
                };
                if let Some(func) = backtest_functions.get(j){
                    let value = func(&self)[i];
                    print!("{} = {:.2}  ",j,value)
                };
            }
            print!("   - net worth = {:.2}",self.quotes.close()[i]*self.position()[i]+self.account()[i]);
            println!();
        }
    }
    pub fn calculate(&mut self){
        let mut stance = Stance::NULL;
        let mut previous_position = 0.;
        let mut previous_account = 100000.;
        for i in 1..self.quotes.timestamps().len(){
            match self.strategy.choices()[i]{
                orders::Order::BUY=>{
                    if stance!=Stance::LONG{
                        let networth = previous_account + previous_position * self.quotes.close()[i];//to be changed to open
                        //self.position[i] = ((self.account[i]/self.quotes.close()[i]) as i64) as f64;
                        self.position[i] = ((networth/self.quotes.close()[i]) as i64) as f64;
                        self.account[i] = networth-self.position[i]*self.quotes.close()[i];
                        stance = Stance::LONG;
                    } else {
                        self.position[i] = previous_position;
                        self.account[i] = previous_account;
                    }
                }
                orders::Order::SHORTSELL=>{
                    if stance!=Stance::SHORT{
                        let networth = previous_account + previous_position * self.quotes.close()[i];//to be changed to open
                        //self.position[i] = -((self.account[i]/self.quotes.close()[i]) as i64) as f64;
                        self.position[i] = -((networth/self.quotes.close()[i]) as i64) as f64;
                        self.account[i] = networth-self.position[i]*self.quotes.close()[i];
                        stance = Stance::SHORT;
                    } else {
                        self.position[i] = previous_position;
                        self.account[i] = previous_account;
                    }
                }
                orders::Order::NULL=>{
                    if stance!=Stance::NULL{
                        let networth = previous_account + previous_position * self.quotes.close()[i];//to be changed to open
                        self.position[i]=0.;
                        self.account[i]=networth;
                        stance = Stance::NULL;
                    } else {
                        self.position[i] = previous_position;
                        self.account[i] = previous_account;

                    }
                }
            }
            previous_account = self.account[i];
            previous_position = self.position[i];
        }
    }
}