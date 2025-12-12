use rs_backtester::backtester::Backtest;
use rs_backtester::data::Data;
use rs_backtester::strategies::{buy_n_hold, sma_strategy};
use std::error::Error;
use std::sync::Arc;

fn load_data() -> Result<Arc<Data>, Box<dyn Error>> {
    let filename = "test_data//NVDA.csv";
    Ok(Data::load(filename, "NVDA")?)
}
#[test]
fn buynhold_metrics_tests() {
    let data = load_data().unwrap();
    let buynhold = buy_n_hold(data.clone());
    let buynhold_bt = Backtest::new(buynhold, 100_000.);
    assert_eq!(buynhold_bt.metrics.bt_return.unwrap(), 260.30207539871964);
    //exposure time is calculated on # working days not on dates (e.g. 1255 total indices vs 1823 total days)
    assert_eq!(buynhold_bt.metrics.exposure_time.unwrap(), 1253. / 1255.);
    assert_eq!(buynhold_bt.metrics.trades_nr.unwrap(), 1);
    //max p&l, average and min p&l are all  calculated on execution time (open by default):by definition are different to p&l
    assert_eq!(buynhold_bt.metrics.max_pl, buynhold_bt.metrics.min_pl);
    assert_eq!(buynhold_bt.metrics.average_pl, buynhold_bt.metrics.max_pl);
    assert_eq!(buynhold_bt.metrics.win_rate.unwrap(), 1.);
    assert_eq!(buynhold_bt.metrics.avg_duration.unwrap(), 1821.); //1st order is executed in 3rd period (due to change in choice and execution time)
    assert_eq!(buynhold_bt.metrics.max_drawd.unwrap(), 0.6636127098371273);
    assert_eq!(
        buynhold_bt.metrics.sharpe.unwrap() * 252f64.sqrt(),
        1.0083070085617758
    ); //sharpe ratio needs to be annualized (and is on working days returns but annualized by days). metrics should already have the annualization?
}
#[test]
fn sma_metrics_tests() {
    let data = load_data().unwrap();
    let sma = sma_strategy(data, 10);
    let sma_bt = Backtest::new(sma, 100_000.);/*
    assert_eq!(sma_bt.metrics.bt_return.unwrap(), 170.91523018742043);
    assert_eq!(sma_bt.metrics.exposure_time.unwrap(), 1245f64 / 1255f64);
    assert_eq!(sma_bt.metrics.trades_nr.unwrap(), 144);
    assert_eq!(sma_bt.metrics.max_pl.unwrap(), 42.3980541595539);
    assert_eq!(sma_bt.metrics.min_pl.unwrap(), -17.225731871090343);
    assert_eq!(sma_bt.metrics.average_pl.unwrap(), 1.4476878752979434);
    assert_eq!(sma_bt.metrics.win_rate.unwrap(), 0.3958333333333333);
    assert_eq!(sma_bt.metrics.avg_duration.unwrap(), 12.527777777777779);
    assert_eq!(sma_bt.metrics.max_drawd.unwrap(), 0.4355942469089307);
    assert_eq!(
        sma_bt.metrics.sharpe.unwrap() * 252f64.sqrt(),
        0.7009220717545875
    ); //sharpe ratio considers rf=0*/
    assert_eq!(sma_bt.metrics.bt_return.unwrap(), 50.28445094325992);
    assert_eq!(sma_bt.metrics.exposure_time.unwrap(), 1245f64 / 1255f64);
    assert_eq!(sma_bt.metrics.trades_nr.unwrap(), 180);

    assert_eq!(sma_bt.metrics.max_pl.unwrap(), 42.3980541595539);
    assert_eq!(sma_bt.metrics.min_pl.unwrap(), -17.225731871090343);
    assert_eq!(sma_bt.metrics.average_pl.unwrap(), 1.4476878752979434);
    assert_eq!(sma_bt.metrics.win_rate.unwrap(), 0.3958333333333333);
    assert_eq!(sma_bt.metrics.avg_duration.unwrap(), 12.527777777777779);
    assert_eq!(sma_bt.metrics.max_drawd.unwrap(), 0.4355942469089307);
    assert_eq!(
        sma_bt.metrics.sharpe.unwrap() * 252f64.sqrt(),
        0.7009220717545875
    ); //sharpe ratio considers rf=0*/

}
