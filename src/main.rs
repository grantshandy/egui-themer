use eframe::{
    egui::{Frame, Layout, Margin, RichText, SidePanel, Ui, Visuals},
    emath::Align,
    NativeOptions,
};
use egui_demo_lib::DemoWindows;
use export::ExportMenu;
use visuals::VisualsMenu;

mod export;
mod pickers;
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
        SidePanel::left("themer_side_panel")
            .min_width(315.0)
            .show(ctx, |ui| {
                let mut style = (*ctx.style()).clone();

                ui.heading("Egui Themer");
                ui.label("Create an egui theme and export it to a Rust source file.");
                ui.columns(2, |cols| {
                    cols[0].label("Reset:");
                    cols[1].with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui.button("Light").clicked() {
                            style.visuals = Visuals::light();
                            self.visuals = VisualsMenu::default();
                        }

                        if ui.button("Dark").clicked() {
                            style.visuals = Visuals::dark();
                            self.visuals = VisualsMenu::default();
                        }
                    })
                });
                ui.separator();

                self.export.ui(ui, ctx, &style);
                ui.separator();

                self.visuals.ui(ui, &mut style.visuals);

                ctx.set_style(style);
            });

        self.demo.ui(ctx);
    }
}

pub fn section_title(ui: &mut Ui, name: &str) {
    ui.label(RichText::new(name).size(17.0));
    ui.add_space(2.0);
}

pub fn picker_frame(ui: &Ui) -> Frame {
    let style = ui.style();

    Frame {
        //     inner_margin: style.spacing.menu_margin,
        inner_margin: Margin::same(4.0),
        rounding: style.visuals.menu_rounding,
        //     shadow: Shadow::NONE,
        //     fill: style.visuals.window_fill(),
        //     stroke: style.visuals.window_stroke(),
        fill: style.visuals.extreme_bg_color,
        ..Frame::none()
    }
}
