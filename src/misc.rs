use eframe::{
    egui::{Button, ComboBox, Layout, Style, Ui},
    emath::Align,
};

use crate::{
    picker_frame,
    pickers::{bool_picker, float_picker},
    section_title,
};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct MiscMenu;

impl MiscMenu {
    pub fn ui(&mut self, ui: &mut Ui, style: &mut Style) {
        ui.add(section_title(
            "Miscellaneous",
            Some("https://docs.rs/egui/0.21.0/egui/style/struct.Style.html"),
        ));

        ui.add(float_picker(
            "Animation Time",
            &mut style.animation_time,
            Style::default().animation_time,
        ));
        ui.add(bool_picker(
            "Explanation Tooltips",
            &mut style.explanation_tooltips,
            Style::default().explanation_tooltips,
        ));
        wrap_picker(ui, &mut style.wrap);
    }
}

fn wrap_picker(ui: &mut Ui, wrap: &mut Option<bool>) {
    ui.add(|ui: &mut Ui| {
        picker_frame(ui)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Wrap");
                    ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                        if ui.add_enabled(*wrap != None, Button::new("⟲")).clicked() {
                            *wrap = None;
                        }

                        ComboBox::new("Wrap", "")
                            .selected_text(match wrap {
                                None => "Follow Layout",
                                Some(true) => "Default On",
                                Some(false) => "Default Off",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(wrap, None, "None - Follow Layout");
                                ui.selectable_value(wrap, Some(true), "Some(true) - Default On");
                                ui.selectable_value(wrap, Some(false), "Some(false) - Default Off");
                            });
                    });
                });
            })
            .response
    });
}
