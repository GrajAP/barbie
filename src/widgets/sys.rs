use anyhow::Result;
use core::f32;
use glib::*;
use gtk::{traits::*, *};
use std::thread;
use std::time::Duration;
use sysinfo::{Components,System,MemoryRefreshKind, RefreshKind,};


use super::widget;

type Stats = (f32, f32);

pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let face = Label::new(None);
    face.set_widget_name("sys");
    face.set_width_request(74);

    let (sender, receiver) = async_channel::unbounded::<Stats>();

    widgetbox.add(&face);

    gio::spawn_blocking(move || loop {
        sender
            .send_blocking(get_stats().expect("Error while fetching system stats"))
            .unwrap();
        thread::sleep(Duration::from_secs(3));
    });

    glib::spawn_future_local(clone!(@weak face=> async move {
        while let Ok((cpu, mem)) = receiver.recv().await {
//            face.set_tooltip_markup(Some(&format!("<b>CPU</b> {:.2}% <b>MEM</b> {:.2}%", cpu, mem)));
            face.set_label(&format!("  {:.0}°C   {:.0}GB", cpu , mem).to_string());

        }
    }));
    Ok(())
}

fn get_stats() -> Result<Stats> {
    let sys = System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()));
    let cpu = Components::new_with_refreshed_list()[2].temperature();
    let mem: f32 = (sys.used_memory() / 1000000000) as f32;
    Ok((cpu, mem))
}
