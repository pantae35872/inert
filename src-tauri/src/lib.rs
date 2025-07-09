use tauri::AppHandle;
#[cfg(feature = "rpi")]
use {
    rppal::gpio::{Gpio, OutputPin},
    tauri::Manager,
    tokio::sync::Mutex,
};

#[cfg(feature = "rpi")]
struct AppGpioPins {
    gpio_23: Mutex<OutputPin>,
}

#[cfg(feature = "rpi")]
#[tauri::command]
async fn toggle_gpio_23(app: AppHandle) {
    let gpio_pins = app.state::<AppGpioPins>();
    gpio_pins.gpio_23.lock().await.toggle();
}

#[cfg(not(feature = "rpi"))]
#[tauri::command]
async fn toggle_gpio_23(_: AppHandle) {
    print!("GPIO Pin 23 toggle");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(feature = "rpi")]
            {
                app.manage(AppGpioPins {
                    gpio_23: Gpio::new()
                        .expect("Failed to get gpio")
                        .get(23)
                        .unwrap()
                        .into_output()
                        .into(),
                });
            }

            #[cfg(not(feature = "rpi"))]
            let _ = app.handle(); // suppress unused warning
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![toggle_gpio_23])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
