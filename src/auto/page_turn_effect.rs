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
use super::DeformEffect;
use super::Effect;
use super::OffscreenEffect;

glib_wrapper! {
    pub struct PageTurnEffect(Object<ffi::ClutterPageTurnEffect, ffi::ClutterPageTurnEffectClass, PageTurnEffectClass>) @extends DeformEffect, OffscreenEffect, Effect, ActorMeta;

    match fn {
        get_type => || ffi::clutter_page_turn_effect_get_type(),
    }
}

impl PageTurnEffect {
    pub fn new(period: f64, angle: f64, radius: f32) -> PageTurnEffect {
        assert_initialized_main_thread!();
        unsafe {
            Effect::from_glib_none(ffi::clutter_page_turn_effect_new(
                period, angle, radius,
            ))
            .unsafe_cast()
        }
    }

    pub fn get_angle(&self) -> f64 {
        unsafe { ffi::clutter_page_turn_effect_get_angle(self.to_glib_none().0) }
    }

    pub fn get_period(&self) -> f64 {
        unsafe { ffi::clutter_page_turn_effect_get_period(self.to_glib_none().0) }
    }

    pub fn get_radius(&self) -> f32 {
        unsafe { ffi::clutter_page_turn_effect_get_radius(self.to_glib_none().0) }
    }

    pub fn set_angle(&self, angle: f64) {
        unsafe {
            ffi::clutter_page_turn_effect_set_angle(self.to_glib_none().0, angle);
        }
    }

    pub fn set_period(&self, period: f64) {
        unsafe {
            ffi::clutter_page_turn_effect_set_period(self.to_glib_none().0, period);
        }
    }

    pub fn set_radius(&self, radius: f32) {
        unsafe {
            ffi::clutter_page_turn_effect_set_radius(self.to_glib_none().0, radius);
        }
    }

    pub fn connect_property_angle_notify<F: Fn(&PageTurnEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_angle_trampoline<F: Fn(&PageTurnEffect) + 'static>(
            this: *mut ffi::ClutterPageTurnEffect,
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
                b"notify::angle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_angle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_period_notify<F: Fn(&PageTurnEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_period_trampoline<F: Fn(&PageTurnEffect) + 'static>(
            this: *mut ffi::ClutterPageTurnEffect,
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
                b"notify::period\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_period_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_radius_notify<F: Fn(&PageTurnEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_radius_trampoline<F: Fn(&PageTurnEffect) + 'static>(
            this: *mut ffi::ClutterPageTurnEffect,
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
                b"notify::radius\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_radius_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PageTurnEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PageTurnEffect")
    }
}