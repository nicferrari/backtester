use crate::backtester::{Backtest};
use crate::trades::TradeList;

const TAB:usize=15;

#[derive(Default, Clone)]
pub struct Metrics {
    pub ticker:Option<String>,
    pub strategy_name:Option<String>,
    //broker metrics
    pub bt_return:Option<f64>,
    pub exposure_time:Option<f64>,
    pub max_drawd:Option<f64>,
    pub sharpe:Option<f64>,
    //trades metrics
    pub trades_nr:Option<usize>,
    pub max_pl:Option<f64>,
    pub min_pl:Option<f64>,
    pub average_pl:Option<f64>,
    pub win_rate:Option<f64>,
    pub avg_duration:Option<f64>,
    //trades indices from broker
    pub trades_indices: Option<TradeList>,
}

macro_rules! print_custom_row_with_headers_aligned {
    ($instance:expr, {
        $first_field:ident => ($first_header:expr, $first_fmt:expr),
        $($field:ident => ($header:expr, $formatter:expr)),* $(,)?
    }) => {
        // Header row
        print!("| {:<width$} |", $first_header, width = TAB); // left-aligned
        $(
            if $instance.$field.is_some() {
                print!(" {:>width$} |", $header, width = TAB); // right-aligned
            }
        )*
        println!();

        // Value row
        if let Some(val) = &$instance.$first_field {
            print!("| {:<width$} |", $first_fmt(val), width = TAB); // left-aligned
        } else {
            print!("| {:<width$} |", "", width = TAB); // empty if None
        }

        $(
            if let Some(val) = &$instance.$field {
                print!(" {:>width$} |", $formatter(val), width = TAB); // right-aligned
            }
        )*
        println!();
    };
}

impl Metrics{
    fn print_horizontally(&self){
        println!("{}","_".repeat(12*TAB+3*12));
        print_custom_row_with_headers_aligned!(self, {
            ticker => ("Ticker", |v:&String| {let end = v.len().min(TAB);v[..end].to_string()}),
            strategy_name => ("Strategy", |v:&String| {let end = v.len().min(15);v[..end].to_string()}),
            bt_return => ("Return", |v| format!("{:.2}%", v)),
            exposure_time => ("Exp time", |v| format!("{:.2}%", v*100.)),
            trades_nr => ("Trades #", |v| format!("{}", v)),
            max_pl => ("Max p&l", |v| format!("{:.2}%", v)),
            min_pl => ("Min p&l", |v| format!("{:.2}%", v)),
            average_pl => ("Avg p&l", |v| format!("{:.2}%", v)),
            win_rate => ("Win rate", |v| format!("{:.2}%", v*100.)),
            avg_duration => ("Avg dur (d)", |v| format!("{:.2}", v)),
            max_drawd => ("Max Drawdown", |v| format!("{:.2}%", v*100.)),
            sharpe => ("Sharpe r", |v| format!("{:.2}", v*252f64.sqrt())),
        });
        println!("{}","_".repeat(12*TAB+3*12));
    }
}

pub fn report_horizontal(backtests: &[&Backtest]){
    for item in backtests{
        println!("Backtesting period {} - {} ({} days)",item.strategy.data.datetime.first().unwrap().date_naive(),item.strategy.data.datetime.last().unwrap().date_naive(),
                 (*item.strategy.data.datetime.last().unwrap()-item.strategy.data.datetime.first().unwrap()).num_days());
        item.metrics.print_horizontally();
    }
}

macro_rules! print_field {
    ($label:expr, $items:expr, $getter:expr, $formatter:expr, $width:expr) => {
        print!("{:<width$} |", $label, width=TAB);
        for item in $items {
            match $getter(item) {
                Some(val) => print!("{:>width$} |", $formatter(val), width = $width),
                None => print!("{:>width$} |", "-", width = $width),
            }
        }
        println!();
    };
}
pub fn report_vertical(backtests: &[&Backtest]) {
    println!("{}", "_".repeat(100));
    print_field!("Start Date", backtests, |i:&Backtest| Some(i.strategy.data.datetime.first().unwrap().date_naive().to_string()), |s|  s, TAB);
    print_field!("End Date", backtests, |i:&Backtest| Some(i.strategy.data.datetime.last().unwrap().date_naive().to_string()), |s|  s, TAB);
    print_field!("Ticker", backtests, |i:&Backtest| i.metrics.ticker.clone(), |s|  s, TAB);
    print_field!("Strategy", backtests, |i:&Backtest| i.metrics.strategy_name.clone(), |s|  s, TAB);
    print_field!("Return", backtests, |i:&Backtest| i.metrics.bt_return, |r| format!("{:.2}%", r), TAB);
    print_field!("Exp time", backtests, |i:&Backtest| i.metrics.exposure_time, |s| format!("{:.2}%",s*100.), TAB);
    print_field!("Trades #", backtests, |i:&Backtest| i.metrics.trades_nr, |a| a, TAB);
    print_field!("Max P&L", backtests, |i:&Backtest| i.metrics.max_pl, |s| format!("{:.2}%",s), TAB);
    print_field!("Min P&L", backtests, |i:&Backtest| i.metrics.min_pl, |s| format!("{:.2}%",s), TAB);
    print_field!("Avg P&L", backtests, |i:&Backtest| i.metrics.average_pl, |s| format!("{:.2}%",s), TAB);
    print_field!("Win rate", backtests, |i:&Backtest| i.metrics.win_rate, |s| format!("{:.2}%",s*100.), TAB);
    print_field!("Avg dur (d)", backtests, |i:&Backtest| i.metrics.avg_duration, |s| format!("{:.2}",s), TAB);
    print_field!("Max Drawdown", backtests, |i:&Backtest| i.metrics.max_drawd, |s| format!("{:.2}%",s*100.), TAB);
    print_field!("Sharpe r", backtests, |i:&Backtest| i.metrics.sharpe, |s| format!("{:.2}",s*252f64.sqrt()), TAB);
    println!("{}", "_".repeat(100));
    for (index, item) in backtests.iter().enumerate() {
        item.print_config(index);
    }
}