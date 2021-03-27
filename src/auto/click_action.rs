// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::Action;
use super::Actor;
use super::ActorMeta;
use super::LongPressState;
use super::ModifierType;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct ClickAction(Object<ffi::ClutterClickAction, ffi::ClutterClickActionClass, ClickActionClass>) @extends Action, ActorMeta;

    match fn {
        get_type => || ffi::clutter_click_action_get_type(),
    }
}

impl ClickAction {
    pub fn new() -> ClickAction {
        assert_initialized_main_thread!();
        unsafe { Action::from_glib_none(ffi::clutter_click_action_new()).unsafe_cast() }
    }
}

impl Default for ClickAction {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CLICK_ACTION: Option<&ClickAction> = None;

pub trait ClickActionExt: 'static {
    fn get_button(&self) -> u32;

    fn get_coords(&self) -> (f32, f32);

    fn get_state(&self) -> ModifierType;

    fn release(&self);

    fn get_property_held(&self) -> bool;

    fn get_property_long_press_duration(&self) -> i32;

    fn set_property_long_press_duration(&self, long_press_duration: i32);

    fn get_property_long_press_threshold(&self) -> i32;

    fn set_property_long_press_threshold(&self, long_press_threshold: i32);

    fn get_property_pressed(&self) -> bool;

    fn connect_clicked<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_long_press<F: Fn(&Self, &Actor, LongPressState) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_held_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_long_press_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_long_press_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_pressed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ClickAction>> ClickActionExt for O {
    fn get_button(&self) -> u32 {
        unsafe { ffi::clutter_click_action_get_button(self.as_ref().to_glib_none().0) }
    }

    fn get_coords(&self) -> (f32, f32) {
        unsafe {
            let mut press_x = mem::MaybeUninit::uninit();
            let mut press_y = mem::MaybeUninit::uninit();
            ffi::clutter_click_action_get_coords(
                self.as_ref().to_glib_none().0,
                press_x.as_mut_ptr(),
                press_y.as_mut_ptr(),
            );
            let press_x = press_x.assume_init();
            let press_y = press_y.assume_init();
            (press_x, press_y)
        }
    }

    fn get_state(&self) -> ModifierType {
        unsafe {
            from_glib(ffi::clutter_click_action_get_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn release(&self) {
        unsafe {
            ffi::clutter_click_action_release(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_held(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"held\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `held` getter")
                .unwrap()
        }
    }

    fn get_property_long_press_duration(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"long-press-duration\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `long-press-duration` getter")
                .unwrap()
        }
    }

    fn set_property_long_press_duration(&self, long_press_duration: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"long-press-duration\0".as_ptr() as *const _,
                Value::from(&long_press_duration).to_glib_none().0,
            );
        }
    }

    fn get_property_long_press_threshold(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"long-press-threshold\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `long-press-threshold` getter")
                .unwrap()
        }
    }

    fn set_property_long_press_threshold(&self, long_press_threshold: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"long-press-threshold\0".as_ptr() as *const _,
                Value::from(&long_press_threshold).to_glib_none().0,
            );
        }
    }

    fn get_property_pressed(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pressed\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pressed` getter")
                .unwrap()
        }
    }

    fn connect_clicked<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn clicked_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterClickAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ClickAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"clicked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    clicked_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_long_press<F: Fn(&Self, &Actor, LongPressState) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn long_press_trampoline<
            P,
            F: Fn(&P, &Actor, LongPressState) -> bool + 'static,
        >(
            this: *mut ffi::ClutterClickAction,
            actor: *mut ffi::ClutterActor,
            state: ffi::ClutterLongPressState,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ClickAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                from_glib(state),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"long-press\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    long_press_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_held_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_held_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterClickAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(&ClickAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::held\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_held_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_long_press_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_long_press_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterClickAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(&ClickAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::long-press-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_long_press_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_long_press_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_long_press_threshold_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterClickAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(&ClickAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::long-press-threshold\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_long_press_threshold_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pressed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pressed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterClickAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(&ClickAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pressed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pressed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ClickAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClickAction")
    }
}
