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
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use super::Actor;
use super::ActorMeta;
use super::Constraint;
use super::SnapEdge;

glib_wrapper! {
    pub struct SnapConstraint(Object<ffi::ClutterSnapConstraint, ffi::ClutterSnapConstraintClass, SnapConstraintClass>) @extends Constraint, ActorMeta;

    match fn {
        get_type => || ffi::clutter_snap_constraint_get_type(),
    }
}

impl SnapConstraint {
    pub fn new<P: IsA<Actor>>(
        source: Option<&P>,
        from_edge: SnapEdge,
        to_edge: SnapEdge,
        offset: f32,
    ) -> SnapConstraint {
        assert_initialized_main_thread!();
        unsafe {
            Constraint::from_glib_none(ffi::clutter_snap_constraint_new(
                source.map(|p| p.as_ref()).to_glib_none().0,
                from_edge.to_glib(),
                to_edge.to_glib(),
                offset,
            ))
            .unsafe_cast()
        }
    }

    pub fn get_edges(&self) -> (SnapEdge, SnapEdge) {
        unsafe {
            let mut from_edge = mem::MaybeUninit::uninit();
            let mut to_edge = mem::MaybeUninit::uninit();
            ffi::clutter_snap_constraint_get_edges(
                self.to_glib_none().0,
                from_edge.as_mut_ptr(),
                to_edge.as_mut_ptr(),
            );
            let from_edge = from_edge.assume_init();
            let to_edge = to_edge.assume_init();
            (from_glib(from_edge), from_glib(to_edge))
        }
    }

    pub fn get_offset(&self) -> f32 {
        unsafe { ffi::clutter_snap_constraint_get_offset(self.to_glib_none().0) }
    }

    pub fn get_source(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_snap_constraint_get_source(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_edges(&self, from_edge: SnapEdge, to_edge: SnapEdge) {
        unsafe {
            ffi::clutter_snap_constraint_set_edges(
                self.to_glib_none().0,
                from_edge.to_glib(),
                to_edge.to_glib(),
            );
        }
    }

    pub fn set_offset(&self, offset: f32) {
        unsafe {
            ffi::clutter_snap_constraint_set_offset(self.to_glib_none().0, offset);
        }
    }

    pub fn set_source<P: IsA<Actor>>(&self, source: Option<&P>) {
        unsafe {
            ffi::clutter_snap_constraint_set_source(
                self.to_glib_none().0,
                source.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn get_property_from_edge(&self) -> SnapEdge {
        unsafe {
            let mut value = Value::from_type(<SnapEdge as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"from-edge\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `from-edge` getter")
                .unwrap()
        }
    }

    pub fn set_property_from_edge(&self, from_edge: SnapEdge) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"from-edge\0".as_ptr() as *const _,
                Value::from(&from_edge).to_glib_none().0,
            );
        }
    }

    pub fn get_property_to_edge(&self) -> SnapEdge {
        unsafe {
            let mut value = Value::from_type(<SnapEdge as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"to-edge\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `to-edge` getter")
                .unwrap()
        }
    }

    pub fn set_property_to_edge(&self, to_edge: SnapEdge) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"to-edge\0".as_ptr() as *const _,
                Value::from(&to_edge).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_from_edge_notify<F: Fn(&SnapConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_from_edge_trampoline<F: Fn(&SnapConstraint) + 'static>(
            this: *mut ffi::ClutterSnapConstraint,
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
                b"notify::from-edge\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_from_edge_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_offset_notify<F: Fn(&SnapConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_trampoline<F: Fn(&SnapConstraint) + 'static>(
            this: *mut ffi::ClutterSnapConstraint,
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

    pub fn connect_property_source_notify<F: Fn(&SnapConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_source_trampoline<F: Fn(&SnapConstraint) + 'static>(
            this: *mut ffi::ClutterSnapConstraint,
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
                b"notify::source\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_source_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_to_edge_notify<F: Fn(&SnapConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_to_edge_trampoline<F: Fn(&SnapConstraint) + 'static>(
            this: *mut ffi::ClutterSnapConstraint,
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
                b"notify::to-edge\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_to_edge_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SnapConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SnapConstraint")
    }
}
