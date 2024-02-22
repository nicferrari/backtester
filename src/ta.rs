use crate::datas::Data;
use csv::Writer;
use std::error::Error;

pub struct Indicator{
    pub indicator:Vec<f64>,
    pub quotes:Data,
}

impl Indicator{
    pub fn to_csv(&self)->Result<(), Box<dyn Error>>{
        let mut wrt = Writer::from_path("provacsv.csv")?;
        wrt.write_record(self.indicator.iter().map(|e| e.to_string()))?;
        wrt.write_record(self.quotes.close().iter().map(|e| e.to_string()))?;
        wrt.flush()?;
        Ok(())
    }
    pub fn quotes(&self)->Data{
        return self.quotes.clone();
    }
}

pub fn sma(quotes:&Data, period:usize)->Vec<f64>{
    let mut indicator:Vec<f64> = vec![-1.;period-1];
    let length = quotes.timestamps().len();
    for i in period..length+1{
        let slice = &quotes.close()[i-period..i];
        let sum:f64 =Iterator::sum(slice.iter());
        let sma = sum/(period as f64);
        indicator.append(&mut vec![sma;1]);
    }
    return indicator;
}