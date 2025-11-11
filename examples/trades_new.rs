use rs_backtester::backtester_new::Backtest_arc;
use rs_backtester::datas::Data;
use rs_backtester::metrics::report_horizontal_arc;
use rs_backtester::strategies::simple_sma_arc;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let quotes = Data::new_from_yahoo_arc("NVDA","1d","6mo")?;
    let sma_strategy = simple_sma_arc(quotes.clone(),10);
    let sma_bt = Backtest_arc::new(sma_strategy.clone(),100_000.);
    report_horizontal_arc(&[&sma_bt]);
    sma_bt.print_all_trades();
    //instead of the full list of trades of a Backtest, it is also possible to print a specific trade (by position)
    //suppose we want to print trade #5
    println!("{}","_".repeat(30));
    println!("Trade position 5");
    sma_bt.print_single_trade(5);
    Ok(())
}