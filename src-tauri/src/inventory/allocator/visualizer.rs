use eframe::{Frame, egui};
use egui::{Color32, PopupAnchor, Rect, RichText, Stroke};

use crate::inventory::{allocator::FreeSpace, db::PhysicalItem};

struct VisualizerApp {
    items: Vec<PhysicalItem>,
    free_list: Vec<FreeSpace>,
}

pub fn visualize(items: Vec<PhysicalItem>, free_list: Vec<FreeSpace>) {
    println!("Visualizing items: {items:?} free_list: {free_list:?}");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Item Allocator Visualizer",
        native_options,
        Box::new(|cc| Ok(Box::new(VisualizerApp::new(cc, items, free_list)))),
    )
    .unwrap();
}

impl VisualizerApp {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        items: Vec<PhysicalItem>,
        free_list: Vec<FreeSpace>,
    ) -> Self {
        Self { items, free_list }
    }
}

impl eframe::App for VisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Item Allocator Visualization");

            let (response, painter) =
                ui.allocate_painter(egui::Vec2::new(400.0, 400.0), egui::Sense::hover());

            let rect = response.rect;
            let scale = 1.0;

            // Draw background
            painter.rect_filled(rect, 0.0, egui::Color32::WHITE);

            let mut lables_rect: Vec<(Rect, String)> = Vec::new();

            // Draw Items (Green)
            for item in &self.items {
                let rect = Rect::from_min_size(
                    rect.left_top() + egui::vec2(item.pos_x as f32, item.pos_y as f32) * scale,
                    egui::vec2(item.width as f32, item.height as f32) * scale,
                );

                painter.rect_filled(rect, 0.0, egui::Color32::LIGHT_RED);

                let stroke = Stroke::new(3.0, Color32::WHITE); // Change thickness and color as needed
                painter.rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Inside);

                lables_rect.push((
                    rect,
                    format!(
                        "Item \"{}\", X: {}, Y: {}, Width: {}, Height: {}",
                        item.display_name, item.pos_x, item.pos_y, item.width, item.height
                    ),
                ));
            }

            // Draw FreeSpaces (Red => Blue => ORANGE => GOLD)
            for spaces in self.free_list.chunks(4) {
                let mut color = egui::Color32::GREEN;
                for space in spaces {
                    let rect = Rect::from_min_size(
                        rect.left_top() + egui::vec2(space.x as f32, space.y as f32) * scale,
                        egui::vec2(space.width as f32, space.height as f32) * scale,
                    );

                    painter.rect_filled(rect, 0.0, color);

                    let stroke = Stroke::new(3.0, Color32::BLACK); // Change thickness and color as needed
                    painter.rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Inside);

                    lables_rect.push((
                        rect,
                        format!(
                            "Free X: {}, Y: {}, Width: {}, Height: {}",
                            space.x, space.y, space.width, space.height
                        ),
                    ));

                    color = match color {
                        Color32::GREEN => Color32::BLUE,
                        Color32::BLUE => Color32::ORANGE,
                        Color32::ORANGE => Color32::LIGHT_BLUE,
                        _ => Color32::RED,
                    };
                }
            }

            if let Some(pointer_pos) = response.hover_pos() {
                if let Some(name) = lables_rect.iter().find_map(|(rect, s)| {
                    if rect.contains(pointer_pos) {
                        Some(s)
                    } else {
                        None
                    }
                }) {
                    egui::Tooltip::always_open(
                        ui.ctx().clone(),
                        painter.layer_id(),
                        egui::Id::new("Tooltip"),
                        PopupAnchor::Pointer,
                    )
                    .layout(egui::Layout::top_down_justified(egui::Align::Center))
                    .show(|ui| {
                        ui.label(RichText::new(name).size(18.0));
                    });
                }
            }
        });
    }
}
