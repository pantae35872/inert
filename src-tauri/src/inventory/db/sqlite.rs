use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use directories_next::ProjectDirs;
use rusqlite::Row;
use tokio_rusqlite::{Connection, OpenFlags, params};

use crate::inventory::db::Item;

#[derive(Debug)]
pub struct InventoryDBImpl<I: Item> {
    db: Connection,

    item: PhantomData<I>,
}

#[derive(Debug, Clone)]
pub struct StoredItem<I: Item> {
    pub id: i64,
    pub name: String,
    pub amount: u64,
    pub data: I,
}

impl<I: Item> DerefMut for StoredItem<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<I: Item> Deref for StoredItem<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<I: Item> StoredItem<I> {
    pub fn into_inner(self) -> I {
        self.data
    }
}

impl<I: Item> TryFrom<&Row<'_>> for StoredItem<I> {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: value.get(0)?,
            name: value.get(1)?,
            amount: value.get(2)?,
            data: serde_json::from_str(&value.get::<_, String>(3)?).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    2,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?,
        })
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
                    amount INTEGER,
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

    pub async fn add_item(&self, name: impl AsRef<str>, amount: usize, item: &I) -> i64 {
        let name = name.as_ref().to_string();
        let json_data = serde_json::to_string(item).expect("Serialization failed");

        self.db
            .call(move |conn| {
                conn.execute(
                    "INSERT OR REPLACE INTO items (name, amount, data) VALUES (?1, ?2, ?3)",
                    params![name, amount, json_data],
                )?;
                let id = conn.last_insert_rowid();
                Ok(id)
            })
            .await
            .expect("Failed to save item data")
    }

    pub async fn remove_item_by_id(&self, id: i64) {
        self.db
            .call(move |conn| {
                conn.execute("DELETE FROM items WHERE id = ?1", params![id])?;
                Ok(())
            })
            .await
            .expect("Failed to remove item by ID")
    }

    pub async fn find_item_by_id(&self, id: i64) -> StoredItem<I> {
        self.db
            .call(move |conn| {
                let mut stmt = conn
                    .prepare("SELECT * FROM items WHERE id = ?1")
                    .expect("Prepare failed");

                Ok(stmt.query_row(params![id], |row| StoredItem::try_from(row))?)
            })
            .await
            .expect("Find failed")
    }

    pub async fn find_item_by_name(&self, name: &str) -> Vec<StoredItem<I>> {
        let name = name.to_string();

        self.db
            .call(move |conn| {
                let mut stmt = conn
                    .prepare("SELECT * FROM items WHERE name = ?1")
                    .expect("Prepare failed");

                let rows = stmt
                    .query_map(params![name], |row| StoredItem::try_from(row))
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
                    .query_map([], |row| StoredItem::try_from(row))
                    .expect("Query failed");

                rows.map(|v| v.map_err(tokio_rusqlite::Error::Rusqlite))
                    .collect::<Result<Vec<_>, _>>()
            })
            .await
            .expect("Failed to fetch items")
    }
}
