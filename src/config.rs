use crate::broker::Execution;
use crate::broker::Execution::AtOpen;
use crate::risk_manager::{AllInSizerWholeUnits, Sizer};
use once_cell::sync::Lazy;
use std::sync::RwLock;

///global configuration
///
/// - commission_rate = fees as a percentage. applied on both long and short trades
/// - execution_time = currently allows only AtOpen(n) -> execution of orders will be performed at next n-th open
/// - sizer = one of the Sizer
#[derive(Debug, Clone)]
pub struct Config {
    pub commission_rate: f64,
    pub execution_time: Execution,
    pub sizer: Box<dyn Sizer>,
}

///default configuration
impl Default for Config {
    fn default() -> Self {
        Config {
            commission_rate: 0.,
            execution_time: AtOpen(1),
            sizer: Box::new(AllInSizerWholeUnits),
        }
    }
}

pub static CONFIG: Lazy<RwLock<Option<Config>>> = Lazy::new(|| RwLock::new(None));

pub fn set_config(cfg: Config) {
    let mut lock = CONFIG.write().unwrap();
    *lock = Some(cfg);
}

///Get last updated configuration
///
/// Note: in case you change it you need to recall it with get_config() if you need to use last update not initial value
///
/// ```no_run
/// use rs_backtester::config::{get_config, update_config};
/// use rs_backtester::broker::Execution::AtOpen;
///
/// let cfg = get_config();
/// println!("Initial execution time {:?}",cfg.execution_time);
/// update_config(|cfg|{cfg.execution_time=AtOpen(3)});
/// let cfg = get_config();
/// println!("Execution time modified to {:?}",cfg.execution_time);
/// ```
pub fn get_config() -> Config {
    //CONFIG.read().unwrap().clone().unwrap_or_else(Config::default)
    CONFIG.read().unwrap().clone().unwrap_or_default()
}

/// Applies a custom update to the global config.
/// Initializes with default config if not set.
///
/// To modify, call with:
/// ```
/// use rs_backtester::config::update_config;
///
/// update_config(|cfg| {
///     cfg.commission_rate = 0.01;});
///```
/// or
///```
/// use rs_backtester::config::{Config, update_config};
/// use rs_backtester::broker::Execution::AtOpen;
/// use rs_backtester::risk_manager::AllInSizerWholeUnits;
///
/// update_config(|cfg| {
///     *cfg = Config {
///         execution_time: AtOpen(2),
///         commission_rate: 0.01,
///         sizer:Box::new(AllInSizerWholeUnits),
///     };
/// });
/// ```
///
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
