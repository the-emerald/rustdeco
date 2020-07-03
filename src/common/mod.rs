use time::Duration;
use num_traits::cast::FromPrimitive;
use std::isize;
#[allow(unused_imports)] // Seriously, my IDE complains if I don't put that there.
use num_traits::real::Real;

pub const DEFAULT_DESCENT_RATE: isize = 30;
pub const DEFAULT_ASCENT_RATE: isize = -18;

pub const DENSITY_FRESHWATER: f64 = 997.0;
pub const DENSITY_SALTWATER: f64 = 1023.6;

pub mod dive_segment;
pub mod gas;
pub mod tank;

pub fn bar_mtr(bar: f64, metres_per_bar: f64) -> f64 {
    (bar-1.0) * metres_per_bar
}

pub fn mtr_bar(mtr: f64, metres_per_bar: f64) -> f64 {
    (mtr/metres_per_bar) + 1.0
}

pub fn time_taken(rate: isize, depth_1: usize, depth_2: usize) -> Duration {
    let delta_depth = ((depth_1 as isize) - (depth_2 as isize)).abs();
    let rate_seconds = rate.abs() as f64 / 60.0;
    Duration::seconds(
        i64::from_f64(delta_depth as f64 / rate_seconds).expect("overflow in time taken")
    )
}