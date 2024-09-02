// Generated by egui-themer (https://github.com/grantshandy/egui-themer).

use eframe::egui;

use egui::{
    epaint::Shadow,
    style::{
        Interaction, ScrollStyle, Selection, Spacing, TextCursorStyle, WidgetVisuals, Widgets,
    },
    Color32, Margin, Rounding, Stroke, Style, Vec2, Visuals,
};

pub fn style() -> Style {
    Style {
        // override the text styles here:
        // override_text_style: Option<TextStyle>

        // override the font id here:
        // override_font_id: Option<FontId>

        // set your text styles here:
        // text_styles: BTreeMap<TextStyle, FontId>,

        // set your drag value text style:
        // drag_value_text_style: TextStyle,
        spacing: Spacing {
            item_spacing: Vec2 { x: 8.0, y: 3.0 },
            window_margin: Margin {
                left: 6.0,
                right: 6.0,
                top: 6.0,
                bottom: 6.0,
            },
            button_padding: Vec2 { x: 4.0, y: 1.0 },
            menu_margin: Margin {
                left: 6.0,
                right: 6.0,
                top: 6.0,
                bottom: 6.0,
            },
            indent: 18.0,
            interact_size: Vec2 { x: 40.0, y: 18.0 },
            slider_width: 100.0,
            combo_width: 100.0,
            text_edit_width: 280.0,
            icon_width: 14.0,
            icon_width_inner: 8.0,
            icon_spacing: 4.0,
            tooltip_width: 500.0,
            indent_ends_with_horizontal_line: false,
            combo_height: 200.0,
            scroll: ScrollStyle {
                bar_width: 10.0,
                handle_min_length: 12.0,
                bar_inner_margin: 4.0,
                bar_outer_margin: 0.0,
                ..Default::default()
            },
            ..Default::default()
        },
        interaction: Interaction {
            resize_grab_radius_side: 5.0,
            resize_grab_radius_corner: 10.0,
            show_tooltips_only_when_still: true,
            ..Default::default()
        },
        visuals: Visuals {
            dark_mode: false,
            override_text_color: None,
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(248, 248, 248, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(248, 248, 248, 255),
                    bg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(190, 190, 190, 255),
                    },
                    rounding: Rounding {
                        nw: 2.0,
                        ne: 2.0,
                        sw: 2.0,
                        se: 2.0,
                    },
                    fg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(80, 80, 80, 255),
                    },
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(230, 230, 230, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(230, 230, 230, 255),
                    bg_stroke: Stroke {
                        width: 0.0,
                        color: Color32::from_rgba_premultiplied(0, 0, 0, 0),
                    },
                    rounding: Rounding {
                        nw: 2.0,
                        ne: 2.0,
                        sw: 2.0,
                        se: 2.0,
                    },
                    fg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(60, 60, 60, 255),
                    },
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(220, 220, 220, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(220, 220, 220, 255),
                    bg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(105, 105, 105, 255),
                    },
                    rounding: Rounding {
                        nw: 3.0,
                        ne: 3.0,
                        sw: 3.0,
                        se: 3.0,
                    },
                    fg_stroke: Stroke {
                        width: 1.5,
                        color: Color32::from_rgba_premultiplied(0, 0, 0, 255),
                    },
                    expansion: 1.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(165, 165, 165, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(165, 165, 165, 255),
                    bg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(0, 0, 0, 255),
                    },
                    rounding: Rounding {
                        nw: 2.0,
                        ne: 2.0,
                        sw: 2.0,
                        se: 2.0,
                    },
                    fg_stroke: Stroke {
                        width: 2.0,
                        color: Color32::from_rgba_premultiplied(0, 0, 0, 255),
                    },
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(220, 220, 220, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(220, 220, 220, 255),
                    bg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(160, 160, 160, 255),
                    },
                    rounding: Rounding {
                        nw: 2.0,
                        ne: 2.0,
                        sw: 2.0,
                        se: 2.0,
                    },
                    fg_stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(0, 0, 0, 255),
                    },
                    expansion: 0.0,
                },
            },
            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(144, 209, 255, 255),
                stroke: Stroke {
                    width: 1.0,
                    color: Color32::from_rgba_premultiplied(0, 83, 125, 255),
                },
            },
            hyperlink_color: Color32::from_rgba_premultiplied(0, 155, 255, 255),
            faint_bg_color: Color32::from_rgba_premultiplied(5, 5, 5, 0),
            extreme_bg_color: Color32::from_rgba_premultiplied(255, 255, 255, 255),
            code_bg_color: Color32::from_rgba_premultiplied(230, 230, 230, 255),
            warn_fg_color: Color32::from_rgba_premultiplied(255, 100, 0, 255),
            error_fg_color: Color32::from_rgba_premultiplied(255, 0, 0, 255),
            window_rounding: Rounding {
                nw: 6.0,
                ne: 6.0,
                sw: 6.0,
                se: 6.0,
            },
            window_shadow: Shadow {
                spread: 0.0,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 25),
                blur: 15.0,
                offset: Vec2 { x: 10.0, y: 20.0 },
            },
            window_fill: Color32::from_rgba_premultiplied(248, 248, 248, 255),
            window_stroke: Stroke {
                width: 1.0,
                color: Color32::from_rgba_premultiplied(190, 190, 190, 255),
            },
            menu_rounding: Rounding {
                nw: 6.0,
                ne: 6.0,
                sw: 6.0,
                se: 6.0,
            },
            panel_fill: Color32::from_rgba_premultiplied(248, 248, 248, 255),
            popup_shadow: Shadow {
                spread: 0.0,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 25),
                blur: 8.0,
                offset: Vec2 { x: 6.0, y: 10.0 },
            },
            resize_corner_size: 12.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke {
                    width: 2.0,
                    color: Color32::from_rgba_premultiplied(0, 83, 125, 255),
                },
                preview: false,
                ..Default::default()
            },
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: false,
            ..Default::default()
        },
        animation_time: 0.0833333358168602,
        explanation_tooltips: false,
        ..Default::default()
    }
}
