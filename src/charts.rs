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