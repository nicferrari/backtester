use rs_backtester::backtester_new::Backtest;
//use rs_backtester::broker::Execution::AtOpen;
use rs_backtester::datas::Data;
use rs_backtester::metrics::{report_horizontal, report_vertical};
use rs_backtester::strategies::sma_cross;
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
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let backtest = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100_000.);
    println!("---------------------------");
    println!("With zero commission rate");
    println!("---------------------------");
    report_horizontal(&[&backtest]);

    update_config(|cfg| {cfg.commission_rate = 0.01;});
    let backtest_with_comm = Backtest::new(quotes,sma_cross_strategy,100_000.);
    println!("---------------------------");
    let cfg = get_config();
    println!("Now with {:.2}% commission rate",cfg.commission_rate*100.);
    println!("---------------------------");
    report_horizontal(&[&backtest_with_comm]);
    report_vertical(&[&backtest,&backtest_with_comm]);
    Ok(())
}