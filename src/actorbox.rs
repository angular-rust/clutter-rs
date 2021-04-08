use super::ActorBox;
use glib::{translate::*, GString};
use std::{fmt, mem};

impl ActorBox {}

#[doc(hidden)]
impl Uninitialized for ActorBox {
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
