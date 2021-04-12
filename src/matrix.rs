use crate::Matrix;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl Matrix {}

#[doc(hidden)]
impl Uninitialized for Matrix {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::alloc().unwrap()
    }
}

// impl Default for Color {
//     fn default() -> Self {
//         unimplemented!() // TODO

//         // Self::new()
//     }
// }
