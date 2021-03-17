// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use super::Actor;
use super::ActorMeta;
use super::Constraint;
use super::Path;

glib_wrapper! {
    pub struct PathConstraint(Object<ffi::ClutterPathConstraint, ffi::ClutterPathConstraintClass, PathConstraintClass>) @extends Constraint, ActorMeta;

    match fn {
        get_type => || ffi::clutter_path_constraint_get_type(),
    }
}

impl PathConstraint {
    pub fn new<P: IsA<Path>>(path: Option<&P>, offset: f32) -> PathConstraint {
        assert_initialized_main_thread!();
        unsafe {
            Constraint::from_glib_none(ffi::clutter_path_constraint_new(
                path.map(|p| p.as_ref()).to_glib_none().0,
                offset,
            ))
            .unsafe_cast()
        }
    }

    pub fn get_offset(&self) -> f32 {
        unsafe { ffi::clutter_path_constraint_get_offset(self.to_glib_none().0) }
    }

    pub fn get_path(&self) -> Option<Path> {
        unsafe {
            from_glib_none(ffi::clutter_path_constraint_get_path(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_offset(&self, offset: f32) {
        unsafe {
            ffi::clutter_path_constraint_set_offset(self.to_glib_none().0, offset);
        }
    }

    pub fn set_path<P: IsA<Path>>(&self, path: Option<&P>) {
        unsafe {
            ffi::clutter_path_constraint_set_path(
                self.to_glib_none().0,
                path.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn connect_node_reached<F: Fn(&PathConstraint, &Actor, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn node_reached_trampoline<
            F: Fn(&PathConstraint, &Actor, u32) + 'static,
        >(
            this: *mut ffi::ClutterPathConstraint,
            actor: *mut ffi::ClutterActor,
            index: libc::c_uint,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(actor), index)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"node-reached\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    node_reached_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_offset_notify<F: Fn(&PathConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_trampoline<F: Fn(&PathConstraint) + 'static>(
            this: *mut ffi::ClutterPathConstraint,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_offset_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_path_notify<F: Fn(&PathConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_path_trampoline<F: Fn(&PathConstraint) + 'static>(
            this: *mut ffi::ClutterPathConstraint,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_path_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PathConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PathConstraint")
    }
}