use std::ops::Not;

use thiserror::Error;
use tokio::sync::{Mutex, MutexGuard};

#[cfg(feature = "rpi")]
pub type Backend = BackendImpl<rpi::RpiBackend>;

#[cfg(feature = "rpi")]
mod rpi;

#[cfg(feature = "sim")]
pub type Backend = BackendImpl<sim::FakeBackend>;

#[cfg(feature = "sim")]
mod sim;

pub struct BackendImpl<B: BackendComponents> {
    camera: Mutex<B::Camera>,
    actuator: Mutex<B::Actuator>,
    magnet: Mutex<B::Magnet>,

    motor_x: Mutex<B::Motor>,
    motor_y: Mutex<B::Motor>,

    limit_x_l: Mutex<B::Limit>,
    limit_x_r: Mutex<B::Limit>,
    limit_y_l: Mutex<B::Limit>,
    limit_y_r: Mutex<B::Limit>,
}

impl<B: BackendComponents> BackendImpl<B> {
    pub fn new() -> Self {
        Self {
            actuator: B::actuator().into(),
            magnet: B::magnet().into(),
            camera: B::camera().into(),

            motor_x: B::motor_x().into(),
            motor_y: B::motor_y().into(),

            limit_x_l: B::limit_x_l().into(),
            limit_x_r: B::limit_x_r().into(),
            limit_y_l: B::limit_y_l().into(),
            limit_y_r: B::limit_y_r().into(),
        }
    }

    pub async fn motor_y(&self) -> ProtectedMotor<'_, B::Motor, B::Limit> {
        ProtectedMotor {
            motor: self.motor_y.lock().await,
            limit_l: self.limit_y_l.lock().await,
            limit_r: self.limit_y_r.lock().await,
        }
    }

    pub async fn motor_x(&self) -> ProtectedMotor<'_, B::Motor, B::Limit> {
        ProtectedMotor {
            motor: self.motor_x.lock().await,
            limit_l: self.limit_x_l.lock().await,
            limit_r: self.limit_x_r.lock().await,
        }
    }

    pub async fn motor_x_raw(&self) -> MutexGuard<'_, B::Motor> {
        self.motor_x.lock().await
    }

    pub async fn motor_y_raw(&self) -> MutexGuard<'_, B::Motor> {
        self.motor_y.lock().await
    }

    pub async fn camera(&self) -> MutexGuard<'_, B::Camera> {
        self.camera.lock().await
    }

    pub async fn actuator(&self) -> MutexGuard<'_, B::Actuator> {
        self.actuator.lock().await
    }

    pub async fn magnet(&self) -> MutexGuard<'_, B::Magnet> {
        self.magnet.lock().await
    }
}

pub struct ProtectedMotor<'a, M: MotorBackend, S: LimitSwitchBackend> {
    motor: MutexGuard<'a, M>,
    limit_l: MutexGuard<'a, S>,
    limit_r: MutexGuard<'a, S>,
}

#[derive(Debug, Error)]
pub enum ProtectedMotorError {
    #[error("motor limit switch has hit program trying to rotate pass that by {left_over} block")]
    LimitHit { left_over: usize },
}

impl<'a, M: MotorBackend, S: LimitSwitchBackend> ProtectedMotor<'a, M, S> {
    pub async fn rotate_block(
        &mut self,
        motor_direction: MotorDirection,
        amount: usize,
    ) -> Result<(), ProtectedMotorError> {
        if amount == 0 {
            return Ok(());
        }

        const BLOCK_TURN: f32 = 0.07;

        let rotation_need = MotorRotation {
            turns: BLOCK_TURN * amount as f32,
        };

        let moved = self
            .motor
            .rotate(motor_direction, rotation_need, || match motor_direction {
                MotorDirection::AntiClockwise => self.limit_r.is_pressed(),
                MotorDirection::Clockwise => self.limit_l.is_pressed(),
            })
            .await;

        let missed_turns = (rotation_need.turns - moved.turns).max(0.0);
        if missed_turns > self.motor.epsilon() {
            return Err(ProtectedMotorError::LimitHit {
                left_over: (missed_turns as f32 / BLOCK_TURN).round() as usize,
            });
        }

        Ok(())
    }
}

pub trait BackendComponents {
    type Motor: MotorBackend;
    type Camera: CameraBackend;
    type Actuator: ActuatorBackend;
    type Magnet: MagnetBackend;
    type Limit: LimitSwitchBackend;

    fn motor_x() -> Self::Motor;
    fn motor_y() -> Self::Motor;
    fn actuator() -> Self::Actuator;
    fn magnet() -> Self::Magnet;
    fn camera() -> Self::Camera;

    fn limit_x_l() -> Self::Limit;
    fn limit_x_r() -> Self::Limit;
    fn limit_y_l() -> Self::Limit;
    fn limit_y_r() -> Self::Limit;
}

pub trait LimitSwitchBackend {
    fn is_pressed(&mut self) -> bool;
}

pub trait ActuatorBackend {
    async fn contract(&mut self);
    async fn extend(&mut self);
}

pub trait MagnetBackend {
    async fn set(&mut self, on: bool);
}

pub trait MotorBackend {
    async fn rotate(
        &mut self,
        direction: MotorDirection,
        rotation: MotorRotation,
        should_step_back_and_stop: impl FnMut() -> bool,
    ) -> MotorRotation;

    fn epsilon(&self) -> f32 {
        0.0
    }
}

pub trait CameraFrame {
    async fn take(self) -> Option<Vec<u8>>;
}

pub trait CameraBackend {
    type FrameType: CameraFrame;

    /// Start the camera server and return a url to that
    async fn start(&mut self) -> String;

    /// Capture a single frame
    fn capture(&mut self) -> Self::FrameType;

    /// Stop the camera server
    async fn stop(&mut self);
}

/// Respresent MotorRotation in turns
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct MotorRotation {
    pub turns: f32, // In turns, 0.0 to N.0 (e.g. 0.75 = 270°)
}

impl MotorRotation {
    pub fn full() -> Self {
        Self { turns: 1.0 }
    }

    pub fn half() -> Self {
        Self { turns: 0.5 }
    }

    pub fn quarter() -> Self {
        Self { turns: 0.25 }
    }
}

impl Default for MotorRotation {
    fn default() -> Self {
        Self::full()
    }
}

impl From<MotorRotation> for f32 {
    fn from(value: MotorRotation) -> Self {
        value.turns
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MotorDirection {
    Clockwise,
    AntiClockwise,
}

impl From<bool> for MotorDirection {
    fn from(value: bool) -> Self {
        if value {
            Self::AntiClockwise
        } else {
            Self::Clockwise
        }
    }
}

impl Not for MotorDirection {
    type Output = MotorDirection;

    fn not(self) -> Self::Output {
        match self {
            Self::Clockwise => Self::AntiClockwise,
            Self::AntiClockwise => Self::Clockwise,
        }
    }
}
