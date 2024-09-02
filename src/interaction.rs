use eframe::egui::{style::Interaction, Ui};

use crate::{
    pickers::{bool_picker, float_picker},
    section_title,
};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct InteractionMenu;

impl InteractionMenu {
    pub fn ui(&mut self, ui: &mut Ui, interaction: &mut Interaction) {
        let default = Interaction::default();

        ui.add(section_title(
            "Interaction",
            Some(crate::egui_doc_link!("/style/struct.Interaction.html")),
        ));
        ui.add(float_picker(
            "Resize Grab Radius - Side",
            &mut interaction.resize_grab_radius_side,
            default.resize_grab_radius_side,
        ));
        ui.add(float_picker(
            "Resize Grab Radius - Corner",
            &mut interaction.resize_grab_radius_corner,
            default.resize_grab_radius_corner,
        ));
        ui.add(bool_picker(
            "Show Tooltips Only When Still",
            &mut interaction.show_tooltips_only_when_still,
            default.show_tooltips_only_when_still,
        ));
    }
}
