use rs_backtester::backtester_new::{Backtest, Backtest_arc};
//use rs_backtester::broker::Execution::AtOpen;
use rs_backtester::datas::Data;
use rs_backtester::metrics::{report_horizontal, report_vertical, report_horizontal_arc, report_vertical_arc};
use rs_backtester::strategies::{sma_cross, sma_cross_arc};
use rs_backtester::config::{get_config, update_config};
//use rs_backtester::ta::sma;

fn main() -> Result<(), Box<dyn std::error::Error>> {
/*
    let quotes = Data::new_from_yahoo("NVDA","1d","6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let backtest = Backtest::new(quotes.clone(), sma_cross_strategy, 100_000.);
//    backtest.metrics.print_horizontally();
//    backtest.metrics.print_vertically();
    backtest.to_csv("output_new_backtest.csv")?;
    let sma_cross_strategy2 = sma_cross(quotes.clone(),15, 30);
    let backtest2 = Backtest::new(quotes.clone(), sma_cross_strategy2, 100_000.);
    report_horizontal(&[&backtest.metrics,&backtest2.metrics]);
    report_vertical(&[&backtest.metrics,&backtest2.metrics]);
    println!("{}","_".repeat(100));
    report_vertical(&[&backtest.metrics]);
    backtest.trade_list();
    Ok(())
 */
    let quotes = Data::new_from_yahoo("NVDA","1d","6mo")?;
    let quotes_arc = Data::new_from_yahoo_arc("NVDA","1d","6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let sma_cross_strategy_arc = sma_cross_arc(quotes_arc.clone(),10,20);
    let backtest = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100_000.);
    let backtest_arc = Backtest_arc::new(sma_cross_strategy_arc.clone(),100_000.);
    println!("---------------------------");
    println!("With zero commission rate");
    println!("---------------------------");
    report_horizontal(&[&backtest]);
    report_horizontal_arc(&[&backtest_arc]);
    backtest_arc.to_csv_arc("save_arc.csv");
    update_config(|cfg| {cfg.commission_rate = 0.01;});
    let backtest_with_comm = Backtest::new(quotes,sma_cross_strategy,100_000.);
    let backtest_with_comm_arc = Backtest_arc::new(sma_cross_strategy_arc, 100_000.);
    println!("---------------------------");
    let cfg = get_config();
    println!("Now with {:.2}% commission rate",cfg.commission_rate*100.);
    println!("---------------------------");
    report_horizontal(&[&backtest_with_comm]);
    report_horizontal_arc(&[&backtest_with_comm_arc]);
    report_vertical(&[&backtest,&backtest_with_comm]);
    report_vertical_arc(&[&backtest_arc,&backtest_with_comm_arc]);
    Ok(())
}