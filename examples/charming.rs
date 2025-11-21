use charming::{component::{Axis,DataZoom,DataZoomType}, df, series::Candlestick, Chart, HtmlRenderer};
use charming::component::Grid;
use charming::element::{AreaStyle, Tooltip, Trigger};
//use charming::series::Series::Bar;
use rs_backtester::backtester::Backtest;
use rs_backtester::data::Data;
use rs_backtester::strategies::sma_strategy;
use charming::series::{Bar, Line};

pub fn chart_new(bt:Backtest)->Chart{
    //let candles = bt.strategy.data.open.iter().zip(bt.strategy.data.high.iter().zip(bt.strategy.data.high.iter().zip(bt.strategy.data.low.clone()).collect())).collect()
    let candles = (0..bt.strategy.data.open.len()).map(|i|vec![bt.strategy.data.open[i],bt.strategy.data.close[i],bt.strategy.data.low[i],bt.strategy.data.high[i]]).collect();
    Chart::new().data_zoom(DataZoom::new().type_(DataZoomType::Slider)).x_axis(Axis::new().data(bt.strategy.data.datetime.iter().map(|a|a.to_string()).collect()))
        .y_axis(Axis::new()).series(Candlestick::new().data(candles)).tooltip(Tooltip::new().trigger(Trigger::Axis))
}

pub fn chart_with_volume(bt:Backtest)->Chart{
    let candles = (0..bt.strategy.data.open.len()).map(|i|vec![bt.strategy.data.open[i],bt.strategy.data.close[i],bt.strategy.data.low[i],bt.strategy.data.high[i]]).collect();

    Chart::new().data_zoom(DataZoom::new().x_axis_index(vec![0,1,2]).type_(DataZoomType::Slider)).grid(Grid::new().top("10%").height("50%"))
        .x_axis(Axis::new().grid_index(0).data(bt.strategy.data.datetime.iter().map(|a|a.to_string()).collect()))
        .y_axis(Axis::new().grid_index(0).min(100.)).series(Candlestick::new().data(candles))
        .grid(Grid::new().top("65%").height("20%"))
        .x_axis(Axis::new().grid_index(1).data(bt.strategy.data.datetime.iter().map(|a|a.to_string()).collect()))
        .y_axis(Axis::new().grid_index(1))
        .series(Bar::new().x_axis_index(1).y_axis_index(1).data(bt.strategy.data.volume.iter().map(|v|v.to_string()).collect()))
        .grid(Grid::new().top("90%").height("10%"))
        .x_axis(Axis::new().grid_index(2).data(bt.strategy.data.datetime.iter().map(|a|a.to_string()).collect()))
        .y_axis(Axis::new().grid_index(2))
        .series(Line::new().x_axis_index(2).y_axis_index(2).data(bt.broker.networth.iter().map(|n|n.to_string()).collect()).area_style(AreaStyle::new()))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
}

pub fn main()-> Result<(), Box<dyn std::error::Error>> {
    //let chart = chart();
    let quotes = Data::new_from_yahoo("NVDA","1d","6mo")?;
    let sma = sma_strategy(quotes,10);
    let bt = Backtest::new(sma,100_000.);
    //let chart = chart_new(bt);
    let chart = chart_with_volume(bt);
    let mut renderer = HtmlRenderer::new("my charts", 1000, 800);
    // Render the chart as HTML string.
    //let html_str = renderer.render(&chart).unwrap();
    // Save the chart as HTML file.
    //println!("{}", html_str);
    renderer.save(&chart, "chart.html").unwrap();
    Ok(())
}