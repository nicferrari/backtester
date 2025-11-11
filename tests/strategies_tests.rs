use rs_backtester::datas::Data;
use std::error::Error;
use rs_backtester::backtester_new::Backtest_arc;
use rs_backtester::strategies::{buy_n_hold_arc, simple_sma};
use std::sync::Arc;

fn load_data()->Result<Arc<Data>,Box<dyn Error>>{
    let filename = "test_data//NVDA.csv";
    Ok(Data::load_arc(filename, "NVDA")?)
}
#[test]
fn buynhold_metrics_tests(){
    let data = load_data().unwrap();
    let buynhold = buy_n_hold_arc(data.clone());
    let buynhold_bt = Backtest_arc::new(buynhold,100_000.);
    assert_eq!(buynhold_bt.metrics.avg_duration.unwrap(),1821.);
    // exposure time is calculated on indices of data not on dates (e.g. 1255 total indices vs 1823 total days)
    //assert_eq!(buynhold_bt.metrics.exposure_time.unwrap(),1821./1823.);
    //max p&l, average and min p&l are all wrongly calculated on open
    //assert_eq!(buynhold_bt.metrics.bt_return.unwrap(),260.30207539871964);
    //assert_eq!(buynhold_bt.metrics.bt_return.unwrap(),(data.close.last().unwrap()/data.open[2]).ln());//small difference due to allin with int unit
}

