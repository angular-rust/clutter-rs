// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ShaderInt(Object<ffi::ClutterShaderInt, ShaderIntClass>);

    match fn {
        get_type => || ffi::clutter_shader_int_get_type(),
    }
}

impl ShaderInt {}

impl fmt::Display for ShaderInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShaderInt")
    }
}
