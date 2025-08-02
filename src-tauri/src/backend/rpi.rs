use std::time::{Duration, Instant};

use rppal::gpio::Gpio;

use crate::backend::{
    rpi::{
        actuator::LinearActuator, cam_server::CameraServer, drv8825::Drv8825Motor,
        magnet::ElectroMagnet,
    },
    BackendComponents,
};

const MOTOR1_STEP_PIN: u8 = 23;
const MOTOR1_DIR_PIN: u8 = 24;

const MOTOR2_STEP_PIN: u8 = 5;
const MOTOR2_DIR_PIN: u8 = 6;

const LINEAR_FORWARD_PIN: u8 = 13;
const LINEAR_BACKWARD_PIN: u8 = 16;

const MAGNET_PIN: u8 = 12;

pub mod actuator;
pub mod cam_server;
pub mod drv8825;
pub mod magnet;

async fn busy_wait_us(microseconds: u64) {
    tokio::task::spawn_blocking(move || {
        let now = Instant::now();
        let wait = Duration::from_micros(microseconds);
        while now.elapsed() < wait {}
    })
    .await
    .unwrap();
}

pub struct RpiBackend;

impl BackendComponents for RpiBackend {
    type Motor = Drv8825Motor;
    type Camera = CameraServer;
    type Actuator = LinearActuator;
    type Magnet = ElectroMagnet;

    fn motor_1() -> Drv8825Motor {
        let gpio = Gpio::new().expect("Failed to get gpio");
        Drv8825Motor::new(
            gpio.get(MOTOR1_STEP_PIN).unwrap().into_output_low(),
            gpio.get(MOTOR1_DIR_PIN).unwrap().into_output_low(),
        )
    }

    fn motor_2() -> Drv8825Motor {
        let gpio = Gpio::new().expect("Failed to get gpio");

        Drv8825Motor::new(
            gpio.get(MOTOR2_STEP_PIN).unwrap().into_output_low(),
            gpio.get(MOTOR2_DIR_PIN).unwrap().into_output_low(),
        )
    }

    fn actuator() -> LinearActuator {
        let gpio = Gpio::new().expect("Failed to get gpio");

        LinearActuator::new(
            gpio.get(LINEAR_FORWARD_PIN).unwrap().into_output_low(),
            gpio.get(LINEAR_BACKWARD_PIN).unwrap().into_output_low(),
        )
    }

    fn magnet() -> ElectroMagnet {
        let gpio = Gpio::new().expect("Failed to get gpio");

        ElectroMagnet::new(gpio.get(MAGNET_PIN).unwrap().into_output_low())
    }

    fn camera() -> CameraServer {
        CameraServer::new()
    }
}
