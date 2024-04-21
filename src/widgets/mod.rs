use gtk::*;

pub mod battery;
pub mod brightness;
pub mod volume;
pub mod clock;
pub mod hyprland;
pub mod sys;

pub fn widget() -> Box {
    let widgetbox = Box::new(Orientation::Horizontal, 0);
    widgetbox
}
