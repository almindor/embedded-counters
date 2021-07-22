use core::convert::Infallible;
use std::fmt::Display;

use embedded_hal::blocking::delay::*;
use embedded_hal::digital::v2::OutputPin;

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct PinCounter {
    pub changes: usize,
}

#[derive(Debug, Default, Serialize)]
pub struct DelayCounter {
    pub delays: u64,
}

impl DelayUs<u32> for DelayCounter {
    fn delay_us(&mut self, delay: u32) {
        self.delays += u64::from(delay);
    }
}

impl DelayMs<u32> for DelayCounter {
    fn delay_ms(&mut self, delay: u32) {
        self.delays += u64::from(delay) * 1000u64;
    }
}

impl Display for DelayCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Delays: {}us", self.delays)
    }
}

impl OutputPin for PinCounter {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.changes += 1;

        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.changes += 1;

        Ok(())
    }
}

impl Display for PinCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Changes: {}", self.changes)
    }
}
