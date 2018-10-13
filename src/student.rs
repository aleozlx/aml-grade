// use super::*;
extern crate glib;
extern crate gio;
extern crate gtk;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gobject_subclass;

// use gio::prelude::*;
use gtk::prelude::*;

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
        sso: RefCell<Option<String>>,
        notebook_abs: RefCell<Option<String>>,
        selected: RefCell<bool>,
    }

    // GObject property definitions for our two values
    static PROPERTIES: [Property; 3] = [
        Property::String(
            "sso",
            "SSO",
            "SSO",
            None, // Default value
            PropertyMutability::ReadWrite,
        ),
        Property::String(
            "notebook_abs",
            "NotebookAbsPath",
            "NotebookAbsPath",
            None,
            PropertyMutability::ReadWrite,
        ),
        Property::Boolean(
            "selected",
            "Selected",
            "Selected",
            false,
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
                sso: RefCell::new(None),
                notebook_abs: RefCell::new(None),
                selected: RefCell::new(false),
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
                Property::String("sso", ..) => {
                    let sso = value.get();
                    self.sso.replace(sso.clone());
                },
                Property::String("notebook_abs", ..) => {
                    let notebook_abs = value.get();
                    self.notebook_abs.replace(notebook_abs.clone());
                },
                Property::Boolean("selected", ..) => {
                    let selected = value.get().unwrap();
                    self.selected.replace(selected);
                },
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("sso", ..) => Ok(self.sso.borrow().clone().to_value()),
                Property::String("notebook_abs", ..) => Ok(self.notebook_abs.borrow().clone().to_value()),
                Property::Boolean("selected", ..) => Ok(self.selected.borrow().clone().to_value()),
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
    pub fn new(sso: &str, notebook_abs: Option<String>) -> RowData {
        use glib::object::Downcast;

        unsafe {
            glib::Object::new(
                Self::static_type(),
                &[
                    ("sso", &sso),
                    ("notebook_abs", &notebook_abs),
                    ("selected", &false),
                ])
                .unwrap()
                .downcast_unchecked()
        }
    }
}
