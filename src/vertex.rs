use crate::Vertex;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl Vertex {
}

#[doc(hidden)]
impl Uninitialized for Vertex {
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
