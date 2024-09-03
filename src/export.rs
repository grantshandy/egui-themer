use std::time::Duration;

use eframe::{
    egui::{ComboBox, Direction, Layout, Style, Ui},
    emath::Align,
};
use egui_notify::Toasts;
use handlebars::{handlebars_helper, Handlebars, JsonValue};
use rfd::AsyncFileDialog;

#[cfg(not(target_arch = "wasm32"))]
use rust_format::{Formatter, RustFmt};

const TEMPLATE: &str = include_str!("template.rs.hbs");

#[derive(Default)]
pub struct ExportMenu {
    eframe: bool,
    export_format: ExportFormat,
    json_pretty: bool,
}

impl ExportMenu {
    pub fn ui(&mut self, ui: &mut Ui, style: &Style, toasts: &mut Toasts) {
        ui.add(crate::section_title("Export", None));

        ui.horizontal(|ui| {
            ui.label("Export Format");

            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ComboBox::from_label("")
                    .selected_text(self.export_format.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.export_format,
                            ExportFormat::RustSource,
                            ExportFormat::RustSource.name(),
                        );
                        ui.selectable_value(
                            &mut self.export_format,
                            ExportFormat::Json,
                            ExportFormat::Json.name(),
                        );
                    });
            });
        });

        match self.export_format {
            ExportFormat::RustSource => {
                ui.horizontal(|ui| {
                    ui.label("Eframe Support");
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.checkbox(&mut self.eframe, "")
                    });
                });
            }
            ExportFormat::Json => {
                ui.horizontal(|ui| {
                    ui.label("Pretty JSON");
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.checkbox(&mut self.json_pretty, "")
                    });
                });
            }
        }

        ui.allocate_ui_with_layout(
            [ui.available_width(), 0.0].into(),
            Layout::centered_and_justified(Direction::TopDown),
            |ui| {
                if ui
                    .button(format!("Export {}", self.export_format.name()))
                    .clicked()
                {
                    self.export(style, toasts);
                }
            },
        );
    }

    pub fn export(&mut self, style: &Style, toasts: &mut Toasts) {
        let generated = match (self.export_format, self.json_pretty) {
            (ExportFormat::RustSource, _) => self.generate_source(style),
            (ExportFormat::Json, true) => {
                serde_json::to_string_pretty(&style).map_err(|e| e.to_string())
            }
            (ExportFormat::Json, false) => serde_json::to_string(&style).map_err(|e| e.to_string()),
        };

        match generated {
            Ok(result) => {
                let dialog = AsyncFileDialog::new()
                    .set_file_name(format!("style.{}", self.export_format.extension()))
                    .add_filter(self.export_format.name(), &[self.export_format.extension()])
                    .save_file();

                crate::execute_future(async move {
                    if let Some(file) = dialog.await {
                        _ = file.write(result.as_bytes()).await;
                    }
                });
            }
            Err(err) => {
                toasts
                    .error(format!("Export Error: {err}"))
                    .set_duration(Some(Duration::from_secs(5)));
            }
        }
    }

    fn generate_source(&self, style: &Style) -> Result<String, String> {
        let mut reg = Handlebars::new();

        reg.register_template_string("template", TEMPLATE)
            .map_err(|err| err.to_string())?;

        reg.register_helper("vec2", Box::new(vec2));
        reg.register_helper("stroke", Box::new(stroke));
        reg.register_helper("rounding", Box::new(rounding));
        reg.register_helper("color32", Box::new(color32));
        reg.register_helper("widgetvisuals", Box::new(widgetvisuals));

        let res = reg
            .render(
                "template",
                &serde_json::json!({
                    "eframe": self.eframe,
                    "style": style,
                }),
            )
            .map_err(|err| err.to_string())?;

        #[cfg(not(target_arch = "wasm32"))]
        let res = RustFmt::default()
            .format_str(res)
            .map_err(|err| err.to_string())?;

        Ok(res)
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum ExportFormat {
    #[default]
    RustSource,
    Json,
}

impl ExportFormat {
    pub fn name(self) -> &'static str {
        match self {
            ExportFormat::RustSource => "Rust Source",
            ExportFormat::Json => "JSON",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            ExportFormat::RustSource => "rs",
            ExportFormat::Json => "json",
        }
    }
}

handlebars_helper!(vec2: |value: JsonValue| format!("Vec2 {{ x: {}, y: {}}}", &value["x"], &value["y"]));
handlebars_helper!(stroke: |value: JsonValue| gen_stroke(&value));
handlebars_helper!(rounding: |value: JsonValue| gen_rounding(&value));
handlebars_helper!(color32: |color: JsonValue| gen_color32(&color));
handlebars_helper!(widgetvisuals: |value: JsonValue| {
    format!("WidgetVisuals {{
        bg_fill: {},
        weak_bg_fill: {},
        bg_stroke: {},
        rounding: {},
        fg_stroke: {},
        expansion: {},
    }}",
        gen_color32(&value["bg_fill"]),
        gen_color32(&value["weak_bg_fill"]),
        gen_stroke(&value["bg_stroke"]),
        gen_rounding(&value["rounding"]),
        gen_stroke(&value["fg_stroke"]),
        value["expansion"]
    )
});

fn gen_color32(value: &JsonValue) -> String {
    format!(
        "Color32::from_rgba_premultiplied({}, {}, {}, {})",
        value[0], value[1], value[2], value[3]
    )
}

fn gen_stroke(value: &JsonValue) -> String {
    format!(
        "Stroke {{
            width: {},
            color: {},
        }}",
        value["width"],
        gen_color32(&value["color"]),
    )
}

fn gen_rounding(value: &JsonValue) -> String {
    format!(
        "Rounding {{ nw: {}, ne: {}, sw: {}, se: {} }}",
        value["nw"], value["ne"], value["sw"], value["se"]
    )
}
