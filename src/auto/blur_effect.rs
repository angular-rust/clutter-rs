// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::ActorMeta;
use super::Effect;
use super::OffscreenEffect;
use ffi;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct BlurEffect(Object<ffi::ClutterBlurEffect, ffi::ClutterBlurEffectClass, BlurEffectClass>) @extends OffscreenEffect, Effect, ActorMeta;

    match fn {
        get_type => || ffi::clutter_blur_effect_get_type(),
    }
}

impl BlurEffect {
    pub fn new() -> BlurEffect {
        assert_initialized_main_thread!();
        unsafe { Effect::from_glib_none(ffi::clutter_blur_effect_new()).unsafe_cast() }
    }
}

impl Default for BlurEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BlurEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BlurEffect")
    }
}
