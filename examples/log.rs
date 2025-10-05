use std::error::Error;
use rs_backtester::backtester::{Backtest, Commission};
use rs_backtester::datas::Data;
use rs_backtester::strategies::sma_cross;
use rs_backtester::utilities::{write_combined_csv, SerializeAsCsv};

fn main() ->Result<(),Box<dyn Error>> {
    //example to log or debug backtesting
    let quotes = Data::new_from_yahoo("PLTR","1d","6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64, Commission::default());
    sma_cross_tester.log(&["date","open","high","low","close","position","account","indicator"]);
    sma_cross_tester.to_csv("sma_cross.csv")?;
    sma_cross_strategy.to_csv("strategy.csv")?;
    quotes.to_csv("quotes.csv")?;
    //it's possible to append multiple csv reports in a single csv
    let datasets: Vec<&dyn SerializeAsCsv> = vec![&quotes, &sma_cross_strategy];
    write_combined_csv("output.csv", &datasets[..])?;
    Ok(())
}