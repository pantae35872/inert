use std::time::{Duration, Instant};

use rppal::gpio::Gpio;

pub type Motor = Drv8825Motor;

use crate::backend::rpi::drv8825::Drv8825Motor;

const MOTOR1_STEP_PIN: u8 = 23;
const MOTOR1_DIR_PIN: u8 = 24;

const MOTOR2_STEP_PIN: u8 = 5;
const MOTOR2_DIR_PIN: u8 = 6;

pub mod drv8825;

async fn busy_wait_us(microseconds: u64) {
    tokio::task::spawn_blocking(move || {
        let now = Instant::now();
        let wait = Duration::from_micros(microseconds);
        while now.elapsed() < wait {}
    })
    .await
    .unwrap();
}

pub fn motor_1() -> Drv8825Motor {
    let gpio = Gpio::new().expect("Failed to get gpio");
    Drv8825Motor::new(
        gpio.get(MOTOR1_STEP_PIN).unwrap().into_output_low(),
        gpio.get(MOTOR1_DIR_PIN).unwrap().into_output_low(),
    )
}

pub fn motor_2() -> Drv8825Motor {
    let gpio = Gpio::new().expect("Failed to get gpio");

    Drv8825Motor::new(
        gpio.get(MOTOR2_STEP_PIN).unwrap().into_output_low(),
        gpio.get(MOTOR2_DIR_PIN).unwrap().into_output_low(),
    )
}
