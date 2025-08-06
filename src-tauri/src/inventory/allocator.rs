use std::collections::VecDeque;

use crate::inventory::db::{InventoryDB, PhysicalItem};

use serde::{Deserialize, Serialize};

#[cfg(feature = "visualization")]
pub mod visualizer;

#[derive(Debug)]
pub struct ItemAllocator {
    free_list: Vec<FreeSpace>,
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "visualization", derive(Serialize, Deserialize))]
pub struct FreeSpace {
    x: usize,
    y: usize,

    width: usize,
    height: usize,
}

impl From<Rectangle> for FreeSpace {
    fn from(value: Rectangle) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

impl From<FreeSpace> for Rectangle {
    fn from(value: FreeSpace) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
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

    fn test_alloc(
        &mut self,
        width: usize,
        height: usize,
        custom_item: &mut Vec<PhysicalItem>,
        name: impl ToString,
    ) -> Rectangle {
        let area = self.allocate(width, height).expect("Fails to allocate");
        custom_item.push(PhysicalItem {
            pos_x: area.x,
            pos_y: area.y,
            width: area.width,
            height: area.height,

            image_path: String::new(),
            display_name: name.to_string(),
        });

        area
    }

    pub fn deallocate(&mut self, rect: Rectangle) {
        // TODO: VERIFY?
        self.free_list.push(rect.into());
    }

    /// Allocate a new area and returns the x y position
    pub fn allocate(&mut self, width: usize, height: usize) -> Option<Rectangle> {
        for free in &self.free_list {
            for x in free.x..free.x + free.width {
                for y in free.y..free.y + free.height {
                    if self.is_valid_in_free_list(x, y, width, height) {
                        let mut queue = VecDeque::new();
                        queue.push_back(Rectangle {
                            x,
                            y,
                            width,
                            height,
                        });

                        let mut new_free = Vec::new();

                        while let Some(rect) = queue.pop_front() {
                            let mut i = 0;
                            while i < self.free_list.len() {
                                let space = &self.free_list[i];

                                if let Some(overlap) = get_overlap(&rect, space) {
                                    let remaining_parts = subtract_rect(&rect, &overlap);
                                    for r in remaining_parts {
                                        queue.push_back(r);
                                    }
                                    let new_space = subtract_rect(&space.clone().into(), &rect);

                                    self.free_list.remove(i);

                                    for free in new_space {
                                        new_free.push(free.into());
                                    }

                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                        }

                        self.free_list.extend_from_slice(&new_free);

                        return Some(Rectangle {
                            x,
                            y,
                            width,
                            height,
                        });
                    }
                }
            }
        }
        None
    }

    fn is_valid_in_free_list(&self, x: usize, y: usize, width: usize, height: usize) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(Rectangle {
            x,
            y,
            width,
            height,
        });

        while let Some(rect) = queue.pop_front() {
            let mut found_overlap = false;

            for space in &self.free_list {
                if let Some(overlap) = get_overlap(&rect, space) {
                    found_overlap = true;

                    // Subtract the overlapping part, queue the remaining regions
                    let remaining_parts = subtract_rect(&rect, &overlap);
                    for r in remaining_parts {
                        queue.push_back(r);
                    }

                    break;
                }
            }

            if !found_overlap {
                return false;
            }
        }

        true
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

/// Subtracts `sub` from `rect`, returns the list of remaining rectangles
fn subtract_rect(rect: &Rectangle, sub: &Rectangle) -> Vec<Rectangle> {
    let mut result = Vec::new();

    let rect_right = rect.x + rect.width;
    let rect_bottom = rect.y + rect.height;
    let sub_right = sub.x + sub.width;
    let sub_bottom = sub.y + sub.height;

    // Top
    if sub.y > rect.y {
        result.push(Rectangle {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: sub.y - rect.y,
        });
    }

    // Bottom
    if sub_bottom < rect_bottom {
        result.push(Rectangle {
            x: rect.x,
            y: sub_bottom,
            width: rect.width,
            height: rect_bottom - sub_bottom,
        });
    }

    // Left
    if sub.x > rect.x {
        let top = sub.y.max(rect.y);
        let bottom = sub_bottom.min(rect_bottom);
        result.push(Rectangle {
            x: rect.x,
            y: top,
            width: sub.x - rect.x,
            height: bottom - top,
        });
    }

    // Right
    if sub_right < rect_right {
        let top = sub.y.max(rect.y);
        let bottom = sub_bottom.min(rect_bottom);
        result.push(Rectangle {
            x: sub_right,
            y: top,
            width: rect_right - sub_right,
            height: bottom - top,
        });
    }

    result
}

fn get_overlap(a: &Rectangle, b: &FreeSpace) -> Option<Rectangle> {
    let x1 = a.x.max(b.x);
    let y1 = a.y.max(b.y);
    let x2 = (a.x + a.width).min(b.x + b.width);
    let y2 = (a.y + a.height).min(b.y + b.height);

    if x1 < x2 && y1 < y2 {
        Some(Rectangle {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        })
    } else {
        None
    }
}
