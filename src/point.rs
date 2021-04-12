use crate::Point;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl Point {}

#[doc(hidden)]
impl Uninitialized for Point {
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
