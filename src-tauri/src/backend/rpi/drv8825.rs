use std::time::Duration;

use rppal::gpio::{Level, OutputPin};
use tokio::time::sleep;

use crate::backend::{MotorBackend, MotorDirection, MotorRotation, rpi::busy_wait_us};

const STEP_BACK_AMOUNT: f32 = 0.1;

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

    async fn step(
        &mut self,
        dir: MotorDirection,
        steps: usize,
        mut should_step_back_and_stop: impl FnMut() -> bool,
    ) -> usize {
        self.dir_pin.write(dir.into());
        sleep(Duration::from_millis(50)).await;

        for i in 0..steps {
            self.step_pin.set_high();
            busy_wait_us(5000).await;
            self.step_pin.set_low();
            busy_wait_us(5000).await;

            if should_step_back_and_stop() {
                let step_back_steps =
                    ((STEP_BACK_AMOUNT * self.steps_per_turn as f32).round() as usize).min(i);

                for _ in 0..step_back_steps {
                    self.step_pin.set_high();
                    busy_wait_us(5000).await;
                    self.step_pin.set_low();
                    busy_wait_us(5000).await;
                }

                return i - step_back_steps;
            }
        }

        steps
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
        direction: MotorDirection,
        rotation: crate::backend::MotorRotation,
        should_step_back_and_stop: impl FnMut() -> bool,
    ) -> crate::backend::MotorRotation {
        let steps_need = (rotation.turns * self.steps_per_turn as f32).round() as usize;
        let steps_taken = self
            .step(direction, steps_need, should_step_back_and_stop)
            .await;

        let steps_missed = steps_need - steps_taken;
        MotorRotation {
            turns: rotation.turns - (steps_missed * self.steps_per_turn) as f32,
        }
    }
}
