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
    pub fn sign(&self)->i8{
        match self {
            Order::BUY=>1,
            Order::SHORTSELL=>-1,
            Order::NULL=>0,
        }
    }
}