// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use super::ActorMeta;
use super::Effect;
use super::OffscreenEffect;

glib_wrapper! {
    pub struct DesaturateEffect(Object<ffi::ClutterDesaturateEffect, ffi::ClutterDesaturateEffectClass, DesaturateEffectClass>) @extends OffscreenEffect, Effect, ActorMeta;

    match fn {
        get_type => || ffi::clutter_desaturate_effect_get_type(),
    }
}

impl DesaturateEffect {
    pub fn new(factor: f64) -> DesaturateEffect {
        assert_initialized_main_thread!();
        unsafe {
            Effect::from_glib_none(ffi::clutter_desaturate_effect_new(factor)).unsafe_cast()
        }
    }

    pub fn get_factor(&self) -> f64 {
        unsafe { ffi::clutter_desaturate_effect_get_factor(self.to_glib_none().0) }
    }

    pub fn set_factor(&self, factor: f64) {
        unsafe {
            ffi::clutter_desaturate_effect_set_factor(self.to_glib_none().0, factor);
        }
    }

    pub fn connect_property_factor_notify<F: Fn(&DesaturateEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_factor_trampoline<F: Fn(&DesaturateEffect) + 'static>(
            this: *mut ffi::ClutterDesaturateEffect,
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
                b"notify::factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DesaturateEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DesaturateEffect")
    }
}