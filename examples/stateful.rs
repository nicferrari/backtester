use rs_backtester::data::Data;
//use rs_backtester::orders::Order::BUY;
use rs_backtester::stateful::{
    broker, sma, sma_cross, Order, OrderStatus, OrderStatus::*, OrderType, Signal, SlippageMode,
    SlippageMode::*, State,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let quotes = Data::new_from_yahoo("PLTR", "1d", "6mo")?;
    let mut previous = Signal::FLAT;
    //let quotes2 = quotes.slice(30);
    for i in 0..2 {
        let quotes2 = quotes.slice(i, 20).unwrap();
        print!("\n{:?} - ", quotes2.datetime.last().unwrap());
        print!("{:?} ", sma(quotes2.clone(), 10).unwrap_or(-1.0));
        print!("{:?} ", sma(quotes2.clone(), 20).unwrap_or(-1.0));
        let mut current = sma_cross(quotes2.clone(), 10, 20);
        print!("{:?} ", current);
        let mut order = Order {
            order_status: OrderStatus::OPEN,
            order_type: OrderType::BUY,
        };
        print!(" - {:?} vs {:?} - ",current,previous);
        if current != previous {
            order = Order {
                order_status: OrderStatus::OPEN,
                order_type: OrderType::BUY,
            }
        };
        let state = State {
            account: 100000.,
            position: 0.,
            order,
            delay: 1,
        };
        let position = broker(quotes2.clone(), state, SlippageMode::NEXTCLOSE(1));
        print!("{:?}", position);
        previous = current;
    }
    quotes.save("stateful.csv")?;
    Ok(())
}
