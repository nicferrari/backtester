use serde::Serialize;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, PartialOrd)]
pub enum Order {
    BUY,
    SHORTSELL,
    NULL,
}
impl Order {
    /*
    pub fn to_string(&self) -> String {
        match self {
            Order::BUY => "buy".to_string(),
            Order::NULL => "".to_string(),
            Order::SHORTSELL => "short".to_string(),
        }
    }*/
    pub fn sign(&self) -> i8 {
        match self {
            Order::BUY => 1,
            Order::SHORTSELL => -1,
            Order::NULL => 0,
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Order::BUY => "buy",
            Order::NULL => "",
            Order::SHORTSELL => "short",
        };
        write!(f, "{}", s)
    }
}
