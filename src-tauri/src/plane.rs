use std::sync::Arc;

use tokio::{
    join,
    sync::{Mutex, MutexGuard},
};

use crate::backend::{Backend, MotorDirection, ProtectedMotorError};

pub struct Plane {
    plane: Mutex<PlaneData>,
}

impl Plane {
    pub async fn new(backend: Arc<Backend>) -> Self {
        let data = Mutex::new(PlaneData::default());
        PlaneImpl {
            backend,
            data: data.lock().await,
        }
        .setup()
        .await;

        {
            let plane = data.lock().await;
            println!(
                "Plane width: {}, Plane height: {};",
                plane.width, plane.height
            );
        }

        Self { plane: data }
    }

    pub async fn get(&self, backend: Arc<Backend>) -> PlaneImpl<'_> {
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

    width: usize,
    height: usize,
}

pub struct PlaneImpl<'a> {
    backend: Arc<Backend>,
    data: MutexGuard<'a, PlaneData>,
}

impl PlaneImpl<'_> {
    pub fn current_x_y(&self) -> (usize, usize) {
        (self.data.cur_x, self.data.cur_y)
    }

    pub async fn setup(&mut self) {
        self.homeing().await;

        const MAX_WIDTH: usize = 1000;
        const MAX_HEIGHT: usize = 1000;

        let backend = self.backend.clone();
        let width = tokio::spawn(async move {
            let mut motor_x = backend.motor_x().await;
            match motor_x
                .rotate_block(MotorDirection::Clockwise, MAX_WIDTH)
                .await
            {
                Ok(_) => panic!("The plane shouldn't be wider than {} units", MAX_WIDTH),
                Err(ProtectedMotorError::LimitHit { left_over }) => MAX_WIDTH - left_over,
            }
        });

        let backend = self.backend.clone();
        let height = tokio::spawn(async move {
            let mut motor_y = backend.motor_y().await;

            match motor_y
                .rotate_block(MotorDirection::AntiClockwise, MAX_HEIGHT)
                .await
            {
                Ok(_) => panic!("The plane shouldn't be higher than {} units", MAX_HEIGHT),
                Err(ProtectedMotorError::LimitHit { left_over }) => MAX_HEIGHT - left_over,
            }
        });

        let (width, height) = join!(width, height);
        let (width, height) = (width.unwrap(), height.unwrap());

        self.homeing().await;

        self.data.width = width;
        self.data.height = height;
    }

    /// Home the plane to 0, 0
    pub async fn homeing(&mut self) {
        let backend = self.backend.clone();
        let x_dir = tokio::spawn(async move {
            let mut motor_x = backend.motor_x().await;
            let mut timeout = 10;

            while let Ok(()) = motor_x
                .rotate_block(MotorDirection::AntiClockwise, 200)
                .await
                && timeout > 0
            {
                timeout -= 1;
            }
        });

        let backend = self.backend.clone();
        let y_dir = tokio::spawn(async move {
            let mut motor_y = backend.motor_y().await;
            let mut timeout = 10;

            while let Ok(()) = motor_y.rotate_block(MotorDirection::Clockwise, 200).await
                && timeout > 0
            {
                timeout -= 1;
            }
        });

        let (x_dir, y_dir) = join!(x_dir, y_dir);
        let (_, _) = (x_dir.unwrap(), y_dir.unwrap());

        self.data.cur_x = 0;
        self.data.cur_y = 0;
    }

    /// Add the provide x and y with the current position,
    /// # Note
    /// Moves the plane relative to the current position by (ax, ay), not to an absolute position.
    pub async fn move_with(&mut self, ax: isize, ay: isize) {
        let backend = self.backend.clone();
        let x_moved = tokio::spawn(async move {
            let mut motor_x = backend.motor_x().await;
            motor_x
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
                })
        });

        let backend = self.backend.clone();
        let y_moved = tokio::spawn(async move {
            let mut motor_y = backend.motor_y().await;
            motor_y
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
                })
        });

        let (x_moved, y_moved) = join!(x_moved, y_moved);
        let (x_moved, y_moved) = (x_moved.unwrap(), y_moved.unwrap());

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
