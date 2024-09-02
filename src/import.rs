use eframe::egui::{Direction, Layout, Style, Ui};
use crate::export::execute;
use crate::section_title;

#[derive(Default)]
pub struct ImportMenu {
}

impl ImportMenu {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context) {
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
                    execute(async move {
                        let file = task.await;
                        if let Some(file) = file {
                            let text = file.read().await;
                            let string = String::from_utf8_lossy(&text).to_string();
                            match serde_json::from_str::<Style>(&string) {
                                Ok(style) => {
                                    ctx.set_style(style);
                                }
                                Err(_) => {
                                    println!("Failed to parse JSON");
                                }
                            }
                        }
                    });
                }
            }
        );
    }
}
