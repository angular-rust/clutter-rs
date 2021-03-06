use glib::{
    object::ObjectType as ObjectType_,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    GString, StaticType, Value,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct Settings(Object<ffi::ClutterSettings, ffi::ClutterSettingsClass, SettingsClass>);

    match fn {
        get_type => || ffi::clutter_settings_get_type(),
    }
}

impl Settings {
    /// The default distance that the cursor of a pointer device
    /// should travel before a drag operation should start.
    pub fn get_property_dnd_drag_threshold(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"dnd-drag-threshold\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `dnd-drag-threshold` getter")
                .unwrap()
        }
    }

    /// The default distance that the cursor of a pointer device
    /// should travel before a drag operation should start.
    pub fn set_property_dnd_drag_threshold(&self, dnd_drag_threshold: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"dnd-drag-threshold\0".as_ptr() as *const _,
                Value::from(&dnd_drag_threshold).to_glib_none().0,
            );
        }
    }

    /// The maximum distance, in pixels, between button-press events that
    /// determines whether or not to increase the click count by 1.
    pub fn get_property_double_click_distance(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"double-click-distance\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `double-click-distance` getter")
                .unwrap()
        }
    }

    /// The maximum distance, in pixels, between button-press events that
    /// determines whether or not to increase the click count by 1.
    pub fn set_property_double_click_distance(&self, double_click_distance: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"double-click-distance\0".as_ptr() as *const _,
                Value::from(&double_click_distance).to_glib_none().0,
            );
        }
    }

    /// The time, in milliseconds, that should elapse between button-press
    /// events in order to increase the click count by 1.
    pub fn get_property_double_click_time(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"double-click-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `double-click-time` getter")
                .unwrap()
        }
    }

    /// The time, in milliseconds, that should elapse between button-press
    /// events in order to increase the click count by 1.
    pub fn set_property_double_click_time(&self, double_click_time: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"double-click-time\0".as_ptr() as *const _,
                Value::from(&double_click_time).to_glib_none().0,
            );
        }
    }

    /// Whether or not to use antialiasing when rendering text; a value
    /// of 1 enables it unconditionally; a value of 0 disables it
    /// unconditionally; and -1 will use the system's default.
    pub fn get_property_font_antialias(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-antialias\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `font-antialias` getter")
                .unwrap()
        }
    }

    /// Whether or not to use antialiasing when rendering text; a value
    /// of 1 enables it unconditionally; a value of 0 disables it
    /// unconditionally; and -1 will use the system's default.
    pub fn set_property_font_antialias(&self, font_antialias: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-antialias\0".as_ptr() as *const _,
                Value::from(&font_antialias).to_glib_none().0,
            );
        }
    }

    /// The DPI used when rendering text, as a value of 1024 * dots/inch.
    ///
    /// If set to -1, the system's default will be used instead
    pub fn get_property_font_dpi(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-dpi\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `font-dpi` getter")
                .unwrap()
        }
    }

    /// The DPI used when rendering text, as a value of 1024 * dots/inch.
    ///
    /// If set to -1, the system's default will be used instead
    pub fn set_property_font_dpi(&self, font_dpi: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-dpi\0".as_ptr() as *const _,
                Value::from(&font_dpi).to_glib_none().0,
            );
        }
    }

    /// The style of the hinting used when rendering text. Valid values
    /// are:
    ///
    ///  - hintnone
    ///  - hintslight
    ///  - hintmedium
    ///  - hintfull
    pub fn get_property_font_hint_style(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-hint-style\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `font-hint-style` getter")
        }
    }

    /// The style of the hinting used when rendering text. Valid values
    /// are:
    ///
    ///  - hintnone
    ///  - hintslight
    ///  - hintmedium
    ///  - hintfull
    pub fn set_property_font_hint_style(&self, font_hint_style: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-hint-style\0".as_ptr() as *const _,
                Value::from(font_hint_style).to_glib_none().0,
            );
        }
    }

    /// Whether or not to use hinting when rendering text; a value of 1
    /// unconditionally enables it; a value of 0 unconditionally disables
    /// it; and a value of -1 will use the system's default.
    pub fn get_property_font_hinting(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-hinting\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `font-hinting` getter")
                .unwrap()
        }
    }

    /// Whether or not to use hinting when rendering text; a value of 1
    /// unconditionally enables it; a value of 0 unconditionally disables
    /// it; and a value of -1 will use the system's default.
    pub fn set_property_font_hinting(&self, font_hinting: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-hinting\0".as_ptr() as *const _,
                Value::from(&font_hinting).to_glib_none().0,
            );
        }
    }

    /// The default font name that should be used by text actors, as
    /// a string that can be passed to `pango::FontDescription::from_string`.
    pub fn get_property_font_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `font-name` getter")
        }
    }

    /// The default font name that should be used by text actors, as
    /// a string that can be passed to `pango::FontDescription::from_string`.
    pub fn set_property_font_name(&self, font_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-name\0".as_ptr() as *const _,
                Value::from(font_name).to_glib_none().0,
            );
        }
    }

    /// The type of sub-pixel antialiasing used when rendering text. Valid
    /// values are:
    ///
    ///  - none
    ///  - rgb
    ///  - bgr
    ///  - vrgb
    ///  - vbgr
    pub fn get_property_font_subpixel_order(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-subpixel-order\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `font-subpixel-order` getter")
        }
    }

    /// The type of sub-pixel antialiasing used when rendering text. Valid
    /// values are:
    ///
    ///  - none
    ///  - rgb
    ///  - bgr
    ///  - vrgb
    ///  - vbgr
    pub fn set_property_font_subpixel_order(&self, font_subpixel_order: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"font-subpixel-order\0".as_ptr() as *const _,
                Value::from(font_subpixel_order).to_glib_none().0,
            );
        }
    }

    pub fn set_property_fontconfig_timestamp(&self, fontconfig_timestamp: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"fontconfig-timestamp\0".as_ptr() as *const _,
                Value::from(&fontconfig_timestamp).to_glib_none().0,
            );
        }
    }

    /// Sets the minimum duration for a press to be recognized as a long press
    /// gesture. The duration is expressed in milliseconds.
    ///
    /// See also `ClickAction:long-press-duration`.
    pub fn get_property_long_press_duration(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"long-press-duration\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `long-press-duration` getter")
                .unwrap()
        }
    }

    /// Sets the minimum duration for a press to be recognized as a long press
    /// gesture. The duration is expressed in milliseconds.
    ///
    /// See also `ClickAction:long-press-duration`.
    pub fn set_property_long_press_duration(&self, long_press_duration: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"long-press-duration\0".as_ptr() as *const _,
                Value::from(&long_press_duration).to_glib_none().0,
            );
        }
    }

    pub fn get_property_password_hint_time(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"password-hint-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `password-hint-time` getter")
                .unwrap()
        }
    }

    pub fn set_property_password_hint_time(&self, password_hint_time: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"password-hint-time\0".as_ptr() as *const _,
                Value::from(&password_hint_time).to_glib_none().0,
            );
        }
    }

    pub fn set_property_unscaled_font_dpi(&self, unscaled_font_dpi: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"unscaled-font-dpi\0".as_ptr() as *const _,
                Value::from(&unscaled_font_dpi).to_glib_none().0,
            );
        }
    }

    pub fn get_property_window_scaling_factor(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"window-scaling-factor\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `window-scaling-factor` getter")
                .unwrap()
        }
    }

    pub fn set_property_window_scaling_factor(&self, window_scaling_factor: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"window-scaling-factor\0".as_ptr() as *const _,
                Value::from(&window_scaling_factor).to_glib_none().0,
            );
        }
    }

    /// Retrieves the singleton instance of `Settings`
    ///
    /// # Returns
    ///
    /// the instance of `Settings`. The
    ///  returned object is owned by Clutter and it should not be unreferenced
    ///  directly
    pub fn get_default() -> Option<Settings> {
        unsafe { from_glib_none(ffi::clutter_settings_get_default()) }
    }

    pub fn connect_property_dnd_drag_threshold_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_dnd_drag_threshold_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::dnd-drag-threshold\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dnd_drag_threshold_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_double_click_distance_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_double_click_distance_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::double-click-distance\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_double_click_distance_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_double_click_time_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_double_click_time_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::double-click-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_double_click_time_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_font_antialias_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_antialias_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::font-antialias\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_antialias_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_font_dpi_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_dpi_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::font-dpi\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_dpi_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_font_hint_style_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_hint_style_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::font-hint-style\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_hint_style_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_font_hinting_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_hinting_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::font-hinting\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_hinting_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_font_name_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_name_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::font-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_font_subpixel_order_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_subpixel_order_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::font-subpixel-order\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_subpixel_order_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_fontconfig_timestamp_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_fontconfig_timestamp_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::fontconfig-timestamp\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fontconfig_timestamp_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_long_press_duration_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_long_press_duration_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::long-press-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_long_press_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_password_hint_time_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_hint_time_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::password-hint-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_hint_time_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_unscaled_font_dpi_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_unscaled_font_dpi_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::unscaled-font-dpi\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_unscaled_font_dpi_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_window_scaling_factor_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_window_scaling_factor_trampoline<F: Fn(&Settings) + 'static>(
            this: *mut ffi::ClutterSettings,
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
                b"notify::window-scaling-factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_window_scaling_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Settings")
    }
}
