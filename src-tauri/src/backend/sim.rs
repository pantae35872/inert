use crate::backend::{CameraBackend, CameraFrame, MotorBackend};

pub type Motor = FakeMotor;
pub type Camera = FakeCamera;

pub struct FakeMotor(usize);

pub struct FakeCamera;

impl MotorBackend for FakeMotor {
    async fn rotate(&mut self, direction: super::MotorDirection, rotation: super::MotorRotation) {
        println!(
            "FakeMotor {} turned, Direction: {direction:?}, Rotation: {rotation:?}",
            self.0
        );
    }
}

pub struct FakeCameraFrame;

impl CameraFrame for FakeCameraFrame {
    async fn take(self) -> Option<Vec<u8>> {
        Some(Vec::new())
    }
}

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

pub fn motor_1() -> FakeMotor {
    FakeMotor(1)
}

pub fn motor_2() -> FakeMotor {
    FakeMotor(2)
}

pub fn camera() -> FakeCamera {
    FakeCamera
}
