use crate::backtester::Backtest;
use crate::broker::Status::Executed;
use crate::orders;
use crate::orders::Order::{BUY, SHORTSELL};
use charming::component::{Axis, DataZoom, DataZoomType, Grid, Title};
use charming::datatype::{CompositeValue, DataPoint};
use charming::element::{AreaStyle, AxisLabel, ItemStyle, Symbol, Tooltip, Trigger};
use charming::series::Scatter;
use charming::series::{Bar, Candlestick, Line};
use charming::{Chart, HtmlRenderer};
use chrono::{DateTime, FixedOffset};
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use plotters::style::full_palette::{GREEN_900, GREY, ORANGE};
use std::env;

///configuration used in charts
pub struct PlotConfig {
    pub display_indic: bool,
    pub display_networth: bool,
    pub display_marker_label: bool,
}
///default for PlotConfig
impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            display_indic: true,
            display_networth: false,
            display_marker_label: false,
        }
    }
}

///function used to plot data, indicators and equity
///modify Plot_Config to define different chart parameters or apply default
pub fn plot(
    backtest: &Backtest,
    config: PlotConfig,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let yahoo_datetimes = &backtest.strategy.data.datetime;
    let opens = &backtest.strategy.data.open;
    let highs = &backtest.strategy.data.high;
    let lows = &backtest.strategy.data.low;
    let closes = &backtest.strategy.data.close;

    let path = env::current_dir()?.into_os_string().into_string().unwrap() + "\\" + filename;
    let root = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();
    let _ = root.fill(&WHITE);

    let (upper, lower) = root.split_vertically(512);

    let min_lows = *lows
        .iter()
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
        * 0.995;
    let max_highs = *highs
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
        * 1.005;

    let mut chart: ChartContext<
        BitMapBackend,
        Cartesian2d<RangedDateTime<DateTime<FixedOffset>>, RangedCoordf64>,
    >;

    if config.display_networth {
        chart = ChartBuilder::on(&upper)
            .margin(5)
            .caption(
                "Chart ".to_owned() + &*backtest.strategy.data.ticker,
                ("sans-serif", 30).into_font(),
            )
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len() - 1],
                min_lows..max_highs,
            )
            .unwrap();
    } else {
        chart = ChartBuilder::on(&root)
            .margin(5)
            .caption(
                "Chart ".to_owned() + &*backtest.strategy.data.ticker,
                ("sans-serif", 30).into_font(),
            )
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len() - 1],
                min_lows..max_highs,
            )
            .unwrap();
    };

    chart
        .configure_mesh()
        .x_label_formatter(&|dt| dt.format("%Y-%m-%d").to_string())
        .draw()
        .unwrap();

    if config.display_indic {
        let Some(indicator) = backtest.strategy.clone().indicator else {
            todo!()
        };
        let colors = [CYAN, ORANGE];
        let colors_iter = colors.iter().cycle();
        for (nr, color) in indicator.iter().zip(colors_iter) {
            let color_clone = *color;
            let Some(index) = nr.iter().position(|&x| x != -1.0) else {
                todo!()
            };
            let _ = chart
                .draw_series(LineSeries::new(
                    (index..closes.len()).map(|i| (yahoo_datetimes[i], nr[i])),
                    color_clone,
                ))
                .unwrap()
                .label("indic")
                .legend(move |(x, y)| Circle::new((x, y), 5, color_clone.filled()));
        }
    }

    struct CustomRow {
        date: DateTime<FixedOffset>,
        value1: f64,
        value2: f64,
        value3: f64,
        value4: f64,
    }
    let x: Vec<CustomRow> = yahoo_datetimes
        .iter()
        .zip(opens.iter())
        .zip(highs.iter())
        .zip(lows.iter())
        .zip(closes.iter())
        .map(|((((date, &open), &high), &low), &close)| CustomRow {
            date: *date,
            value1: open,
            value2: high,
            value3: low,
            value4: close,
        })
        .collect();

    let _ = chart.draw_series(x.iter().map(|x| {
        CandleStick::new(
            x.date,
            x.value1,
            x.value2,
            x.value3,
            x.value4,
            GREEN.filled(),
            RED.filled(),
            500 / yahoo_datetimes.len() as u32,
        )
    }));

    //add marker and label
    if config.display_marker_label {
        let mut prev_order = orders::Order::NULL;
        for ((x, y), z) in yahoo_datetimes
            .iter()
            .zip(closes.iter())
            .zip(backtest.strategy.choices.iter())
        {
            if *z != prev_order {
                chart.draw_series(PointSeries::of_element(
                    vec![(*x, *y)],
                    5,    // Circle marker size
                    &RED, // Red color
                    &|c, _s, _st| {
                        EmptyElement::at(c)
                            + match z {
                                orders::Order::BUY => {
                                    Polygon::new([(0, 0), (6, 0), (3, -6)], GREEN_900)
                                }
                                orders::Order::SHORTSELL => {
                                    Polygon::new([(0, 0), (6, 0), (3, 6)], RED)
                                }
                                orders::Order::NULL => Polygon::new([(0, 0), (6, 0)], GREY),
                            }
                    },
                ))?;
                prev_order = *z;
            };
        }
    }

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(YELLOW.mix(0.8))
        .draw()
        .unwrap();

    if config.display_networth {
        //let networth: Vec<f64> = closes.iter().zip(backtest.broker.position.iter()).zip(backtest.broker.account.iter()).map(|((&a, &b), &c)| a * b as f64 + c).collect();
        let networth = backtest.broker.networth.clone();
        let min_nw = *networth
            .iter()
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap()
            - 5000.0;
        let max_nw = *networth
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap()
            + 5000.0;

        let mut chart_low = ChartBuilder::on(&lower)
            .margin(5)
            .caption("Net worth", ("sans-serif", 30).into_font())
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len() - 1],
                min_nw..max_nw,
            )
            .unwrap();

        chart_low
            .configure_mesh()
            .x_label_formatter(&|dt| dt.format("%Y-%m-%d").to_string())
            .draw()
            .unwrap();

        chart_low
            .draw_series(LineSeries::new(
                (0..networth.len()).map(|i| (yahoo_datetimes[i], networth[i])),
                &BLUE,
            ))
            .unwrap()
            .label("networth");

        // area fill (GREEN, RED)
        let gains: Vec<f64> = networth
            .clone()
            .iter()
            .map(|value| {
                if *value > networth[0] {
                    *value
                } else {
                    networth[0]
                }
            })
            .collect();
        let losses: Vec<f64> = networth
            .clone()
            .iter()
            .map(|value| {
                if *value < networth[0] {
                    *value
                } else {
                    networth[0]
                }
            })
            .collect();

        chart_low
            .draw_series(AreaSeries::new(
                (0..yahoo_datetimes.len()).map(|i| (yahoo_datetimes[i], gains[i])),
                networth[0],
                GREEN.mix(0.2),
            ))
            .unwrap();
        chart_low
            .draw_series(AreaSeries::new(
                (0..yahoo_datetimes.len()).map(|i| (yahoo_datetimes[i], losses[i])),
                networth[0],
                RED.mix(0.2),
            ))
            .unwrap();

        chart_low
            .configure_series_labels()
            .border_style(BLACK)
            .background_style(YELLOW.mix(0.8))
            .draw()
            .unwrap();
    }
    println!("Chart saved as = {}", path);
    Ok(())
}

fn interactive_chart(bt: Backtest) -> Chart {
    let min_value = *bt
        .strategy
        .data
        .low
        .iter()
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    let max_value = *bt
        .strategy
        .data
        .low
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    let candles = (0..bt.strategy.data.open.len())
        .map(|i| {
            vec![
                bt.strategy.data.open[i],
                bt.strategy.data.close[i],
                bt.strategy.data.low[i],
                bt.strategy.data.high[i],
            ]
        })
        .collect();
    let combined = bt
        .broker
        .status
        .iter()
        .zip(bt.strategy.choices.iter())
        .zip(bt.strategy.data.open.iter())
        .enumerate();
    let buys: Vec<(usize, f64)> = combined
        .clone()
        .filter_map(|(i, ((exec, ord), price))| {
            if *exec == Executed && *ord == BUY {
                Some((i, *price))
            } else {
                None
            }
        })
        .collect();
    let buys_points: Vec<DataPoint> = buys
        .iter()
        .map(|(i, price)| {
            DataPoint::Value(CompositeValue::Array(vec![
                CompositeValue::Number(charming::datatype::NumericValue::Float(*i as f64)),
                CompositeValue::Number(charming::datatype::NumericValue::Float(*price)),
            ]))
        })
        .collect();
    let sells: Vec<(usize, f64)> = combined
        .clone()
        .filter_map(|(i, ((exec, ord), price))| {
            if *exec == Executed && *ord == SHORTSELL {
                Some((i, *price))
            } else {
                None
            }
        })
        .collect();
    let sells_points: Vec<DataPoint> = sells
        .iter()
        .map(|(i, price)| {
            DataPoint::Value(CompositeValue::Array(vec![
                CompositeValue::Number(charming::datatype::NumericValue::Float(*i as f64)),
                CompositeValue::Number(charming::datatype::NumericValue::Float(*price)),
            ]))
        })
        .collect();

    let down_triangle = "path://M0,10 L-8,-6 L8,-6 Z";

    Chart::new()
        .title(
            Title::new()
                .text(bt.strategy.data.ticker.clone())
                .left("center"),
        )
        .data_zoom(
            DataZoom::new()
                .x_axis_index(vec![0, 1, 2])
                .type_(DataZoomType::Slider),
        )
        .grid(Grid::new().top("10%").height("50%"))
        .x_axis(
            Axis::new().grid_index(0).data(
                bt.strategy
                    .data
                    .datetime
                    .iter()
                    .map(|a| a.date_naive().to_string())
                    .collect(),
            ),
        )
        .y_axis(
            Axis::new()
                .grid_index(0)
                .min((min_value * 0.95) as i64)
                .max((max_value * 1.05) as i64)
                .axis_label(AxisLabel::new()),
        )
        .series(Candlestick::new().data(candles))
        .series(
            Scatter::new()
                .name("Buys")
                .data(buys_points)
                .symbol(Symbol::Triangle)
                .item_style(ItemStyle::new().color("black")),
        )
        .series(
            Scatter::new()
                .name("Sells")
                .data(sells_points)
                .symbol(Symbol::Custom(down_triangle.to_string()))
                .item_style(ItemStyle::new().color("black")),
        )
        .grid(Grid::new().top("65%").height("10%"))
        .x_axis(
            Axis::new().grid_index(1).data(
                bt.strategy
                    .data
                    .datetime
                    .iter()
                    .map(|a| a.date_naive().to_string())
                    .collect(),
            ),
        )
        .y_axis(Axis::new().grid_index(1))
        .series(
            Bar::new()
                .x_axis_index(1)
                .y_axis_index(1)
                .data(bt.strategy.data.volume.iter().map(|v| v.as_f64()).collect()),
        )
        .grid(Grid::new().top("80%").height("10%"))
        .x_axis(
            Axis::new().grid_index(2).data(
                bt.strategy
                    .data
                    .datetime
                    .iter()
                    .map(|a| a.to_string())
                    .collect(),
            ),
        )
        .y_axis(Axis::new().grid_index(2))
        .series(
            Line::new()
                .x_axis_index(2)
                .y_axis_index(2)
                .data(bt.broker.networth)
                .area_style(AreaStyle::new()),
        )
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
}
pub fn i_chart(bt: Backtest, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let chart = interactive_chart(bt);
    let mut renderer = HtmlRenderer::new("i_chart", 1000, 800);
    let path = env::current_dir()?.into_os_string().into_string().unwrap() + "\\" + filename;
    println!("Interactive charts saves as = {}", path);
    renderer.save(&chart, filename).unwrap();
    Ok(())
}
