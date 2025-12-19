use crate::broker::Broker;
use crate::data::Data;
use crate::strategies::Strategy;
use crate::trades::TradeList;
use csv::Writer;
use std::error::Error;
use std::sync::Arc;

pub trait SerializeAsCsv {
    fn headers(&self) -> Vec<String>;
    fn to_rows(&self) -> Vec<Vec<String>>;
    fn to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>>;
}

pub fn write_combined_csv(
    file_path: &str,
    datasets: &[&dyn SerializeAsCsv],
) -> Result<(), Box<dyn Error>> {
    let mut all_headers = Vec::new();
    let mut all_rows: Vec<Vec<String>> = Vec::new();

    // Collect headers
    for dataset in datasets {
        all_headers.extend(dataset.headers());
    }

    // Determine max row count
    let max_len = datasets
        .iter()
        .map(|d| d.to_rows().len())
        .max()
        .unwrap_or(0);

    // Collect rows
    for i in 0..max_len {
        let mut row = Vec::new();
        for dataset in datasets {
            let rows = dataset.to_rows();
            if let Some(r) = rows.get(i) {
                row.extend(r.clone());
            } else {
                row.extend(vec!["".to_string(); dataset.headers().len()]);
            }
        }
        all_rows.push(row);
    }

    // Write to CSV
    let mut wtr = Writer::from_path(file_path)?;
    wtr.write_record(&all_headers)?;
    for row in all_rows {
        wtr.write_record(&row)?;
    }
    wtr.flush()?;
    Ok(())
}
impl SerializeAsCsv for Broker {
    fn headers(&self) -> Vec<String> {
        vec![
            "Execution".to_string(),
            "Side".to_string(),
            "Status".to_string(),
            "Available".to_string(),
            "Positions".to_string(),
            "Invested".to_string(),
            "Fees".to_string(),
            "Account".to_string(),
            "Cash".to_string(),
            "MtM".to_string(),
            "Networth".to_string(),
        ]
    }
    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        for i in 0..self.execution.len() {
            rows.push(vec![
                self.execution[i].to_string(),
                self.side[i].to_string(),
                self.status[i].to_string(),
                self.available[i].to_string(),
                self.position[i].to_string(),
                self.invested[i].to_string(),
                self.fees[i].to_string(),
                self.account[i].to_string(),
                self.cash[i].to_string(),
                self.mtm[i].to_string(),
                self.networth[i].to_string(),
            ]);
        }
        rows
    }
    fn to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let datasets: Vec<&dyn SerializeAsCsv> = vec![self];
        write_combined_csv(file_path, &datasets[..])?;
        Ok(())
    }
}

impl SerializeAsCsv for Strategy {
    fn headers(&self) -> Vec<String> {
        let mut header = vec!["Name".to_string(), "Choice".to_string()];
        if let Some(indicators) = &self.indicator {
            for i in 0..indicators.len() {
                header.push(format!("Indicator {}", i + 1));
            }
        }
        header
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        if let Some(indicators) = &self.indicator {
            for i in 0..self.choices.len() {
                let mut row = vec![self.name.clone(), self.choices[i].to_string()];
                row.extend(indicators.iter().map(|ind| ind[i].to_string()));
                rows.push(row);
            }
        }
        rows
    }
    fn to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let datasets: Vec<&dyn SerializeAsCsv> = vec![self];
        write_combined_csv(file_path, &datasets[..])?;
        Ok(())
    }
}

impl SerializeAsCsv for Arc<Data> {
    fn headers(&self) -> Vec<String> {
        vec![
            "ticker".to_string(),
            "date".to_string(),
            "open".to_string(),
            "high".to_string(),
            "low".to_string(),
            "close".to_string(),
            "volume".to_string(),
        ]
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        for i in 0..self.datetime.len() {
            rows.push(vec![
                self.ticker.clone(),
                self.datetime[i].to_string(),
                self.open[i].to_string(),
                self.high[i].to_string(),
                self.low[i].to_string(),
                self.close[i].to_string(),
                self.volume[i].to_string(),
            ]);
        }
        rows
    }
    fn to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let datasets: Vec<&dyn SerializeAsCsv> = vec![self];
        write_combined_csv(file_path, &datasets[..])?;
        Ok(())
    }
}
impl SerializeAsCsv for TradeList {
    fn headers(&self) -> Vec<String> {
        vec![
            "position".to_string(),
            "open index".to_string(),
            "close_index".to_string(),
        ]
    }
    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        for i in 0..self.indices.len() {
            rows.push(vec![
                self.indices[i].position.to_string(),
                self.indices[i].open_index.to_string(),
                self.indices[i].close_index.to_string(),
            ]);
        }
        rows
    }
    fn to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let datasets: Vec<&dyn SerializeAsCsv> = vec![self];
        write_combined_csv(file_path, &datasets[..])?;
        Ok(())
    }
}
