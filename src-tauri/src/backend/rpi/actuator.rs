use std::time::Duration;

use rppal::gpio::OutputPin;
use tokio::time::sleep;

use crate::backend::ActuatorBackend;

pub struct LinearActuator {
    linear_forward: OutputPin,
    linear_backward: OutputPin,
}

impl LinearActuator {
    pub fn new(linear_forward: OutputPin, linear_backward: OutputPin) -> Self {
        Self {
            linear_backward,
            linear_forward,
        }
    }
}

impl ActuatorBackend for LinearActuator {
    async fn contract(&mut self) {
        self.linear_backward.set_low();

        self.linear_forward.set_high();
        sleep(Duration::from_secs(5)).await;
        self.linear_forward.set_low();
    }

    async fn extend(&mut self) {
        self.linear_forward.set_low();

        self.linear_backward.set_high();
        sleep(Duration::from_secs(5)).await;
        self.linear_backward.set_low();
    }
}
