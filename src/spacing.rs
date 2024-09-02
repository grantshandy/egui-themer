use eframe::egui::{style::Spacing, Style, Ui};

use crate::{
    pickers::{bool_picker, float_picker, margin_picker, vec2_picker},
    section_title,
};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct SpacingMenu;

impl SpacingMenu {
    pub fn ui(&mut self, ui: &mut Ui, spacing: &mut Spacing) {
        let default = Style::default().spacing;

        ui.add(section_title(
            "Spacing",
            Some("https://docs.rs/egui/0.21.0/egui/style/struct.Spacing.html"),
        ));
        ui.add(vec2_picker(
            "Item Spacing",
            &mut spacing.item_spacing,
            default.item_spacing,
        ));
        ui.add(margin_picker(
            "Window Margin",
            &mut spacing.window_margin,
            default.window_margin,
        ));
        ui.add(vec2_picker(
            "Button Padding",
            &mut spacing.button_padding,
            default.button_padding,
        ));
        ui.add(margin_picker(
            "Menu Margin",
            &mut spacing.menu_margin,
            default.menu_margin,
        ));
        ui.add(float_picker("Indent", &mut spacing.indent, default.indent));
        ui.add(vec2_picker(
            "Interact Size",
            &mut spacing.interact_size,
            default.interact_size,
        ));
        ui.add(float_picker(
            "Slider Width",
            &mut spacing.slider_width,
            default.slider_width,
        ));
        ui.add(float_picker(
            "Combo Width",
            &mut spacing.combo_width,
            default.combo_width,
        ));
        ui.add(float_picker(
            "Text Edit Width",
            &mut spacing.text_edit_width,
            default.text_edit_width,
        ));
        ui.add(float_picker(
            "Icon Width",
            &mut spacing.icon_width,
            default.icon_width,
        ));
        ui.add(float_picker(
            "Icon Width Inner",
            &mut spacing.icon_width_inner,
            default.icon_width_inner,
        ));
        ui.add(float_picker(
            "Icon Spacing",
            &mut spacing.icon_spacing,
            default.icon_spacing,
        ));
        ui.add(float_picker(
            "Tooltip Width",
            &mut spacing.tooltip_width,
            default.tooltip_width,
        ));
        ui.add(bool_picker(
            "Indent Ends With Horizontal Line",
            &mut spacing.indent_ends_with_horizontal_line,
            default.indent_ends_with_horizontal_line,
        ));
        ui.add(float_picker(
            "Combo Height",
            &mut spacing.combo_height,
            default.combo_height,
        ));
        ui.add(float_picker(
            "Scroll Bar Width",
            &mut spacing.scroll.bar_width,
            default.scroll.bar_width,
        ));
        ui.add(float_picker(
            "Scroll Bar Handle Min Length",
            &mut spacing.scroll.handle_min_length,
            default.scroll.handle_min_length,
        ));
        ui.add(float_picker(
            "Scroll Bar Inner Margin",
            &mut spacing.scroll.bar_inner_margin,
            default.scroll.bar_inner_margin,
        ));
        ui.add(float_picker(
            "Scroll Bar Outer Margin",
            &mut spacing.scroll.bar_outer_margin,
            default.scroll.bar_outer_margin,
        ));
    }
}
