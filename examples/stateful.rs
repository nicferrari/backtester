use rs_backtester::data::Data;
use rs_backtester::orders::Order::BUY;
use rs_backtester::stateful::{
    broker, sma, sma_cross, OrderStatus::*, SlippageMode, SlippageMode::*, State,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let quotes = Data::new_from_yahoo("PLTR", "1d", "6mo")?;
    //let quotes2 = quotes.slice(30);
    let quotes2 = quotes.slice(0, 19).unwrap();
    print!("{:?} - ", quotes2.datetime.last().unwrap());
    print!("{:?} ", sma(quotes2.clone(), 10).unwrap_or(-1.0));
    print!("{:?} ", sma(quotes2.clone(), 20).unwrap_or(-1.0));
    print!("{:?} ", sma_cross(quotes2.clone(), 10, 20));
    let state = State {
        account: 100000.,
        position: 0.,
        order_status: OPEN,
        delay: 1,
    };
    let position = broker(
        quotes2.clone(),
        sma_cross(quotes2.clone(), 10, 20),
        state,
        SlippageMode::NEXTCLOSE(1),
    );
    print!("{:?}", position);
    //quotes2.save("stateful.csv")?;
    Ok(())
}
