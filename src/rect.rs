use crate::Rect;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl Rect {}

#[doc(hidden)]
impl Uninitialized for Rect {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::alloc()
    }
}

// impl Default for Rect {
//     fn default() -> Self {
//         unimplemented!() // TODO

//         // Self::new()
//     }
// }
