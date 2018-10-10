extern crate gio;
extern crate gtk;
extern crate clap;

use gio::prelude::*;
use gtk::prelude::*;
use gio::{ApplicationExt, ApplicationExtManual};
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, IconSize, Image, Label,
    Menu, MenuBar, MenuItem, WindowPosition, FileChooserDialog, FileChooserAction, ResponseType, Builder
};
use std::env::args;
use std::path::Path;

fn build_ui(application: &gtk::Application) {
    let matches = clap::App::new("aml-grade")
        .version("1.0")
        .author("Alex Yang <zy5f9@mail.missouri.edu>")
        .about("AML Grading")
        .arg(clap::Arg::with_name("collection")
            .help("Collection folder")
            .required(true)
            .index(1))
        .get_matches();

    let glade_src = include_str!("aml-grade.glade");
    // println!("{}", glade_src);
    let builder = Builder::new_from_string(glade_src);
    let window: ApplicationWindow = builder.get_object("windowMain").expect("Main window undefined");

    window.set_application(application);
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);
    window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });

    let collection = matches.value_of("collection").unwrap();
    if Path::new(collection).exists() {
        let dialog = FileChooserDialog::new(Some("Choose a notebook"), Some(&window), FileChooserAction::Open);
        dialog.add_buttons(&[
            ("Select", ResponseType::Ok.into()),
            ("Cancel", ResponseType::Cancel.into())
        ]);
        dialog.set_current_folder(collection);
        dialog.run();
        // let files = dialog.get_filenames();
        dialog.destroy();
    }


    window.show_all();
}

fn main() {
    //https://developer.gnome.org/gtk3/stable/ch03.html
    //https://github.com/gtk-rs/examples/blob/b364f284139c084049c54c10382f8727ab12b60c/src/bin/listbox_model.rs
    let application = gtk::Application::new("dsa.aml.grade", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}
