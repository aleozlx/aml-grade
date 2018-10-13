#[macro_use]
extern crate glib;
extern crate gio;
extern crate gtk;
extern crate clap;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gobject_subclass;

use gio::prelude::*;
use gtk::prelude::*;
use gio::{ApplicationExt, ApplicationExtManual};
use gtk::{
    ApplicationWindow, WindowPosition, FileChooserDialog, FileChooserAction, ResponseType, Builder
};

use std::env::args;
use std::path::Path;
mod student;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn strip_sso<P: AsRef<Path>, Q: AsRef<Path>>(path: P, collection: Q) -> Option<std::path::PathBuf> {
    let path = path.as_ref().strip_prefix(collection).unwrap();
    let mut str_components = Vec::new();
    str_components.extend(path.components().into_iter().map(|val| {
        val.as_os_str().to_str().unwrap()
    }));
    Some(std::path::PathBuf::from(
        str_components[1..].join(&std::path::MAIN_SEPARATOR.to_string())))
}

fn locate_notebook<P: AsRef<Path>, Q: AsRef<Path>>(path: P, sso: &str, collection: Q) -> Option<std::path::PathBuf> {
    let p = collection.as_ref().join(sso).join(path);
    match p.exists() {
        true => Some(p),
        false => None
    }
}

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

    let window_weak = window.downgrade();
    let model = gio::ListStore::new(student::RowData::static_type());
    let listbox: gtk::ListBox = builder.get_object("listStudents").expect("listStudents undefined");
    listbox.bind_model(&model, clone!(window_weak => move |item| {
        let box_ = gtk::ListBoxRow::new();
        let item = item.downcast_ref::<student::RowData>().unwrap();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        hbox.set_spacing(4);
        let label = gtk::Label::new(None);
        label.set_xalign(0.0);
        item.bind_property("sso", &label, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        hbox.pack_start(&label, true, true, 0);

        let check = gtk::CheckButton::new();
        item.bind_property("selected", &check, "active")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();
        hbox.pack_start(&check, false, false, 0);

        let sites = vec!["Local", "Europa", "Callisto", "Kore", "Dia"];
        for site in sites {
            let open = gtk::Button::new();
            open.set_label(site);
            if site == "Local" {
                if item.get_property("notebook_abs").unwrap().get::<String>().is_none() {
                    open.set_sensitive(false);
                }
            }
            else {
                open.set_sensitive(false);
            }
            hbox.pack_start(&open, false, false, 0);
        }
        

        box_.add(&hbox);
        box_.show_all();
        box_
    }));

    let collection = matches.value_of("collection").unwrap();
    if Path::new(collection).exists() {
        let dialog = FileChooserDialog::new(Some("Choose a notebook"), Some(&window), FileChooserAction::Open);
        dialog.add_buttons(&[
            ("Select", ResponseType::Ok.into()),
            ("Cancel", ResponseType::Cancel.into())
        ]);
        dialog.set_current_folder(collection);
        if ResponseType::from(dialog.run()) == ResponseType::Ok {
            let notebook = strip_sso(dialog.get_filename().unwrap(), collection).unwrap();
            // println!("{}", notebook.display());

            let paths = std::fs::read_dir(collection).unwrap();
            for i in paths {
                let path = i.unwrap().path();
                let sso = path.file_name().unwrap().to_str().unwrap();
                model.append(&student::RowData::new(
                    sso,
                    match locate_notebook(&notebook, sso, collection) {
                        Some(notebook_abs) => Some(String::from(notebook_abs.to_str().unwrap())),
                        None => None
                    }
                ));

                if let Some(_notebook_abs) = locate_notebook(&notebook, sso, collection) {
                //     model.append(&student::RowData::new(sso));
                }
                else {
                //     model.append(&student::RowData::new(&format!("{} [NX]", sso)));
                    println!("NX({})", sso);
                }
            }
        }
        else {
            application.quit();
        }

        dialog.destroy();
    }
    else {
        application.quit();
    }
    window.show_all();
}

fn main() {
    //https://developer.gnome.org/gtk3/stable/ch03.html
    let application = gtk::Application::new("dsa.aml.grade", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}

