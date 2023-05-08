use eframe::egui::Style;
use handlebars::{handlebars_helper, Handlebars, JsonValue};
use rust_format::{Formatter, RustFmt};

const TEMPLATE: &str = include_str!("template.rs");

pub fn generate_source(style: &Style, eframe: bool) -> Result<String, String> {
    let mut reg = Handlebars::new();

    reg.register_template_string("template", TEMPLATE)
        .map_err(|err| err.to_string())?;

    reg.register_helper("vec2", Box::new(vec2));
    reg.register_helper("stroke", Box::new(stroke));
    reg.register_helper("rounding", Box::new(rounding));
    reg.register_helper("color32", Box::new(color32));
    reg.register_helper("widgetvisuals", Box::new(widgetvisuals));

    let raw = reg
        .render(
            "template",
            &serde_json::json!({
                "eframe": eframe,
                "style": style,
            }),
        )
        .map_err(|err| err.to_string())?;

    RustFmt::default()
        .format_str(raw)
        .map_err(|err| err.to_string())
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
