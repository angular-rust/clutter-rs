use super::Color;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl Color {
}

#[doc(hidden)]
impl Uninitialized for Color {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::alloc()
    }
}

// impl Default for Color {
//     fn default() -> Self {
//         unimplemented!() // TODO

//         // Self::new()
//     }
// }
