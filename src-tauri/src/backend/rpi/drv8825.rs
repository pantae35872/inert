use std::time::{Duration, Instant};

use rppal::gpio::{Level, OutputPin};
use tokio::time::{sleep, sleep_until};

use crate::backend::{MotorBackend, MotorDirection, MotorRotation, rpi::busy_wait_us};

const STEP_BACK_AMOUNT: f32 = 0.25;
const MOTOR_STEP_TIME: u64 = 5;
const MOTOR_WAIT_TIME: u64 = 100;

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
            steps_per_turn: 3200,
        } // Steps per turn is 3200 with M2 high and other 
        // M pin low
    }

    #[inline(always)]
    fn step_one(&mut self) -> Instant {
        self.step_pin.set_high();
        busy_wait_us(MOTOR_STEP_TIME);
        self.step_pin.set_low();
        Instant::now()
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
            let wait = self.step_one();

            if should_step_back_and_stop() {
                let step_back_steps =
                    ((STEP_BACK_AMOUNT * self.steps_per_turn as f32).round() as usize).min(i);

                self.dir_pin.write((!dir).into());
                sleep(Duration::from_millis(50)).await;

                for _ in 0..step_back_steps {
                    let wait = self.step_one();

                    motor_delay_async(wait).await;
                }

                return i - step_back_steps;
            }

            motor_delay_async(wait).await;
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
async fn motor_delay_async(wait: Instant) {
    let target = wait + Duration::from_micros(MOTOR_WAIT_TIME);
    let now = Instant::now();

    if target > now {
        let delay = target - now;
        sleep_until(tokio::time::Instant::now() + delay).await;
    }
}

impl MotorBackend for Drv8825Motor {
    async fn rotate(
        &mut self,
        direction: MotorDirection,
        rotation: crate::backend::MotorRotation,
        should_step_back_and_stop: impl FnMut() -> bool,
    ) -> crate::backend::MotorRotation {
        #[cfg(feature = "logging")]
        {
            println!(
                "RpiMotor Dir: {}, Step: {};  turned, Direction: {direction:?}, Rotation: {rotation:?}",
                self.dir_pin.pin(),
                self.step_pin.pin()
            );
        }

        let steps_need = (rotation.turns * self.steps_per_turn as f32).round() as usize;
        let steps_taken = self
            .step(direction, steps_need, should_step_back_and_stop)
            .await;

        MotorRotation {
            turns: steps_taken as f32 / self.steps_per_turn as f32,
        }
    }

    fn epsilon(&self) -> f32 {
        0.5 / self.steps_per_turn as f32
    }
}
