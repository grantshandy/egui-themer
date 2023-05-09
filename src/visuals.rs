use eframe::{
    egui::{
        collapsing_header::CollapsingState, Button, DragValue, Label, Layout, Sense, Slider, Ui,
        Visuals, Widget,
    },
    emath::Align,
    epaint::{Color32, Rounding},
};

use crate::{add_double_row, section_title};

pub struct VisualsMenu {
    window_rounding: (bool, f32),
}

impl Default for VisualsMenu {
    fn default() -> Self {
        Self {
            window_rounding: (true, 6.0),
        }
    }
}

impl VisualsMenu {
    pub fn ui(&mut self, ui: &mut Ui, visuals: &mut Visuals) {
        let visuals_default = if visuals.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };
        let overall_default = Self::default();

        section_title(ui, "Visuals");

        ui.horizontal(|ui| {
            if ui
                .add(Label::new("Dark Mode").sense(Sense::click()))
                .clicked()
            {
                visuals.dark_mode = !visuals.dark_mode;
            };
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.selectable_value(&mut visuals.dark_mode, false, "☀ Light");
                ui.selectable_value(&mut visuals.dark_mode, true, "🌙 Dark");
            });
        });

        ui.add(color_picker(
            "Hyperlink",
            &mut visuals.hyperlink_color,
            visuals_default.hyperlink_color,
        ));
        ui.add(color_picker(
            "Faint Background",
            &mut visuals.faint_bg_color,
            visuals_default.faint_bg_color,
        ));
        ui.add(color_picker(
            "Extreme Background",
            &mut visuals.extreme_bg_color,
            visuals_default.extreme_bg_color,
        ));
        ui.add(color_picker(
            "Code Background",
            &mut visuals.code_bg_color,
            visuals_default.code_bg_color,
        ));
        ui.add(color_picker(
            "Warning Foreground",
            &mut visuals.warn_fg_color,
            visuals_default.warn_fg_color,
        ));
        ui.add(color_picker(
            "Error Foreground",
            &mut visuals.error_fg_color,
            visuals_default.error_fg_color,
        ));

        ui.add(rounding(
            "Window Rounding",
            &mut self.window_rounding,
            &mut visuals.window_rounding,
            (
                visuals_default.window_rounding,
                overall_default.window_rounding,
            ),
        ));
        ui.add(color_picker(
            "Window Fill",
            &mut visuals.window_fill,
            visuals_default.window_fill,
        ));
    }
}

fn color_picker<'a>(title: &'a str, color: &'a mut Color32, default: Color32) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        let mut state =
            CollapsingState::load_with_default_open(ui.ctx(), ui.make_persistent_id(title), false);

        let resp = ui
            .horizontal(|ui| {
                if ui.add(Label::new(title).sense(Sense::click())).clicked() {
                    state.toggle(ui);
                }

                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    if ui
                        .add_enabled(*color != default, Button::new("⟲"))
                        .clicked()
                    {
                        *color = default;
                    }
                    ui.color_edit_button_srgba(color);
                });
            })
            .response;

        state.show_body_unindented(ui, |ui| {
            ui.allocate_space([ui.available_width(), 1.75].into());
            ui.columns(4, |cols| {
                cols[0].add(DragValue::new(&mut color[0]).prefix("R: "));
                cols[1].add(DragValue::new(&mut color[1]).prefix("G: "));
                cols[2].add(DragValue::new(&mut color[2]).prefix("B: "));
                cols[3].add(DragValue::new(&mut color[3]).prefix("A: "));
            });
            ui.separator();
        });

        resp
    }
}

fn rounding<'a>(
    title: &'a str,
    (uniform_enabled, uniform_rounding): &'a mut (bool, f32),
    rounding: &'a mut Rounding,
    (default_rounding, (default_uniform_enabled, default_uniform_rounding)): (
        Rounding,
        (bool, f32),
    ),
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        let mut state =
            CollapsingState::load_with_default_open(ui.ctx(), ui.make_persistent_id(title), false);

        let resp = ui
            .horizontal(|ui| {
                if ui.add(Label::new(title).sense(Sense::click())).clicked() {
                    *uniform_enabled = !*uniform_enabled;
                }
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    if ui
                        .add_enabled(
                            (*uniform_enabled && (*uniform_rounding != default_uniform_rounding))
                                || (*rounding != default_rounding),
                            Button::new("⟲"),
                        )
                        .clicked()
                    {
                        *rounding = default_rounding;
                        (*uniform_enabled, *uniform_rounding) =
                            (default_uniform_enabled, default_uniform_rounding);
                    }

                    ui.add_enabled(
                        *uniform_enabled,
                        DragValue::new(uniform_rounding)
                            .clamp_range(0.0..=40.0)
                            .min_decimals(1)
                            .speed(0.05),
                    );
                    ui.checkbox(uniform_enabled, "Uniform");

                    if *uniform_enabled {
                        rounding.nw = *uniform_rounding;
                        rounding.ne = *uniform_rounding;
                        rounding.sw = *uniform_rounding;
                        rounding.se = *uniform_rounding;
                    }
                });
            })
            .response;

        state.set_open(!*uniform_enabled);
        state.show_body_unindented(ui, |ui| {
            ui.allocate_space([ui.available_width(), 1.75].into());
            ui.columns(4, |cols| {
                cols[0].add(
                    DragValue::new(&mut rounding.nw)
                        .clamp_range(0.0..=40.0)
                        .min_decimals(1)
                        .speed(0.05)
                        .prefix("NW: "),
                );
                cols[1].add(
                    DragValue::new(&mut rounding.ne)
                        .clamp_range(0.0..=40.0)
                        .min_decimals(1)
                        .speed(0.05)
                        .prefix("NE: "),
                );
                cols[2].add(
                    DragValue::new(&mut rounding.sw)
                        .clamp_range(0.0..=40.0)
                        .min_decimals(1)
                        .speed(0.05)
                        .prefix("SW: "),
                );
                cols[3].add(
                    DragValue::new(&mut rounding.se)
                        .clamp_range(0.0..=40.0)
                        .min_decimals(1)
                        .speed(0.05)
                        .prefix("SE: "),
                );
            });
            ui.separator();
        });

        resp
    }
}
