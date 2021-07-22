use core::convert::Infallible;
use std::fmt::Display;

use embedded_hal::digital::v2::OutputPin;

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct PinCounter {
    pub changes: usize,
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
