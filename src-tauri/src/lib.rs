use tauri::AppHandle;

#[cfg(feature = "rpi")]
mod rpi {
    use std::time::Duration;

    use rppal::gpio::{Gpio, Level, OutputPin};
    use tokio::{sync::Mutex, time::sleep};

    const STEP_PIN: u8 = 23;
    const DIR_PIN: u8 = 24;

    pub enum StepDirection {
        Forward,
        Reverse,
    }

    impl From<StepDirection> for Level {
        fn from(value: StepDirection) -> Self {
            match value {
                StepDirection::Forward => Self::Low,
                StepDirection::Reverse => Self::High,
            }
        }
    }

    pub struct RpiControl {
        step: Mutex<OutputPin>,
        dir: Mutex<OutputPin>,
    }

    impl RpiControl {
        pub fn new() -> Self {
            let gpio = Gpio::new().expect("Failed to get gpio");

            Self {
                step: gpio.get(STEP_PIN).unwrap().into_output().into(),
                dir: gpio.get(DIR_PIN).unwrap().into_output().into(),
            }
        }

        pub async fn step(&self, dir: StepDirection, steps: usize) {
            let mut dir_pin = self.dir.lock().await;
            let mut step_pin = self.step.lock().await;
            dir_pin.write(dir.into());
            sleep(Duration::from_millis(50)).await;

            for _ in 0..steps {
                step_pin.set_high();
                sleep(Duration::from_micros(800)).await;
                step_pin.set_low();
                sleep(Duration::from_micros(800)).await;
            }
        }
    }
}

#[cfg(feature = "rpi")]
#[tauri::command]
async fn test_motor(app: AppHandle) {
    use tauri::Manager;

    let rpi = app.state::<rpi::RpiControl>();
    rpi.step(rpi::StepDirection::Forward, 200).await;
}

#[cfg(not(feature = "rpi"))]
#[tauri::command]
async fn test_motor(_: AppHandle) {
    println!("Test motor");
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
