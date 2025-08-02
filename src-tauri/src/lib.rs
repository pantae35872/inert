#![feature(iter_next_chunk)]

use std::{
    env,
    path::{Path, PathBuf},
    process::Stdio,
    str::FromStr,
};

use tauri::{AppHandle, Emitter, Manager};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};

use crate::backend::{
    ActuatorBackend, Backend, CameraBackend, CameraFrame, MagnetBackend, MotorBackend,
    MotorDirection, MotorRotation,
};

mod backend;

#[tauri::command]
async fn test_magnet(app: AppHandle, state: bool) {
    let backend = app.state::<Backend>();
    backend.magnet().await.set(state).await;
}

#[tauri::command]
async fn actuator_contract(app: AppHandle) {
    let backend = app.state::<Backend>();
    backend.actuator().await.contract().await;
}

#[tauri::command]
async fn actuator_extend(app: AppHandle) {
    let backend = app.state::<Backend>();
    backend.actuator().await.extend().await;
}

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

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DetectObjectResult {
    name: String,
    percentage: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            use tauri::Manager;

            app.manage(tauri::async_runtime::block_on(async { Backend::new() }));

            let handle = app.handle().clone();

            let rpi_recognition = match env::var("RPI_RECOGNITION_PATH") {
                Ok(p) => PathBuf::from_str(&p).unwrap(),
                Err(_) => Path::new("/home")
                    .join(env::var("USER").expect("no USER environment variable"))
                    .join("inert")
                    .join("rpi-recognition"),
            };

            tauri::async_runtime::spawn(async move {
                let backend = handle.state::<Backend>();
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                    let frame = backend.camera().await.capture();
                    if let Some(image) = frame.take().await {
                        File::create("/tmp/object.jpeg")
                            .await
                            .unwrap()
                            .write_all(image.as_slice())
                            .await
                            .unwrap();

                        let mut child = Command::new(rpi_recognition.join("classify_object.sh"))
                            .args(["/tmp/object.jpeg"])
                            .stdout(Stdio::piped())
                            .stderr(Stdio::null())
                            .current_dir(rpi_recognition.clone())
                            .spawn()
                            .unwrap();

                        let mut stdout = child.stdout.take().expect("no stdout");
                        let mut buf = String::new();
                        stdout.read_to_string(&mut buf).await.unwrap();
                        if let Some([name, percentage]) = buf
                            .split("\n")
                            .next()
                            .map(|buf| buf.split(":").map(|e| e.trim()))
                            .and_then(|mut buf| buf.next_chunk::<2>().ok())
                        {
                            handle
                                .emit(
                                    "update-detected-object",
                                    DetectObjectResult {
                                        name: name.to_string(),
                                        percentage: percentage.to_string(),
                                    },
                                )
                                .unwrap();
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            test_motor,
            test_camera,
            exit,
            serve_rpi_cam,
            stop_rpi_cam,
            actuator_contract,
            actuator_extend,
            test_magnet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
