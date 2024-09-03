use std::{
    future::Future,
    sync::mpsc::{self, Receiver, Sender},
};

use eframe::{
    egui::{
        Context, Frame, Layout, Margin, Response, RichText, ScrollArea, SidePanel, Style, Ui,
        Visuals, Widget,
    },
    emath::Align,
};
use egui_demo_lib::DemoWindows;
use egui_notify::{Toast, Toasts};
use export::ExportMenu;
use import::ImportMenu;
use interaction::InteractionMenu;
use misc::MiscMenu;
use spacing::SpacingMenu;
use visuals::VisualsMenu;

mod export;
mod import;
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
    toasts_tx: Sender<Toast>,
    toasts_rx: Receiver<Toast>,

    import: ImportMenu,
    export: ExportMenu,

    visuals: VisualsMenu,
    misc: MiscMenu,
    spacing: SpacingMenu,
    interaction: InteractionMenu,

    demo: DemoWindows,
}

impl Themer {
    fn new() -> Self {
        let (toasts_tx, toasts_rx) = mpsc::channel();

        Self {
            toasts: Default::default(),
            toasts_tx,
            toasts_rx,
            import: Default::default(),
            export: Default::default(),
            visuals: Default::default(),
            misc: Default::default(),
            spacing: Default::default(),
            interaction: Default::default(),
            demo: Default::default(),
        }
    }
}

impl eframe::App for Themer {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if let Ok(toast) = self.toasts_rx.try_recv() {
            self.toasts.add(toast);
            ctx.request_repaint();
        }

        SidePanel::left("themer_side_panel")
            .min_width(370.0)
            .max_width(ctx.available_rect().width() / 2.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.hyperlink_to(
                    RichText::new("Egui Themer").heading(),
                    "https://github.com/grantshandy/egui-themer/",
                );

                ui.add_space(5.0);
                ui.label("Create an egui theme with live updates then export it to machine-readable Rust source or JSON files.");
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Reset");
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui.button("Default Light").clicked() {
                            ctx.set_style(Style {
                                visuals: Visuals::light(),
                                ..Default::default()
                            });
                            self.visuals = VisualsMenu::default();
                        }

                        if ui.button("Default Dark").clicked() {
                            ctx.set_style(Style {
                                visuals: Visuals::dark(),
                                ..Default::default()
                            });
                            self.visuals = VisualsMenu::default();
                        }
                    });
                });

                ui.separator();

                self.import.ui(ui, ctx, self.toasts_tx.clone());
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
        ui.add_space(3.0);

        let text = RichText::new(name).size(17.0);
        let resp = match url {
            Some(url) => ui.hyperlink_to(text, url),
            None => ui.label(text),
        };

        ui.add_space(3.0);

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

#[cfg(not(target_arch = "wasm32"))]
pub fn execute_future<F: Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
pub fn execute_future<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
