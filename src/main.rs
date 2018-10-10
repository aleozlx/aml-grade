extern crate gio;
extern crate gtk;

use gio::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::*;
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, IconSize, Image, Label,
    Menu, MenuBar, MenuItem, WindowPosition,
};
use std::env::args;

fn build_ui(application: &gtk::Application) {
let window = ApplicationWindow::new(application);

    window.set_title("AML Grading");
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);
    window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });
    window.show_all();
}

fn main() {
    let application = gtk::Application::new("dsa.aml.grade", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}
