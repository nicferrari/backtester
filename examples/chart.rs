use rs_backtester::backtester::Backtest;
use rs_backtester::charts::{i_chart, plot, PlotConfig};
use rs_backtester::data::Data;
use rs_backtester::strategies::sma_cross;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    //example to plot backtesting results and how to change chart configuration
    let data = Data::new_from_yahoo("GOOG", "1d", "1y")?;
    let strategy = sma_cross(data.clone(), 5, 15);
    let backtest = Backtest::new(strategy, 100000.);

    //let's change default plot settings
    let mut plot_config = PlotConfig::default();
    plot_config.display_marker_label = true;
    plot_config.display_networth = true;

    //png plot
    plot(&backtest, plot_config, "plot.png")?;
    //interactive html chart
    i_chart(backtest, "i_chart.html")?;

    Ok(())
}
