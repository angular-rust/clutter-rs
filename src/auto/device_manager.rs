// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::Backend;
use super::InputDevice;
use super::InputDeviceType;
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
use std::mem::transmute;

glib_wrapper! {
    pub struct DeviceManager(Object<ffi::ClutterDeviceManager, ffi::ClutterDeviceManagerClass, DeviceManagerClass>);

    match fn {
        get_type => || ffi::clutter_device_manager_get_type(),
    }
}

impl DeviceManager {
    pub fn get_default() -> Option<DeviceManager> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::clutter_device_manager_get_default()) }
    }
}

pub const NONE_DEVICE_MANAGER: Option<&DeviceManager> = None;

pub trait DeviceManagerExt: 'static {
    fn get_core_device(&self, device_type: InputDeviceType) -> Option<InputDevice>;

    fn get_device(&self, device_id: i32) -> Option<InputDevice>;

    fn list_devices(&self) -> Vec<InputDevice>;

    fn peek_devices(&self) -> Vec<InputDevice>;

    fn get_property_backend(&self) -> Option<Backend>;

    fn connect_device_added<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_device_removed<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<DeviceManager>> DeviceManagerExt for O {
    fn get_core_device(&self, device_type: InputDeviceType) -> Option<InputDevice> {
        unsafe {
            from_glib_none(ffi::clutter_device_manager_get_core_device(
                self.as_ref().to_glib_none().0,
                device_type.to_glib(),
            ))
        }
    }

    fn get_device(&self, device_id: i32) -> Option<InputDevice> {
        unsafe {
            from_glib_none(ffi::clutter_device_manager_get_device(
                self.as_ref().to_glib_none().0,
                device_id,
            ))
        }
    }

    fn list_devices(&self) -> Vec<InputDevice> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::clutter_device_manager_list_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn peek_devices(&self) -> Vec<InputDevice> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::clutter_device_manager_peek_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_backend(&self) -> Option<Backend> {
        unsafe {
            let mut value = Value::from_type(<Backend as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"backend\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `backend` getter")
        }
    }

    fn connect_device_added<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<P, F: Fn(&P, &InputDevice) + 'static>(
            this: *mut ffi::ClutterDeviceManager,
            device: *mut ffi::ClutterInputDevice,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DeviceManager>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DeviceManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(device),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_device_removed<F: Fn(&Self, &InputDevice) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<P, F: Fn(&P, &InputDevice) + 'static>(
            this: *mut ffi::ClutterDeviceManager,
            device: *mut ffi::ClutterInputDevice,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DeviceManager>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DeviceManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(device),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceManager")
    }
}
