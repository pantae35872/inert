use tokio::sync::{Mutex, MutexGuard};

#[cfg(feature = "rpi")]
mod rpi;

#[cfg(feature = "sim")]
mod sim;

pub struct BackendImpl<M: MotorBackend, C: CameraBackend> {
    motor_1: Mutex<M>,
    motor_2: Mutex<M>,
    camera: Mutex<C>,
}

impl<M: MotorBackend, C: CameraBackend> BackendImpl<M, C> {
    pub async fn motor_1(&self) -> MutexGuard<'_, M> {
        self.motor_1.lock().await
    }

    pub async fn motor_2(&self) -> MutexGuard<'_, M> {
        self.motor_2.lock().await
    }

    pub async fn camera(&self) -> MutexGuard<'_, C> {
        self.camera.lock().await
    }
}

macro_rules! define_backend {
    ($feature:literal, $modname:ident) => {
        #[cfg(feature = $feature)]
        pub type Backend = BackendImpl<$modname::Motor, $modname::Camera>;

        #[cfg(feature = $feature)]
        impl BackendImpl<$modname::Motor, $modname::Camera> {
            pub fn new() -> Self {
                use $modname::camera;
                use $modname::motor_1;
                use $modname::motor_2;

                Self {
                    motor_1: motor_1().into(),
                    motor_2: motor_2().into(),
                    camera: camera().into(),
                }
            }
        }
    };
}

define_backend!("sim", sim);
define_backend!("rpi", rpi);

pub trait MotorBackend {
    async fn rotate(&mut self, direction: MotorDirection, rotation: MotorRotation);
}

pub trait CameraBackend {
    /// Start the camera server and return a url to that
    async fn start(&mut self) -> String;

    /// Capture a single frame
    async fn capture(&mut self) -> Option<Vec<u8>>;

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
