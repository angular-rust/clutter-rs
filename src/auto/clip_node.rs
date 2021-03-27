// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::PaintNode;
use ffi;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ClipNode(Object<ffi::ClutterClipNode, ffi::ClutterClipNodeClass, ClipNodeClass>) @extends PaintNode;

    match fn {
        get_type => || ffi::clutter_clip_node_get_type(),
    }
}

impl ClipNode {
    pub fn new() -> ClipNode {
        assert_initialized_main_thread!();
        unsafe { PaintNode::from_glib_full(ffi::clutter_clip_node_new()).unsafe_cast() }
    }
}

impl Default for ClipNode {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ClipNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClipNode")
    }
}
