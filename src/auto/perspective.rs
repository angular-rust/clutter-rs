// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use gobject_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Perspective(Boxed<ffi::ClutterPerspective>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ffi::clutter_perspective_get_type(), ptr as *mut _) as *mut ffi::ClutterPerspective,
        free => |ptr| gobject_sys::g_boxed_free(ffi::clutter_perspective_get_type(), ptr as *mut _),
        get_type => || ffi::clutter_perspective_get_type(),
    }
}