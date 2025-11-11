use std::error::Error;
use rs_backtester::backtester_new::Backtest_arc;
use rs_backtester::charts::{plot_arc, PlotConfig};
use rs_backtester::datas::Data;
use rs_backtester::strategies::sma_cross_arc;
use rs_backtester::chart_test::show_equity_curve;

pub fn main() -> Result<(),Box<dyn Error>> {
    //example to plot backtesting results and how to change chart configuration
    let data = Data::new_from_yahoo_arc("GOOG","1d", "1y")?;
    let strategy = sma_cross_arc(data.clone(), 5, 15);
    let backtest = Backtest_arc::new(strategy, 100000.);

    //let's change default plot settings
    let mut plot_config = PlotConfig::default();
    plot_config.display_marker_label=true;
    plot_config.display_networth = true;

    plot_arc(&backtest,plot_config,"plot.png")?;
    show_equity_curve(data.datetime.clone().into_iter().map(|dt|dt.date_naive()).collect(),data.close.clone());
    Ok(())
}