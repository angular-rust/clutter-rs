// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use super::Actor;
use super::Backend;
use super::DeviceManager;
use crate::Event;
use super::EventSequence;
use super::InputAxis;
use super::InputDeviceType;
use super::InputMode;
use super::ModifierType;
use super::Stage;

glib_wrapper! {
    pub struct InputDevice(Object<ffi::ClutterInputDevice, ffi::ClutterInputDeviceClass, InputDeviceClass>);

    match fn {
        get_type => || ffi::clutter_input_device_get_type(),
    }
}

impl InputDevice {
    pub fn get_associated_device(&self) -> Option<InputDevice> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_associated_device(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_axis(&self, index_: u32) -> InputAxis {
        unsafe {
            from_glib(ffi::clutter_input_device_get_axis(
                self.to_glib_none().0,
                index_,
            ))
        }
    }

    //pub fn get_axis_value(&self, axes: &[f64], axis: InputAxis) -> Option<f64> {
    //    unsafe { TODO: call clutter_sys:clutter_input_device_get_axis_value() }
    //}

    pub fn get_device_id(&self) -> i32 {
        unsafe { ffi::clutter_input_device_get_device_id(self.to_glib_none().0) }
    }

    pub fn get_device_mode(&self) -> InputMode {
        unsafe {
            from_glib(ffi::clutter_input_device_get_device_mode(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_device_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_device_name(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_device_type(&self) -> InputDeviceType {
        unsafe {
            from_glib(ffi::clutter_input_device_get_device_type(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_input_device_get_enabled(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_grabbed_actor(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_grabbed_actor(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_has_cursor(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_input_device_get_has_cursor(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_key(&self, index_: u32) -> Option<(u32, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut modifiers = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::clutter_input_device_get_key(
                self.to_glib_none().0,
                index_,
                keyval.as_mut_ptr(),
                modifiers.as_mut_ptr(),
            ));
            let keyval = keyval.assume_init();
            let modifiers = modifiers.assume_init();
            if ret {
                Some((keyval, from_glib(modifiers)))
            } else {
                None
            }
        }
    }

    pub fn get_modifier_state(&self) -> ModifierType {
        unsafe {
            from_glib(ffi::clutter_input_device_get_modifier_state(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_n_axes(&self) -> u32 {
        unsafe { ffi::clutter_input_device_get_n_axes(self.to_glib_none().0) }
    }

    pub fn get_n_keys(&self) -> u32 {
        unsafe { ffi::clutter_input_device_get_n_keys(self.to_glib_none().0) }
    }

    pub fn get_pointer_actor(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_pointer_actor(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_pointer_stage(&self) -> Option<Stage> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_pointer_stage(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_product_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_product_id(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_slave_devices(&self) -> Vec<InputDevice> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::clutter_input_device_get_slave_devices(self.to_glib_none().0),
            )
        }
    }

    pub fn get_vendor_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_vendor_id(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn grab<P: IsA<Actor>>(&self, actor: &P) {
        unsafe {
            ffi::clutter_input_device_grab(
                self.to_glib_none().0,
                actor.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn sequence_get_grabbed_actor(&self, sequence: &mut EventSequence) -> Option<Actor> {
        unsafe {
            from_glib_none(
                ffi::clutter_input_device_sequence_get_grabbed_actor(
                    self.to_glib_none().0,
                    sequence.to_glib_none_mut().0,
                ),
            )
        }
    }

    pub fn sequence_grab<P: IsA<Actor>>(&self, sequence: &mut EventSequence, actor: &P) {
        unsafe {
            ffi::clutter_input_device_sequence_grab(
                self.to_glib_none().0,
                sequence.to_glib_none_mut().0,
                actor.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn sequence_ungrab(&self, sequence: &mut EventSequence) {
        unsafe {
            ffi::clutter_input_device_sequence_ungrab(
                self.to_glib_none().0,
                sequence.to_glib_none_mut().0,
            );
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::clutter_input_device_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_key(&self, index_: u32, keyval: u32, modifiers: ModifierType) {
        unsafe {
            ffi::clutter_input_device_set_key(
                self.to_glib_none().0,
                index_,
                keyval,
                modifiers.to_glib(),
            );
        }
    }

    pub fn ungrab(&self) {
        unsafe {
            ffi::clutter_input_device_ungrab(self.to_glib_none().0);
        }
    }

    pub fn update_from_event(&self, event: &mut Event, update_stage: bool) {
        unsafe {
            ffi::clutter_input_device_update_from_event(
                self.to_glib_none().0,
                event.to_glib_none_mut().0,
                update_stage.to_glib(),
            );
        }
    }

    pub fn get_property_backend(&self) -> Option<Backend> {
        unsafe {
            let mut value = Value::from_type(<Backend as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"backend\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `backend` getter")
        }
    }

    pub fn get_property_device_manager(&self) -> Option<DeviceManager> {
        unsafe {
            let mut value = Value::from_type(<DeviceManager as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"device-manager\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `device-manager` getter")
        }
    }

    pub fn get_property_id(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `id` getter")
                .unwrap()
        }
    }

    pub fn get_property_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `name` getter")
        }
    }

    pub fn connect_property_enabled_notify<F: Fn(&InputDevice) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<F: Fn(&InputDevice) + 'static>(
            this: *mut ffi::ClutterInputDevice,
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
                b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enabled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_n_axes_notify<F: Fn(&InputDevice) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_axes_trampoline<F: Fn(&InputDevice) + 'static>(
            this: *mut ffi::ClutterInputDevice,
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
                b"notify::n-axes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_axes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for InputDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputDevice")
    }
}
