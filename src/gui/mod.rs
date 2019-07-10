pub mod gui_main;

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;
use self::gtk::{CellRendererPixbufBuilder, Image, Widget, Orientation};

fn build_map_widget() -> Image {
    let widget = gtk::Image::new();

    widget
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::new_with_label("Click me!");

    let map = build_map_widget();

    let vbox = gtk::Box::new(Orientation::Vertical, 10);
    vbox.add(&map);
    vbox.add(&button);

    window.add(&vbox);

    window.show_all();
}

pub fn gui_main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&[]);
}