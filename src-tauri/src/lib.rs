#![feature(iter_next_chunk)]

use std::{
    env,
    path::{Path, PathBuf},
    process::Stdio,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};
use ts_rs::TS;

use crate::{
    backend::{ActuatorBackend, Backend, CameraBackend, CameraFrame, MagnetBackend},
    inventory::{Inventory, visualizer::visualize_child},
    plane::Plane,
};

mod backend;
mod inventory;
mod plane;

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
async fn homing(app: AppHandle) {
    let backend = app.state::<Backend>();
    let plane = app.state::<Plane>();
    let mut plane = plane.get(&backend).await;

    plane.homeing().await;
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[tauri::command]
async fn move_to(app: AppHandle, x: usize, y: usize) {
    let backend = app.state::<Backend>();
    let plane = app.state::<Plane>();
    let mut plane = plane.get(&backend).await;

    println!("Moving to posisiton: {x}, {y}");
    plane.move_to(x, y).await;
    let (x, y) = plane.current_x_y();
    println!("Current posisiton: {x}, {y}");
}

#[tauri::command]
async fn move_by(app: AppHandle, direction: Direction, amount: usize) {
    let amount = amount as isize;
    let backend = app.state::<Backend>();
    let plane = app.state::<Plane>();
    let mut plane = plane.get(&backend).await;
    match direction {
        Direction::North => plane.move_with(0, amount).await,
        Direction::South => plane.move_with(0, -amount).await,
        Direction::East => plane.move_with(amount, 0).await,
        Direction::West => plane.move_with(-amount, 0).await,
    }
    let (x, y) = plane.current_x_y();
    println!("Current posisiton: {x}, {y}");
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
    let mut args = env::args();
    let _exe = args.next();

    match args.next().as_deref() {
        Some("child") => {
            visualize_child(&args.next().expect("No visualize data passed in."));
            return;
        }
        _ => {}
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            use tauri::Manager;

            let (backend, plane, inventory) = tauri::async_runtime::block_on(async {
                let backend = Backend::new();
                let plane = Plane::new(&backend).await;
                (backend, plane, Inventory::new().await)
            });
            app.manage(backend);
            app.manage(plane);
            app.manage(inventory);

            let handle = app.handle().clone();

            let rpi_recognition = match env::var("RPI_RECOGNITION_PATH") {
                Ok(p) => PathBuf::from_str(&p).unwrap(),
                Err(_) => Path::new("/home")
                    .join(env::var("USER").expect("no USER environment variable"))
                    .join("inert")
                    .join("rpi-recognition"),
            };

            // TODO: Move this somewhere else
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
            test_camera,
            exit,
            serve_rpi_cam,
            stop_rpi_cam,
            actuator_contract,
            actuator_extend,
            test_magnet,
            move_by,
            move_to,
            homing,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
