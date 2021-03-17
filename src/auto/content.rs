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
use std::mem;
use std::mem::transmute;
use super::Actor;

glib_wrapper! {
    pub struct Content(Interface<ffi::ClutterContent>);

    match fn {
        get_type => || ffi::clutter_content_get_type(),
    }
}

pub const NONE_CONTENT: Option<&Content> = None;

pub trait ContentExt: 'static {
    fn get_preferred_size(&self) -> Option<(f32, f32)>;

    fn invalidate(&self);

    fn connect_attached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_detached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Content>> ContentExt for O {
    fn get_preferred_size(&self) -> Option<(f32, f32)> {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::clutter_content_get_preferred_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            ));
            let width = width.assume_init();
            let height = height.assume_init();
            if ret {
                Some((width, height))
            } else {
                None
            }
        }
    }

    fn invalidate(&self) {
        unsafe {
            ffi::clutter_content_invalidate(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_attached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn attached_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterContent,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Content>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Content::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"attached\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    attached_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_detached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn detached_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterContent,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Content>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Content::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"detached\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    detached_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Content")
    }
}
