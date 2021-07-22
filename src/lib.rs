use std::fmt::Display;
use serde::Serialize;

pub mod gpio;
pub mod delay;
pub mod display_interface;

pub use gpio::*;
pub use delay::*;
pub use ::display_interface::*;

#[derive(Debug, Default, Serialize)]
pub struct Counter {
    pub iterations: usize,
    pub bytes: usize,
}

impl Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "iterations: {} bytes: {}", self.iterations, self.bytes)
    }
}
