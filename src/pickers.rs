use eframe::{
    egui::{
        collapsing_header::CollapsingState, style::Selection, Button, DragValue, Label, Layout,
        Margin, Sense, Ui, Widget,
    },
    emath::Align,
    epaint::{Color32, Rounding, Shadow, Stroke, Vec2},
};

use crate::picker_frame;

pub fn color_picker<'a>(
    title: &'a str,
    color: &'a mut Color32,
    default: Color32,
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            let mut state = CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(title),
                false,
            );

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
                ui.separator();
                ui.horizontal(|ui| ui.add(color_row(color)));
            });

            resp
        })
    }
}

pub fn rounding_picker<'a>(
    title: &'a str,
    (uniform_enabled, uniform_rounding): &'a mut (bool, f32),
    rounding: &'a mut Rounding,
    (default_rounding, (default_uniform_enabled, default_uniform_rounding)): (
        Rounding,
        (bool, f32),
    ),
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            let mut state = CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(title),
                false,
            );

            let resp = ui
                .horizontal(|ui| {
                    if ui.add(Label::new(title).sense(Sense::click())).clicked() {
                        *uniform_enabled = !*uniform_enabled;
                    }
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui
                            .add_enabled(
                                (*uniform_enabled
                                    && (*uniform_rounding != default_uniform_rounding))
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
                                .range(0.0..=25.0)
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
                ui.separator();
                ui.columns(4, |cols| {
                    cols[0].add(
                        DragValue::new(&mut rounding.nw)
                            .range(0.0..=40.0)
                            .min_decimals(1)
                            .speed(0.05)
                            .prefix("NW:"),
                    );
                    cols[1].add(
                        DragValue::new(&mut rounding.ne)
                            .range(0.0..=40.0)
                            .min_decimals(1)
                            .speed(0.05)
                            .prefix("NE:"),
                    );
                    cols[2].add(
                        DragValue::new(&mut rounding.sw)
                            .range(0.0..=40.0)
                            .min_decimals(1)
                            .speed(0.05)
                            .prefix("SW:"),
                    );
                    cols[3].add(
                        DragValue::new(&mut rounding.se)
                            .range(0.0..=40.0)
                            .min_decimals(1)
                            .speed(0.05)
                            .prefix("SE:"),
                    );
                })
            });

            resp
        })
    }
}

pub fn shadow_picker<'a>(
    title: &'a str,
    shadow: &'a mut Shadow,
    default: Shadow,
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            let mut state = CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(title),
                false,
            );

            let resp = ui
                .horizontal(|ui| {
                    if ui.add(Label::new(title).sense(Sense::click())).clicked() {
                        state.toggle(ui);
                    }

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui
                            .add_enabled(*shadow != default, Button::new("⟲"))
                            .clicked()
                        {
                            *shadow = default;
                        }
                        ui.color_edit_button_srgba(&mut shadow.color);
                        ui.add(
                            DragValue::new(&mut shadow.spread)
                                .range(0.0..=40.0)
                                .min_decimals(1)
                                .speed(0.05),
                        );
                    });
                })
                .response;

            state.show_body_unindented(ui, |ui| {
                ui.separator();
                ui.horizontal(|ui| ui.add(color_row(&mut shadow.color)));
            });

            resp
        })
    }
}

pub fn stroke_picker<'a>(
    title: &'a str,
    stroke: &'a mut Stroke,
    default: Stroke,
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            let mut state = CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(title),
                false,
            );

            let resp = ui
                .horizontal(|ui| {
                    if ui.add(Label::new(title).sense(Sense::click())).clicked() {
                        state.toggle(ui);
                    }

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui
                            .add_enabled(*stroke != default, Button::new("⟲"))
                            .clicked()
                        {
                            *stroke = default;
                        }
                        ui.color_edit_button_srgba(&mut stroke.color);
                        ui.add(
                            DragValue::new(&mut stroke.width)
                                .range(0.0..=40.0)
                                .min_decimals(1)
                                .speed(0.05),
                        );
                    });
                })
                .response;

            state.show_body_unindented(ui, |ui| {
                ui.separator();
                ui.horizontal(|ui| ui.add(color_row(&mut stroke.color)));
            });

            resp
        })
    }
}

pub fn float_picker<'a>(title: &'a str, float: &'a mut f32, default: f32) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.label(title);
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    if ui
                        .add_enabled(*float != default, Button::new("⟲"))
                        .clicked()
                    {
                        *float = default;
                    }
                    ui.add(
                        DragValue::new(float)
                            .range(0.0..=1000.0)
                            .min_decimals(1)
                            .speed(0.05),
                    )
                });
            })
            .response
        })
    }
}

pub fn bool_picker<'a>(title: &'a str, bool: &'a mut bool, default: bool) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.label(title);
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    if ui.add_enabled(*bool != default, Button::new("⟲")).clicked() {
                        *bool = default;
                    }
                    ui.checkbox(bool, "");
                });
            })
            .response
        })
    }
}

pub fn selection_picker<'a>(
    title: &'a str,
    selection: &'a mut Selection,
    default: Selection,
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            let mut state = CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(title),
                false,
            );

            let resp = ui
                .horizontal(|ui| {
                    if ui.add(Label::new(title).sense(Sense::click())).clicked() {
                        state.toggle(ui);
                    }

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui
                            .add_enabled(*selection != default, Button::new("⟲"))
                            .clicked()
                        {
                            *selection = default;
                        }
                        ui.color_edit_button_srgba(&mut selection.bg_fill);
                        ui.color_edit_button_srgba(&mut selection.stroke.color);
                        ui.add(
                            DragValue::new(&mut selection.stroke.width)
                                .range(0.0..=40.0)
                                .min_decimals(1)
                                .speed(0.05),
                        );
                    });
                })
                .response;

            state.show_body_unindented(ui, |ui| {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Background");
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.color_edit_button_srgba(&mut selection.bg_fill);
                    });
                });
                ui.add(color_row(&mut selection.bg_fill));

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Stroke");
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.color_edit_button_srgba(&mut selection.stroke.color);
                        ui.add(
                            DragValue::new(&mut selection.stroke.width)
                                .range(0.0..=40.0)
                                .min_decimals(1)
                                .speed(0.05)
                                .prefix("Width: "),
                        );
                    });
                });
                ui.add(color_row(&mut selection.stroke.color))
            });

            resp
        })
    }
}

pub fn color_picker_optional<'a>(
    title: &'a str,
    color: &'a mut Option<Color32>,
    default: Option<Color32>,
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            let mut state = CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(title),
                false,
            );

            let resp = ui
                .horizontal(|ui| {
                    ui.label(title);
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui
                            .add_enabled(*color != default, Button::new("⟲"))
                            .clicked()
                        {
                            *color = default;
                        }

                        if let Some(color) = color {
                            ui.color_edit_button_srgba(color);
                        }

                        let mut update = color.is_some();
                        if ui.checkbox(&mut update, "Override").clicked() {
                            *color = match color.is_some() {
                                true => None,
                                false => Some(Color32::WHITE),
                            }
                        }
                    });
                })
                .response;

            state.set_open(color.is_some());
            state.show_body_unindented(ui, |ui| {
                ui.separator();
                if let Some(color) = color {
                    ui.horizontal(|ui| ui.add(color_row(color)));
                }
            });

            resp
        })
    }
}

pub fn vec2_picker<'a>(title: &'a str, vec: &'a mut Vec2, default: Vec2) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.label(title);
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    if ui.add_enabled(*vec != default, Button::new("⟲")).clicked() {
                        *vec = default;
                    }

                    ui.add(
                        DragValue::new(&mut vec.y)
                            .prefix("Y:")
                            .range(0.0..=100.0)
                            .min_decimals(1)
                            .speed(0.05),
                    );
                    ui.add(
                        DragValue::new(&mut vec.x)
                            .prefix("X:")
                            .range(0.0..=100.0)
                            .min_decimals(1)
                            .speed(0.05),
                    )
                })
            })
            .response
        })
    }
}

pub fn margin_picker<'a>(
    title: &'a str,
    margin: &'a mut Margin,
    default: Margin,
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.label(title);
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    if ui
                        .add_enabled(*margin != default, Button::new("⟲"))
                        .clicked()
                    {
                        *margin = default;
                    }

                    ui.add(
                        DragValue::new(&mut margin.left)
                            .prefix("⬅:")
                            .range(0.0..=100.0)
                            .min_decimals(1)
                            .speed(0.05),
                    );
                    ui.add(
                        DragValue::new(&mut margin.right)
                            .prefix("➡:")
                            .range(0.0..=100.0)
                            .min_decimals(1)
                            .speed(0.05),
                    );
                    ui.add(
                        DragValue::new(&mut margin.top)
                            .prefix("⬆:")
                            .range(0.0..=100.0)
                            .min_decimals(1)
                            .speed(0.05),
                    );
                    ui.add(
                        DragValue::new(&mut margin.bottom)
                            .prefix("⬇:")
                            .range(0.0..=100.0)
                            .min_decimals(1)
                            .speed(0.05),
                    )
                });
            })
            .response
        })
    }
}

fn color_row(color: &mut Color32) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        ui.columns(4, |cols| {
            cols[0].add(DragValue::new(&mut color[0]).prefix("R:"));
            cols[1].add(DragValue::new(&mut color[1]).prefix("G:"));
            cols[2].add(DragValue::new(&mut color[2]).prefix("B:"));
            cols[3].add(DragValue::new(&mut color[3]).prefix("A:"))
        })
    }
}
