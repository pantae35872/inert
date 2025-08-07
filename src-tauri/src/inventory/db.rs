use std::path::{Path, PathBuf};

use directories_next::ProjectDirs;
use rand::{Rng, distr::Alphanumeric};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{
    fs::{File, create_dir},
    io::AsyncWriteExt,
};
use warp::Filter;

use crate::inventory::{
    Rectangle,
    db::sqlite::{InventoryDBImpl, StoredItem},
};

pub trait Item: Serialize + DeserializeOwned + Send + 'static + Clone {}

impl<T> Item for T where T: Serialize + DeserializeOwned + Send + 'static + Clone {}

pub mod sqlite;

type InventoryDB = InventoryDBImpl<super::Item>;

#[derive(Debug)]
pub struct Database {
    db: InventoryDB,

    image_db_path: PathBuf,
}

impl Database {
    pub async fn new() -> Self {
        let proj_dir =
            ProjectDirs::from("io.github", "pantae35872", "inert").expect("No data directory");

        let image_db = proj_dir.data_dir().join("image_db");
        if !image_db.exists() {
            create_dir(&image_db)
                .await
                .expect("Create image directory failed");
        }

        let image_db_2 = image_db.clone();
        tokio::spawn(async move {
            // Route: GET /images/{filename...}
            let images_route = warp::path("item_images").and(warp::path::tail()).and_then(
                move |tail: warp::path::Tail| {
                    let base_dir = image_db_2.clone();
                    async move {
                        let full_path = base_dir.join(tail.as_str());

                        if full_path.exists() && full_path.is_file() {
                            Ok(warp::reply::with_header(
                                tokio::fs::read(full_path)
                                    .await
                                    .unwrap_or_else(|_| Vec::new()),
                                "Content-Type",
                                "image/jpeg",
                            )) as Result<_, warp::Rejection>
                        } else {
                            Err(warp::reject::not_found())
                        }
                    }
                },
            );

            warp::serve(images_route).run(([127, 0, 0, 1], 5000)).await;
        });

        Self {
            db: InventoryDB::new().await,
            image_db_path: image_db,
        }
    }

    pub async fn add_item(
        &self,
        name: impl AsRef<str>,
        amount: usize,
        rect: Rectangle,
        img: Vec<u8>,
    ) -> i64 {
        let image_name: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        let image_path = self.image_db_path.join(image_name.as_str());
        let mut image_file = File::create(&image_path)
            .await
            .expect("Fail to open image file");
        image_file
            .write_all(img.as_slice())
            .await
            .expect("Fail to write to an image file");

        self.db
            .add_item(
                name.as_ref(),
                amount,
                &super::Item {
                    rect,
                    display_name: name.as_ref().to_string(),
                    image_id: format!("http://127.0.0.1:5000/item_images/{image_name}"),
                },
            )
            .await
    }

    pub async fn remove_item_by_id(&self, id: i64) {
        self.db.remove_item_by_id(id).await
    }

    pub async fn find_item_by_id(&self, id: i64) -> StoredItem<super::Item> {
        self.db.find_item_by_id(id).await
    }

    pub async fn find_item_by_name(&self, name: &str) -> Vec<StoredItem<super::Item>> {
        self.db.find_item_by_name(name).await
    }

    pub async fn list_all_items(&self) -> Vec<StoredItem<super::Item>> {
        self.db.list_all_items().await
    }
}
