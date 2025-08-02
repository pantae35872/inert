use std::time::Duration;

use rppal::gpio::{Level, OutputPin};
use tokio::time::sleep;

use crate::backend::{rpi::busy_wait_us, MotorBackend, MotorDirection};

pub struct Drv8825Motor {
    step_pin: OutputPin,
    dir_pin: OutputPin,
    steps_per_turn: usize,
}

impl Drv8825Motor {
    pub fn new(step_pin: OutputPin, dir_pin: OutputPin) -> Self {
        Self {
            step_pin,
            dir_pin,
            steps_per_turn: 200,
        } // Steps per turn is 200 with all the M
          // control pin low
    }

    async fn step(&mut self, dir: MotorDirection, steps: usize) {
        self.dir_pin.write(dir.into());
        sleep(Duration::from_millis(50)).await;

        for _ in 0..steps {
            self.step_pin.set_high();
            busy_wait_us(5000).await;
            self.step_pin.set_low();
            busy_wait_us(5000).await;
        }
    }
}

impl From<MotorDirection> for Level {
    fn from(value: MotorDirection) -> Self {
        match value {
            MotorDirection::Clockwise => Self::Low,
            MotorDirection::AntiClockwise => Self::High,
        }
    }
}

impl MotorBackend for Drv8825Motor {
    async fn rotate(
        &mut self,
        direction: crate::backend::MotorDirection,
        rotation: crate::backend::MotorRotation,
    ) {
        let steps = (rotation.turns * self.steps_per_turn as f32).round() as usize;
        self.step(direction, steps).await;
    }
}
