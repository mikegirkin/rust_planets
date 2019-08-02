pub mod gui_main;

extern crate gio;
extern crate gtk;
extern crate cairo;

use gio::prelude::*;
use gtk::prelude::*;
use cairo::{Context, FontSlant, FontWeight};

use std::f64::consts::PI;
use std::env::args;
use self::gtk::{CellRendererPixbufBuilder, Window, Image, Widget, Orientation, render_background};
use crate::model::game::Game;
use crate::util::path_in_test_directory;
use std::borrow::{BorrowMut, Borrow};

fn map_draw(game: &Game, widget: &gtk::DrawingArea, cr: &cairo::Context) -> Inhibit {
//    let width = widget.get_allocated_width();
//    let height = widget.get_allocated_height();

    cr.paint();
    //cr.scale(0.2, 0.2);
    cr.translate(-1000.0, -1000.0);
//    cr.set_line_width(2.0);
    cr.set_source_rgb(1.0, 1.0, 1.0);
    //cr.rectangle(20.0, 20.0, 30.0, 30.0);
    //cr.fill();


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
    cr.set_source_rgb(1.0, 1.0, 0.2);
    cr.arc(20.04, 20.53, 10.02, 0.0, PI * 2.);
    cr.stroke();
    cr.arc(200.27, 200.65, 10.02, 0.0, PI * 2.);

    game.planets.iter().for_each(|planet| {
        cr.arc(planet.position.x as f64, planet.position.y as f64, 5f64, 0.0, PI*2.0);
        cr.fill();
    });

    Inhibit(false)
}

fn build_map_widget(game: &Game) -> gtk::DrawingArea {
//    let draw:for<'r, 's> fn(&'r gtk::DrawingArea, &'s cairo::Context) -> Inhibit = |w, cr| {
//        map_draw(game, w, cr)
//    };
    let drawing_area = gtk::DrawingArea::new();
    let inner_game = game.clone();
    drawing_area.connect_draw(move |w, cr| {
        map_draw(&inner_game, w, cr)
    });
    drawing_area.set_size_request(400, 400);
    drawing_area
}

fn build_ui(application: &gtk::Application, game: &Game) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);

    let button = gtk::Button::new_with_label("Click me!");

    let map = build_map_widget(game);

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

    let game = Game::read(path_in_test_directory("pleiades10"));

    application.connect_activate(move |app| {
        build_ui(app, &(game.clone()));
    });

    application.run(&[]);
}