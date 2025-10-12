use std::mem::take;
use chrono::{DateTime, FixedOffset};
use crate::backtester::Backtest;
use crate::broker::{Broker, Status};
use crate::datas::Data;
use crate::orders::Order;
use crate::strategies::Strategy;

pub struct TradeList{
    trades:Vec<Trade>,
}
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Trade{
    open_date:DateTime<FixedOffset>,
    close_date:DateTime<FixedOffset>,
    order: Order,
    open_price:f64,
    close_price:f64,
    pl:f64,
    pl_net:f64,
}

///Produce the list of trades executed by the strategy
pub fn trade_list(backtest: Backtest)->TradeList{
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
                        pl_net:(pl*(1.-backtest.commission_rate())/(1.+backtest.commission_rate())-1.0)*100.,
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
                        pl_net:(pl*(1.-backtest.commission_rate())/(1.+backtest.commission_rate())-1.0)*100.,
                    };
                    trades.push(newtrade);
                };
            }
        };
    }
    //adding case of trade opened on last date (+ automatic close)
    if backtest.strategy().choices[backtest.strategy().choices.len()-2]!=backtest.strategy().choices[backtest.strategy().choices.len()-3]&&backtest.strategy().choices[backtest.strategy().choices.len()-2]!=Order::NULL{
        let pl = if backtest.strategy().choices[backtest.strategy().choices.len()-2]==Order::BUY {backtest.quotes().close[backtest.quotes().close.len()-1]/backtest.quotes().open[backtest.strategy().choices.len()-1]}
            else{backtest.quotes().close[backtest.strategy().choices.len()-1]/backtest.quotes().open[backtest.quotes().close.len()-1]};
        let newtrade = Trade{
            open_date: backtest.quotes().datetime[backtest.strategy().choices.len()-1],
            close_date: backtest.quotes().datetime[backtest.strategy().choices.len()-1],
            order: backtest.strategy().choices[backtest.strategy().choices.len()-2],
            open_price:backtest.quotes().open[backtest.strategy().choices.len()-1],
            close_price:backtest.quotes().close[backtest.strategy().choices.len()-1],
            pl:(pl-1.0)*100.,
            pl_net:(pl*(1.-backtest.commission_rate())/(1.+backtest.commission_rate())-1.0)*100.,
        };
        trades.push(newtrade);
    }
    for i in 0..trades.len(){
        println!("Trade {:} opened at {:?} closed at {:?}, order = {:?} executed at {:.2} and closed at {:.2}, p&l = {:.2}%  net = {:.2}%"
                 ,i+1,trades[i].open_date.date_naive(),trades[i].close_date.date_naive(), trades[i].order, trades[i].open_price, trades[i].close_price,
        trades[i].pl, trades[i].pl_net);
    }
    TradeList{trades}
}

pub fn trade_list_from_broker (broker: Broker, quotes: Data, strategy: Strategy)->TradeList{
    //let mut trades:Vec<Trade> = Vec::new();
    let indices:Vec<usize> = broker.status.iter().enumerate().filter_map(|(i,v)| if *v == Status::Executed {Some(i)} else {None}).collect();
    let mut pairs:Vec<(usize,usize)> = indices.windows(2).map(|w|(w[0],w[1])).collect();
    //force close last trade if open
    if let Some(&last_match) = indices.last(){pairs.push((last_match,broker.status.len()-1))};
    let trades:Vec<Trade> = pairs.iter().map(|&(i,j)|Trade{
        open_date:quotes.datetime[i],
        close_date:quotes.datetime[j],
        order:strategy.choices[i-1],// todo:adapt to slippage
        open_price:quotes.open[i],
        close_price:quotes.open[j],
        pl:strategy.choices[i-1].sign() as f64 *(quotes.open[j]/quotes.open[i]-1.)*100.,
        pl_net:0.,//todo: remove (not necessary)
    }).collect();
    TradeList{trades}
}

pub fn report_trade(trades_list: TradeList){
    for i in 0..trades_list.trades.len(){
        println!("Trade {:} opened at {:?} closed at {:?}, order = {:?} executed at {:.2} and closed at {:.2}, p&l = {:.2}%  net = {:.2}%"
                 ,i+1,trades_list.trades[i].open_date.date_naive(),trades_list.trades[i].close_date.date_naive(), trades_list.trades[i].order, trades_list.trades[i].open_price, trades_list.trades[i].close_price,
                 trades_list.trades[i].pl, trades_list.trades[i].pl_net);
    }
    let nr_trades = trades_list.trades.len();
    let win_rate = (trades_list.trades.iter().filter(|&i|i.pl>0.).count() as f64)/(nr_trades as f64)*100.;
    let best_trade = trades_list.trades.iter().max_by(|a,b|a.pl.partial_cmp(&b.pl).unwrap());
    let worst_trade = trades_list.trades.iter().min_by(|a,b|a.pl.partial_cmp(&b.pl).unwrap());
    println!("Trades # = {:}",nr_trades);
    println!("Win rate = {:.2}%",win_rate);
    println!("Best trade = {:.2}%",best_trade.unwrap().pl);
    println!("Worst trade = {:.2}%",worst_trade.unwrap().pl);
}