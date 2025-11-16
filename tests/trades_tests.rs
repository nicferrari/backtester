use rs_backtester::data::Data;
use std::error::Error;
use std::sync::Arc;

fn load_data() -> Result<Arc<Data>, Box<dyn Error>> {
    let filename = "test_data//NVDA.csv";
    Ok(Data::load(filename, "NVDA")?)
}
#[test]
fn trades_tests() {
    let _quotes = load_data().unwrap();
}
