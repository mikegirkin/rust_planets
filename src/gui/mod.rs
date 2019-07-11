pub mod gui_main;

extern crate gio;
extern crate gtk;
extern crate cairo;

use gio::prelude::*;
use gtk::prelude::*;
use cairo::{Context, FontSlant, FontWeight};

use std::f64::consts::PI;
use std::env::args;
use self::gtk::{CellRendererPixbufBuilder, Image, Widget, Orientation, render_background};

fn map_draw(widget: &gtk::DrawingArea, cr: &cairo::Context) -> Inhibit {
    let width = widget.get_allocated_width();
    let height = widget.get_allocated_height();

    cr.paint();
//    cr.set_line_width(2.0);
//    cr.set_source_rgb(0.0, 1.0, 0.0);
//    cr.rectangle(20.0, 20.0, 30.0, 30.0);

    cr.scale(200f64, 200f64);

//    cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
//    cr.set_font_size(0.35);
//
//    cr.move_to(0.04, 0.53);
//    cr.show_text("Hello");

//    cr.move_to(0.27, 0.65);
//    cr.text_path("void");
//    cr.set_source_rgb(0.5, 0.5, 1.0);
//    cr.fill_preserve();
//    cr.set_source_rgb(0.0, 0.0, 0.0);
//    cr.set_line_width(0.01);
//    cr.stroke();
//
    cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
    cr.arc(0.04, 0.53, 0.02, 0.0, PI * 2.);
    cr.arc(0.27, 0.65, 0.02, 0.0, PI * 2.);

    cr.rectangle(0.10, 0.10, 0.20, 0.20);

    cr.fill();

    Inhibit(false)
}

fn build_map_widget() -> gtk::DrawingArea {
    let drawingArea = gtk::DrawingArea::new();
    drawingArea.connect_draw(map_draw);
    drawingArea
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 300);

    let button = gtk::Button::new_with_label("Click me!");

    let map = build_map_widget();

    let vbox = gtk::Box::new(Orientation::Vertical, 10);
    vbox.pack_start(&map, true, true, 0);
    vbox.pack_end(&button, false, false, 0);

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