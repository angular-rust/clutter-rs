// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Script(Object<ffi::ClutterScript, ffi::ClutterScriptClass, ScriptClass>);

    match fn {
        get_type => || ffi::clutter_script_get_type(),
    }
}

impl Script {
    pub fn new() -> Script {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::clutter_script_new()) }
    }
}

impl Default for Script {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SCRIPT: Option<&Script> = None;

pub trait ScriptExt: 'static {
    fn add_search_paths(&self, paths: &[&str]);

    //fn connect_signals(&self, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    //fn connect_signals_full(&self, func: /*Unimplemented*/FnMut(&Script, /*Ignored*/glib::Object, &str, &str, /*Ignored*/glib::Object, /*Ignored*/glib::ConnectFlags), user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    fn ensure_objects(&self);

    //fn get_object(&self, name: &str) -> /*Ignored*/Option<glib::Object>;

    //fn get_objects(&self, first_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> i32;

    fn get_translation_domain(&self) -> Option<GString>;

    fn get_type_from_name(&self, type_name: &str) -> glib::types::Type;

    //fn list_objects(&self) -> /*Ignored*/Vec<glib::Object>;

    fn load_from_data(&self, data: &str) -> Result<(), glib::Error>;

    fn load_from_file(&self, filename: &str) -> Result<(), glib::Error>;

    fn load_from_resource(&self, resource_path: &str) -> Result<(), glib::Error>;

    fn lookup_filename(&self, filename: &str) -> Option<GString>;

    fn set_translation_domain(&self, domain: Option<&str>);

    fn unmerge_objects(&self, merge_id: u32);

    fn get_property_filename(&self) -> Option<GString>;

    fn get_property_filename_set(&self) -> bool;

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filename_set_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_translation_domain_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Script>> ScriptExt for O {
    fn add_search_paths(&self, paths: &[&str]) {
        let n_paths = paths.len() as usize;
        unsafe {
            ffi::clutter_script_add_search_paths(
                self.as_ref().to_glib_none().0,
                paths.to_glib_none().0,
                n_paths,
            );
        }
    }

    //fn connect_signals(&self, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call clutter_sys:clutter_script_connect_signals() }
    //}

    //fn connect_signals_full(&self, func: /*Unimplemented*/FnMut(&Script, /*Ignored*/glib::Object, &str, &str, /*Ignored*/glib::Object, /*Ignored*/glib::ConnectFlags), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call clutter_sys:clutter_script_connect_signals_full() }
    //}

    fn ensure_objects(&self) {
        unsafe {
            ffi::clutter_script_ensure_objects(self.as_ref().to_glib_none().0);
        }
    }

    //fn get_object(&self, name: &str) -> /*Ignored*/Option<glib::Object> {
    //    unsafe { TODO: call clutter_sys:clutter_script_get_object() }
    //}

    //fn get_objects(&self, first_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> i32 {
    //    unsafe { TODO: call clutter_sys:clutter_script_get_objects() }
    //}

    fn get_translation_domain(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_script_get_translation_domain(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_type_from_name(&self, type_name: &str) -> glib::types::Type {
        unsafe {
            from_glib(ffi::clutter_script_get_type_from_name(
                self.as_ref().to_glib_none().0,
                type_name.to_glib_none().0,
            ))
        }
    }

    //fn list_objects(&self) -> /*Ignored*/Vec<glib::Object> {
    //    unsafe { TODO: call clutter_sys:clutter_script_list_objects() }
    //}

    fn load_from_data(&self, data: &str) -> Result<(), glib::Error> {
        let length = data.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::clutter_script_load_from_data(
                self.as_ref().to_glib_none().0,
                data.to_glib_none().0,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn load_from_file(&self, filename: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::clutter_script_load_from_file(
                self.as_ref().to_glib_none().0,
                filename.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn load_from_resource(&self, resource_path: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::clutter_script_load_from_resource(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn lookup_filename(&self, filename: &str) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::clutter_script_lookup_filename(
                self.as_ref().to_glib_none().0,
                filename.to_glib_none().0,
            ))
        }
    }

    fn set_translation_domain(&self, domain: Option<&str>) {
        unsafe {
            ffi::clutter_script_set_translation_domain(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
            );
        }
    }

    fn unmerge_objects(&self, merge_id: u32) {
        unsafe {
            ffi::clutter_script_unmerge_objects(self.as_ref().to_glib_none().0, merge_id);
        }
    }

    fn get_property_filename(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"filename\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `filename` getter")
        }
    }

    fn get_property_filename_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"filename-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `filename-set` getter")
                .unwrap()
        }
    }

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filename_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterScript,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Script>,
        {
            let f: &F = &*(f as *const F);
            f(&Script::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filename\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filename_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_filename_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_filename_set_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterScript,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Script>,
        {
            let f: &F = &*(f as *const F);
            f(&Script::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filename-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filename_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_translation_domain_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_translation_domain_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterScript,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Script>,
        {
            let f: &F = &*(f as *const F);
            f(&Script::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::translation-domain\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_translation_domain_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Script")
    }
}
