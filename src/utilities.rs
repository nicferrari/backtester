use csv::Writer;
use std::error::Error;
use std::fs::File;
use crate::datas::Data;
use crate::strategies::Strategy;

// Define a trait for custom data structures
pub trait SerializeAsCsv {
    fn to_csv(&self, writer: &mut Writer<std::fs::File>) -> Result<(), Box<dyn Error>>;
}


// General function to serialize any type that implements SerializeAsCsv
pub fn serialize_to_csv<T: SerializeAsCsv>(data: &T, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;
    data.to_csv(&mut wtr)?;
    wtr.flush()?;
    Ok(())
}

impl SerializeAsCsv for Data {
    fn to_csv(&self, writer: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
        writer.serialize(("ticker","date","open","high","low","close","volume")).expect("couldn't write csv");
        for i in 0..self.datetime.len() {
            writer.write_record(&[
                self.ticker.clone(),
                self.datetime[i].to_string(),
                self.open[i].to_string(),
                self.high[i].to_string(),
                self.low[i].to_string(),
                self.close[i].to_string(),
                self.volume[i].to_string(),
            ])?;
        }
        Ok(())
    }
}

impl SerializeAsCsv for Strategy {
    fn to_csv(&self, writer: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
        if let Some(indicators) = self.indicator.clone() {
            let mut header: Vec<String> = Vec::new();
            header.push("Name".to_string());
            header.push("Choice".to_string());
            for (index,_) in indicators.iter().enumerate(){
                header.push(format!("Indicator {}", index+1));
            }
            writer.write_record(&header)?;
            for i in 0..self.choices.len() {
                let mut row: Vec<String> = vec![self.name.to_string(),self.choices[i].to_string().into()];
                row.extend(indicators.iter().map(|indicator|indicator[i].to_string()));
                writer.write_record(&row)?;
            }
        }
        Ok(())
    }
}