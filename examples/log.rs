use rs_backtester::backtester::Backtest;
use rs_backtester::data::Data;
use rs_backtester::strategies::sma_cross;
use rs_backtester::utilities::{write_combined_csv, SerializeAsCsv};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let quotes = Data::new_from_yahoo("PLTR", "1d", "6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10, 20);
    let sma_cross_bt = Backtest::new(sma_cross_strategy.clone(), 100000f64);
    quotes.to_csv("quotes.csv")?;
    sma_cross_strategy.to_csv("strategy.csv")?;
    sma_cross_bt.to_csv("bt.csv")?;
    //it is also possible to combine multiple parts together
    //suppose we want to have both quotes and strategy in the same csv
    let datasets: Vec<&dyn SerializeAsCsv> = vec![&quotes, &sma_cross_strategy];
    write_combined_csv("combined.csv", &datasets[..])?;
    Ok(())
}
