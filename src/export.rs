use std::{future::Future, time::Duration};

use eframe::{
    egui::{ComboBox, Context, Direction, Layout, Style, Ui},
    emath::Align,
};
use egui_notify::Toasts;
use handlebars::{handlebars_helper, Handlebars, JsonValue};
use rfd::AsyncFileDialog;

#[cfg(not(target_arch = "wasm32"))]
use rust_format::{Formatter, RustFmt};

use crate::section_title;

const TEMPLATE: &str = include_str!("template.rs.hbs");

#[derive(Default)]
pub struct ExportMenu {
    toasts: Toasts,
    eframe: bool,
    export_format: ExportFormat,
    json_pretty: bool,
}

impl ExportMenu {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context, style: &Style) {
        ui.add(section_title("Export", None));

        ui.with_layout(Layout::top_down(Align::Max), |ui| {
            ui.checkbox(&mut self.eframe, "Eframe Support");

            ComboBox::from_label("Export Format")
                .selected_text(match self.export_format {
                    ExportFormat::RustSource => "Rust Source Code",
                    ExportFormat::Json => "JSON",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.export_format,
                        ExportFormat::RustSource,
                        "Rust Source Code",
                    );
                    ui.selectable_value(&mut self.export_format, ExportFormat::Json, "JSON");
                });

            if self.export_format == ExportFormat::Json {
                ui.checkbox(&mut self.json_pretty, "Pretty JSON");
            }
        });

        ui.allocate_ui_with_layout(
            [ui.available_width(), 0.0].into(),
            Layout::centered_and_justified(Direction::TopDown),
            |ui| {
                if ui.button("Export").clicked() {
                    self.export(style);
                }
            },
        );

        self.toasts.show(ctx);
    }

    pub fn export(&mut self, style: &Style) {
        let generated = match (self.export_format, self.json_pretty) {
            (ExportFormat::RustSource, _) => self.generate_source(style),
            (ExportFormat::Json, true) => serde_json::to_string_pretty(&style).map_err(|e| e.to_string()),
            (ExportFormat::Json, false) => serde_json::to_string(&style).map_err(|e| e.to_string()),
        };

        match generated {
            Ok(result) => {
                let dialog = AsyncFileDialog::new()
                    .set_file_name(format!("style.{}", self.export_format.file_extension()))
                    .add_filter(
                        self.export_format.description(),
                        &[self.export_format.file_extension()],
                    )
                    .save_file();

                execute(async move {
                    let file = dialog.await;
                    if let Some(file) = file {
                        _ = file.write(result.as_bytes()).await;
                    }
                });
            }
            Err(err) => {
                self.toasts
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
    pub fn file_extension(self) -> &'static str {
        match self {
            ExportFormat::RustSource => "rs",
            ExportFormat::Json => "json",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            ExportFormat::RustSource => "Rust Source Code",
            ExportFormat::Json => "JSON",
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

#[cfg(not(target_arch = "wasm32"))]
pub fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
pub fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
