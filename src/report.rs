use crate::backtester::Backtest;
use crate::orders::Order::{BUY, SHORTSELL, NULL};

pub trait BacktestNr {
    fn uniquereport(&self);
}

impl BacktestNr for Backtest{
    fn uniquereport(&self) {
        vec![self.clone()].uniquereport();
    }
}

impl BacktestNr for Vec<Backtest>{
    fn uniquereport(&self) {
        // TODO: metrics should be adjusted for commissions (now they're not considered)
        print!("{}",format!("{:<width$}","Strategies",width=20));
        print!("{}",format!("{:>width$}","Return",width=20));
        print!("{}",format!("{:>width$}","Exposure Time %",width=20));
        print!("{}",format!("{:>width$}","Trades #",width=20));
        print!("{}",format!("{:>width$}","Win Rate [%]",width=20));
        print!("{}",format!("{:>width$}","Best Trade [%]",width=20));
        println!("{}",format!("{:>width$}","Worst Trade [%]",width=20));
        for i in self.iter(){
            let equity_final = i.position().last().unwrap()*i.quotes().close().last().unwrap()+i.account().last().unwrap();
            let ret = (equity_final-100000.)/100000.;
            let null_count = i.position().iter().filter(|&&num|num==0.).count();
            let mut trade_count=0;
            if i.strategy().choices[0]!=NULL {trade_count=1};
            let mut profit=0f64;
            let mut max_profit = 0f64;
            let mut max_loss = 0f64;
            let mut starting_value=i.quotes().open()[1];
            let mut n_win_trades = 0;
            for j in 1..i.strategy().choices().len()-1{
                if i.strategy().choices()[j]!=i.strategy().choices()[j-1]{
                    if trade_count!=0{
                        if i.strategy().choices()[j-1]==BUY{profit = i.quotes().open()[j+1]*(1.-i.commission_rate())/starting_value-1.}
                            else{profit = starting_value/i.quotes().open()[j+1]*(1.+i.commission_rate())-1.}
                    };
                    trade_count = trade_count+1;
                    if i.strategy().choices()[j]==BUY{starting_value = i.quotes().open()[j+1]*(1.+i.commission_rate())}
                        else{starting_value = i.quotes().open()[j+1]*(1.-i.commission_rate())};
                    max_profit = f64::max(max_profit, profit);
                    max_loss = f64::min(max_loss, profit);
                    if profit>0.{n_win_trades = n_win_trades+1};
                }
            }
            //implement force-close of an existing position
            if *i.strategy().choices().iter().nth_back(1).unwrap()==BUY{profit = *i.quotes().close().last().unwrap()*(1.-i.commission_rate())/starting_value-1.}
            else if *i.strategy().choices().iter().nth_back(1).unwrap()==SHORTSELL{profit = starting_value/i.quotes().close().last().unwrap()*(1.+i.commission_rate())-1.}
            max_profit = f64::max(max_profit, profit);
            max_loss = f64::min(max_loss, profit);
            if profit>0.{n_win_trades = n_win_trades+1};

            //if last period has a choice but second-to-last hasn't, trade has still to happen
            if (*i.strategy().choices().iter().nth_back(1).unwrap()==NULL) & (*i.strategy().choices().iter().last().unwrap()!=NULL) {trade_count=trade_count-1;}

            print!("{}",format!("{:<width$}", i.strategy().name(), width = 20));
            print!("{}",format!("{:>width$}",format!("{:.2}%",ret*100.),width=20));
            print!("{}",format!("{:>width$}",format!("{:.2}%",100.-(null_count as f64)/(i.quotes().timestamps().len() as f64)*100.),width=20));
            print!("{}",format!("{:>width$}", trade_count, width = 20));
            print!("{}",format!("{:>width$}",format!("{:.2}%",n_win_trades as f64/trade_count as f64 *100.),width=20));
            print!("{}",format!("{:>width$}",format!("{:.2}%",max_profit*100.),width=20));
            println!("{}",format!("{:>width$}",format!("{:.2}%",max_loss*100.),width=20));
        }
    }
}

pub fn report<T: BacktestNr>(items: T){
    items.uniquereport();
}