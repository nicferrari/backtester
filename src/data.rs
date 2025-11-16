use std::env;
use chrono::{DateTime, FixedOffset, TimeZone};
use csv::{Writer};
use yahoo_finance_api as yahoo;
use yahoo_finance_api::{Quote};
use tokio_test;
use std::error::Error;
use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq};
use std::sync::Arc;

fn download_data(ticker:&str, interval:&str, range:&str) ->Result<Vec<Quote>,Box<dyn Error>>{
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = tokio_test::block_on(provider.get_quote_range(ticker, interval, range)).unwrap();
    let quotes = response.quotes().unwrap();
    Ok(quotes)
}
///struct to contain all market data (ticker + OHLCV)
#[derive(Clone, Serialize)]
pub struct Data{
    pub ticker:String,
    #[serde(serialize_with = "serialize_datetime_vec")]
    pub datetime:Vec<DateTime<FixedOffset>>,
    pub open:Vec<f64>,
    pub high:Vec<f64>,
    pub low:Vec<f64>,
    pub close:Vec<f64>,
    pub volume:Vec<u64>,
}

fn serialize_datetime_vec<S>(datetimes: &Vec<DateTime<FixedOffset>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(datetimes.len()))?;
    for datetime in datetimes {
        seq.serialize_element(&datetime.to_rfc3339())?;
    }
    seq.end()
}

impl Data{
    ///retrieve OHLC data from yahoo finance
    pub fn new_from_yahoo(ticker:&str, interval:&str, range:&str) ->Result<Arc<Self>, Box<dyn Error>>{
        let quotes = download_data(ticker, interval, range)?;
        let timestamps:Vec<u64> = quotes.iter().map(|s|s.timestamp).collect();
        let yahoo_datetimes: Vec<DateTime<FixedOffset>> = timestamps.iter().map(|&ts|{FixedOffset::east_opt(0).unwrap().timestamp_opt(ts as i64,0).unwrap()}).collect();
        let opens:Vec<f64> = quotes.iter().map(|s|s.open).collect();
        let highs:Vec<f64> = quotes.iter().map(|s|s.high).collect();
        let lows:Vec<f64> = quotes.iter().map(|s|s.low).collect();
        let closes:Vec<f64> = quotes.iter().map(|s|s.close).collect();
        let volumes:Vec<u64> = quotes.iter().map(|s|s.volume).collect();
        println!("\x1b[34m{} {} data ({}) downloaded from Yahoo!",ticker, range, interval);
        Ok(Arc::new(Data{
            ticker:ticker.to_string(),
            datetime:yahoo_datetimes,
            open:opens,
            high:highs,
            low:lows,
            close:closes,
            volume:volumes,
        }))
    }
    ///save data to filename
    pub fn save(&self, filename:&str)->Result<(), Box<dyn Error>>{
        let mut wrt = Writer::from_path(filename).expect("invalid path");
        let dates_t:Vec<Vec<String>> = self.datetime.iter().map(|e|vec![e.to_string()]).collect();
        let open_t:Vec<Vec<String>> = self.open.iter().map(|e|vec![e.to_string()]).collect();
        let high_t:Vec<Vec<String>> = self.high.iter().map(|e|vec![e.to_string()]).collect();
        let low_t:Vec<Vec<String>> = self.low.iter().map(|e|vec![e.to_string()]).collect();
        let close_t:Vec<Vec<String>> = self.close.iter().map(|e|vec![e.to_string()]).collect();
        let volumes_t:Vec<Vec<String>> = self.volume.iter().map(|e|vec![e.to_string()]).collect();
        wrt.serialize(("DATE","OPEN","HIGH","LOW","CLOSE","VOLUME")).expect("cannot write data");
        for (((((date,open),high),low),close),volume) in dates_t.iter().zip(open_t.iter()).zip(high_t.iter()).zip(low_t.iter()).zip(close_t.iter()).zip(volumes_t.iter()){
            wrt.serialize((date,open,high,low,close,volume)).expect("cannot write data");
        }
        wrt.flush().expect("cannot write file");
        Ok(())
    }

    ///load data from csv OHLC format at specified path
    pub fn load(path:&str, ticker:&str) ->Result<Arc<Self>,Box<dyn Error>>{
        let path2 = env::current_dir();
        let mut rdr = csv::Reader::from_path(path).unwrap_or_else(|_| panic!("couldn't read file in {:?}",path2));
        let mut datetime= Vec::new();
        let mut open = Vec::new();
        let mut high = Vec::new();
        let mut low = Vec::new();
        let mut close = Vec::new();
        let mut volume= Vec::new();
        for result in rdr.records(){
            let record = result.expect("couldn't read data");
            let dates:DateTime<FixedOffset> = record[0].parse().expect("couldn't read data");
            let opens:f64 = record[1].parse().expect("couldn't read data");
            let highs:f64 = record[2].parse().expect("couldn't read data");
            let lows:f64 = record[3].parse().expect("couldn't read data");
            let closes:f64 = record[4].parse().expect("couldn't read data");
            let volumes:u64 = record[5].parse().expect("couldn't read data");
            datetime.push(dates);
            open.push(opens);
            high.push(highs);
            low.push(lows);
            close.push(closes);
            volume.push(volumes);
        }
        println!("\x1b[34mLoading filename = {:?}",path2.unwrap().join(ticker));
        Ok(Arc::new(Data{
            ticker:ticker.to_string(),
            datetime,
            open,
            high,
            low,
            close,
            volume,
        }))
    }
}