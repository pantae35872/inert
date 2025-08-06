use std::marker::PhantomData;

use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio_rusqlite::{Connection, OpenFlags, params};

pub type InventoryDB = InventoryDBImpl<PhysicalItem>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PhysicalItem {
    pub pos_x: usize,
    pub pos_y: usize,

    pub width: usize,
    pub height: usize,

    pub image_path: String,
    pub display_name: String,
}

pub trait Item: Serialize + DeserializeOwned + Send + 'static + Clone {}

impl<T> Item for T where T: Serialize + DeserializeOwned + Send + 'static + Clone {}

#[derive(Debug)]
pub struct InventoryDBImpl<I: Item> {
    db: Connection,

    item: PhantomData<I>,
}

#[derive(Debug, Clone)]
pub struct StoredItem<I: Item> {
    pub id: i64,
    pub name: String,
    pub data: I,
}

impl<I: Item> StoredItem<I> {
    pub fn into_inner(self) -> I {
        self.data
    }
}

impl<I: Item> InventoryDBImpl<I> {
    pub async fn new() -> Self {
        let proj_dir =
            ProjectDirs::from("io.github", "pantae35872", "inert").expect("No data directory");

        let db = Connection::open_with_flags(
            proj_dir.data_dir().join("item_db.sqlite"),
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )
        .await
        .expect("Database open failed");
        db.call(|conn| {
            Ok(conn.execute(
                r#"
                CREATE TABLE IF NOT EXISTS items (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    data TEXT NOT NULL
                )
                "#,
                params![],
            )?)
        })
        .await
        .expect("Failed to create items table");

        Self {
            db,
            item: PhantomData,
        }
    }

    pub async fn add_item(&self, name: impl AsRef<str>, item: &I) {
        let name = name.as_ref().to_string();
        let json_data = serde_json::to_string(item).expect("Serialization failed");

        self.db
            .call(move |conn| {
                Ok(conn.execute(
                    "INSERT OR REPLACE INTO items (name, data) VALUES (?1, ?2)",
                    params![name, json_data],
                )?)
            })
            .await
            .expect("Failed to save item data");
    }

    pub async fn find_item_by_name(&self, name: &str) -> Vec<StoredItem<I>> {
        let name = name.to_string();

        self.db
            .call(move |conn| {
                let mut stmt = conn
                    .prepare("SELECT * FROM items WHERE name = ?1")
                    .expect("Prepare failed");

                let rows = stmt
                    .query_map(params![name], |row| {
                        let id: i64 = row.get(0)?;
                        let name: String = row.get(1)?;
                        let data_str: String = row.get(2)?;
                        let data: I = serde_json::from_str(&data_str).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                2,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;
                        Ok(StoredItem { id, name, data })
                    })
                    .expect("Query failed");

                rows.map(|v| v.map_err(tokio_rusqlite::Error::Rusqlite))
                    .collect::<Result<Vec<_>, _>>()
            })
            .await
            .expect("Find failed")
    }

    pub async fn list_all_items(&self) -> Vec<StoredItem<I>> {
        self.db
            .call(|conn| {
                let mut stmt = conn
                    .prepare("SELECT * FROM items")
                    .expect("Failed to prepare SELECT statement");

                let rows = stmt
                    .query_map([], |row| {
                        let id: i64 = row.get(0)?;
                        let name: String = row.get(1)?;
                        let data_str: String = row.get(2)?;
                        let data: I = serde_json::from_str(&data_str).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                0,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;
                        Ok(StoredItem { id, name, data })
                    })
                    .expect("Query failed");

                rows.map(|v| v.map_err(tokio_rusqlite::Error::Rusqlite))
                    .collect::<Result<Vec<_>, _>>()
            })
            .await
            .expect("Failed to fetch items")
    }
}
