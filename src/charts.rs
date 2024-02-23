use plotters::prelude::*;
use crate::{datas, Result};
use chrono::{DateTime, FixedOffset};
use datas::Data;

pub fn plot(quotes:&Data)->Result<()>{

    let yahoo_datetimes:Vec<DateTime<FixedOffset>> = quotes.timestamps();
    let opens:Vec<f64> = quotes.open();
    let closes:Vec<f64> = quotes.close();

    // if folder does not exists no plotting happens: should implement check and create folder
    let root = BitMapBackend::new("examples/images/plot.png", (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);

    let min_opens = *opens.iter().min_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()-5.0;
    let max_opens = *opens.iter().max_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()+5.0;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .caption("Chart ".to_owned() + quotes.ticker(), ("sans-serif", 30).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len()-1], min_opens..max_opens).unwrap();

    chart.configure_mesh().x_label_formatter(&|dt|dt.format("%Y-%m-%d").to_string()).draw().unwrap();

    let _ = chart.draw_series(LineSeries::new((0..opens.len()).map(|i| (yahoo_datetimes[i], opens[i])), &BLUE)).unwrap().label("open");
    let _ = chart.draw_series(LineSeries::new((0..closes.len()).map(|i|(yahoo_datetimes[i], closes[i])),&GREEN)).unwrap().label("close");

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    println!("Plotting {:?} from {:?}",{quotes.ticker()},{min_opens});
    Ok(())
}

pub fn plot_nw(quotes:&Data, position:&Vec<f64>, account:&Vec<f64>)->Result<()>{
    let yahoo_datetimes:Vec<DateTime<FixedOffset>> = quotes.timestamps();
    let opens:Vec<f64> = quotes.open();
    let closes:Vec<f64> = quotes.close();

    // if folder does not exists no plotting happens: should implement check and create folder
    let root = BitMapBackend::new("examples/images/plot.png", (1024, 768)).into_drawing_area();
    let _ = root.fill(&WHITE);

    let (upper,lower) = root.split_vertically(512);

    let min_opens = *opens.iter().min_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()-5.0;
    let max_opens = *opens.iter().max_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()+5.0;

    let mut chart = ChartBuilder::on(&upper)
        .margin(5)
        .caption("Chart ".to_owned() + quotes.ticker(), ("sans-serif", 30).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len()-1], min_opens..max_opens).unwrap();

    chart.configure_mesh().x_label_formatter(&|dt|dt.format("%Y-%m-%d").to_string()).draw().unwrap();

    let _ = chart.draw_series(LineSeries::new((0..opens.len()).map(|i| (yahoo_datetimes[i], opens[i])), &BLUE)).unwrap().label("open");
    let _ = chart.draw_series(LineSeries::new((0..closes.len()).map(|i|(yahoo_datetimes[i], closes[i])),&GREEN)).unwrap().label("close");


    struct CustomRow {
        date: DateTime<FixedOffset>,
        value1: f64,
        value2: f64,
    }
    let x:Vec<CustomRow> = yahoo_datetimes.iter().zip(opens.iter()).zip(closes.iter()).map(|((date,&open),&close)|CustomRow{date:*date,value1:open,value2:close}).collect();

    let _ = chart.draw_series(
        x.iter().map(|x| {
            CandleStick::new(x.date,x.value1, x.value1, x.value2, x.value2, GREEN.filled(), RED, 15)
        }),);

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();


    let networth:Vec<f64> = closes.iter().zip(position.iter()).zip(account.iter()).map(|((&a,&b),&c)|a*b+c).collect();
    let min_nw = *networth.iter().min_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()-5000.0;
    let max_nw = *networth.iter().max_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()+5000.0;

    let mut chart_low = ChartBuilder::on(&lower)
        .margin(5)
        .caption("Net worth", ("sans-serif", 30).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len()-1], min_nw..max_nw).unwrap();

    chart_low.configure_mesh().x_label_formatter(&|dt|dt.format("%Y-%m-%d").to_string()).draw().unwrap();


    let _ = chart_low.draw_series(LineSeries::new((0..networth.len()).map(|i| (yahoo_datetimes[i], networth[i])), &BLUE)).unwrap().label("networth");

    chart_low.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    Ok(())
}