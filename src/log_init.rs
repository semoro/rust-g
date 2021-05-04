

#[cfg(feature = "feature-log-panics")]
extern crate log;

#[cfg(feature = "feature-log-panics")]
use log::LevelFilter;

#[cfg(feature = "feature-log-panics")]
pub fn log_init() {
    simple_logging::log_to_file("data/logs/rust_g.log", LevelFilter::Info).unwrap();

    log_panics::init();

    log::info!("Rust G initialized, panic handler setup");
}