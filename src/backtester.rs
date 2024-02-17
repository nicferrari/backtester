use yahoo_finance_api::Quote;
use crate::strategies::Strategy;
use crate::{orders, Result};
use crate::datas::Data;

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
    pub fn print_report(&self){
        for i in 1..self.quotes.timestamps().len(){
            println!("date = {} - close = {:.2} - choice was = {:?} under strategy = {:} with a position = {:} worth = {:.2} and account {:.2}\
             for a net worth = {:.2}",
                     self.quotes.timestamps()[i].format("%Y-%m-%d"),self.quotes.close()[i],
                     self.strategy.choices()[i],self.strategy.name(),self.position[i],self.position[i]*self.quotes.close()[i],self.account[i],
            self.account[i]+self.position[i]*self.quotes.close()[i]);
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
                        self.account[i] = self.account[i]-self.position[i]*self.quotes.close()[i];
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
                        self.account[i] = self.account[i]-self.position[i]*self.quotes.close()[i];
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