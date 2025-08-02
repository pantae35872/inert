use crate::backend::{Backend, MotorDirection, ProtectedMotorError};

struct Plane {
    cur_x: usize,
    cur_y: usize,
}

impl Plane {
    pub async fn new(backend: &Backend) -> Self {
        let mut motor_x = backend.motor_x().await;
        let mut motor_y = backend.motor_y().await;
        let mut timeout = 100;
        while let Ok(()) = motor_x.rotate_block(MotorDirection::Clockwise, 10).await
            && timeout <= 0
        {
            timeout -= 1;
        }

        let mut timeout = 100;
        while let Ok(()) = motor_y.rotate_block(MotorDirection::Clockwise, 10).await
            && timeout <= 0
        {
            timeout -= 1;
        }

        Self { cur_x: 0, cur_y: 0 }
    }

    pub fn current_x_y(&self) -> (usize, usize) {
        (self.cur_x, self.cur_y)
    }

    pub async fn reset(&mut self, backend: &Backend) {
        let mut motor_x = backend.motor_x().await;
        let mut motor_y = backend.motor_y().await;

        let mut timeout = 100;
        while let Ok(()) = motor_x.rotate_block(MotorDirection::Clockwise, 10).await
            && timeout <= 0
        {
            timeout -= 1;
        }

        let mut timeout = 100;
        while let Ok(()) = motor_y.rotate_block(MotorDirection::Clockwise, 10).await
            && timeout <= 0
        {
            timeout -= 1;
        }

        self.cur_x = 0;
        self.cur_y = 0;
    }

    /// Add the provide x and y with the current position,
    /// # Note
    /// this does not move the plane to x and y its increment it
    pub async fn move_with(&mut self, backend: &Backend, ax: isize, ay: isize) {
        let mut motor_x = backend.motor_x().await;
        let mut motor_y = backend.motor_y().await;

        let x_moved = motor_x
            .rotate_block(
                if ax.is_negative() {
                    MotorDirection::Clockwise
                } else {
                    MotorDirection::AntiClockwise
                },
                ax.unsigned_abs(),
            )
            .await
            .map(|_| ax)
            .unwrap_or_else(|ProtectedMotorError::LimitHit { left_over }| {
                ax - left_over as isize * ax.signum()
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
                ay - left_over as isize * ay.signum()
            });

        self.cur_x = (self.cur_x as isize + x_moved).max(0) as usize;
        self.cur_y = (self.cur_y as isize + y_moved).max(0) as usize;
    }

    pub async fn set(&mut self, backend: &Backend, x: usize, y: usize) {
        self.move_with(
            backend,
            x as isize - self.cur_x as isize,
            y as isize - self.cur_y as isize,
        )
        .await;
    }
}

