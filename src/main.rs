use eframe::{
    egui::{
        Frame, Hyperlink, Layout, Margin, Response, RichText, ScrollArea, SidePanel, Style, Ui,
        Visuals, Widget,
    },
    emath::Align,
};
use egui_demo_lib::DemoWindows;
use export::ExportMenu;
use interaction::InteractionMenu;
use misc::MiscMenu;
use spacing::SpacingMenu;
use visuals::VisualsMenu;

mod export;
mod interaction;
mod misc;
mod pickers;
mod spacing;
mod visuals;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eframe::run_native(
        "Egui Themer",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Themer::default()))),
    )
    .expect("run eframe native app");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new().start(
                "the_canvas_id",
                eframe::WebOptions::default(),
                Box::new(|_| Ok(Box::new(Themer::default()))),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[derive(Default)]
struct Themer {
    demo: DemoWindows,
    export: ExportMenu,
    visuals: VisualsMenu,
    misc: MiscMenu,
    spacing: SpacingMenu,
    interaction: InteractionMenu,
}

impl eframe::App for Themer {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("themer_side_panel")
            .min_width(370.0)
            .show(ctx, |ui| {
                let mut style = (*ctx.style()).clone();

                ui.heading("Egui Themer");
                ui.label("Create an egui theme and export it to a Rust source file.");
                ui.columns(2, |cols| {
                    cols[0].label("Reset:");
                    cols[1].with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui.button("Light").clicked() {
                            style = Style {
                                visuals: Visuals::light(),
                                ..Default::default()
                            };
                            self.visuals = VisualsMenu::default();
                        }

                        if ui.button("Dark").clicked() {
                            style = Style {
                                visuals: Visuals::dark(),
                                ..Default::default()
                            };
                            self.visuals = VisualsMenu::default();
                        }
                    })
                });
                ui.separator();

                self.export.ui(ui, ctx, &style);
                ui.separator();

                ScrollArea::both().show(ui, |ui| {
                    self.visuals.ui(ui, &mut style.visuals);
                    ui.separator();

                    self.spacing.ui(ui, &mut style.spacing);
                    ui.separator();

                    self.interaction.ui(ui, &mut style.interaction);
                    ui.separator();

                    self.misc.ui(ui, &mut style);
                });

                ctx.set_style(style);
            });

        self.demo.ui(ctx);
    }
}

pub fn section_title<'a>(name: &'a str, url: Option<&'a str>) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        let resp = match url {
            Some(url) => ui.add(Hyperlink::from_label_and_url(
                RichText::new(name).size(17.0),
                url,
            )),
            None => ui.label(RichText::new(name).size(17.0)),
        };

        ui.add_space(2.0);

        resp
    }
}

pub fn picker_frame(ui: &mut Ui, show: impl Widget) -> Response {
    let style = ui.style();

    Frame {
        inner_margin: Margin::same(4.0),
        rounding: style.visuals.menu_rounding,
        fill: style.visuals.extreme_bg_color,
        ..Frame::none()
    }
    .show(ui, |ui| {
        ui.add(show);
    })
    .response
}
