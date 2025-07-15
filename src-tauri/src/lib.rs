use tauri::AppHandle;

#[cfg(feature = "rpi")]
pub mod drv8825;

#[cfg(feature = "rpi")]
mod rpi {

    use rppal::gpio::Gpio;
    use tokio::sync::{Mutex, MutexGuard};

    use crate::drv8825::{Drv8825Motor, MicroStepMode};

    const STEP_PIN: u8 = 23;
    const DIR_PIN: u8 = 24;
    const M0_PIN: u8 = 17;
    const M1_PIN: u8 = 27;
    const M2_PIN: u8 = 22;

    pub struct RpiControl {
        pub motor_1: Mutex<Drv8825Motor>,
    }

    impl RpiControl {
        pub fn new() -> Self {
            let gpio = Gpio::new().expect("Failed to get gpio");

            Self {
                motor_1: Drv8825Motor::new(
                    gpio.get(STEP_PIN).unwrap().into_output(),
                    gpio.get(DIR_PIN).unwrap().into_output(),
                    gpio.get(M0_PIN).unwrap().into_output(),
                    gpio.get(M1_PIN).unwrap().into_output(),
                    gpio.get(M2_PIN).unwrap().into_output(),
                    MicroStepMode::Step16,
                )
                .into(),
            }
        }

        pub async fn motor_1(&self) -> MutexGuard<'_, Drv8825Motor> {
            self.motor_1.lock().await
        }
    }
}

#[cfg(feature = "rpi")]
#[tauri::command]
async fn test_motor(app: AppHandle, direction: bool, steps: usize) {
    use tauri::Manager;

    use crate::drv8825::StepDirection;

    let rpi = app.state::<rpi::RpiControl>();
    rpi.motor_1()
        .await
        .step(StepDirection::from(direction), steps)
        .await;
}

#[cfg(not(feature = "rpi"))]
#[tauri::command]
async fn test_motor(_: AppHandle, direction: bool, steps: usize) {
    println!("Test motor direction: {direction} steps: {steps}");
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
        .invoke_handler(tauri::generate_handler![test_motor, exit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
