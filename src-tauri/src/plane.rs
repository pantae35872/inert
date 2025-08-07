use tokio::sync::{Mutex, MutexGuard};

use crate::backend::{Backend, MotorDirection, ProtectedMotorError};

pub struct Plane {
    plane: Mutex<PlaneData>,
}

impl Plane {
    pub async fn new(backend: &Backend) -> Self {
        let data = Mutex::new(PlaneData::default());
        PlaneImpl {
            backend,
            data: data.lock().await,
        }
        .homeing()
        .await;

        Self { plane: data }
    }

    pub async fn get<'a>(&'a self, backend: &'a Backend) -> PlaneImpl<'a> {
        PlaneImpl {
            backend,
            data: self.plane.lock().await,
        }
    }
}

#[derive(Debug, Default)]
struct PlaneData {
    cur_x: usize,
    cur_y: usize,
}

pub struct PlaneImpl<'a> {
    backend: &'a Backend,
    data: MutexGuard<'a, PlaneData>,
}

impl PlaneImpl<'_> {
    pub fn current_x_y(&self) -> (usize, usize) {
        (self.data.cur_x, self.data.cur_y)
    }

    /// Home the plane to 0, 0
    pub async fn homeing(&mut self) {
        let mut motor_x = self.backend.motor_x().await;
        let mut motor_y = self.backend.motor_y().await;

        let mut timeout = 10;
        while let Ok(()) = motor_x
            .rotate_block(MotorDirection::AntiClockwise, 100)
            .await
            && timeout > 0
        {
            timeout -= 1;
        }

        let mut timeout = 10;
        while let Ok(()) = motor_y.rotate_block(MotorDirection::Clockwise, 100).await
            && timeout > 0
        {
            timeout -= 1;
        }

        self.data.cur_x = 0;
        self.data.cur_y = 0;
    }

    /// Add the provide x and y with the current position,
    /// # Note
    /// Moves the plane relative to the current position by (ax, ay), not to an absolute position.
    pub async fn move_with(&mut self, ax: isize, ay: isize) {
        let mut motor_x = self.backend.motor_x().await;
        let mut motor_y = self.backend.motor_y().await;

        // TODO: use join to run this simultaneously maybe?
        let x_moved = motor_x
            .rotate_block(
                if ax.is_negative() {
                    MotorDirection::AntiClockwise
                } else {
                    MotorDirection::Clockwise
                },
                ax.unsigned_abs(),
            )
            .await
            .map(|_| ax)
            .unwrap_or_else(|ProtectedMotorError::LimitHit { left_over }| {
                (ax.unsigned_abs() - left_over) as isize * ax.signum()
            });

        let y_moved = motor_y
            .rotate_block(
                if ay.is_negative() {
                    MotorDirection::Clockwise
                } else {
                    MotorDirection::AntiClockwise
                },
                ay.unsigned_abs(),
            )
            .await
            .map(|_| ay)
            .unwrap_or_else(|ProtectedMotorError::LimitHit { left_over }| {
                (ay.unsigned_abs() - left_over) as isize * ay.signum()
            });

        self.data.cur_x = (self.data.cur_x as isize + x_moved).max(0) as usize;
        self.data.cur_y = (self.data.cur_y as isize + y_moved).max(0) as usize;
    }

    pub async fn move_to(&mut self, x: usize, y: usize) {
        self.move_with(
            x as isize - self.data.cur_x as isize,
            y as isize - self.data.cur_y as isize,
        )
        .await;
    }
}
