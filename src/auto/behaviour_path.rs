// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::Path;
use crate::Behaviour;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct BehaviourPath(Object<ffi::ClutterBehaviourPath, ffi::ClutterBehaviourPathClass, BehaviourPathClass>) @extends Behaviour;

    match fn {
        get_type => || ffi::clutter_behaviour_path_get_type(),
    }
}

pub const NONE_BEHAVIOUR_PATH: Option<&BehaviourPath> = None;

pub trait BehaviourPathExt: 'static {
    fn get_property_path(&self) -> Option<Path>;

    fn set_property_path<P: IsA<Path> + SetValueOptional>(&self, path: Option<&P>);

    fn connect_property_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BehaviourPath>> BehaviourPathExt for O {
    fn get_property_path(&self) -> Option<Path> {
        unsafe {
            let mut value = Value::from_type(<Path as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"path\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `path` getter")
        }
    }

    fn set_property_path<P: IsA<Path> + SetValueOptional>(&self, path: Option<&P>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"path\0".as_ptr() as *const _,
                Value::from(path).to_glib_none().0,
            );
        }
    }

    fn connect_property_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_path_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourPath,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourPath>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourPath::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_path_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for BehaviourPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BehaviourPath")
    }
}
