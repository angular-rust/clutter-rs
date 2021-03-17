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
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use super::Actor;
use super::Animatable;
use super::Container;
use super::Point;
use super::Rect;
use super::ScrollMode;

glib_wrapper! {
    pub struct ScrollActor(Object<ffi::ClutterScrollActor, ffi::ClutterScrollActorClass, ScrollActorClass>) @extends Actor, @implements Animatable, Container;

    match fn {
        get_type => || ffi::clutter_scroll_actor_get_type(),
    }
}

impl ScrollActor {
    pub fn new() -> ScrollActor {
        assert_initialized_main_thread!();
        unsafe { Actor::from_glib_none(ffi::clutter_scroll_actor_new()).unsafe_cast() }
    }
}

impl Default for ScrollActor {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SCROLL_ACTOR: Option<&ScrollActor> = None;

pub trait ScrollActorExt: 'static {
    fn get_scroll_mode(&self) -> ScrollMode;

    fn scroll_to_point(&self, point: &Point);

    fn scroll_to_rect(&self, rect: &Rect);

    fn set_scroll_mode(&self, mode: ScrollMode);

    fn connect_property_scroll_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ScrollActor>> ScrollActorExt for O {
    fn get_scroll_mode(&self) -> ScrollMode {
        unsafe {
            from_glib(ffi::clutter_scroll_actor_get_scroll_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn scroll_to_point(&self, point: &Point) {
        unsafe {
            ffi::clutter_scroll_actor_scroll_to_point(
                self.as_ref().to_glib_none().0,
                point.to_glib_none().0,
            );
        }
    }

    fn scroll_to_rect(&self, rect: &Rect) {
        unsafe {
            ffi::clutter_scroll_actor_scroll_to_rect(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none().0,
            );
        }
    }

    fn set_scroll_mode(&self, mode: ScrollMode) {
        unsafe {
            ffi::clutter_scroll_actor_set_scroll_mode(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn connect_property_scroll_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scroll_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterScrollActor,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ScrollActor>,
        {
            let f: &F = &*(f as *const F);
            f(&ScrollActor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scroll-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scroll_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ScrollActor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrollActor")
    }
}