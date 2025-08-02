use rppal::gpio::{InputPin, Level};

use crate::backend::LimitSwitchBackend;

pub struct LimitSwitch {
    limit_pin_in: InputPin,
}

impl LimitSwitch {
    pub fn new(limit_pin_in: InputPin) -> Self {
        Self { limit_pin_in }
    }
}

impl LimitSwitchBackend for LimitSwitch {
    fn is_pressed(&mut self) -> bool {
        matches!(self.limit_pin_in.read(), Level::High)
    }
}
