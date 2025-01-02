use std::error::Error;
use rs_backtester::datas::Data;


#[test]
fn download_test()->Result<(), Box<dyn Error>>{
    let a:Data=Data::new_from_yahoo("AAPL","1d","1mo")?;
    assert_eq!(a.ticker(),"AAPL");
    a.save("savedata.csv")?;
    let b = Data::load("savedata.csv","test")?;
    assert_eq!(b.ticker(),"test");
    Ok(())
}