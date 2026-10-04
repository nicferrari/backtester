use crate::data::Data;
//use crate::orders::Order;
use std::sync::Arc;
//use crate::stateful::OrderStatus::EXECUTED;

pub struct State {
    pub account: f64,
    pub position: f64,
    pub order: Order,
    pub delay: u32,
}

#[derive(PartialEq, Debug)]
pub enum OrderStatus {
    OPEN,
    EXECUTED,
    NONE,
}
#[derive(PartialEq)]
pub enum SlippageMode {
    NEXTCLOSE(u32),
}

#[derive(Debug)]
pub enum OrderType {
    BUY,
    SELL,
    NONE,
}

#[derive(Debug)]
pub struct Order {
    pub order_type: OrderType,
    pub order_status: OrderStatus,
}

#[derive(PartialEq, Debug)]
pub enum Signal {
    LONG,
    SHORT,
    FLAT,
}

pub fn sma(data: Arc<Data>, period: usize) -> Option<f64> {
    if period == 0 || data.close.len() < period {
        return None;
    }
    let window = &data.close[data.close.len() - period..];
    Some(window.iter().sum::<f64>() / period as f64)
}

pub fn sma_cross(data: Arc<Data>, period_short: usize, period_long: usize) -> Signal {
    if ((sma(data.clone(), period_short)) == None) || ((sma(data.clone(), period_long)) == None) {
        Signal::FLAT
    } else if sma(data.clone(), period_short) >= sma(data.clone(), period_long) {
        Signal::LONG
    } else {
        Signal::SHORT
    }
}

pub fn broker(data: Arc<Data>, mut state: State, slippage_mode: SlippageMode) -> f64 {
    if state.order.order_status == OrderStatus::OPEN
        && SlippageMode::NEXTCLOSE(state.delay) == slippage_mode
    {
        match state.order.order_type {
            OrderType::BUY => {
                state.order.order_status = OrderStatus::EXECUTED;
                state.account / data.open.last().unwrap()
            }
            OrderType::SELL => 0.,
            OrderType::NONE => 0.,
        }
    } else {
        state.position
    }
}
