// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use std::fmt;
use super::ActorMeta;

glib_wrapper! {
    pub struct Constraint(Object<ffi::ClutterConstraint, ffi::ClutterConstraintClass, ConstraintClass>) @extends ActorMeta;

    match fn {
        get_type => || ffi::clutter_constraint_get_type(),
    }
}

impl Constraint {}

pub const NONE_CONSTRAINT: Option<&Constraint> = None;

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Constraint")
    }
}