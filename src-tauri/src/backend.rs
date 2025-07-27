use tokio::sync::{Mutex, MutexGuard};

#[cfg(feature = "rpi")]
mod rpi;

#[cfg(feature = "sim")]
mod sim;

pub struct BackendImpl<M: MotorBackend> {
    motor_1: Mutex<M>,
    motor_2: Mutex<M>,
}

impl<M: MotorBackend> BackendImpl<M> {
    pub async fn motor_1(&self) -> MutexGuard<'_, M> {
        self.motor_1.lock().await
    }

    pub async fn motor_2(&self) -> MutexGuard<'_, M> {
        self.motor_2.lock().await
    }
}

macro_rules! define_backend {
    ($feature:literal, $modname:ident) => {
        #[cfg(feature = $feature)]
        pub type Backend = BackendImpl<$modname::Motor>;

        #[cfg(feature = $feature)]
        impl BackendImpl<$modname::Motor> {
            pub fn new() -> Self {
                use $modname::motor_1;
                use $modname::motor_2;

                Self {
                    motor_1: motor_1().into(),
                    motor_2: motor_2().into(),
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
