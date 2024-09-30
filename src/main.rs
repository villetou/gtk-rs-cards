use gtk::ffi::GtkDrawingArea;
use rand::Rng;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
// use std::str::FromStr;
// use glib::clone;
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

fn create_card() -> DrawingArea {
    let card = DrawingArea::builder()
        .content_width(63 * 2)
        .content_height(88 * 2)
        .build();
    card.add_css_class("card");

    let _poker_green = RGBA::parse("#1E5C3A").unwrap();
    let _card_white = RGBA::parse("#fefefe").unwrap();
    let text_black = RGBA::parse("#101010").unwrap();

    card.set_draw_func(move |_, cr, _ctx_width, _ctx_height| {
        let scale: f64 = 2.0;
        let x: f64 = 0.0;
        let y: f64 = 0.0;

        // Draw numbers
        cr.set_font_size(16.0 * scale);
        cr.select_font_face(
            "Serif",
            gtk::cairo::FontSlant::Normal,
            gtk::cairo::FontWeight::Bold,
        );
        cr.set_source_color(&text_black);
        cr.move_to(x + 2.0 * scale, y + 14.0 * scale);
        let _ = cr.show_text("4");

        cr.move_to(x + (63.0 - 12.0) * scale, y + (88.0 - 4.0) * scale);
        let _ = cr.show_text("4");
    });

    return card;
}

fn build_ui(app: &Application) {
    let card = create_card();
    // Add buttons to `gtk_box`
    let playarea = gtk::Fixed::builder().height_request(800).build();
    playarea.add_css_class("playarea");
    playarea.put(&card, 50.0, 50.0);

    // Deal card button
    let button = Button::builder()
        .label("Deal card")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.add_css_class("deal_button");

    button.connect_clicked({
        let playarea = playarea.downgrade();

        move |_| {
            if let Some(playarea) = playarea.upgrade() {
                let card = create_card();
                let mut rng = rand::thread_rng();
                let x: f64 = rng.gen();
                let y: f64 = rng.gen();

                playarea.put(&card, x * 0.0, y * 0.0);
                playarea.move_(&card, x * 600.0, y * 600.0);
            } else {
                println!("The Vec has been dropped.");
            }
        }
    });

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
