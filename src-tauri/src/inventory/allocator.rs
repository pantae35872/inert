use std::collections::VecDeque;

use crate::inventory::{Rectangle, db::Database};

#[cfg(feature = "visualization")]
pub mod visualizer;

#[derive(Debug)]
pub struct ItemAllocator {
    free_list: Vec<Rectangle>,
}

impl ItemAllocator {
    pub async fn new(db: &Database, width: usize, height: usize) -> Self {
        let mut free_list = vec![Rectangle {
            x: 0,
            y: 0,
            width,
            height,
        }];

        let items = db.list_all_items().await;

        for item in &items {
            let mut new_list = Vec::new();

            for space in free_list.drain(..) {
                let mut split = subtract_rect(&space, &item.data.rect);
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

fn get_overlap(a: &Rectangle, b: &Rectangle) -> Option<Rectangle> {
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
