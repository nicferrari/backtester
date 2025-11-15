use std::error::Error;
use rs_backtester::backtester_new::Backtest_arc;
use rs_backtester::datas::Data;
use rs_backtester::metrics::{report_vertical_arc};
use rs_backtester::strategies::{buy_n_hold_arc, rsi_strategy_arc, sma_arc, sma_cross_arc};
use rs_backtester::utilities::SerializeAsCsv;

pub fn main() ->Result<(),Box<dyn Error>>{
    let filename = "test_data//NVDA.csv";
    let quotes_arc = Data::load_arc(filename,"NVDA")?;
    let buynhold = buy_n_hold_arc(quotes_arc.clone());
    let sma_cross = sma_cross_arc(quotes_arc.clone(),10,20);
    let sma = sma_arc(quotes_arc.clone(),10);
    let rsi = rsi_strategy_arc(quotes_arc.clone(),15);
    let buynhold_bt = Backtest_arc::new(buynhold,100_000.);
    let sma_cross_bt = Backtest_arc::new(sma_cross,100_000.);
    let sma_bt =Backtest_arc::new(sma,100_000.);
    let rsi_bt = Backtest_arc::new(rsi,100_000.);
    report_vertical_arc(&[&buynhold_bt,&sma_bt,&sma_cross_bt,&rsi_bt]);
    Ok(())
}