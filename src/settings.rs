use eframe::{
    egui::{
        style::{WidgetVisuals, Widgets},
        Grid, Slider, Style, Ui,
    },
    epaint::Color32,
};

use crate::MiscSettingsState;

pub(crate) fn menu(ui: &mut Ui, style: &mut Style, misc: &mut MiscSettingsState) {
    widgets(ui, &mut style.visuals.widgets, misc);
}

fn widgets(ui: &mut Ui, widgets: &mut Widgets, misc: &mut MiscSettingsState) {
    ui.heading("Widgets");
    widget_setting(
        ui,
        "Open",
        "w-open",
        &mut widgets.open,
        &mut misc.widget_open_rounding_uniform,
    );
    widget_setting(
        ui,
        "Active",
        "w-active",
        &mut widgets.active,
        &mut misc.widget_active_rounding_uniform,
    );
    widget_setting(
        ui,
        "Hovered",
        "w-hovered",
        &mut widgets.hovered,
        &mut misc.widget_hovered_rounding_uniform,
    );
    widget_setting(
        ui,
        "Inactive",
        "w-inactive",
        &mut widgets.inactive,
        &mut misc.widget_inactive_rounding_uniform,
    );
    widget_setting(
        ui,
        "Noninteractive",
        "w-non",
        &mut widgets.noninteractive,
        &mut misc.widget_noninteractive_rounding_uniform,
    );
}

fn widget_setting(
    ui: &mut Ui,
    name: &str,
    id: &str,
    widget: &mut WidgetVisuals,
    rounding_uniform: &mut (bool, f32),
) {
    ui.collapsing(name, |ui| {
        Grid::new(id).num_columns(2).striped(true).show(ui, |ui| {
            color_picker(ui, "Background Fill:", &mut widget.bg_fill);
            ui.end_row();
            color_picker(ui, "Weak Background Fill:", &mut widget.weak_bg_fill);
            ui.end_row();

            ui.label("BG Stroke TODO");
            ui.end_row();
            ui.label("FG Stroke TODO");
            ui.end_row();

            ui.collapsing("Rounding", |ui| {
                ui.checkbox(&mut rounding_uniform.0, "Uniform");
                if rounding_uniform.0 {
                    ui.add(Slider::new(&mut rounding_uniform.1, 0.0..=10.0));
                    
                    widget.rounding.nw = rounding_uniform.1;
                    widget.rounding.ne = rounding_uniform.1;
                    widget.rounding.sw = rounding_uniform.1;
                    widget.rounding.se = rounding_uniform.1;
                } else {
                    ui.add(Slider::new(&mut widget.rounding.nw, 0.0..=10.0).text("NW"));
                    ui.add(Slider::new(&mut widget.rounding.ne, 0.0..=10.0).text("NE"));
                    ui.add(Slider::new(&mut widget.rounding.sw, 0.0..=10.0).text("SW"));
                    ui.add(Slider::new(&mut widget.rounding.se, 0.0..=10.0).text("SE"));                    
                }
            });
            ui.end_row();

            ui.label("Expansion TODO");
            ui.end_row();
        });
    });
}

fn color_picker(ui: &mut Ui, title: &str, color: &mut Color32) {
    ui.horizontal(|ui| {
        ui.label(title);
        ui.color_edit_button_srgba(color);
    });
}
