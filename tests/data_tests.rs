use std::error::Error;
use rs_backtester::datas::Data;


#[test]
fn download_test()->Result<(), Box<dyn Error>>{
    let a:Data=Data::new_from_yahoo("NVDA","1d","5y")?;
    assert_eq!(a.ticker(),"NVDA");
    a.save("test_data/NVDA.csv")?;
    let b = Data::load("test_data/NVDA.csv","test")?;
    assert_eq!(b.ticker(),"test");
    Ok(())
}