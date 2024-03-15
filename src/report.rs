use std::error::Error;
//use serde::ser::Error;
use crate::backtester::Backtest;
use crate::orders::Order;

pub fn report(backtest: &Backtest){
    let start = backtest.quotes().timestamps()[0];
    println!("Start   {:?}",start);
    let end = *backtest.quotes().timestamps().last().expect("bad read");
    println!("End   {:?}",end);
    let duration = end-start;
    println!("Duration  {:?} days",duration.num_days());
    let null_count = backtest.strategy().choices().iter().filter(|&&num|num==Order::NULL).count();
    println!("Exposure Time %   {:.2}%",100.-(null_count as f64)/(backtest.quotes().timestamps().len() as f64)*100.);
    let equity_final = backtest.position().last().unwrap()*backtest.quotes().close().last().unwrap()+backtest.account().last().unwrap();
    println!("Equity Final [$]  {:.2}",equity_final);
    let equity_peak = backtest.position().iter().zip(backtest.quotes().close().iter()).zip(backtest.account().iter()).map(|((&vi,&wi),&zi)|vi*wi+zi).fold(
        f64::NEG_INFINITY,f64::max);
    println!("Equity Peak [$]   {:.2}",equity_peak);
    let ret = (equity_final-100000.)/100000.;
    println!("Return    {:.2}%",ret*100.);
    println!("Return (Ann.) [%]     {:.2}",(equity_final/100000.).powf(1./((duration.num_days() as f64)/365.))*100.-100.);
    println!("Buy & Hold Return     {:.2}%",(backtest.quotes().close().last().unwrap()/backtest.quotes().close()[0]-1.)*100.);
    let mut trade_count=0;
    let mut profit=0f64;//to be fixed for only loss or only gain
    let mut max_profit = 0f64;
    let mut max_loss = 0f64;
    let mut starting_value=1.;
    let mut n_win_trades = 0;
    for i in 1..backtest.strategy().choices().len(){
        if backtest.strategy().choices()[i]!=backtest.strategy().choices()[i-1]{
            if trade_count!=0{profit = backtest.quotes().open()[i]/starting_value-1.};
            trade_count = trade_count+1;
            starting_value = backtest.quotes().open()[i];
            max_profit = f64::max(max_profit, profit);
            max_loss = f64::min(max_loss, profit);
            if profit>0.{n_win_trades = n_win_trades+1};
        }
    }
    println!("# Trades  {:}",trade_count);
    println!("Win Rate [%]  {:.2}",n_win_trades as f64/trade_count as f64 *100.);
    println!("Best Trade [%]    {:.2}",max_profit*100.);
    println!("Worst Trade [%]    {:.2}",max_loss*100.);
//    Ok(())
}
pub fn compare(backtests:Vec<Backtest>){
    //println!("Strategies    {:} vs {:}",backtests[0].strategy().name(),backtests[1].strategy().name());
    print!("Strategies    ");
    for i in 0..backtests.len(){
        print!("{:}     ",backtests[i].strategy().name());
    }
}