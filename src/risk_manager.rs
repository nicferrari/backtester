use crate::config::get_config;
use std::fmt::Debug;

pub trait Sizer: Send + Sync + Debug {
    fn position(&self, available: f64, price: f64) -> f64;
    fn box_clone(&self) -> Box<dyn Sizer>;
}

impl Clone for Box<dyn Sizer> {
    fn clone(&self) -> Box<dyn Sizer> {
        self.box_clone()
    }
}
#[derive(Clone, Debug)]
pub struct AllInSizerWholeUnits;
#[derive(Clone, Debug)]
pub struct AllInSizer;
#[derive(Clone, Debug)]
pub struct FixedSizer {
    pub fixed_size: f64,
}
#[derive(Clone, Debug)]
pub struct FixedFractionSizer {
    pub fixed_fraction: f64,
}
impl Sizer for AllInSizerWholeUnits {
    fn position(&self, available: f64, price: f64) -> f64 {
        let cfg = get_config();
        (available / price / (1. + cfg.commission_rate)).trunc()
    }
    fn box_clone(&self) -> Box<dyn Sizer> {
        Box::new(self.clone())
    }
}
impl Sizer for AllInSizer {
    fn position(&self, available: f64, price: f64) -> f64 {
        let cfg = get_config();
        available / price / (1. + cfg.commission_rate)
    }
    fn box_clone(&self) -> Box<dyn Sizer> {
        Box::new(self.clone())
    }
}
impl Sizer for FixedSizer {
    fn position(&self, available: f64, price: f64) -> f64 {
        let cfg = get_config();
        self.fixed_size
            .min(available / price / (1. + cfg.commission_rate))
    }
    fn box_clone(&self) -> Box<dyn Sizer> {
        Box::new(self.clone())
    }
}
impl Sizer for FixedFractionSizer {
    fn position(&self, available: f64, price: f64) -> f64 {
        let cfg = get_config();
        self.fixed_fraction * available / price / (1. + cfg.commission_rate)
    }
    fn box_clone(&self) -> Box<dyn Sizer> {
        Box::new(self.clone())
    }
}
