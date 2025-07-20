use crate::backend::MotorBackend;

pub type Motor = FakeMotor;

pub struct FakeMotor;

impl MotorBackend for FakeMotor {
    async fn rotate(&mut self, _direction: super::MotorDirection, _rotation: super::MotorRotation) {
        todo!("Simulate motor")
    }
}

pub fn motor_1() -> FakeMotor {
    FakeMotor
}

pub fn motor_2() -> FakeMotor {
    FakeMotor
}
