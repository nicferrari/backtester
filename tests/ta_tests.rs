use std::error::Error;
use std::sync::Arc;
use rs_backtester::backtester_new::Backtest_arc;
use rs_backtester::datas::Data;
use rs_backtester::orders::Order::{BUY, NULL};
use rs_backtester::strategies::sma_arc;

fn load_data() ->Result<Arc<Data>,Box<dyn Error>>{
    let filename = "test_data//NVDA.csv";
    Ok(Data::load_arc(filename, "NVDA")?)
}
#[test]
fn ta_tests() ->Result<(), Box<dyn std::error::Error>>{
    let quotes = load_data().unwrap();
    let sma = sma_arc(quotes,10);
    let sma_bt = Backtest_arc::new(sma,100_000.);
    assert_eq!(sma_bt.strategy.indicator.unwrap()[0][9],13.54124994277954);
    assert_eq!(sma_bt.strategy.choices[8],NULL);
    assert_eq!(sma_bt.strategy.choices[9],BUY);
    Ok(())
}