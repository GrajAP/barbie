#![feature(async_closure)]
use gtk::gdk::*;
use gtk::glib::Propagation;
use gtk::prelude::*;

use gtk::*;
use gtk_layer_shell::{Edge, Layer, LayerShell};
use log::info;

//mod socket;
mod ui;
mod widgets;

/// Initializes the status bar.
fn activate(application: &Application) {
macro_rules! activate{
    ($widget:ident $left:ident $right:ident ) => {
    let $widget = ApplicationWindow::new(application);
    $widget.connect_screen_changed(set_visual);
    $widget.connect_draw(draw);

    LayerShell::init_layer_shell(&$widget);
    LayerShell::set_layer(&$widget, Layer::Top);

    let display = Display::default().expect("couldnt get display");
    let monitor = display.monitor(0).expect("couldnt get monitor");

    let anchors = [
        (Edge::Left, $left),
        (Edge::Right, $right),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];
    for (anchor, state) in anchors {
        LayerShell::set_anchor(&$widget, anchor, state);
    }

    LayerShell::set_namespace(&$widget, "gtk-layer-shell");

    LayerShell::set_monitor(&$widget, &monitor);
    $widget.set_app_paintable(true);

    ui::display_widgets(&$widget, stringify!($widget));

          };
}
activate!(right false true);
activate!(left true false);
activate!(center false false);
   }

/// Applies custom visuals.
fn set_visual(window: &ApplicationWindow, screen: Option<&Screen>) {
    if let Some(screen) = screen {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // Needed for transparency, not available in GTK 4+ so
                                             // F.
        }
    }
}

/// Draws the window using a custom color and opacity.
fn draw(_: &ApplicationWindow, ctx: &cairo::Context) -> Propagation {
    // Apply
    ctx.set_source_rgba(0.0,0.0,0.0,0.0);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("couldnt draw");

    Propagation::Proceed
}

fn load_scss() {
    let provider = CssProvider::new();
    let (style, _) = turf::inline_style_sheet!("assets/style.scss");

    provider
        .load_from_data(style.as_bytes())
        .expect("couldnt load css");

    StyleContext::add_provider_for_screen(
        &Screen::default().expect("couldnt get default screen"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting CrabPulsar");

    color_eyre::config::HookBuilder::default()
        .display_location_section(false)
        .panic_section("something brokie :3")
        .display_env_section(false)
        .install()
        .unwrap();

    let app = Application::builder()
        .application_id("dev.sioodmy.grajap.crabpulsar")
        .build();

    app.connect_startup(|_| load_scss());

    app.connect_activate(move |app| {
        activate(app);
    });

    app.run();
}
