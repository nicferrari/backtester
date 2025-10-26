/*
mod module_a;
mod module_b;

pub use module_a::use_config_a;
pub use module_b::use_config_b;*/

use once_cell::sync::Lazy;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Config {
    pub debug_mode: bool,
    pub commission_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            debug_mode: false,
            commission_rate: 0.05,
        }
    }
}

pub static CONFIG: Lazy<RwLock<Option<Config>>> = Lazy::new(|| RwLock::new(None));

pub fn set_config(cfg: Config) {
    let mut lock = CONFIG.write().unwrap();
    *lock = Some(cfg);
}

pub fn get_config() -> Config {
    CONFIG.read().unwrap().clone().unwrap_or_else(Config::default)
}

//use crate::{CONFIG, Config};

/// Applies a custom update to the global config.
/// Initializes with default config if not set.
pub fn update_config<F>(modifier: F)
    where
        F: FnOnce(&mut Config),
{
    let mut config_lock = CONFIG.write().unwrap();

    // Initialize with default if not set
    if config_lock.is_none() {
        *config_lock = Some(Config::default());
    }

    // Apply the modifier function
    if let Some(cfg) = config_lock.as_mut() {
        modifier(cfg);
    }
}
/*
update_config(|cfg| {
cfg.debug_mode = true;
});

update_config(|cfg| {
    *cfg = Config {
        debug_mode: false,
        commission_rate: 0.01,
    };
});
*/
