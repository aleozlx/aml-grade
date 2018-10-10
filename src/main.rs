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

mod student {
    use super::*;

    use gobject_subclass::object::*;

    use glib::translate::*;

    use std::ptr;
    use std::mem;

    // Implementation sub-module of the GObject
    mod imp {
        use super::*;
        use std::cell::RefCell;

        // The actual data structure that stores our values. This is not accessible
        // directly from the outside.
        pub struct RowData {
            // pawprint: String, selected: bool
            name: RefCell<Option<String>>,
            count: RefCell<u32>,
        }

        // GObject property definitions for our two values
        static PROPERTIES: [Property; 2] = [
            Property::String(
                "name",
                "Name",
                "Name",
                None, // Default value
                PropertyMutability::ReadWrite,
            ),
            Property::UInt(
                "count",
                "Count",
                "Count",
                (0, 100), 0, // Allowed range and default value
                PropertyMutability::ReadWrite,
            ),
        ];

        impl RowData {
            // glib::Type registration of the RowData type. The very first time
            // this registers the type with GObject and afterwards only returns
            // the type id that was registered the first time
            pub fn get_type() -> glib::Type {
                use std::sync::{Once, ONCE_INIT};

                // unsafe code here because static mut variables are inherently
                // unsafe. Via std::sync::Once we guarantee here that the variable
                // is only ever set once, and from that point onwards is only ever
                // read, which makes its usage safe.
                static ONCE: Once = ONCE_INIT;
                static mut TYPE: glib::Type = glib::Type::Invalid;

                ONCE.call_once(|| {
                    let t = register_type(RowDataStatic);
                    unsafe {
                        TYPE = t;
                    }
                });

                unsafe { TYPE }
            }

            // Called exactly once before the first instantiation of an instance. This
            // sets up any type-specific things, in this specific case it installs the
            // properties so that GObject knows about their existence and they can be
            // used on instances of our type
            fn class_init(klass: &mut ObjectClass) {
                klass.install_properties(&PROPERTIES);
            }

            // Called once at the very beginning of instantiation of each instance and
            // creates the data structure that contains all our state
            fn init(_obj: &Object) -> Box<ObjectImpl<Object>> {
                let imp = Self {
                    name: RefCell::new(None),
                    count: RefCell::new(0),
                };
                Box::new(imp)
            }
        }

        // The ObjectImpl trait provides the setters/getters for GObject properties.
        // Here we need to provide the values that are internally stored back to the
        // caller, or store whatever new value the caller is providing.
        //
        // This maps between the GObject properties and our internal storage of the
        // corresponding values of the properties.
        impl ObjectImpl<Object> for RowData {
            fn set_property(&self, _obj: &glib::Object, id: u32, value: &glib::Value) {
                let prop = &PROPERTIES[id as usize];

                match *prop {
                    Property::String("name", ..) => {
                        let name = value.get();
                        self.name.replace(name.clone());
                    }
                    Property::UInt("count", ..) => {
                        let count = value.get().unwrap();
                        self.count.replace(count);
                    }
                    _ => unimplemented!(),
                }
            }

            fn get_property(&self, _obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
                let prop = &PROPERTIES[id as usize];

                match *prop {
                    Property::String("name", ..) => Ok(self.name.borrow().clone().to_value()),
                    Property::UInt("count", ..) => Ok(self.count.borrow().clone().to_value()),
                    _ => unimplemented!(),
                }
            }
        }

        // Static, per-type data that is used for actually registering the type
        // and providing the name of our type and how to initialize it to GObject
        //
        // It is used above in the get_type() function for passing that information
        // to GObject
        struct RowDataStatic;

        impl ImplTypeStatic<Object> for RowDataStatic {
            fn get_name(&self) -> &str {
                "RowData"
            }

            fn new(&self, obj: &Object) -> Box<ObjectImpl<Object>> {
                RowData::init(obj)
            }

            fn class_init(&self, klass: &mut ObjectClass) {
                RowData::class_init(klass);
            }
        }
    }

    // Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
    // binding
    glib_wrapper! {
        pub struct RowData(Object<imp::RowData>):
            [Object => InstanceStruct<Object>];

        match fn {
            get_type => || imp::RowData::get_type().to_glib(),
        }
    }

    // Constructor for new instances. This simply calls glib::Object::new() with
    // initial values for our two properties and then returns the new instance
    impl RowData {
        pub fn new(name: &str, count: u32) -> RowData {
            use glib::object::Downcast;

            unsafe {
                glib::Object::new(
                    Self::static_type(),
                    &[("name", &name),
                      ("count", &count),
                    ])
                    .unwrap()
                    .downcast_unchecked()
            }
        }
    }
}
