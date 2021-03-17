// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Knot(Boxed<ffi::ClutterKnot>);

    match fn {
        copy => |ptr| ffi::clutter_knot_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_knot_free(ptr),
        get_type => || ffi::clutter_knot_get_type(),
    }
}

impl Knot {
    fn equal(&self, knot_b: &Knot) -> bool {
        unsafe {
            from_glib(ffi::clutter_knot_equal(
                self.to_glib_none().0,
                knot_b.to_glib_none().0,
            ))
        }
    }
}

impl PartialEq for Knot {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Knot {}
