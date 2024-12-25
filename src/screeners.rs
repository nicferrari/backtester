use datas::Data;
use crate::datas;

pub struct ScreenerReturns {
    tickers:Vec<String>,
    terms:Vec<String>,
    quotes:Vec<Data>,
}

impl ScreenerReturns {
    pub fn new(tickers:&[&str], terms:&[&str])->Self{
        let mut quotes = Vec::new();
        for i in tickers{
            let quote = Data::new_from_yahoo(*i,"1d","1y").unwrap();
            quotes.push(quote);
        }
        let tickers = tickers.iter().map(|s|s.to_string()).collect();
        let terms = terms.iter().map(|s|s.to_string()).collect();
        ScreenerReturns {
            tickers,
            terms,
            quotes,
        }
    }
    ///function to output a report on returns of vector of tickers over vectors of timeperiods
    ///tickers and timeperiod are passed in the constructor screener_returns::new
    pub fn report(&self){
        let terms:Vec<&str> = self.terms.iter().map(|s|s.as_str()).collect();
        //print!("  ");
        //terms.iter().for_each(|t|print!("   {}  ",t));
        print!("{}",format!("{:<width$}","Ticker",width=10));
        terms.iter().for_each(|t|print!("{}",format!("{:>width$}",t,width=10)));
        println!();
        //println!("-------------");
        for i in self.quotes.iter(){
            //print!("{}",i.ticker());
            print!("{}",format!("{:<width$}",i.ticker(),width=10));
            let returns = i.ret_from_period(&terms);
            //returns.iter().zip(terms.iter()).for_each(|(r,t)|println!("return over {} is {:.2}%",t,r));
            //returns.iter().for_each(|r|print!(" {:.2}%  ",r));
            returns.iter().for_each(|r|print!("{}",format!("{:>width$}",format!("{:.2}%",r),width=10)));
            println!();
            //println!("-------------");
        }
    }
}