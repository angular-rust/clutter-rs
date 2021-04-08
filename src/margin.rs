use crate::Margin;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl Margin {
}

#[doc(hidden)]
impl Uninitialized for Margin {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::new()
    }
}

// impl Default for Color {
//     fn default() -> Self {
//         unimplemented!() // TODO

//         // Self::new()
//     }
// }
