#![allow(unused_imports)]
#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]
#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms,
    clippy::new_ret_no_self,
    clippy::wrong_self_convention
)]

use std::ptr;

#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod rt;

#[macro_use]
mod macros;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
mod auto;

pub mod prelude;
pub use auto::*;

pub use self::rt::set_initialized;

pub type ActorCreateChildFunc = ffi::ClutterActorCreateChildFunc;
pub type BindingActionFunc = ffi::ClutterBindingActionFunc;
// pub type ScriptConnectFunc = ffi::ClutterScriptConnectFunc;

pub fn init() {
    assert_not_initialized!();
    unsafe {
        ffi::clutter_init(ptr::null_mut(), ptr::null_mut());
        set_initialized();
    }
}

pub fn run() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::clutter_main();
        set_initialized();
    }
}

pub fn quit() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::clutter_main_quit();
    }
}
