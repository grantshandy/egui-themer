use eframe::{
    egui::{ScrollArea, SidePanel},
    NativeOptions,
};
use egui_demo_lib::DemoWindows;
use settings::Settings;

mod gen_template;
mod settings;

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
    settings: Settings,
}

impl eframe::App for Themer {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("themer_side_panel").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.settings.ui(ctx, ui);
            });
        });

        self.demo.ui(ctx);
    }
}

#[macro_export]
macro_rules! egui_doclink {
    ($label: expr, $search_term: expr) => {{
        let url = format!("https://docs.rs/egui/0.21.0/egui/?search={}", $search_term);
        eframe::egui::Hyperlink::from_label_and_url($label, url)
    }};
}