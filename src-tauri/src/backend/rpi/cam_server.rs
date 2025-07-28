use std::{convert::Infallible, process::Stdio};

use bytes::Bytes;
use futures::{Stream, StreamExt};
use tokio::{
    process::{Child, Command},
    sync::broadcast,
    task::JoinHandle,
};
use tokio_util::io::ReaderStream;
use warp::Filter;

use crate::backend::CameraBackend;

pub struct CameraServer {
    rpi_cam_process: Option<Child>,
    rpi_cam_server: Option<JoinHandle<()>>,
    tx: broadcast::Sender<Vec<u8>>,
}

impl CameraServer {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel::<Vec<u8>>(16);

        let tx1 = tx.clone();
        let video_route = warp::path("video").map(move || {
            let rx = tx1.subscribe();
            let stream = rx_to_http_stream(rx);

            warp::http::Response::builder()
                .header("Content-Type", "multipart/x-mixed-replace; boundary=frame")
                .body(warp::hyper::Body::wrap_stream(stream))
                .unwrap()
        });

        tokio::spawn(async move {
            warp::serve(video_route).run(([127, 0, 0, 1], 3030)).await;
        });

        Self {
            rpi_cam_process: None,
            rpi_cam_server: None,
            tx,
        }
    }
}

impl CameraBackend for CameraServer {
    async fn start(&mut self) -> String {
        let mut child = Command::new("rpicam-vid")
            .args(["-t", "0", "-n", "--inline", "--codec", "mjpeg", "-o", "-"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let stdout = child.stdout.take().expect("no stdout");
        self.rpi_cam_server = Some(tokio::spawn(read_mjpeg_stream(stdout, self.tx.clone())));
        self.rpi_cam_process = Some(child);
        "http://127.0.0.1:3030/video".to_string()
    }

    async fn capture(&mut self) -> Option<Vec<u8>> {
        self.tx.subscribe().recv().await.ok()
    }

    async fn stop(&mut self) {
        if let Some(rpi_cam) = self.rpi_cam_process.as_mut() {
            rpi_cam
                .kill()
                .await
                .expect("Failed to kill rpi-cam process");
        }

        if let Some(rpi_cam) = self.rpi_cam_server.as_mut() {
            rpi_cam.await.expect("Failed to kill camera server");
        }
    }
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
