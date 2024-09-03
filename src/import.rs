use std::sync::mpsc::Sender;

use eframe::egui::{Context, Direction, Layout, Style, Ui};
use egui_notify::Toast;

#[derive(Default)]
pub struct ImportMenu;

impl ImportMenu {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context, toasts_tx: Sender<Toast>) {
        ui.add(crate::section_title("Import", None));

        ui.allocate_ui_with_layout(
            [ui.available_width(), 0.0].into(),
            Layout::centered_and_justified(Direction::TopDown),
            |ui| {
                if ui.button("Import JSON").clicked() {
                    self.import(toasts_tx, ctx.clone());
                }
            },
        );
    }

    fn import(&self, toasts_tx: Sender<Toast>, ctx: Context) {
        let task = rfd::AsyncFileDialog::new()
            .add_filter("JSON file", &["json"])
            .pick_file();

        crate::execute_future(async move {
            let file = task.await;
            if let Some(file) = file {
                match serde_json::from_slice::<Style>(&file.read().await) {
                    Ok(style) => {
                        ctx.set_style(style);
                        toasts_tx.send(Toast::info("Import Successful")).unwrap();
                    }
                    Err(e) => {
                        toasts_tx
                            .send(Toast::error(format!("Import Failed: {e}")))
                            .unwrap();
                    }
                }
                ctx.request_repaint();
            }
        });
    }
}
