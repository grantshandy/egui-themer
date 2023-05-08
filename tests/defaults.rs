use eframe::egui::{Style, Visuals};

mod light_default;
mod dark_default;

#[test]
fn dark_default() {   
    use dark_default;
    
    assert_eq!(dark_default::style(), Style {
        visuals: Visuals::dark(),
        ..Default::default()
    });
}

#[test]
fn light_default() {
    use light_default;

    assert_eq!(light_default::style(), Style {
        visuals: Visuals::light(),
        ..Default::default()
    });
}
