use std::env;
use chrono::{DateTime, Duration, FixedOffset, TimeZone};
use csv::{Writer};
use yahoo_finance_api as yahoo;
use yahoo_finance_api::{Quote, time};
use tokio_test;
//use crate::errors::Result;
use std::error::Error;
use yahoo_finance_api::time::macros::datetime;
use std::str::FromStr;

fn download_data(ticker:&str, interval:&str, range:&str) ->Result<Vec<Quote>,Box<dyn Error>>{
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = tokio_test::block_on(provider.get_quote_range(ticker, interval, range)).unwrap();
    //let response = tokio_test::block_on(provider.get_quote_range("AAPL", "1d", "1mo")).unwrap();
    let quotes = response.quotes().unwrap();
    return Ok(quotes);
}
///struct to contain all market data (ticker + OHLC)
#[derive(Clone)]
pub struct Data{
    ticker:String,
    datetime:Vec<DateTime<FixedOffset>>,
    open:Vec<f64>,
    high:Vec<f64>,
    low:Vec<f64>,
    close:Vec<f64>,
}

impl Data{
    ///retrieve OHLC data from yahoo
    pub fn new_from_yahoo(ticker:&str, interval:&str, range:&str) ->Result<Self, Box<dyn Error>>{
        let quotes = download_data(&ticker,interval, range)?;
        let timestamps:Vec<u64> = quotes.iter().map(|s|s.timestamp).collect();
        let yahoo_datetimes: Vec<DateTime<FixedOffset>> = timestamps.iter().map(|&ts|{FixedOffset::east_opt(0).unwrap().timestamp_opt(ts as i64,0).unwrap()}).collect();
        let opens:Vec<f64> = quotes.iter().map(|s|s.open).collect();
        let highs:Vec<f64> = quotes.iter().map(|s|s.high).collect();
        let lows:Vec<f64> = quotes.iter().map(|s|s.low).collect();
        let closes:Vec<f64> = quotes.iter().map(|s|s.close).collect();
        Ok(Data{
            ticker:ticker.to_string(),
            datetime:yahoo_datetimes,
            open:opens,
            high:highs,
            low:lows,
            close:closes,
        })
    }
    pub fn save(&self, filename:&str)->Result<(), Box<dyn Error>>{
        let mut wrt = Writer::from_path(filename).expect("invalid path");
        let dates_t:Vec<Vec<String>> = self.datetime.iter().map(|e|vec![e.to_string()]).collect();
        let open_t:Vec<Vec<String>> = self.open.iter().map(|e|vec![e.to_string()]).collect();
        let high_t:Vec<Vec<String>> = self.high.iter().map(|e|vec![e.to_string()]).collect();
        let low_t:Vec<Vec<String>> = self.low.iter().map(|e|vec![e.to_string()]).collect();
        let close_t:Vec<Vec<String>> = self.close.iter().map(|e|vec![e.to_string()]).collect();
        wrt.serialize(("DATE","OPEN","HIGH","LOW","CLOSE")).expect("cannot write data");
        for ((((date,open),high),low),close) in dates_t.iter().zip(open_t.iter()).zip(high_t.iter()).zip(low_t.iter()).zip(close_t.iter()){
            wrt.serialize((date,open,high,low,close)).expect("cannot write data");
        }
        wrt.flush().expect("cannot write file");
        Ok(())
    }
    ///load data from csv OHLC (no volume) format at specified path
    pub fn load(path:&str, ticker:&str)->Result<Self,Box<dyn Error>>{
        let path2 = env::current_dir();
        let mut rdr = csv::Reader::from_path(path).expect(&format!("couldn't read file in {:?}",path2));
        let mut datetime= Vec::new();
        let mut open = Vec::new();
        let mut high = Vec::new();
        let mut low = Vec::new();
        let mut close = Vec::new();
        for result in rdr.records(){
            let record = result.expect("couldn't read data");
            let dates:DateTime<FixedOffset> = record[0].parse().expect("couldn't read data");
            let opens:f64 = record[1].parse().expect("couldn't read data");
            let highs:f64 = record[2].parse().expect("couldn't read data");
            let lows:f64 = record[3].parse().expect("couldn't read data");
            let closes:f64 = record[4].parse().expect("couldn't read data");
            datetime.push(dates);
            open.push(opens);
            high.push(highs);
            low.push(lows);
            close.push(closes);
        }
        Ok(Data{
            ticker:ticker.to_string(),
            datetime,
            open,
            high,
            low,
            close,
        })
    }
    pub fn ticker(&self)->&str{
        return &*self.ticker;
    }
    pub fn timestamps(&self)->Vec<DateTime<FixedOffset>>{
        return self.datetime.clone();
    }
    pub fn open(&self)->Vec<f64>{
        return self.open.clone();
    }
    pub fn high(&self)->Vec<f64>{return self.high.clone();}
    pub fn low(&self)->Vec<f64>{return self.low.clone();}
    pub fn close(&self)->Vec<f64>{
        return self.close.clone();
    }

    pub fn ret(&self)->f64{
        println!("{}",self.timestamps().first().unwrap());
        println!("{}",self.timestamps().last().unwrap());
        println!("{}",self.close.last().unwrap());
        println!("{}",self.close.first().unwrap());
        return (self.close.last().unwrap()/self.open.first().unwrap()-1.)*100.;
    }
    pub fn ret_from_date(&self, start_date:DateTime<FixedOffset>)->f64{
        let pos = self.datetime.iter().position(|&x|x.date_naive()>=start_date.date_naive()).unwrap();
        return self.close[pos];
    }
    ///give return on given period (accepts xd(-ays) or xw(-eeks) where x is an integer)
    pub fn ret_from_period(&self,term2:&[&str])->Vec<f64>{
        let mut ret = Vec::new();
        let last_date = self.datetime.last().unwrap();
        let term = term2.first().unwrap();
        let sought_date = match term.chars().last(){
            Some('w')=>last_date.checked_sub_signed(Duration::weeks(term[..term.len()-1].parse().unwrap())).unwrap(),
            Some('d')=>last_date.checked_sub_signed(Duration::days(term[..term.len()-1].parse().unwrap())).unwrap(),
            Some(c)=>*last_date,
            None=>*last_date,
        };
        println!("looking for {:}",sought_date.date_naive());
        let pos = self.datetime.iter().position(|&x|x.date_naive()>=sought_date.date_naive()).unwrap();
        println!("date found was {:} - {:}",self.datetime[pos].date_naive(),self.close[pos]);
        println!("final valuation date {:} - {:}",self.datetime.last().unwrap(),self.close().last().unwrap());
        ret.push((self.close.last().unwrap()/self.close[pos]-1.)*100.);
        return ret;
    }
    ///helper function to show data (datetime - close)
    pub fn show(&self){
        for i in 1 .. self.datetime.len(){
            println!("{} - {}",self.datetime[i],self.close[i]);
        }
    }

}