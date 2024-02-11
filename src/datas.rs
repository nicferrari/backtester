use chrono::{DateTime, FixedOffset, TimeZone};
use yahoo_finance_api as yahoo;
use yahoo_finance_api::Quote;
use tokio_test;
use crate::errors::Result;

pub fn download_data(ticker:&str, interval:&str, range:&str)->Result<Vec<Quote>>{
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range(ticker, interval, range)).unwrap();
    let quotes = response.quotes().unwrap();
    return Ok(quotes);
}
#[derive(Clone)]
pub struct Data{
    ticker:String,
    datetime:Vec<DateTime<FixedOffset>>,
    open:Vec<f64>,
    close:Vec<f64>,
}

impl Data{
    pub fn new_from_yahoo(ticker:String) ->Result<Self>{
        let quotes = download_data(&ticker,"1d","1mo")?;
        let timestamps:Vec<u64> = quotes.iter().map(|s|s.timestamp).collect();
        let yahoo_datetimes: Vec<DateTime<FixedOffset>> = timestamps.iter().map(|&ts|{FixedOffset::east_opt(0).unwrap().timestamp_opt(ts as i64,0).unwrap()}).collect();
        let opens:Vec<f64> = quotes.iter().map(|s|s.open).collect();
        let closes:Vec<f64> = quotes.iter().map(|s|s.close).collect();
        Ok(Data{
            ticker:ticker,
            datetime:yahoo_datetimes,
            open:opens,
            close:closes,
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
    pub fn close(&self)->Vec<f64>{
        return self.close.clone();
    }
}