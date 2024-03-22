use backtester::datas::download_data;
use backtester::Result;
use backtester::datas::Data;


#[test]
fn download_test()->Result<()>{
    let quotes = download_data("AAPL","1d","1mo")?;
    //assert_eq!(quotes,1);
    let a:Data=Data::new_from_yahoo("AAPL")?;
    assert_eq!(a.ticker(),"AAPL");
    let quotes = download_data(a.ticker(),"1d","1mo")?;
    a.save()?;
    let b = Data::load("savedata.csv","test")?;
    assert_eq!(b.ticker(),"test");
    Ok(())
}