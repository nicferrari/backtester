use crate::datas::Data;
use std::sync::Arc;

///container for checking calculation of indicator vs mktdata
#[derive(Clone)]
pub struct Indicator{
    pub indicator:Vec<f64>,
    pub quotes:Data,
}

pub struct Indicator_arc{
    pub indicator:Vec<f64>,
    pub quotes:Arc<Data>,
}


impl Indicator{
    /*
    pub fn to_csv(&self, filename:&str)->Result<(), Box<dyn Error>>{
        let mut wrt = Writer::from_path(filename)?;
        let transpose_indic:Vec<Vec<String>> = self.indicator.iter().map(|e|vec![e.clone().to_string()]).collect();
        let transpose_quote:Vec<Vec<String>> = self.quotes.close().iter().map(|e|vec![e.clone().to_string()]).collect();
        wrt.serialize(("close","indicator"))?;
        for (col1,col2) in transpose_quote.iter().zip(transpose_indic.iter()){
            wrt.serialize((col1,col2))?;
        }
        wrt.flush()?;
        Ok(())
    }*/
    pub fn quotes(&self)->Data{
        return self.quotes.clone();
    }
    pub fn indicator(&self)->Vec<f64>{
        return self.indicator.clone();
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
pub fn rsi(quotes:&Data, period:usize)->Vec<f64>{
    let mut indicator:Vec<f64> = vec![-1.;period-1];
    let length = quotes.timestamps().len();
    let diff:&Vec<f64> = &quotes.close().iter().zip(quotes.open().iter()).map(|(a,b)|a-b).collect();
    for i in period..length+1{
        let slice = &diff[i-period..i];
        let positive:Vec<f64> = slice.iter().cloned().filter(|&x|x>0.0).collect();
        let negative:Vec<f64> = slice.iter().cloned().filter(|&x|x<0.0).collect();
        let sum_pos:f64 = Iterator::sum(positive.iter());
        let sum_neg:f64 = Iterator::sum(negative.iter());
        let rsi = 100. * (sum_pos/(positive.len() as f64))/(sum_pos/(positive.len() as f64)-sum_neg/(negative.len() as f64));
        indicator.append(&mut vec![rsi;1])
    }
    indicator
}