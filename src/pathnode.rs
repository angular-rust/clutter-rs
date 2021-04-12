use crate::PathNode;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl PathNode {}

#[doc(hidden)]
impl Uninitialized for PathNode {
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
