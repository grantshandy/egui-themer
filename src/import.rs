use std::sync::mpsc::SyncSender;
use eframe::egui::{Direction, Layout, Style, Ui};
use egui_notify::Toasts;
use crate::export::execute;
use crate::section_title;

#[derive(Default)]
pub struct ImportMenu {
}

impl ImportMenu {
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        ctx: &eframe::egui::Context,
        toasts_tx: &SyncSender<Box<dyn Fn(&mut Toasts) + Send>>,
    ) {
        ui.add(section_title("Import", None));

        ui.allocate_ui_with_layout(
            [ui.available_width(), 0.0].into(),
            Layout::centered_and_justified(Direction::TopDown),
            |ui| {
                if ui.button("Import JSON").clicked() {
                    let task = rfd::AsyncFileDialog::new()
                        .add_filter("JSON file", &["json"])
                        .pick_file();
                    let ctx = ctx.clone();
                    let toasts_tx = toasts_tx.clone();
                    execute(async move {
                        let file = task.await;
                        if let Some(file) = file {
                            let text = file.read().await;
                            let string = String::from_utf8_lossy(&text).to_string();
                            match serde_json::from_str::<Style>(&string) {
                                Ok(style) => {
                                    ctx.set_style(style);
                                    toasts_tx.try_send(Box::new(|toasts| {
                                        toasts.info("Import successful");
                                    })).unwrap();
                                }
                                Err(e) => {
                                    let message = format!("Import failed: {}", e);
                                    toasts_tx.try_send(Box::new(move |toasts| {
                                        toasts.error(message.clone());
                                    })).unwrap();
                                }
                            }
                            ctx.request_repaint();
                        }
                    });
                }
            }
        );
    }
}
