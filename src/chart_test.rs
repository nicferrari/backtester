use plotly::{Plot};
use chrono::NaiveDate;
//use plotly::common::PlotType::Scatter;
use plotly::Scatter;
use plotly::common::Mode;
use crate::backtester::Backtest;

pub fn backtest_i_chart(bt:&Backtest) {
    // Convert dates to strings (Plotly expects string or numeric types)
    let dates: Vec<String> = bt.strategy.data.datetime
        .iter()
        .map(|d| d.date_naive().to_string())
        .collect();

    // Networths should be numeric, not strings, if you want a line chart
    let networths: Vec<f64> = bt.broker.networth.iter().map(|d| *d).collect();
    //let date_strings: Vec<String> = dates.iter().map(|d| d.to_string()).collect();

    let trace = Scatter::new(dates, networths)
        .name("Equity Curve")
        .mode(plotly::common::Mode::LinesMarkers);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show(); // Opens in browser
}
