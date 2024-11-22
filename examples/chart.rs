use backtester::backtester::Backtest;
use backtester::datas::Data;
use backtester::strategies::{buy_n_hold, sma_cross};
use backtester::charts::{plot, Plot_Config};
use std::error::Error;

pub fn main() -> Result<(),Box<dyn Error>> {
    let data = Data::new_from_yahoo("GOOG","1d", "1y")?;
    // TODO: calling chart with buy_n_hold or simply add possibility to chart only data;
    let strategy = sma_cross(data.clone(), 5, 15);
    let mut backtest = Backtest::new(data.clone(), strategy, 100000.);
    backtest.calculate();
    let mut plot_config = Plot_Config::default();
    plot_config.display_marker_label=true;
    plot_config.display_networth = true;
    plot(backtest.clone(),plot_config)?;
    Ok(())
}