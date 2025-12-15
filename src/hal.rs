//! Miscellaneous `embedded-hal` trait implementations.
//!
//! The `hal` module consists of a collection of `embedded-hal` trait
//! implementations for traits that aren't tied to a specific peripheral.
//!
//! This module is only included when the `hal` flag is enabled.

use std::time::Duration;

/// Implements the `embedded-hal` `DelayMs` and `DelayNs` traits.
#[derive(Debug, Default)]
pub struct Delay;

impl Delay {
    /// Constructs a new `Delay`.
    pub fn new() -> Delay {
        Delay {}
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::delay::DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        spin_sleep::sleep(Duration::from_nanos(ns.into()));
    }

    fn delay_us(&mut self, us: u32) {
        spin_sleep::sleep(Duration::from_micros(us.into()));
    }

    fn delay_ms(&mut self, ms: u32) {
        spin_sleep::sleep(Duration::from_millis(ms.into()));
    }
}

/// Newtype wrapper for `f64`. Converts into `Duration`.
pub struct Hertz(pub f64);

const MICROS_PER_SEC: f64 = 1_000_000.0;

impl From<Hertz> for Duration {
    fn from(item: Hertz) -> Self {
        if item.0 > 0.0 && item.0.is_finite() {
            Duration::from_micros(((1.0 / item.0) * MICROS_PER_SEC) as u64)
        } else {
            Duration::default()
        }
    }
}

