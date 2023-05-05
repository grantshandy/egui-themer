use std::{fs, sync::Arc, thread};

use eframe::{
    egui::{menu, ScrollArea, SidePanel, Style, TopBottomPanel},
    NativeOptions,
};
use egui_demo_lib::DemoWindows;
use handlebars::{handlebars_helper, Handlebars, JsonValue};
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageLevel};
use rust_format::{Formatter, RustFmt};

mod exampletheme;

const TEMPLATE: &str = include_str!("template.rs");

fn main() {
    eframe::run_native(
        "egui-themer",
        NativeOptions::default(),
        Box::new(|cc| Box::new(Themer::new(cc))),
    )
    .expect("run eframe native app");
}

#[derive(Default)]
struct Themer {
    demo: DemoWindows,
}

impl Themer {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_style(exampletheme::style());

        Self {
            demo: DemoWindows::default(),
        }
    }
}

impl eframe::App for Themer {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("themer_top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                if ui.button("Export").clicked() {
                    let style = ctx.style().clone();

                    thread::spawn(move || {
                        let Some(file) = FileDialog::new()
                            .set_title("Set Save File")
                            .add_filter("Rust Source Code", &["rs", "RS"])
                            .save_file() else { return };

                        let src = match generate_source(style) {
                            Ok(src) => src,
                            Err(err) => {
                                MessageDialog::new()
                                    .set_title("Export Error")
                                    .set_description(&format!("Error generating source: {err}"))
                                    .set_level(MessageLevel::Error)
                                    .set_buttons(MessageButtons::Ok)
                                    .show();

                                return;
                            }
                        };

                        if let Err(err) = fs::write(file, src) {
                            MessageDialog::new()
                                .set_title("Export Error")
                                .set_description(&format!("Error writing source to disk: {err}"))
                                .set_level(MessageLevel::Error)
                                .set_buttons(MessageButtons::Ok)
                                .show();
                        }
                    });
                }
            })
        });

        SidePanel::left("themer_side_panel").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Egui Themer");
                ui.label("Edit your theme in real time and export it to a source file for no-overhead theming!");
                ui.separator();
                ctx.settings_ui(ui);
            });
        });

        self.demo.ui(ctx);
    }
}

fn generate_source(style: Arc<Style>) -> Result<String, String> {
    let mut reg = Handlebars::new();

    reg.register_template_string("template", TEMPLATE)
        .map_err(|err| err.to_string())?;

    reg.register_helper("vec2", Box::new(vec2));
    reg.register_helper("stroke", Box::new(stroke));
    reg.register_helper("rounding", Box::new(rounding));
    reg.register_helper("color32", Box::new(color32));
    reg.register_helper("widgetvisuals", Box::new(widgetvisuals));

    let raw = reg
        .render("template", &style)
        .map_err(|err| err.to_string())?;

    RustFmt::default()
        .format_str(raw)
        .map_err(|err| err.to_string())
}

fn gencolor32(value: &JsonValue) -> String {
    format!(
        "Color32::from_rgba_premultiplied({}, {}, {}, {})",
        value[0], value[1], value[2], value[3]
    )
}

fn genstroke(value: &JsonValue) -> String {
    format!(
        "Stroke {{
            width: {},
            color: {},
        }}",
        value["width"],
        gencolor32(&value["color"]),
    )
}

fn genrounding(value: &JsonValue) -> String {
    format!(
        "Rounding {{ nw: {}, ne: {}, sw: {}, se: {} }}",
        value["nw"], value["ne"], value["sw"], value["se"]
    )
}

handlebars_helper!(vec2: |value: JsonValue| format!("Vec2 {{ x: {}, y: {}}}", &value["x"], &value["y"]));
handlebars_helper!(stroke: |value: JsonValue| genstroke(&value));
handlebars_helper!(rounding: |value: JsonValue| genrounding(&value));
handlebars_helper!(color32: |color: JsonValue| gencolor32(&color));
handlebars_helper!(widgetvisuals: |value: JsonValue| {
    format!("WidgetVisuals {{
        bg_fill: {},
        weak_bg_fill: {},
        bg_stroke: {},
        rounding: {},
        fg_stroke: {},
        expansion: {},
    }}",
        gencolor32(&value["bg_fill"]),
        gencolor32(&value["weak_bg_fill"]),
        genstroke(&value["bg_stroke"]),
        genrounding(&value["rounding"]),
        genstroke(&value["fg_stroke"]),
        value["expansion"]
    )
});

