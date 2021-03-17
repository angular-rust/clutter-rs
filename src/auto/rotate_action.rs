// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use super::Action;
use super::Actor;
use super::ActorMeta;
use super::GestureAction;

glib_wrapper! {
    pub struct RotateAction(Object<ffi::ClutterRotateAction, ffi::ClutterRotateActionClass, RotateActionClass>) @extends GestureAction, Action, ActorMeta;

    match fn {
        get_type => || ffi::clutter_rotate_action_get_type(),
    }
}

impl RotateAction {
    pub fn new() -> RotateAction {
        assert_initialized_main_thread!();
        unsafe { Action::from_glib_none(ffi::clutter_rotate_action_new()).unsafe_cast() }
    }
}

impl Default for RotateAction {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ROTATE_ACTION: Option<&RotateAction> = None;

pub trait RotateActionExt: 'static {
    fn connect_rotate<F: Fn(&Self, &Actor, f64) -> bool + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RotateAction>> RotateActionExt for O {
    fn connect_rotate<F: Fn(&Self, &Actor, f64) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn rotate_trampoline<P, F: Fn(&P, &Actor, f64) -> bool + 'static>(
            this: *mut ffi::ClutterRotateAction,
            actor: *mut ffi::ClutterActor,
            angle: libc::c_double,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<RotateAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &RotateAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                angle,
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"rotate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    rotate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for RotateAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RotateAction")
    }
}
