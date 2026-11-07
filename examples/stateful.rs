use rs_backtester::data::Data;
//use rs_backtester::orders::Order::BUY;
use rs_backtester::stateful::{broker, sma, sma_cross, Order, OrderStatus, OrderStatus::*, OrderType, Signal, SlippageMode, State, ConditionalOrderStatus, Conditional};
use std::error::Error;
use rs_backtester::stateful::ConditionalType::STOPLOSS;

fn main() -> Result<(), Box<dyn Error>> {
    //loading data and initializing
    let sl_level = 0.1;
    let quotes = Data::new_from_yahoo("PLTR", "1d", "6mo")?;
    let mut previous = Signal::FLAT;
    let order = Order {
        order_status: OrderStatus::NONE,
        order_type: OrderType::NONE,
    };
    let sl = Conditional {
        cond_type:STOPLOSS,
        level: -1.,
        status: ConditionalOrderStatus::OPEN,
    };
    let mut state = State {
        account: 100000.,
        position: 0.,
        order,
        delay: 0,
        conditional: sl,
        cooldown: 0,
    };


    for i in 0..quotes.datetime.len()-20+1 {
        let quotes2 = quotes.slice(i, 20).unwrap();
        print!("\n{:?} - ", quotes2.datetime.last().unwrap());
        print!("{:?} ", sma(quotes2.clone(), 10).unwrap_or(-1.0));
        print!("{:?} ", sma(quotes2.clone(), 20).unwrap_or(-1.0));
        let current = sma_cross(quotes2.clone(), 10, 20);
        print!(" - current = {:?} vs previous = {:?} - ",current,previous);

        if state.cooldown==0{
            match state.order.order_type {
                OrderType::BUY=>{
                    if quotes2.low.last().unwrap()<=&state.conditional.level{
                        print!("SL triggered");
                        state.order.order_type=OrderType::NONE;state.delay=1;state.cooldown=5;
                    }
                }
                OrderType::SELL=>{
                    if quotes2.high.last().unwrap()>=&state.conditional.level{
                        print!("SL triggered");
                        state.order.order_type=OrderType::NONE;state.delay=1;state.cooldown=5;
                    }
                }
                OrderType::NONE=>{}
            };
        }

        if (current != previous) & (state.cooldown==0){
            if state.order.order_status==OPEN && SlippageMode::NEXTCLOSE(state.delay) == SlippageMode::NEXTCLOSE(1){
                print!(" ### previous order was cancelled ### ")
            }
            match current {
                Signal::LONG=>{state.order.order_type=OrderType::BUY;state.delay=0}
                Signal::SHORT=>{state.order.order_type=OrderType::SELL;state.delay=0}
                Signal::FLAT=>{state.order.order_type=OrderType::NONE;state.delay=0}
            };
            state.order.order_status=OPEN;
        };

        broker(quotes2.clone(), &mut state, SlippageMode::NEXTCLOSE(1), sl_level);
        print!(" {:?} {:?} ",state.order.order_type,state.order.order_status);
        print!("pos = {:?}, acc = {:?}", state.position, state.account);
        print!(", {:?}={:?} @ {:?}",state.conditional.cond_type,state.conditional.status,state.conditional.level);
        previous = current;
    }
    println!("\nReturn = {:?}",(state.position*quotes.close[quotes.datetime.len()-1]+state.account)/100_000.-1.);
    quotes.save("stateful.csv")?;
    Ok(())
}
