// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct PathNode(Boxed<ffi::ClutterPathNode>);

    match fn {
        copy => |ptr| ffi::clutter_path_node_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_path_node_free(ptr),
        get_type => || ffi::clutter_path_node_get_type(),
    }
}

impl PathNode {
    fn equal(&self, node_b: &PathNode) -> bool {
        unsafe {
            from_glib(ffi::clutter_path_node_equal(
                self.to_glib_none().0,
                node_b.to_glib_none().0,
            ))
        }
    }
}

impl PartialEq for PathNode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for PathNode {}