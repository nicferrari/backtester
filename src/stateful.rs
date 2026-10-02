use crate::data::Data;
use crate::orders::Order;
use std::sync::Arc;

pub fn sma(data: Arc<Data>, period: usize) -> Option<f64> {
    if period == 0 || data.close.len() < period {
        return None;
    }
    let window = &data.close[data.close.len() - period..];
    Some(window.iter().sum::<f64>() / period as f64)
}

pub fn sma_cross(data: Arc<Data>, period_short: usize, period_long: usize) -> Order {
    if ((sma(data.clone(), period_short)) == None) || ((sma(data.clone(), period_long)) == None) {
        Order::NULL
    } else if sma(data.clone(), period_short) >= sma(data.clone(), period_long) {
        Order::BUY
    } else {
        Order::SHORTSELL
    }
}

pub struct State {
    pub account: f64,
    pub position: f64,
    pub order_status: OrderStatus,
    pub delay: u32,
}

#[derive(PartialEq)]
pub enum OrderStatus {
    OPEN,
    EXECUTED,
    NONE,
}
#[derive(PartialEq)]
pub enum SlippageMode {
    NEXTCLOSE(u32),
}

pub fn broker(data: Arc<Data>, order: Order, state: State, slippage_mode: SlippageMode) -> f64 {
    if state.order_status == OrderStatus::OPEN
        && SlippageMode::NEXTCLOSE(state.delay) == slippage_mode
    {
        match order {
            Order::BUY => state.account / data.open.last().unwrap(),
            Order::SHORTSELL => 0.,
            Order::NULL => 0.,
        }
    } else {
        state.position
    }
}
