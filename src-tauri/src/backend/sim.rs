use crate::backend::{
    ActuatorBackend, BackendComponents, CameraBackend, CameraFrame, MagnetBackend, MotorBackend,
};

pub struct FakeBackend;

impl BackendComponents for FakeBackend {
    type Motor = FakeMotor;
    type Camera = FakeCamera;
    type Actuator = FakeActuator;
    type Magnet = FakeMagnet;

    fn motor_1() -> FakeMotor {
        FakeMotor(1)
    }

    fn motor_2() -> FakeMotor {
        FakeMotor(2)
    }

    fn actuator() -> FakeActuator {
        FakeActuator
    }

    fn magnet() -> FakeMagnet {
        FakeMagnet
    }

    fn camera() -> FakeCamera {
        FakeCamera
    }
}

pub struct FakeActuator;

impl ActuatorBackend for FakeActuator {
    async fn contract(&mut self) {
        println!("FakeActuator contract");
    }
    async fn extend(&mut self) {
        println!("FakeActuator extend");
    }
}

pub struct FakeMagnet;

impl MagnetBackend for FakeMagnet {
    async fn set(&mut self, on: bool) {
        println!("FakeMagnet on?: {on}");
    }
}

pub struct FakeMotor(usize);

impl MotorBackend for FakeMotor {
    async fn rotate(&mut self, direction: super::MotorDirection, rotation: super::MotorRotation) {
        println!(
            "FakeMotor {} turned, Direction: {direction:?}, Rotation: {rotation:?}",
            self.0
        );
    }
}

pub struct FakeCamera;

impl CameraBackend for FakeCamera {
    type FrameType = FakeCameraFrame;

    async fn start(&mut self) -> String {
        println!("Start camera");
        format!("https://cdn.mos.cms.futurecdn.net/4wpKrH93D37dDPTisdqGy4-1200-80.jpg")
    }

    fn capture(&mut self) -> FakeCameraFrame {
        FakeCameraFrame
    }

    async fn stop(&mut self) {
        println!("Stop camera");
    }
}

pub struct FakeCameraFrame;

impl CameraFrame for FakeCameraFrame {
    async fn take(self) -> Option<Vec<u8>> {
        Some(Vec::new())
    }
}
