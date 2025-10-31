const TAB:usize=15;

#[derive(Default, Clone)]
pub struct Metrics {
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
    //todo! add trade indices and calculate automatically during backtest initialization
}



macro_rules! print_defined_fields {
    ($instance:expr, { $($field:ident),* $(,)? }) => {
        $(
            if let Some(value) = &$instance.$field {
                println!("{}: {:?}", stringify!($field), value);
            }
        )*
    };
}

macro_rules! print_custom_fields {
    ($instance:expr, {
        $($field:ident => $formatter:expr),* $(,)?
    }) => {
        $(
            if let Some(value) = &$instance.$field {
                println!("{}", $formatter(value));
            }
        )*
    };
}

macro_rules! print_custom_row_with_headers {
    ($instance:expr, {
        $($field:ident => ($header:expr, $formatter:expr)),* $(,)?
    }) => {
        $(
            if $instance.$field.is_some() {
                print!("{:>width$}",$header,width=20);
            }
        )*
        println!();
        println!("{}","_".repeat(180));
        $(
            if let Some(val) = &$instance.$field {
                print!("{:>width$}",$formatter(val),width=20);
            }
        )*
        println!();
    };
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
    pub fn print_vertically(&self){
        print_custom_fields!(self, {
            strategy_name => |v| format!("Strategy = {}", v),
            bt_return => |v| format!("Return = {:.2}%", v),
            trades_nr => |v| format!("Trade # = {}", v),
            //todo! to complete for the other fields
        });
    }
    pub fn print_horizontally(&self){
        println!("{}","_".repeat(11*TAB+3*11));
        print_custom_row_with_headers_aligned!(self, {
            strategy_name => ("Strategy", |v:&String| format!("{}",&v[..TAB])),
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
        println!("{}","_".repeat(11*TAB+3*11));
    }
}

pub fn compare_metrics_horizontally(metrics: &[&Metrics]){
    for item in metrics{
        item.print_horizontally();
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



pub fn compare_metrics_vertically(metrics: &[Metrics]) {
    println!("{}","_".repeat(100));
    print_field!("Strategy", metrics, |i:&Metrics| i.strategy_name.clone(), |s|  s, TAB);
    print_field!("Return", metrics, |i:&Metrics| i.bt_return, |r| format!("{:.2}%", r), TAB);
    print_field!("Exp time", metrics, |i:&Metrics| i.exposure_time, |s| format!("{:.2}%",s*100.), TAB);
    print_field!("Trades #", metrics, |i:&Metrics| i.trades_nr, |a| a, TAB);
    print_field!("Max P&L", metrics, |i:&Metrics| i.max_pl, |s| format!("{:.2}%",s), TAB);
    print_field!("Min P&L", metrics, |i:&Metrics| i.min_pl, |s| format!("{:.2}%",s), TAB);
    print_field!("Avg P&L", metrics, |i:&Metrics| i.average_pl, |s| format!("{:.2}%",s), TAB);
    print_field!("Win rate", metrics, |i:&Metrics| i.win_rate, |s| format!("{:.2}%",s*100.), TAB);
    print_field!("Avg dur (d)", metrics, |i:&Metrics| i.avg_duration, |s| format!("{:.2}",s), TAB);
    print_field!("Max Drawdown", metrics, |i:&Metrics| i.max_drawd, |s| format!("{:.2}%",s*100.), TAB);
    print_field!("Sharpe r", metrics, |i:&Metrics| i.sharpe, |s| format!("{:.2}",s*252f64.sqrt()), TAB);
    println!("{}","_".repeat(100));
}
