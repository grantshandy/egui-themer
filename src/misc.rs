use eframe::{
    egui::{Button, ComboBox, Layout, Style, Ui},
    emath::Align,
};
use eframe::egui::TextWrapMode;
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
        wrap_picker(ui, &mut style.wrap_mode);
        picker_frame(ui, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.label("Text Styles");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label("Edit Text Styles Manually in Export")
                });
            })
            .response
        });
    }
}

fn wrap_picker(ui: &mut Ui, wrap_mode: &mut Option<TextWrapMode>) {
    let mut wrap = wrap_mode.iter().map(|x| {
        match x {
            TextWrapMode::Extend => false,
            TextWrapMode::Wrap => true,
            TextWrapMode::Truncate => false,
        }
    }).next();
    ui.add(|ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.label("Wrap");
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    if ui.add_enabled(wrap.is_some(), Button::new("⟲")).clicked() {
                        wrap = None;
                    }

                    ComboBox::new("Wrap", "")
                        .selected_text(match wrap {
                            None => "Follow Layout",
                            Some(true) => "Default On",
                            Some(false) => "Default Off",
                        })
                        .show_ui(ui, |ui| {
                            let mut changed = false;
                            changed |= ui.selectable_value(&mut wrap, None, "None - Follow Layout").changed();
                            changed |= ui.selectable_value(&mut wrap, Some(true), "Some(true) - Default On").changed();
                            changed |= ui.selectable_value(&mut wrap, Some(false), "Some(false) - Default Off").changed();
                            if changed {
                                *wrap_mode = wrap.map(|wrap| {
                                    if wrap {
                                        TextWrapMode::Wrap
                                    } else {
                                        TextWrapMode::Truncate
                                    }
                                });
                            }
                        });
                });
            })
            .response
        })
    });
}
