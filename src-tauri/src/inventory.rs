use serde::{Deserialize, Serialize};

use crate::{
    backend::{Backend, CameraBackend, CameraFrame},
    inventory::{allocator::ItemAllocator, db::Database},
};

mod allocator;
mod db;

#[derive(Debug)]
pub struct Inventory {
    db: Database,
    allocator: ItemAllocator,
}

impl Inventory {
    pub async fn new() -> Self {
        let db = Database::new().await;
        let allocator = ItemAllocator::new(&db).await;
        Self { db, allocator }
    }

    pub async fn add_item(&mut self, backend: &Backend, name: impl AsRef<str>, amount: usize) {
        let rect = self.allocator.allocate(50, 50).unwrap();

        let frame = backend.camera().await.capture();
        if let Some(frame) = frame.take().await {
            self.db.add_item(name, amount, rect, frame).await;
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Item {
    pub rect: Rectangle,

    pub image_path: String,
    pub display_name: String,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
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
