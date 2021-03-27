// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::Timeline;
use super::Transition;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct TransitionGroup(Object<ffi::ClutterTransitionGroup, ffi::ClutterTransitionGroupClass, TransitionGroupClass>) @extends Transition, Timeline;

    match fn {
        get_type => || ffi::clutter_transition_group_get_type(),
    }
}

impl TransitionGroup {
    pub fn new() -> TransitionGroup {
        assert_initialized_main_thread!();
        unsafe { Transition::from_glib_full(ffi::clutter_transition_group_new()).unsafe_cast() }
    }
}

impl Default for TransitionGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TRANSITION_GROUP: Option<&TransitionGroup> = None;

pub trait TransitionGroupExt: 'static {
    fn add_transition<P: IsA<Transition>>(&self, transition: &P);

    fn remove_all(&self);

    fn remove_transition<P: IsA<Transition>>(&self, transition: &P);
}

impl<O: IsA<TransitionGroup>> TransitionGroupExt for O {
    fn add_transition<P: IsA<Transition>>(&self, transition: &P) {
        unsafe {
            ffi::clutter_transition_group_add_transition(
                self.as_ref().to_glib_none().0,
                transition.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove_all(&self) {
        unsafe {
            ffi::clutter_transition_group_remove_all(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_transition<P: IsA<Transition>>(&self, transition: &P) {
        unsafe {
            ffi::clutter_transition_group_remove_transition(
                self.as_ref().to_glib_none().0,
                transition.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for TransitionGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransitionGroup")
    }
}
