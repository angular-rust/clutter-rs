// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::Actor;
use super::Container;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ChildMeta(Object<ffi::ClutterChildMeta, ffi::ClutterChildMetaClass, ChildMetaClass>);

    match fn {
        get_type => || ffi::clutter_child_meta_get_type(),
    }
}

pub const NONE_CHILD_META: Option<&ChildMeta> = None;

pub trait ChildMetaExt: 'static {
    fn get_actor(&self) -> Option<Actor>;

    fn get_container(&self) -> Option<Container>;
}

impl<O: IsA<ChildMeta>> ChildMetaExt for O {
    fn get_actor(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_child_meta_get_actor(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_container(&self) -> Option<Container> {
        unsafe {
            from_glib_none(ffi::clutter_child_meta_get_container(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ChildMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChildMeta")
    }
}
