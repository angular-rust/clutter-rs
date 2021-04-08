use crate::Perspective;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl Perspective {
}

#[doc(hidden)]
impl Uninitialized for Perspective {
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
