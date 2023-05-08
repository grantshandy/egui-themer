use eframe::{
    egui::{Grid, Label, Layout, RichText, ScrollArea, SidePanel, Ui, Visuals, Widget},
    emath::Align,
    NativeOptions,
};
use egui_demo_lib::DemoWindows;
use export::ExportMenu;
use visuals::VisualsMenu;

mod export;
mod visuals;

fn main() {
    eframe::run_native(
        "Egui Themer",
        NativeOptions::default(),
        Box::new(|_| Box::new(Themer::default())),
    )
    .expect("run eframe native app");
}

#[derive(Default)]
struct Themer {
    demo: DemoWindows,
    export: ExportMenu,
    visuals: VisualsMenu,
}

impl eframe::App for Themer {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("themer_side_panel").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let mut style = (*ctx.style()).clone();

                ui.heading("Egui Themer");
                ui.label("Create an egui theme and export it to a Rust source file.");
                Grid::new("reset_settings").num_columns(2).show(ui, |ui| {
                    add_double_row(ui, Label::new("Reset:"), |ui| {
                        if ui.button("Light").clicked() {
                            style.visuals = Visuals::light();
                        }

                        if ui.button("Dark").clicked() {
                            style.visuals = Visuals::dark();
                        }
                    });
                });
                ui.separator();

                self.export.ui(ui, ctx, &style);
                ui.separator();

                self.visuals.ui(ui, &mut style.visuals);
                ui.separator();

                ctx.set_style(style);
            });
        });

        self.demo.ui(ctx);
    }
}

pub fn add_double_row(ui: &mut Ui, left: impl Widget, right: impl FnOnce(&mut Ui)) {
    ui.add(left);
    ui.allocate_ui_with_layout(
        [ui.available_width(), 0.0].into(),
        Layout::right_to_left(Align::Center),
        right,
    );
    ui.end_row();
}

pub fn section_title(ui: &mut Ui, name: &str) {
    ui.label(RichText::new(name).size(15.0));
}

#[macro_export]
macro_rules! egui_doclink {
    ($label: expr, $search_term: expr) => {{
        eframe::egui::Hyperlink::from_label_and_url(
            $label,
            format!("https://docs.rs/egui/0.21.0/egui/?search={}", $search_term),
        )
    }};
}

#[macro_export]
macro_rules! color_picker {
    ($ui:expr, $title:expr, $name:expr, $color:expr) => {
        eframe::egui::collapsing_header::CollapsingState::load_with_default_open(
            $ui.ctx(),
            $ui.make_persistent_id($name),
            false,
        )
        .show_header($ui, |ui| {
            ui.add(egui_doclink!($title, $name));
            ui.with_layout(
                eframe::egui::Layout::right_to_left(eframe::egui::Align::Min),
                |ui| {
                    ui.color_edit_button_srgba($color);
                },
            )
        })
        .body(|ui| {
            ui.horizontal(|ui| {
                ui.label("R:");
                ui.add(eframe::egui::DragValue::new(&mut $color[0]));
                ui.label("G:");
                ui.add(eframe::egui::DragValue::new(&mut $color[1]));
                ui.label("B:");
                ui.add(eframe::egui::DragValue::new(&mut $color[2]));
                ui.label("A:");
                ui.add(eframe::egui::DragValue::new(&mut $color[3]));
            })
        });
    };
}
