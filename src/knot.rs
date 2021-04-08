use crate::Knot;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl Knot {
}

#[doc(hidden)]
impl Uninitialized for Knot {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

// impl Default for Color {
//     fn default() -> Self {
//         unimplemented!() // TODO

//         // Self::new()
//     }
// }
