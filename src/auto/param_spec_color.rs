// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ParamSpecColor(Object<ffi::ClutterParamSpecColor, ParamSpecColorClass>);

    match fn {
        get_type => || ffi::clutter_param_color_get_type(),
    }
}

impl ParamSpecColor {}

impl fmt::Display for ParamSpecColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParamSpecColor")
    }
}
