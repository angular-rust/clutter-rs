// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use super::ChildMeta;
use super::LayoutManager;

glib_wrapper! {
    pub struct LayoutMeta(Object<ffi::ClutterLayoutMeta, ffi::ClutterLayoutMetaClass, LayoutMetaClass>) @extends ChildMeta;

    match fn {
        get_type => || ffi::clutter_layout_meta_get_type(),
    }
}

pub const NONE_LAYOUT_META: Option<&LayoutMeta> = None;

pub trait LayoutMetaExt: 'static {
    fn get_manager(&self) -> Option<LayoutManager>;
}

impl<O: IsA<LayoutMeta>> LayoutMetaExt for O {
    fn get_manager(&self) -> Option<LayoutManager> {
        unsafe {
            from_glib_none(ffi::clutter_layout_meta_get_manager(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for LayoutMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LayoutMeta")
    }
}