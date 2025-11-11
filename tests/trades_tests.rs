use std::error::Error;
use std::sync::Arc;
use rs_backtester::datas::Data;

fn load_data() ->Result<Arc<Data>,Box<dyn Error>>{
    let filename = "test_data//NVDA.csv";
    Ok(Data::load_arc(filename, "NVDA")?)
}
#[test]
fn trades_tests() {}