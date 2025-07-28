use std::{convert::Infallible, process::Stdio};

use bytes::Bytes;
use futures::{Stream, StreamExt};
use tauri::AppHandle;
use tokio::{process::Command, sync::broadcast};
use tokio_util::io::ReaderStream;
use warp::Filter;

use crate::backend::{Backend, MotorBackend, MotorDirection, MotorRotation};

mod backend;

#[tauri::command]
async fn test_motor(app: AppHandle, direction: bool) {
    use tauri::Manager;

    let rpi = app.state::<Backend>();
    let mut motor1 = rpi.motor_1().await;
    let mut motor2 = rpi.motor_2().await;

    tokio::join!(
        motor1.rotate(MotorDirection::from(direction), MotorRotation::full()),
        motor2.rotate(MotorDirection::from(!direction), MotorRotation::half())
    );
}

#[tauri::command]
async fn serve_rpi_cam() {}

#[tauri::command]
async fn test_camera(_app: AppHandle) -> String {
    todo!("Buy new cam for rasberry pi 5")
}

#[tauri::command]
async fn exit(app: AppHandle) {
    app.exit(0);
}

fn rx_to_http_stream(
    mut rx: broadcast::Receiver<Vec<u8>>,
) -> impl Stream<Item = Result<Bytes, Infallible>> + Send + 'static {
    async_stream::stream! {
        while let Ok(frame) = rx.recv().await {
            yield Ok(Bytes::from(format!(
                "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                frame.len()
            )));
            yield Ok(Bytes::from(frame));
            yield Ok(Bytes::from("\r\n"));
        }
    }
}

async fn read_mjpeg_stream<R: tokio::io::AsyncRead + Unpin + Send + 'static>(
    reader: R,
    tx: broadcast::Sender<Vec<u8>>,
) {
    let mut stream = ReaderStream::new(reader);
    let mut buffer = Vec::new();

    while let Some(Ok(chunk)) = stream.next().await {
        buffer.extend_from_slice(&chunk);

        while let Some(start) = find_marker(&buffer, &[0xFF, 0xD8]) {
            if let Some(end) = find_marker(&buffer[start..], &[0xFF, 0xD9]) {
                let end = start + end + 2; // include 0xFFD9
                let _ = tx.send(buffer[start..end].into());
                buffer.drain(..end);
            } else {
                break;
            }
        }
    }
}

fn find_marker(data: &[u8], marker: &[u8]) -> Option<usize> {
    data.windows(marker.len())
        .position(|window| window == marker)
}

fn setup_video() {
    let (tx, _) = broadcast::channel::<Vec<u8>>(16);

    let mut child = Command::new("libcamera-vid")
        .args(["-t", "0", "--inline", "--codec", "mjpeg", "-o", "-"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let stdout = child.stdout.take().expect("no stdout");
    tokio::spawn(read_mjpeg_stream(stdout, tx.clone()));

    let video_route = warp::path("video").map(move || {
        let rx = tx.subscribe();
        let stream = rx_to_http_stream(rx);

        warp::http::Response::builder()
            .header("Content-Type", "multipart/x-mixed-replace; boundary=frame")
            .body(warp::hyper::Body::wrap_stream(stream))
            .unwrap()
    });

    tokio::spawn(async move {
        warp::serve(video_route).run(([127, 0, 0, 1], 3030)).await;
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            use tauri::Manager;

            use crate::backend::Backend;
            tauri::async_runtime::spawn(async {
                setup_video();
            });

            app.manage(Backend::new());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![test_motor, test_camera, exit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
