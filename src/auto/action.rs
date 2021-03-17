use ffi;
use glib::translate::*;
use std::fmt;
use super::ActorMeta;

glib_wrapper! {
    pub struct Action(Object<ffi::ClutterAction, ffi::ClutterActionClass, ActionClass>) @extends ActorMeta;

    match fn {
        get_type => || ffi::clutter_action_get_type(),
    }
}

impl Action {}

pub const NONE_ACTION: Option<&Action> = None;

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action")
    }
}
