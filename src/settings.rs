use std::{fs, path::PathBuf, time::Duration};

use eframe::egui::{
    Align, Button, Context, Grid, Label, Layout, RichText, Style, Ui, Visuals, Widget,
};
use egui_file::FileDialog;
use egui_notify::Toasts;

use crate::egui_doclink;
use crate::gen_template::generate_source;

#[derive(Default)]
pub struct Settings {
    export: Export,
}

impl Settings {
    pub fn ui(&mut self, ctx: &Context, ui: &mut Ui) {
        let mut style = (*ctx.style()).clone();

        ui.heading("Egui Themer");
        ui.label(
            "Edit your theme in real time and export it to a source file for no-overhead theming!",
        );
        ui.separator();

        self.export.ui(ui, ctx, &style);
        ui.separator();

        // visuals(ui, &mut style.visuals);

        ctx.style_ui(ui);

        ctx.set_style(style);
    }
}

#[derive(Default)]
struct Export {
    path: Option<PathBuf>,
    toasts: Toasts,
    file_dialog: Option<FileDialog>,
    eframe: bool,
}

impl Export {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context, style: &Style) {
        ui.label(RichText::new("Export").size(15.0));

        Grid::new("export_settings").num_columns(2).show(ui, |ui| {
            add_double_row(ui, Label::new("eframe"), |ui| {
                ui.checkbox(&mut self.eframe, "");
            });
            add_double_row(ui, Label::new("Path"), |ui| {
                if ui
                    .button(
                        self.path
                            .as_ref()
                            .map(|f| {
                                f.file_name()
                                    .map(|n| n.to_str().unwrap_or("invalid!"))
                                    .unwrap_or("Select a File!")
                            })
                            .unwrap_or("Select"),
                    )
                    .clicked()
                {
                    let mut dialog = FileDialog::save_file(None);
                    dialog.open();
                    self.file_dialog = Some(dialog);
                }

                if let Some(dialog) = &mut self.file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            self.path = Some(file);
                        }
                    }
                }
            });
        });

        if ui.button("Export").clicked() {
            if let Some(path) = &self.path {
                match generate_source(style, self.eframe) {
                    Ok(src) => match fs::write(path, src) {
                        Ok(_) => self.toasts.success("Exported!"),
                        Err(err) => self.toasts.error(format!("Export Error: {err}")),
                    },
                    Err(err) => self.toasts.error(format!("Export Error: {err}")),
                }
            } else {
                self.toasts.error("You must select a save file to export!")
            }
            .set_duration(Some(Duration::from_secs(5)));
        }
        self.toasts.show(ctx);
    }
}

fn visuals(ui: &mut Ui, visuals: &mut Visuals) {
    ui.label(RichText::new("Visuals").size(15.0));

    Grid::new("visuals_settings").num_columns(2).show(ui, |ui| {
        add_double_row(ui, Label::new("Reset"), |ui| {
            if ui.button("Light").clicked() {
                *visuals = Visuals::light();
            }

            if ui.button("Dark").clicked() {
                *visuals = Visuals::dark();
            }
        });
        add_double_row(ui, egui_doclink!("Dark Mode", "Visuals::dark_mode"), |ui| {
            ui.selectable_value(&mut visuals.dark_mode, false, "☀ Light");
            ui.selectable_value(&mut visuals.dark_mode, true, "🌙 Dark");
        });
    });
}

fn add_double_row(ui: &mut Ui, left: impl Widget, right: impl FnOnce(&mut Ui)) {
    ui.add(left);
    ui.with_layout(Layout::right_to_left(Align::Center), right);
    ui.end_row();
}
