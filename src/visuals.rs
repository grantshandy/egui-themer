use eframe::egui::{Ui, Visuals};

use crate::{add_double_row, color_picker, egui_doclink, section_title};

#[derive(Default)]
pub struct VisualsMenu;

impl VisualsMenu {
    pub fn ui(&mut self, ui: &mut Ui, visuals: &mut Visuals) {
        section_title(ui, "Visuals");

        add_double_row(
            ui,
            egui_doclink!("Dark Mode:", "Visuals::dark_mode"),
            |ui| {
                ui.selectable_value(&mut visuals.dark_mode, false, "☀ Light");
                ui.selectable_value(&mut visuals.dark_mode, true, "🌙 Dark");
            },
        );

        color_picker!(
            ui,
            "Hyperlink Color",
            "Visuals::hyperlink_color",
            &mut visuals.hyperlink_color
        );
        color_picker!(
            ui,
            "Faint Background Color",
            "Visuals::faint_bg_color",
            &mut visuals.faint_bg_color
        );
        color_picker!(
            ui,
            "Extreme Background Color",
            "Visuals::extreme_bg_color",
            &mut visuals.extreme_bg_color
        );
        color_picker!(
            ui,
            "Code Background Color",
            "Visuals::code_bg_color",
            &mut visuals.code_bg_color
        );
        color_picker!(
            ui,
            "Warn Foreground Color",
            "Visuals::warn_fg_color",
            &mut visuals.warn_fg_color
        );
        color_picker!(
            ui,
            "Error Foreground Color",
            "Visuals::error_fg_color",
            &mut visuals.error_fg_color
        );
    }
}
