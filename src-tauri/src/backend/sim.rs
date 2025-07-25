use crate::backend::MotorBackend;

pub type Motor = FakeMotor;

pub struct FakeMotor(usize);

impl MotorBackend for FakeMotor {
    async fn rotate(&mut self, direction: super::MotorDirection, rotation: super::MotorRotation) {
        println!(
            "FakeMotor {} turned, Direction: {direction:?}, Rotation: {rotation:?}",
            self.0
        );
    }
}

pub fn motor_1() -> FakeMotor {
    FakeMotor(1)
}

pub fn motor_2() -> FakeMotor {
    FakeMotor(2)
}
