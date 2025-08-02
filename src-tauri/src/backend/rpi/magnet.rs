use rppal::gpio::{Level, OutputPin};

use crate::backend::MagnetBackend;

pub struct ElectroMagnet {
    magnet_pin: OutputPin,
}

impl ElectroMagnet {
    pub fn new(magnet_pin: OutputPin) -> Self {
        Self { magnet_pin }
    }
}

impl MagnetBackend for ElectroMagnet {
    async fn set(&mut self, on: bool) {
        self.magnet_pin
            .write(if on { Level::High } else { Level::Low });
    }
}
