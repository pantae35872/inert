use eframe::{Frame, egui};
use egui::{Color32, PopupAnchor, Rect, RichText, Stroke};

use crate::inventory::{Item, Rectangle};

struct VisualizerApp {
    items: Vec<Item>,
    free_list: Vec<Rectangle>,
}

pub fn visualize(items: Vec<Item>, free_list: Vec<Rectangle>) {
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
        items: Vec<Item>,
        free_list: Vec<Rectangle>,
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
                    rect.left_top() + egui::vec2(item.rect.x as f32, item.rect.y as f32) * scale,
                    egui::vec2(item.rect.width as f32, item.rect.height as f32) * scale,
                );

                painter.rect_filled(rect, 0.0, egui::Color32::LIGHT_RED);

                let stroke = Stroke::new(3.0, Color32::WHITE); // Change thickness and color as needed
                painter.rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Inside);

                lables_rect.push((
                    rect,
                    format!(
                        "Item \"{}\", X: {}, Y: {}, Width: {}, Height: {}",
                        item.display_name,
                        item.rect.x,
                        item.rect.y,
                        item.rect.width,
                        item.rect.height
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
