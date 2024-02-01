use gtk::traits::*;
use gtk::*;
use log::warn;

use crate::widgets::*;

pub fn display_widgets(window: &ApplicationWindow, pos: &str) {
    let root = Box::new(Orientation::Horizontal, 0);
    root.set_widget_name("barbie");
    macro_rules! add {
        ($widget:ident) => {
            match $widget::add_widget(&root) {
                Ok(_) => (),
                Err(_) => warn!("Couldn't load {} widget", stringify!($widget)),
            }
        };
    }
    match pos {
        "left" => add!(sys),
        "right" => add!(clock),
        "center" => add!(hyprland),
        _ => ()

    }

    window.add(&root);
    window.show_all();
}
