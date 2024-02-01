use anyhow::Result;
use chrono::Local;
use gtk::{traits::*, *};

use super::widget;

pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);
    let clock = Label::new(Some(""));

    clock.set_widget_name("clock");
    widgetbox.add(&clock);

    let tick = move || {
        let (time,date) = (
            Local::now().format("%H:%M:%S").to_string(),
            Local::now().format("%d.%m.%Y").to_string());
        clock.set_text(&time);
        clock.set_tooltip_text(Some(&date));
        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
    Ok(())
}
