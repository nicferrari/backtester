pub trait Sizer {
    fn position(&self, available: f64, price: f64, commission_rate: f64) -> f64;
}

pub struct AllInSizerWholeUnits;
pub struct AllInSizer;
pub struct FixedSizer {
    pub fixed_size: f64,
}
impl Sizer for AllInSizerWholeUnits {
    fn position(&self, available: f64, price: f64, commission_rate: f64) -> f64 {
        (available / price / (1. + commission_rate)).trunc()
    }
}
impl Sizer for AllInSizer {
    fn position(&self, available: f64, price: f64, commission_rate: f64) -> f64 {
        available / price / (1. + commission_rate)
    }
}
impl Sizer for FixedSizer {
    fn position(&self, available: f64, price: f64, commission_rate: f64) -> f64 {
        self.fixed_size
            .min(available / price / (1. + commission_rate))
    }
}
