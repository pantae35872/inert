use std::time::{Duration, Instant};

use rppal::gpio::{Level, OutputPin};
use tokio::time::sleep;

pub struct Drv8825Motor {
    step_pin: OutputPin,
    dir_pin: OutputPin,
}

impl Drv8825Motor {
    pub fn new(step_pin: OutputPin, dir_pin: OutputPin) -> Self {
        Self { step_pin, dir_pin }
    }

    pub async fn step(&mut self, dir: StepDirection, steps: usize) {
        self.dir_pin.write(dir.into());
        sleep(Duration::from_millis(50)).await;

        for _ in 0..steps {
            self.step_pin.set_high();
            busy_wait_us(500);
            self.step_pin.set_low();
            busy_wait_us(500);
        }
    }
}

#[derive(Debug)]
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

impl From<bool> for StepDirection {
    fn from(value: bool) -> Self {
        if value {
            Self::Forward
        } else {
            Self::Reverse
        }
    }
}

fn busy_wait_us(microseconds: u64) {
    let now = Instant::now();
    let wait = Duration::from_micros(microseconds);
    while now.elapsed() < wait {}
}
