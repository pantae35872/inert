use std::time::{Duration, Instant};

use rppal::gpio::{Level, OutputPin};
use tokio::time::sleep;

const INITIAL_DELAY_US: u64 = 500;
const MIN_DELAY_US: u64 = 200;
const ACCEL_STEPS: usize = 100;

pub enum MicroStepMode {
    FullStep,
    Step2,
    Step4,
    Step8,
    Step16,
    Step32,
}

pub struct Drv8825Motor {
    step_pin: OutputPin,
    dir_pin: OutputPin,
}

impl Drv8825Motor {
    pub fn new(
        step_pin: OutputPin,
        dir_pin: OutputPin,
        mut m0_pin: OutputPin,
        mut m1_pin: OutputPin,
        mut m2_pin: OutputPin,
        step_mode: MicroStepMode,
    ) -> Self {
        // NOTE: They're probably some smart bit manipulation way to do this
        match step_mode {
            MicroStepMode::FullStep => (m0_pin.set_low(), m1_pin.set_low(), m2_pin.set_low()),
            MicroStepMode::Step2 => (m0_pin.set_high(), m1_pin.set_low(), m2_pin.set_low()),
            MicroStepMode::Step4 => (m0_pin.set_low(), m1_pin.set_high(), m2_pin.set_low()),
            MicroStepMode::Step8 => (m0_pin.set_high(), m1_pin.set_high(), m2_pin.set_low()),
            MicroStepMode::Step16 => (m0_pin.set_low(), m1_pin.set_low(), m2_pin.set_high()),
            MicroStepMode::Step32 => (m0_pin.set_high(), m1_pin.set_high(), m2_pin.set_high()),
        };

        Self { step_pin, dir_pin }
    }

    pub async fn step(&mut self, dir: StepDirection, steps: usize) {
        self.dir_pin.write(dir.into());
        sleep(Duration::from_millis(50)).await;

        for step_count in 0..steps {
            let delay_us = if step_count < ACCEL_STEPS {
                map(
                    step_count as u64,
                    0,
                    ACCEL_STEPS as u64,
                    INITIAL_DELAY_US,
                    MIN_DELAY_US,
                )
            } else if step_count > (steps - ACCEL_STEPS) {
                map(
                    step_count as u64,
                    (steps - ACCEL_STEPS) as u64,
                    steps as u64,
                    MIN_DELAY_US,
                    INITIAL_DELAY_US,
                )
            } else {
                MIN_DELAY_US
            };

            self.step_pin.set_high();
            busy_wait_us(2);
            self.step_pin.set_low();

            busy_wait_us(delay_us);
        }
    }
}

pub enum StepDirection {
    Forward,
    Reverse,
}

impl From<StepDirection> for Level {
    fn from(value: StepDirection) -> Self {
        match value {
            StepDirection::Forward => Self::Low,
            StepDirection::Reverse => Self::High,
        }
    }
}

fn busy_wait_us(microseconds: u64) {
    let now = Instant::now();
    let wait = Duration::from_micros(microseconds);
    while now.elapsed() < wait {}
}

fn map(x: u64, in_min: u64, in_max: u64, out_min: u64, out_max: u64) -> u64 {
    if in_max == in_min {
        return out_min;
    }
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
