use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use eframe::{
    egui::{
        Frame, Hyperlink, Layout, Margin, Response, RichText, ScrollArea, SidePanel, Style, Ui,
        Visuals, Widget,
    },
    emath::Align,
};
use egui_demo_lib::DemoWindows;
use egui_notify::Toasts;
use export::ExportMenu;
use interaction::InteractionMenu;
use misc::MiscMenu;
use spacing::SpacingMenu;
use visuals::VisualsMenu;
use import::ImportMenu;

mod export;
mod interaction;
mod misc;
mod pickers;
mod spacing;
mod visuals;
mod import;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eframe::run_native(
        "Egui Themer",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Themer::new()))),
    )
    .expect("run eframe native app");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "app",
                eframe::WebOptions::default(),
                Box::new(|_| Ok(Box::new(Themer::new()))),
            )
            .await
            .expect("failed to start eframe");
    });
}

struct Themer {
    toasts: Toasts,
    toasts_tx: SyncSender<Box<dyn Fn(&mut Toasts) + Send>>,
    toasts_rx: Receiver<Box<dyn Fn(&mut Toasts) + Send>>,
    demo: DemoWindows,
    import: ImportMenu,
    export: ExportMenu,
    visuals: VisualsMenu,
    misc: MiscMenu,
    spacing: SpacingMenu,
    interaction: InteractionMenu,
}

impl Themer {
    fn new() -> Self {
        let (toasts_tx, toasts_rx) = sync_channel(100);
        Self {
            toasts: Default::default(),
            toasts_tx,
            toasts_rx,
            demo: Default::default(),
            import: Default::default(),
            export: Default::default(),
            visuals: Default::default(),
            misc: Default::default(),
            spacing: Default::default(),
            interaction: Default::default(),
        }
    }
}

impl eframe::App for Themer {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(f) = self.toasts_rx.try_recv() {
            f(&mut self.toasts);
            ctx.request_repaint();
        }

        SidePanel::left("themer_side_panel")
            .min_width(370.0)
            .show(ctx, |ui| {
                ui.hyperlink_to(
                    RichText::new("Egui Themer").heading(),
                    "https://github.com/grantshandy/egui-themer/",
                );
                ui.label("Create an egui theme and export it to a Rust source file.");
                ui.columns(2, |cols| {
                    cols[0].label("Reset:");
                    cols[1].with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui.button("Light").clicked() {
                            let style = Style {
                                visuals: Visuals::light(),
                                ..Default::default()
                            };
                            ctx.set_style(style);
                            self.visuals = VisualsMenu::default();
                        }

                        if ui.button("Dark").clicked() {
                            let style = Style {
                                visuals: Visuals::dark(),
                                ..Default::default()
                            };
                            ctx.set_style(style);
                            self.visuals = VisualsMenu::default();
                        }
                    })
                });
                ui.separator();

                self.import.ui(ui, ctx, &self.toasts_tx);
                ui.separator();

                let mut style = (*ctx.style()).clone();

                self.export.ui(ui, &style, &mut self.toasts);
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

        self.toasts.show(ctx);
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

#[macro_export]
macro_rules! egui_doc_link {
    ($e:expr) => {
        concat!("https://docs.rs/egui/0.28.1/egui", $e)
    };
}
