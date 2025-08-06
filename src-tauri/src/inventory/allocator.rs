use crate::inventory::db::{InventoryDB, PhysicalItem};

use serde::{Deserialize, Serialize};

#[cfg(feature = "visualization")]
pub mod visualizer;

#[derive(Debug)]
pub struct ItemAllocator {
    free_list: Vec<FreeSpace>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "visualization", derive(Serialize, Deserialize))]
pub struct FreeSpace {
    x: usize,
    y: usize,

    width: usize,
    height: usize,
}

impl ItemAllocator {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;

    pub async fn new(db: &InventoryDB) -> Self {
        let mut free_list = vec![FreeSpace {
            x: 0,
            y: 0,
            width: Self::WIDTH,
            height: Self::HEIGHT,
        }];

        let items = db.list_all_items().await;

        for item in &items {
            let mut new_list = Vec::new();

            for space in free_list.drain(..) {
                let mut split = subtract_item(&space, &item.data);
                new_list.append(&mut split);
            }

            free_list = new_list;
        }

        #[cfg(feature = "visualization")]
        {
            super::visualizer::visualize(
                items.iter().map(|e| e.clone().into_inner()).collect(),
                free_list.clone(),
            );
        }

        Self { free_list }
    }
}

fn subtract_item(space: &FreeSpace, item: &PhysicalItem) -> Vec<FreeSpace> {
    let mut result = Vec::new();

    let item_right = item.pos_x + item.width;
    let item_bottom = item.pos_y + item.height;
    let space_right = space.x + space.width;
    let space_bottom = space.y + space.height;

    // Check if they intersect
    if item.pos_x >= space_right
        || item_right <= space.x
        || item.pos_y >= space_bottom
        || item_bottom <= space.y
    {
        // No intersection, return the original space
        result.push(space.clone());
        return result;
    }

    // Top slice
    if item.pos_y > space.y {
        result.push(FreeSpace {
            x: space.x,
            y: space.y,
            width: space.width,
            height: item.pos_y - space.y,
        });
    }

    // Bottom slice
    if item_bottom < space_bottom {
        result.push(FreeSpace {
            x: space.x,
            y: item_bottom,
            width: space.width,
            height: space_bottom - item_bottom,
        });
    }

    // Left slice
    if item.pos_x > space.x {
        let top = item.pos_y.max(space.y);
        let bottom = item_bottom.min(space_bottom);
        if bottom > top {
            result.push(FreeSpace {
                x: space.x,
                y: top,
                width: item.pos_x - space.x,
                height: bottom - top,
            });
        }
    }

    // Right slice
    if item_right < space_right {
        let top = item.pos_y.max(space.y);
        let bottom = item_bottom.min(space_bottom);
        if bottom > top {
            result.push(FreeSpace {
                x: item_right,
                y: top,
                width: space_right - item_right,
                height: bottom - top,
            });
        }
    }

    result
}
