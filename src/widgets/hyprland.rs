use anyhow::Result;
use glib::*;
use gtk::{traits::*, *};
use hyprland::data::Workspaces;
use hyprland::event_listener::*;
use hyprland::prelude::*;

use super::widget;

pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let (sender, receiver) = async_channel::unbounded::<()>();

    let label = Label::new(Some(&workspace_name()));

    label.set_widget_name("hyprland");
    label.set_width_request(30);
    widgetbox.add(&label);

    gio::spawn_blocking(move || {
        let mut listener = EventListener::new();
        listener.add_workspace_change_handler(move |_| {
            sender.send_blocking(()).unwrap();
        });
        listener.start_listener().unwrap();
    });

    glib::spawn_future_local(clone!(@weak label=> async move {
        while (receiver.recv().await).is_ok() {
            label.set_label(&workspace_name());
        }
    }));

    Ok(())
}

fn workspace_name() -> String{
    let mut label: String = String::new();
    let mut workspace: Vec<i32> = Workspaces::get().expect("Couldn't get workspaces").to_vec().iter().map(|x| x.id).filter(|x| *x>0).collect();
    workspace.sort();
    for i in 0..workspace.len() {
//    if Workspace::get_active().unwrap().id == i as i32 {}
    label.push_str(match workspace[i as usize] {
        1 => " ",
        2 => "󰈹 ",
        3 => " ",
        4 => "󰙯 ",
        5 => "󰋄 ",
        6 => "VI ",
        7 => "VII ",
        8 => "VII ",
        9 => "IX ",
        10 => "X ",
        _ => todo!("support more workspaces"),
    });
    }
    label.trim_end().trim_start().to_string()
}
