use std::process;

use eframe::{
    egui::{self, panel::Side, Frame, SidePanel, Style},
    NativeOptions,
};

mod settings;

fn main() {
    if let Err(err) = eframe::run_native(
        "egui-themer",
        NativeOptions::default(),
        Box::new(|_| Box::new(EguiThemer::default())),
    ) {
        eprintln!("Error running window: {err}");
        process::exit(1);
    }
}

#[derive(Default)]
struct EguiThemer {
    demo: egui_demo_lib::DemoWindows,
    misc: MiscSettingsState,
}

impl eframe::App for EguiThemer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style: Style = (*ctx.style()).clone();

        SidePanel::new(Side::Left, "theme_settings")
            .frame(Frame::side_top_panel(&Style::default()).inner_margin(12.0))
            .min_width(150.0)
            .default_width(75.0)
            .show(ctx, |ui| {
                *ui.style_mut() = Style::default();
                settings::menu(ui, &mut style, &mut self.misc);
            });

        self.demo.ui(ctx);

        ctx.set_style(style);
    }
}

struct MiscSettingsState {
    pub widget_open_rounding_uniform: (bool, f32),
    pub widget_active_rounding_uniform: (bool, f32),
    pub widget_hovered_rounding_uniform: (bool, f32),
    pub widget_inactive_rounding_uniform: (bool, f32),
    pub widget_noninteractive_rounding_uniform: (bool, f32),
}

impl Default for MiscSettingsState {
    fn default() -> Self {
        Self {
            widget_open_rounding_uniform: (false, 2.0),
            widget_active_rounding_uniform: (false, 2.0),
            widget_hovered_rounding_uniform: (false, 2.0),
            widget_inactive_rounding_uniform: (false, 2.0),
            widget_noninteractive_rounding_uniform: (false, 2.0),
        }
    }
}
