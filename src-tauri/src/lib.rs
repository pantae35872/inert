use tauri::{AppHandle, Manager};

use crate::backend::{Backend, CameraBackend, MotorBackend, MotorDirection, MotorRotation};

mod backend;

#[tauri::command]
async fn test_motor(app: AppHandle, direction: bool) {
    let rpi = app.state::<Backend>();
    let mut motor1 = rpi.motor_1().await;
    let mut motor2 = rpi.motor_2().await;

    tokio::join!(
        motor1.rotate(MotorDirection::from(direction), MotorRotation::full()),
        motor2.rotate(MotorDirection::from(!direction), MotorRotation::half())
    );
}

#[tauri::command]
async fn serve_rpi_cam(app: AppHandle) -> String {
    let backend = app.state::<Backend>();
    let mut camera = backend.camera().await;
    camera.start().await
}

#[tauri::command]
async fn stop_rpi_cam(app: AppHandle) {
    let backend = app.state::<Backend>();
    let mut camera = backend.camera().await;
    camera.stop().await
}

#[tauri::command]
async fn test_camera(_app: AppHandle) -> String {
    todo!("Buy new cam for rasberry pi 5")
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
            use tauri::Manager;

            use crate::backend::Backend;

            app.manage(tauri::async_runtime::block_on(async { Backend::new() }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            test_motor,
            test_camera,
            exit,
            serve_rpi_cam,
            stop_rpi_cam
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
