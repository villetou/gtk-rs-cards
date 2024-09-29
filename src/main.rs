use std::str::FromStr;

use glib::clone;
use gtk::gdk::{Display, RGBA};
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider, Orientation};
use gtk::{prelude::*, DrawingArea};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("./style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.add_css_class("esko");

    let card = DrawingArea::builder()
        .content_width(63 * 6)
        .content_height(88 * 6)
        .build();
    card.add_css_class("card");

    let poker_green = RGBA::parse("#1E5C3A").unwrap();
    let card_white = RGBA::parse("#fefefe").unwrap();
    let text_black = RGBA::parse("#101010").unwrap();

    card.set_draw_func(move |_, cr, _ctx_width, _ctx_height| {
        // cr.set_source_color(&card_white);
        // cr.paint().expect("Invalid cairo surface state");

        let scale: f64 = 6.0;
        let x: f64 = 0.0;
        let y: f64 = 0.0;
        // const PI: f64 = 3.14159;
        // let width: f64 = 63.0 * scale;
        // let height: f64 = 88.0 * scale;
        // let radius: f64 = 5.0 * scale;

        // This all can be done in CSS
        // cr.arc(x + radius, y + radius, radius, PI, 3.0 * PI / 2.0);
        // cr.arc(x + width - radius, y + radius, radius, 3.0 * PI / 2.0, 0.0);
        // cr.arc(
        //     x + width - radius,
        //     y + height - radius,
        //     radius,
        //     0.0,
        //     PI / 2.0,
        // );
        // cr.arc(x + radius, y + height - radius, radius, PI / 2.0, PI);
        // cr.close_path();

        // cr.set_source_color(&card_white);
        // cr.fill();

        // Draw numbers
        cr.set_font_size(16.0 * scale);
        cr.select_font_face(
            "Serif",
            gtk::cairo::FontSlant::Normal,
            gtk::cairo::FontWeight::Bold,
        );
        cr.set_source_color(&text_black);
        cr.move_to(x + 2.0 * scale, y + 14.0 * scale);
        cr.show_text("4");
    });

    // Add buttons to `gtk_box`
    let playarea = gtk::Fixed::builder().height_request(800).build();
    playarea.add_css_class("playarea");
    playarea.put(&card, 50.0, 50.0);

    let layout = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    layout.append(&playarea);
    layout.append(&button);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cards")
        .default_width(800)
        .default_height(800)
        .child(&layout)
        .build();

    // Present the window
    window.present();
}
