// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ParamSpecFixed(Object<ffi::ClutterParamSpecFixed, ParamSpecFixedClass>);

    match fn {
        get_type => || ffi::clutter_param_fixed_get_type(),
    }
}

impl ParamSpecFixed {}

impl fmt::Display for ParamSpecFixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParamSpecFixed")
    }
}