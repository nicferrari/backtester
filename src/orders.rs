use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum Order{
    BUY,
    SHORTSELL,
    NULL,
}
impl Order{
    pub fn to_string(&self)->&str{
        match self{
            Order::BUY=>"buy",
            Order::NULL=>"null",
            Order::SHORTSELL=>"short",
        }
    }
}