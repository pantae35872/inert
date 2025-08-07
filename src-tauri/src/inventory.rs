use std::{sync::Arc, time::Duration};
use ts_rs::TS;

use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, MutexGuard};

use crate::{
    backend::{ActuatorBackend, Backend, CameraBackend, CameraFrame, MagnetBackend},
    inventory::{allocator::ItemAllocator, db::Database},
    plane::Plane,
};

mod allocator;
mod db;

#[derive(Debug)]
struct InventoryData {
    db: Database,
    allocator: ItemAllocator,
}

#[derive(Debug)]
pub struct Inventory {
    data: Mutex<InventoryData>,
}

impl Inventory {
    pub async fn new() -> Self {
        let db = Database::new().await;
        let allocator = ItemAllocator::new(&db, 100, 100).await;
        let data = Mutex::new(InventoryData { db, allocator });

        Self { data }
    }

    pub async fn get<'a>(&'a self, backend: Arc<Backend>, plane: &'a Plane) -> InventoryImpl<'a> {
        InventoryImpl {
            backend,
            plane,
            data: self.data.lock().await,
        }
    }
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DisplayItem {
    id: i64,
    amount: u64,
    image_path: String,
    display_name: String,
}

pub struct InventoryImpl<'a> {
    backend: Arc<Backend>,
    plane: &'a Plane,
    data: MutexGuard<'a, InventoryData>,
}

impl<'a> InventoryImpl<'a> {
    pub async fn remove_item(&mut self, id: i64) {
        let item = self.data.db.find_item_by_id(id).await;

        let mut plane = self.plane.get(Arc::clone(&self.backend)).await;
        let mut actuator = self.backend.actuator().await;
        let mut magnet = self.backend.magnet().await;

        plane.move_to(item.rect.x, item.rect.y).await;

        actuator.extend().await;
        magnet.set(true).await;
        actuator.contract().await;

        plane.move_to(plane.width(), plane.height()).await;
        actuator.extend().await;
        magnet.set(false).await;
        actuator.contract().await;
    }

    pub async fn list_items(&mut self) -> Vec<DisplayItem> {
        self.data
            .db
            .list_all_items()
            .await
            .iter()
            .map(|result| DisplayItem {
                id: result.id,
                image_path: result.image_id.clone(),
                display_name: result.display_name.clone(),
                amount: result.amount,
            })
            .collect()
    }

    pub async fn prepare_add_item(&mut self) -> Option<Rectangle> {
        let mut plane = self.plane.get(Arc::clone(&self.backend)).await;

        plane.move_to(plane.width(), plane.height()).await;
        self.backend.actuator().await.contract().await;
        self.backend.magnet().await.set(false).await;

        self.data.allocator.allocate(15, 15)
    }

    pub async fn add_item(&mut self, name: impl AsRef<str>, rect: Rectangle, amount: usize) {
        let mut plane = self.plane.get(Arc::clone(&self.backend)).await;
        let mut actuator = self.backend.actuator().await;
        let mut magnet = self.backend.magnet().await;

        let frame = self.backend.camera().await.capture();
        if let Some(frame) = frame.take().await {
            self.data.db.add_item(name, amount, rect, frame).await;
        }

        actuator.extend().await;
        magnet.set(true).await;
        actuator.contract().await;

        tokio::time::sleep(Duration::from_secs(1)).await;

        plane.move_to(rect.x, rect.y).await;

        actuator.extend().await;
        magnet.set(false).await;
        actuator.contract().await;
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Item {
    pub rect: Rectangle,

    pub image_id: String,
    pub display_name: String,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[cfg(feature = "visualization")]
pub mod visualizer {
    use serde::{Deserialize, Serialize};
    use tokio::process::Command;

    use crate::inventory::{Item, Rectangle, allocator::visualizer};

    #[derive(Debug, Serialize, Deserialize)]
    struct VisualizeData {
        physical_item: Vec<Item>,
        free_space: Vec<Rectangle>,
    }

    pub fn visualize_child(data: &str) {
        let data = serde_json::from_str::<VisualizeData>(data).unwrap();
        visualizer::visualize(data.physical_item, data.free_space);
    }

    pub fn visualize(items: Vec<Item>, free_list: Vec<Rectangle>) {
        Command::new(std::env::current_exe().unwrap())
            .arg("child")
            .arg(
                serde_json::to_string(&VisualizeData {
                    physical_item: items,
                    free_space: free_list,
                })
                .unwrap(),
            )
            .spawn()
            .expect("Failed to spawn child");
    }
}
