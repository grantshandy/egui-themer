use eframe::{
    egui::{Grid, Label, Layout,  RichText, ScrollArea, SidePanel, Ui, Visuals, Widget},
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
                    add_double_row(ui, Label::new("Reset to Default:"), |ui| {
                        if ui.button("Light").clicked() {
                            style.visuals = Visuals::light();
                            self.visuals = VisualsMenu::default();
                        }

                        if ui.button("Dark").clicked() {
                            style.visuals = Visuals::dark();
                            self.visuals = VisualsMenu::default();
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
    ui.with_layout(Layout::right_to_left(Align::Center), right);
    ui.end_row();
}

pub fn section_title(ui: &mut Ui, name: &str) {
    ui.label(RichText::new(name).size(15.0));
}
