use crate::inventory::{allocator::ItemAllocator, db::InventoryDB};

mod allocator;
mod db;

#[derive(Debug)]
pub struct Inventory {
    db: InventoryDB,
    allocator: ItemAllocator,
}

impl Inventory {
    pub async fn new() -> Self {
        let db = InventoryDB::new().await;
        let allocator = ItemAllocator::new(&db).await;
        Self { db, allocator }
    }
}

#[cfg(feature = "visualization")]
pub mod visualizer {
    use serde::{Deserialize, Serialize};
    use tokio::process::Command;

    use crate::inventory::{
        allocator::{FreeSpace, visualizer},
        db::PhysicalItem,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct VisualizeData {
        physical_item: Vec<PhysicalItem>,
        free_space: Vec<FreeSpace>,
    }

    pub fn visualize_child(data: &str) {
        let data = serde_json::from_str::<VisualizeData>(data).unwrap();
        visualizer::visualize(data.physical_item, data.free_space);
    }

    pub fn visualize(items: Vec<PhysicalItem>, free_list: Vec<FreeSpace>) {
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
