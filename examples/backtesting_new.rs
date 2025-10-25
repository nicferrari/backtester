use rs_backtester::backtester_new::Backtest;
use rs_backtester::datas::Data;
use rs_backtester::metrics::{compare_metrics_horizontally, compare_metrics_vertically};
use rs_backtester::strategies::sma_cross;

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let quotes = Data::new_from_yahoo("NVDA","1d","6mo")?;
    let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
    let backtest = Backtest::new(quotes.clone(), sma_cross_strategy, 100_000.);
    backtest.metrics.print_horizontally();
    backtest.metrics.print_vertically();
    backtest.save("output_new_backtest.csv");
    let sma_cross_strategy2 = sma_cross(quotes.clone(),15, 30);
    let backtest2 = Backtest::new(quotes, sma_cross_strategy2, 100_000.);
    //compare_metrics_horizontally(&[backtest.metrics,backtest2.metrics]);
    compare_metrics_vertically(&[backtest.metrics,backtest2.metrics]);
    Ok(())
}