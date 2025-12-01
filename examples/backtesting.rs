use rs_backtester::backtester::Backtest;
use rs_backtester::broker::Execution::{AtClose, AtOpen};
use rs_backtester::config::{get_config, update_config};
use rs_backtester::data::Data;
use rs_backtester::metrics::{report_horizontal, report_vertical};
use rs_backtester::strategies::sma_cross;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quotes_arc = Data::new_from_yahoo("NVDA", "1d", "6mo")?;
    let sma_cross_strategy_arc = sma_cross(quotes_arc.clone(), 10, 20);
    let backtest_arc = Backtest::new(sma_cross_strategy_arc.clone(), 100_000.);
    println!("---------------------------");
    println!("With zero commission rate");
    println!("---------------------------");
    report_horizontal(&[&backtest_arc]);
    update_config(|cfg| {
        cfg.commission_rate = 0.01;
    });
    let backtest_with_comm_arc = Backtest::new(sma_cross_strategy_arc, 100_000.);
    println!("---------------------------");
    let cfg = get_config();
    println!(
        "Now with {:.2}% commission rate",
        cfg.commission_rate * 100.
    );
    println!("---------------------------");
    report_horizontal(&[&backtest_with_comm_arc]);
    report_vertical(&[&backtest_arc, &backtest_with_comm_arc]);
    Ok(())
}
