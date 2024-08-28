use eframe::egui::{Style, Visuals};

use pretty_assertions::assert_eq;

mod dark_default;
mod light_default;

#[test]
fn dark_default() {
    use dark_default;

    assert_eq!(
        serde_json::to_string_pretty(&dark_default::style()).unwrap(),
        serde_json::to_string_pretty(&Style {
            visuals: Visuals::dark(),
            ..Default::default()
        }).unwrap(),
    );
}

#[test]
fn light_default() {
    use light_default;

    assert_eq!(
        serde_json::to_string_pretty(&light_default::style()).unwrap(),
        serde_json::to_string_pretty(&Style {
            visuals: Visuals::light(),
            ..Default::default()
        }).unwrap(),
    );
}
