use eframe::{
    egui::{style::WidgetVisuals, Button, Label, Layout, Response, Sense, Ui, Visuals, Widget},
    emath::Align,
};

use crate::{
    picker_frame,
    pickers::{
        bool_picker, color_picker, color_picker_optional, float_picker, rounding_picker,
        selection_picker, shadow_picker, stroke_picker,
    },
    section_title,
};

pub struct VisualsMenu {
    tab_state: TabState,
    widget_tab_state: WidgetTabState,
    visuals_default: Visuals,
    window_rounding: (bool, f32),
    menu_rounding: (bool, f32),
    noninteractive_rounding: (bool, f32),
    inactive_rounding: (bool, f32),
    hovered_rounding: (bool, f32),
    active_rounding: (bool, f32),
    open_rounding: (bool, f32),
}

impl Default for VisualsMenu {
    fn default() -> Self {
        Self {
            tab_state: TabState::Misc,
            widget_tab_state: WidgetTabState::NonInteractive,
            visuals_default: Visuals::dark(),
            window_rounding: (true, 6.0),
            menu_rounding: (true, 6.0),
            noninteractive_rounding: (true, 2.0),
            inactive_rounding: (true, 2.0),
            hovered_rounding: (true, 3.0),
            active_rounding: (true, 2.0),
            open_rounding: (true, 2.0),
        }
    }
}

impl VisualsMenu {
    pub fn ui(&mut self, ui: &mut Ui, visuals: &mut Visuals) {
        self.visuals_default = if visuals.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        ui.add(section_title(
            "Visuals",
            Some(crate::egui_doc_link!("/style/struct.Visuals.html")),
        ));

        self.tab_state.show(ui);
        ui.separator();

        match self.tab_state {
            TabState::Misc => self.misc(ui, visuals),
            TabState::Colors => self.colors(ui, visuals),
            TabState::Window => self.window(ui, visuals),
            TabState::Widgets => self.widgets(ui, visuals),
        }
    }

    fn misc(&mut self, ui: &mut Ui, visuals: &mut Visuals) {
        ui.add(dark_light_mode_picker(&mut visuals.dark_mode));
        ui.add(color_picker_optional(
            "Override Text Color",
            &mut visuals.override_text_color,
            self.visuals_default.override_text_color,
        ));
        ui.add(selection_picker(
            "Selection",
            &mut visuals.selection,
            self.visuals_default.selection,
        ));
        ui.add(rounding_picker(
            "Menu Rounding",
            &mut self.menu_rounding,
            &mut visuals.menu_rounding,
            (
                self.visuals_default.menu_rounding,
                Self::default().menu_rounding,
            ),
        ));
        ui.add(color_picker(
            "Panel Fill",
            &mut visuals.panel_fill,
            self.visuals_default.panel_fill,
        ));
        ui.add(shadow_picker(
            "Popup Shadow",
            &mut visuals.popup_shadow,
            self.visuals_default.popup_shadow,
        ));
        ui.add(float_picker(
            "Resize Corner Size",
            &mut visuals.resize_corner_size,
            self.visuals_default.resize_corner_size,
        ));
        ui.add(float_picker(
            "Text Cursor Width",
            &mut visuals.text_cursor.stroke.width,
            self.visuals_default.text_cursor.stroke.width,
        ));
        ui.add(bool_picker(
            "Text Cursor Preview",
            &mut visuals.text_cursor.preview,
            self.visuals_default.text_cursor.preview,
        ));
        ui.add(float_picker(
            "Clip Rect Margin",
            &mut visuals.clip_rect_margin,
            self.visuals_default.clip_rect_margin,
        ));
        ui.add(bool_picker(
            "Button Frame",
            &mut visuals.button_frame,
            self.visuals_default.button_frame,
        ));
        ui.add(bool_picker(
            "Collapsing Header Frame",
            &mut visuals.collapsing_header_frame,
            self.visuals_default.collapsing_header_frame,
        ));
        ui.add(bool_picker(
            "Indent Left Vline",
            &mut visuals.indent_has_left_vline,
            self.visuals_default.indent_has_left_vline,
        ));
        ui.add(bool_picker(
            "Striped",
            &mut visuals.striped,
            self.visuals_default.striped,
        ));
        ui.add(bool_picker(
            "Slider Trailing Fill",
            &mut visuals.slider_trailing_fill,
            self.visuals_default.slider_trailing_fill,
        ));
    }

    fn window(&mut self, ui: &mut Ui, visuals: &mut Visuals) {
        ui.add(rounding_picker(
            "Rounding",
            &mut self.window_rounding,
            &mut visuals.window_rounding,
            (
                self.visuals_default.window_rounding,
                Self::default().window_rounding,
            ),
        ));
        ui.add(shadow_picker(
            "Shadow",
            &mut visuals.window_shadow,
            self.visuals_default.window_shadow,
        ));
        ui.add(color_picker(
            "Fill",
            &mut visuals.window_fill,
            self.visuals_default.window_fill,
        ));
        ui.add(stroke_picker(
            "Stroke",
            &mut visuals.window_stroke,
            self.visuals_default.window_stroke,
        ));
    }

    fn colors(&self, ui: &mut Ui, visuals: &mut Visuals) {
        ui.add(color_picker(
            "Hyperlink",
            &mut visuals.hyperlink_color,
            self.visuals_default.hyperlink_color,
        ));
        ui.add(color_picker(
            "Faint Background",
            &mut visuals.faint_bg_color,
            self.visuals_default.faint_bg_color,
        ));
        ui.add(color_picker(
            "Extreme Background",
            &mut visuals.extreme_bg_color,
            self.visuals_default.extreme_bg_color,
        ));
        ui.add(color_picker(
            "Code Background",
            &mut visuals.code_bg_color,
            self.visuals_default.code_bg_color,
        ));
        ui.add(color_picker(
            "Warning Foreground",
            &mut visuals.warn_fg_color,
            self.visuals_default.warn_fg_color,
        ));
        ui.add(color_picker(
            "Error Foreground",
            &mut visuals.error_fg_color,
            self.visuals_default.error_fg_color,
        ));
    }

    fn widgets(&mut self, ui: &mut Ui, visuals: &mut Visuals) {
        self.widget_tab_state.show(ui);
        ui.separator();

        let visuals: &mut WidgetVisuals = match self.widget_tab_state {
            WidgetTabState::NonInteractive => &mut visuals.widgets.noninteractive,
            WidgetTabState::Inactive => &mut visuals.widgets.inactive,
            WidgetTabState::Hovered => &mut visuals.widgets.hovered,
            WidgetTabState::Active => &mut visuals.widgets.active,
            WidgetTabState::Open => &mut visuals.widgets.open,
        };

        let visuals_default: WidgetVisuals = match self.widget_tab_state {
            WidgetTabState::NonInteractive => self.visuals_default.widgets.noninteractive,
            WidgetTabState::Inactive => self.visuals_default.widgets.inactive,
            WidgetTabState::Hovered => self.visuals_default.widgets.hovered,
            WidgetTabState::Active => self.visuals_default.widgets.active,
            WidgetTabState::Open => self.visuals_default.widgets.open,
        };

        ui.add(color_picker(
            "Background Fill",
            &mut visuals.bg_fill,
            visuals_default.bg_fill,
        ));
        ui.add(color_picker(
            "Weak Background Fill",
            &mut visuals.weak_bg_fill,
            visuals_default.weak_bg_fill,
        ));
        ui.add(stroke_picker(
            "Background Stroke",
            &mut visuals.bg_stroke,
            visuals_default.bg_stroke,
        ));

        let rounding_uniform: &mut (bool, f32) = match self.widget_tab_state {
            WidgetTabState::NonInteractive => &mut self.noninteractive_rounding,
            WidgetTabState::Inactive => &mut self.inactive_rounding,
            WidgetTabState::Hovered => &mut self.hovered_rounding,
            WidgetTabState::Active => &mut self.active_rounding,
            WidgetTabState::Open => &mut self.open_rounding,
        };

        let rounding_uniform_default: (bool, f32) = match self.widget_tab_state {
            WidgetTabState::NonInteractive => Self::default().noninteractive_rounding,
            WidgetTabState::Inactive => Self::default().inactive_rounding,
            WidgetTabState::Hovered => Self::default().hovered_rounding,
            WidgetTabState::Active => Self::default().active_rounding,
            WidgetTabState::Open => Self::default().open_rounding,
        };

        ui.add(rounding_picker(
            "Rounding",
            rounding_uniform,
            &mut visuals.rounding,
            (visuals_default.rounding, rounding_uniform_default),
        ));
        ui.add(stroke_picker(
            "Foreground Stroke",
            &mut visuals.fg_stroke,
            visuals_default.fg_stroke,
        ));
        ui.add(float_picker(
            "Expansion",
            &mut visuals.expansion,
            visuals_default.expansion,
        ));
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum TabState {
    Misc,
    Window,
    Colors,
    Widgets,
}

impl TabState {
    fn show(&mut self, ui: &mut Ui) -> Response {
        let resp = ui.columns(4, |cols| {
            let misc = cols[0].add_enabled(*self != Self::Misc, Button::new("Misc"));
            if misc.enabled() && misc.clicked() {
                *self = Self::Misc;
            }

            let window = cols[1].add_enabled(*self != Self::Window, Button::new("Window"));
            if window.enabled() && window.clicked() {
                *self = Self::Window;
            }

            let colors = cols[2].add_enabled(*self != Self::Colors, Button::new("Colors"));
            if colors.enabled() && colors.clicked() {
                *self = Self::Colors;
            }

            let widgets = cols[3].add_enabled(*self != Self::Widgets, Button::new("Widgets"));
            if widgets.enabled() && widgets.clicked() {
                *self = Self::Widgets;
            }

            widgets
        });

        ui.add_space(2.5);

        resp
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum WidgetTabState {
    NonInteractive,
    Inactive,
    Hovered,
    Active,
    Open,
}

impl WidgetTabState {
    fn show(&mut self, ui: &mut Ui) -> Response {
        let resp = ui.columns(3, |cols| {
            let noninteractive =
                cols[0].add_enabled(*self != Self::NonInteractive, Button::new("NonInteractive"));
            if noninteractive.enabled() && noninteractive.clicked() {
                *self = Self::NonInteractive;
            }

            let inactive = cols[1].add_enabled(*self != Self::Inactive, Button::new("Inactive"));
            if inactive.enabled() && inactive.clicked() {
                *self = Self::Inactive;
            }

            let hovered = cols[2].add_enabled(*self != Self::Hovered, Button::new("Hovered"));
            if hovered.enabled() && hovered.clicked() {
                *self = Self::Hovered;
            }

            let active = cols[1].add_enabled(*self != Self::Active, Button::new("Active"));
            if active.enabled() && active.clicked() {
                *self = Self::Active;
            }

            let open = cols[2].add_enabled(*self != Self::Open, Button::new("Open"));
            if open.enabled() && open.clicked() {
                *self = Self::Open;
            }

            open
        });

        ui.add_space(2.5);

        resp
    }
}

fn dark_light_mode_picker(mode: &mut bool) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        picker_frame(ui, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(Label::new("Dark Mode").sense(Sense::click()))
                    .clicked()
                {
                    *mode = !*mode;
                };
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.selectable_value(mode, false, "☀ Light");
                    ui.selectable_value(mode, true, "🌙 Dark");
                })
            })
            .response
        })
    }
}
