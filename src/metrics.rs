const TAB:usize=15;

#[derive(Default)]
pub struct Metrics {
    pub strategy_name:Option<String>,
    //broker metrics
    pub bt_return:Option<f64>,
    pub exposure_time:Option<f64>,
    //trades metrics
    pub trades_nr:Option<usize>,
    pub max_pl:Option<f64>,
    pub min_pl:Option<f64>,
    pub average_pl:Option<f64>,
    pub win_rate:Option<f64>,
    pub avg_duration:Option<f64>,
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
        // Header row
        //print!("|");
        $(
            if $instance.$field.is_some() {
                //print!(" {} |", $header);
                print!("{:>width$}",$header,width=20);
            }
        )*
        println!();
        println!("{}","_".repeat(180));
        // Value row
        //print!("|");
        $(
            if let Some(val) = &$instance.$field {
                //print!(" {} |", $formatter(val));
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
        //print_defined_fields!(self, { bt_return, trades_nr});
        print_custom_fields!(self, {
            bt_return => |v| format!("Return = {:.2}%", v*100.),
            trades_nr => |v| format!("Trade # = {}", v),
        });
    }
    pub fn print_horizontally(&self){
        println!("{}","_".repeat(180));
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
        });
        println!("{}","_".repeat(180));
    }
}