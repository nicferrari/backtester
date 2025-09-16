use crate::datas::Data;
use crate::orders::Order;

pub fn sma(data: Data, period: usize) ->Option<f64>{
    if data.close.len()<period{return None}
    let window = &data.close[data.close.len() - period..];
    Some(window.iter().sum::<f64>() / period as f64)
}

pub fn sma_cross(data: Data, period_short:usize, period_long:usize)->Order{
    if sma(data.clone(), period_short)>=sma(data, period_long){Order::BUY}
    else {Order::SHORTSELL}
}

pub struct State{
    pub account: f64,
    pub position: i64,
}