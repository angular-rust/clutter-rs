// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use std::fmt;
use super::Behaviour;

glib_wrapper! {
    pub struct BehaviourScale(Object<ffi::ClutterBehaviourScale, ffi::ClutterBehaviourScaleClass, BehaviourScaleClass>) @extends Behaviour;

    match fn {
        get_type => || ffi::clutter_behaviour_scale_get_type(),
    }
}

impl BehaviourScale {}

pub const NONE_BEHAVIOUR_SCALE: Option<&BehaviourScale> = None;

impl fmt::Display for BehaviourScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BehaviourScale")
    }
}