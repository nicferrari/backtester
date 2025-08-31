use chrono::{DateTime, FixedOffset};
use crate::backtester::Backtest;
use crate::orders::Order;

pub struct TradeList{
    trades:Vec<Trade>,
}
#[derive(Debug)]
pub struct Trade{
    open_date:DateTime<FixedOffset>,
    close_date:DateTime<FixedOffset>,
    order: Order,
    open_price:f64,
    close_price:f64,
    pl:f64,
}

///Produce a list of trades executed by the strategy
pub fn trade_list(backtest: Backtest){
    let mut trades:Vec<Trade> = Vec::new();
    for i in 1..backtest.quotes().close.len()-1{
        if backtest.strategy().choices[i]!=backtest.strategy().choices[i-1] && backtest.strategy().choices[i]!=Order::NULL {
            for j in i+1..backtest.quotes().close.len()-1{
                if backtest.strategy().choices[j]!=backtest.strategy().choices[j-1]{
                    let pl = if backtest.strategy().choices[i]==Order::BUY {backtest.quotes().open[j+1]/backtest.quotes().open[i+1]}
                        else{backtest.quotes().open[i+1]/backtest.quotes().open[j+1]};
                    let newtrade = Trade{
                        open_date: backtest.quotes().datetime[i+1],
                        close_date: backtest.quotes().datetime[j+1],
                        order: backtest.strategy().choices[i],
                        open_price: backtest.quotes().open[i+1],
                        close_price: backtest.quotes().open[j+1],
                        pl:(pl-1.0)*100.,
                    };
                    trades.push(newtrade);
                    break;
                }
                //automatically close last trade
                if j==backtest.strategy().choices.len()-2 && backtest.strategy().choices[i]!=Order::NULL{
                    let pl = if backtest.strategy().choices[i]==Order::BUY {backtest.quotes().open[backtest.quotes().close.len()-1]/backtest.quotes().open[i+1]}
                    else{backtest.quotes().open[i+1]/backtest.quotes().open[backtest.quotes().close.len()-1]};
                    let newtrade = Trade{
                        open_date: backtest.quotes().datetime[i+1],
                        close_date:backtest.quotes().datetime[backtest.quotes().close.len()-1],
                        order: backtest.strategy().choices[i],
                        open_price: backtest.quotes().open[i+1],
                        close_price: backtest.quotes().open[backtest.quotes().close.len()-1],
                        pl:(pl-1.0)*100.,
                    };
                    trades.push(newtrade);
                };
            }
        };
    }
    for i in 0..trades.len(){
        println!("Trade {:} opened at {:?} closed at {:?}, order = {:?} executed at {:.2} and closed at {:.2}, p&l = {:.2}%"
                 ,i+1,trades[i].open_date.date_naive(),trades[i].close_date.date_naive(), trades[i].order, trades[i].open_price, trades[i].close_price,
        trades[i].pl);
    }
}