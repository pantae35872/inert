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
    motor_1: Mutex<B::Motor>,
    motor_2: Mutex<B::Motor>,
    camera: Mutex<B::Camera>,
    actuator: Mutex<B::Actuator>,
    magnet: Mutex<B::Magnet>,
}

impl<B: BackendComponents> BackendImpl<B> {
    pub fn new() -> Self {
        Self {
            motor_1: B::motor_1().into(),
            motor_2: B::motor_2().into(),
            actuator: B::actuator().into(),
            magnet: B::magnet().into(),
            camera: B::camera().into(),
        }
    }

    pub async fn motor_1(&self) -> MutexGuard<'_, B::Motor> {
        self.motor_1.lock().await
    }

    pub async fn motor_2(&self) -> MutexGuard<'_, B::Motor> {
        self.motor_2.lock().await
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

pub trait BackendComponents {
    type Motor: MotorBackend;
    type Camera: CameraBackend;
    type Actuator: ActuatorBackend;
    type Magnet: MagnetBackend;

    fn motor_1() -> Self::Motor;
    fn motor_2() -> Self::Motor;
    fn actuator() -> Self::Actuator;
    fn magnet() -> Self::Magnet;
    fn camera() -> Self::Camera;
}

pub trait ActuatorBackend {
    async fn contract(&mut self);
    async fn extend(&mut self);
}

pub trait MagnetBackend {
    async fn set(&mut self, on: bool);
}

pub trait MotorBackend {
    async fn rotate(&mut self, direction: MotorDirection, rotation: MotorRotation);
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
    turns: f32, // In turns, 0.0 to N.0 (e.g. 0.75 = 270°)
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

    pub fn raw(turns: f32) -> Self {
        Self { turns }
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
