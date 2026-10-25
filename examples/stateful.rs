use rs_backtester::data::Data;
//use rs_backtester::orders::Order::BUY;
use rs_backtester::stateful::{
    broker, sma, sma_cross, Order, OrderStatus, OrderStatus::*, OrderType, Signal, SlippageMode,
    State,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //loading data and initializing
    let quotes = Data::new_from_yahoo("PLTR", "1d", "6mo")?;
    let mut previous = Signal::FLAT;
    let order = Order {
        order_status: OrderStatus::NONE,
        order_type: OrderType::NONE,
    };
    let mut state = State {
        account: 100000.,
        position: 0.,
        order,
        delay: 0,
    };


    for i in 0..10 {
        let quotes2 = quotes.slice(i, 50).unwrap();
        print!("\n{:?} - ", quotes2.datetime.last().unwrap());
        print!("{:?} ", sma(quotes2.clone(), 10).unwrap_or(-1.0));
        print!("{:?} ", sma(quotes2.clone(), 20).unwrap_or(-1.0));
        let current = sma_cross(quotes2.clone(), 10, 20);
        print!(" - current = {:?} vs previous = {:?} - ",current,previous);
        if current != previous {
            match current {
                Signal::LONG=>{state.order.order_type=OrderType::BUY;state.delay=0}
                Signal::SHORT=>{state.order.order_type=OrderType::SELL;state.delay=0}
                Signal::FLAT=>{state.order.order_type=OrderType::NONE;state.delay=0}
            };
            state.order.order_status=OPEN;
        };
        print!(" {:?} {:?} ",state.order.order_type,state.order.order_status);
        broker(quotes2.clone(), &mut state, SlippageMode::NEXTCLOSE(1));
        print!("pos = {:?}, acc = {:?}", state.position, state.account);
        previous = current;
    }
    quotes.save("stateful.csv")?;
    Ok(())
}
