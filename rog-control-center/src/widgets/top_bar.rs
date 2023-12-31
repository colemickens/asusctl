use egui::{vec2, Align2, FontId, Id, Sense};

use crate::system_state::SystemState;
use crate::{RogApp, VERSION};

impl RogApp {
    pub fn top_bar(
        &mut self,
        states: &mut SystemState,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.horizontal(|ui| {
                    self.dark_light_mode_buttons(ui);
                    egui::warn_if_debug_build(ui);
                    if ui.button("Quit app").clicked() {
                        states.run_in_bg = false;
                        frame.close();
                    }
                });

                // Drag area
                let text_color = ctx.style().visuals.text_color();
                let mut titlebar_rect = ui.available_rect_before_wrap();
                titlebar_rect.max.x -= titlebar_rect.height();
                if ui
                    .interact(titlebar_rect, Id::new("title_bar"), Sense::drag())
                    .drag_started()
                {
                    frame.drag_window();
                }

                let height = titlebar_rect.height();

                // Paint the title:
                ui.painter().text(
                    titlebar_rect.right_top() + vec2(0.0, height / 2.0),
                    Align2::RIGHT_CENTER,
                    format!("v{}", VERSION),
                    FontId::proportional(height - 2.0),
                    text_color,
                );
                // // Add the close button:
                // let close_response = ui.put(
                //     egui::Rect::from_min_size(titlebar_rect.right_top(),
                // egui::Vec2::splat(height)),
                //     egui::Button::new(egui::RichText::new("❌").size(height -
                // 4.0)).frame(false), );
                // if close_response.clicked() {
                //     frame.close();
                // }
            });
        });
    }

    fn dark_light_mode_buttons(&mut self, ui: &mut egui::Ui) {
        let load_from_cfg = self.config.dark_mode != ui.ctx().style().visuals.dark_mode;

        if ui
            .add(egui::SelectableLabel::new(
                !self.config.dark_mode,
                "☀ Light",
            ))
            .clicked()
            || (load_from_cfg && !self.config.dark_mode)
        {
            ui.ctx().set_visuals(egui::Visuals::light());
        }
        if ui
            .add(egui::SelectableLabel::new(self.config.dark_mode, "🌙 Dark"))
            .clicked()
            || (load_from_cfg && self.config.dark_mode)
        {
            ui.ctx().set_visuals(egui::Visuals::dark());
        }

        let applied_dark_mode = ui.ctx().style().visuals.dark_mode;

        if self.config.dark_mode != applied_dark_mode {
            self.config.dark_mode = applied_dark_mode;
            let tmp = self.config.enabled_notifications.clone();
            self.config.save(&tmp).ok();
        }
    }
}
