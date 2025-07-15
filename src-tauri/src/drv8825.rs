use std::time::Duration;

use rppal::gpio::{Level, OutputPin};
use tokio::time::sleep;

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

        for _ in 0..steps {
            self.step_pin.set_high();
            sleep(Duration::from_micros(800)).await;
            self.step_pin.set_low();
            sleep(Duration::from_micros(800)).await;
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
