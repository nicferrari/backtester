use backtester::backtester::{Backtest, Commission};
use backtester::datas::Data;
use backtester::strategies::{sma_cross};
use backtester::charts::{plot, PlotConfig};
use std::error::Error;

pub fn main() -> Result<(),Box<dyn Error>> {
    //example to plot backtesting results and how to change chart configuration
    let data = Data::new_from_yahoo("GOOG","1d", "1y")?;
    let strategy = sma_cross(data.clone(), 5, 15);
    let backtest = Backtest::new(data.clone(), strategy, 100000., Commission::default());

    //let's change default plot settings
    let mut plot_config = PlotConfig::default();
    plot_config.display_marker_label=true;
    plot_config.display_networth = true;

    plot(backtest.clone(),plot_config)?;
    Ok(())
}