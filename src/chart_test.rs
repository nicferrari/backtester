use plotly::{Plot};
use chrono::NaiveDate;
//use plotly::common::PlotType::Scatter;
use plotly::Scatter;

pub fn show_equity_curve(dates: Vec<NaiveDate>, values: Vec<f64>) {
    let date_strings: Vec<String> = dates.iter().map(|d| d.to_string()).collect();

    let trace = Scatter::new(date_strings, values)
        .name("Equity Curve")
        .mode(plotly::common::Mode::LinesMarkers);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show(); // Opens in browser
}
