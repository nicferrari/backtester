use std::error::Error;
use rs_backtester::backtester::{Backtest, Commission};
use rs_backtester::datas::Data;
use rs_backtester::strategies::sma_cross;
use rs_backtester::utilities::{write_combined_csv, SerializeAsCsv};
use rs_backtester::broker::calculate;
use rs_backtester::report::{report};
use rs_backtester::trades::{report_trade, trade_indices_from_broker, trade_list, trade_list_from_broker};
use rs_backtester::metrics::Metrics;

fn main() ->Result<(),Box<dyn Error>> {
    /*
    //example to log or debug backtesting
    let quotes = Data::new_from_yahoo("PLTR","1d","6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64, Commission::default());
    sma_cross_tester.log(&["date","open","high","low","close","position","account","indicator"]);
    sma_cross_tester.to_csv("sma_cross.csv")?;
    sma_cross_strategy.to_csv("strategy.csv")?;
    quotes.to_csv("quotes.csv")?;
    //it's possible to append multiple csv reports in a single csv

    let broker = calculate(sma_cross_strategy.clone(), quotes.clone());

    let datasets: Vec<&dyn SerializeAsCsv> = vec![&quotes, &sma_cross_strategy, &broker];
    broker.to_csv("broker.csv")?;

    write_combined_csv("output.csv", &datasets[..])?;
    report(sma_cross_tester.clone());
    trade_list(sma_cross_tester);
    println!("-----------------");
    let trade_list = trade_list_from_broker(broker.clone(),quotes.clone(),sma_cross_strategy.clone());
    report_trade(trade_list);
    let trade_indices = trade_indices_from_broker(broker.clone());
    //trade_indices.indices.first().unwrap().print(quotes.clone(),sma_cross_strategy.clone());
    trade_indices.print_all_trades(quotes.clone(),sma_cross_strategy.clone());
    trade_indices.print(quotes.clone(),sma_cross_strategy.clone());
    broker.print_stats();
    let mut metrics = Metrics::default();
    trade_indices.calculate_metrics(&mut metrics, quotes.clone(), sma_cross_strategy.clone());
    //metrics.print_horizontally();
    broker.calculate_metrics(&mut metrics);
    //metrics.print_horizontally();*/
    Ok(())
}