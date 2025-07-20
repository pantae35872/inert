use tauri::AppHandle;

#[cfg(feature = "rpi")]
pub mod drv8825;
#[cfg(feature = "rpi")]
pub mod esp32_cam;

#[cfg(feature = "rpi")]
mod rpi {

    use rppal::gpio::Gpio;
    use tokio::sync::{Mutex, MutexGuard};

    use crate::drv8825::Drv8825Motor;

    const MOTOR1_STEP_PIN: u8 = 23;
    const MOTOR1_DIR_PIN: u8 = 24;

    const MOTOR2_STEP_PIN: u8 = 5;
    const MOTOR2_DIR_PIN: u8 = 6;

    pub struct RpiControl {
        motor_1: Mutex<Drv8825Motor>,
        motor_2: Mutex<Drv8825Motor>,
    }

    impl RpiControl {
        pub fn new() -> Self {
            let gpio = Gpio::new().expect("Failed to get gpio");

            Self {
                motor_1: Drv8825Motor::new(
                    gpio.get(MOTOR1_STEP_PIN).unwrap().into_output_low(),
                    gpio.get(MOTOR1_DIR_PIN).unwrap().into_output_low(),
                )
                .into(),
                motor_2: Drv8825Motor::new(
                    gpio.get(MOTOR2_STEP_PIN).unwrap().into_output_low(),
                    gpio.get(MOTOR2_DIR_PIN).unwrap().into_output_low(),
                )
                .into(),
            }
        }

        pub async fn motor_1(&self) -> MutexGuard<'_, Drv8825Motor> {
            self.motor_1.lock().await
        }

        pub async fn motor_2(&self) -> MutexGuard<'_, Drv8825Motor> {
            self.motor_2.lock().await
        }
    }
}

#[cfg(feature = "rpi")]
#[tauri::command]
async fn test_motor(app: AppHandle, direction: bool, steps: usize) {
    use tauri::Manager;

    use crate::drv8825::StepDirection;

    let rpi = app.state::<rpi::RpiControl>();
    let mut motor1 = rpi.motor_1().await;
    let mut motor2 = rpi.motor_2().await;
    tokio::join!(
        motor1.step(StepDirection::from(direction), steps),
        motor2.step(StepDirection::from(!direction), steps)
    );
}

#[cfg(feature = "rpi")]
#[tauri::command]
async fn test_camera(_app: AppHandle) -> String {
    todo!("Buy new cam for rasberry pi 5")
}

#[cfg(not(feature = "rpi"))]
#[tauri::command]
async fn test_motor(_: AppHandle, direction: bool, steps: usize) {
    println!("Test motor direction: {direction} steps: {steps}");
}

#[cfg(not(feature = "rpi"))]
#[tauri::command]
async fn test_camera(_: AppHandle) -> String {
    String::new()
}

#[tauri::command]
async fn exit(app: AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(feature = "rpi")]
            {
                use crate::rpi::RpiControl;
                use tauri::Manager;

                app.manage(RpiControl::new());
            }

            #[cfg(not(feature = "rpi"))]
            let _ = app.handle(); // suppress unused warning
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![test_motor, test_camera, exit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
