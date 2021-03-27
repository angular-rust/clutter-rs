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
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Interval(Object<ffi::ClutterInterval, ffi::ClutterIntervalClass, IntervalClass>);

    match fn {
        get_type => || ffi::clutter_interval_get_type(),
    }
}

impl Interval {
    //pub fn new(gtype: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Interval {
    //    unsafe { TODO: call clutter_sys:clutter_interval_new() }
    //}

    //pub fn with_values(gtype: glib::types::Type, initial: /*Ignored*/Option<&glib::Value>, final_: /*Ignored*/Option<&glib::Value>) -> Interval {
    //    unsafe { TODO: call clutter_sys:clutter_interval_new_with_values() }
    //}

    //pub fn register_progress_func(value_type: glib::types::Type, func: /*Unimplemented*/Fn(/*Ignored*/glib::Value, /*Ignored*/glib::Value, f64, /*Ignored*/glib::Value) -> bool) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_register_progress_func() }
    //}
}

pub const NONE_INTERVAL: Option<&Interval> = None;

pub trait IntervalExt: 'static {
    fn clone(&self) -> Option<Interval>;

    //fn compute(&self, factor: f64) -> /*Ignored*/Option<glib::Value>;

    //fn compute_value(&self, factor: f64, value: /*Ignored*/glib::Value) -> bool;

    //fn get_final_value(&self, value: /*Ignored*/glib::Value);

    //fn get_initial_value(&self, value: /*Ignored*/glib::Value);

    //fn get_interval(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_value_type(&self) -> glib::types::Type;

    fn is_valid(&self) -> bool;

    //fn peek_final_value(&self) -> /*Ignored*/Option<glib::Value>;

    //fn peek_initial_value(&self) -> /*Ignored*/Option<glib::Value>;

    //fn set_final(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_final_value(&self, value: /*Ignored*/&glib::Value);

    //fn set_initial(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_initial_value(&self, value: /*Ignored*/&glib::Value);

    //fn set_interval(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn get_property_final(&self) -> /*Ignored*/Option<glib::Value>;

    //fn get_property_initial(&self) -> /*Ignored*/Option<glib::Value>;

    fn connect_property_final_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_initial_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Interval>> IntervalExt for O {
    fn clone(&self) -> Option<Interval> {
        unsafe { from_glib_full(ffi::clutter_interval_clone(self.as_ref().to_glib_none().0)) }
    }

    //fn compute(&self, factor: f64) -> /*Ignored*/Option<glib::Value> {
    //    unsafe { TODO: call clutter_sys:clutter_interval_compute() }
    //}

    //fn compute_value(&self, factor: f64, value: /*Ignored*/glib::Value) -> bool {
    //    unsafe { TODO: call clutter_sys:clutter_interval_compute_value() }
    //}

    //fn get_final_value(&self, value: /*Ignored*/glib::Value) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_get_final_value() }
    //}

    //fn get_initial_value(&self, value: /*Ignored*/glib::Value) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_get_initial_value() }
    //}

    //fn get_interval(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_get_interval() }
    //}

    fn get_value_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::clutter_interval_get_value_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_valid(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_interval_is_valid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn peek_final_value(&self) -> /*Ignored*/Option<glib::Value> {
    //    unsafe { TODO: call clutter_sys:clutter_interval_peek_final_value() }
    //}

    //fn peek_initial_value(&self) -> /*Ignored*/Option<glib::Value> {
    //    unsafe { TODO: call clutter_sys:clutter_interval_peek_initial_value() }
    //}

    //fn set_final(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_set_final() }
    //}

    //fn set_final_value(&self, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_set_final_value() }
    //}

    //fn set_initial(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_set_initial() }
    //}

    //fn set_initial_value(&self, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_set_initial_value() }
    //}

    //fn set_interval(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_set_interval() }
    //}

    //fn get_property_final(&self) -> /*Ignored*/Option<glib::Value> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"final\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `final` getter")
    //    }
    //}

    //fn get_property_initial(&self) -> /*Ignored*/Option<glib::Value> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"initial\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `initial` getter")
    //    }
    //}

    fn connect_property_final_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_final_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterInterval,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Interval>,
        {
            let f: &F = &*(f as *const F);
            f(&Interval::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::final\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_final_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_initial_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_initial_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterInterval,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Interval>,
        {
            let f: &F = &*(f as *const F);
            f(&Interval::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::initial\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_initial_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interval")
    }
}
