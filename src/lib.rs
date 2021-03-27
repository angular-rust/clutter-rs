#![allow(unused_imports)]
#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]

use ffi;

#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod rt;
#[macro_use]
mod event;

#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
mod auto;

pub mod prelude;
// pub use self::auto::functions::*;
pub use auto::*;

mod actorbox;
mod color;
mod knot;
mod margin;
mod matrix;
mod pathnode;
mod perspective;
mod point;
mod rect;
mod vertex;

mod event_button;
mod event_crossing;
mod event_key;
mod event_motion;
mod event_scroll;
mod event_touch;

// pub use gdk_sys::GdkColor as Color;

pub use self::rt::{init, run, set_initialized};

// pub use atom::Atom;
// pub use atom::NONE as ATOM_NONE;
// pub use atom::SELECTION_CLIPBOARD;
// pub use atom::SELECTION_PRIMARY;
// pub use atom::SELECTION_SECONDARY;
// pub use atom::SELECTION_TYPE_ATOM;
// pub use atom::SELECTION_TYPE_BITMAP;
// pub use atom::SELECTION_TYPE_COLORMAP;
// pub use atom::SELECTION_TYPE_DRAWABLE;
// pub use atom::SELECTION_TYPE_INTEGER;
// pub use atom::SELECTION_TYPE_PIXMAP;
// pub use atom::SELECTION_TYPE_STRING;
// pub use atom::SELECTION_TYPE_WINDOW;
// pub use atom::TARGET_BITMAP;
// pub use atom::TARGET_COLORMAP;
// pub use atom::TARGET_DRAWABLE;
// pub use atom::TARGET_PIXMAP;
// pub use atom::TARGET_STRING;
// pub use change_data::ChangeData;

pub use event::Event;
pub use event_button::ButtonEvent;
pub use event_crossing::CrossingEvent;
pub use event_key::KeyEvent;
pub use event_motion::MotionEvent;
pub use event_scroll::ScrollEvent;
pub use event_touch::TouchEvent;

pub type ActorCreateChildFunc = ffi::ClutterActorCreateChildFunc;
pub type BindingActionFunc = ffi::ClutterBindingActionFunc;
pub type ScriptConnectFunc = ffi::ClutterScriptConnectFunc;

// pub use event_configure::EventConfigure;
// pub use event_dnd::EventDND;
// pub use event_expose::EventExpose;
// pub use event_focus::EventFocus;
// pub use event_grab_broken::EventGrabBroken;
// pub use event_owner_change::EventOwnerChange;
// pub use event_pad_axis::EventPadAxis;
// pub use event_pad_button::EventPadButton;
// pub use event_pad_group_mode::EventPadGroupMode;
// pub use event_property::EventProperty;
// pub use event_proximity::EventProximity;
// pub use event_scroll::EventScroll;
// pub use event_selection::EventSelection;
// pub use event_setting::EventSetting;
// pub use event_touch::EventTouch;
// pub use event_touchpad_pinch::EventTouchpadPinch;
// pub use event_touchpad_swipe::EventTouchpadSwipe;
// pub use event_visibility::EventVisibility;
// pub use event_window_state::EventWindowState;
// pub use functions::*;
// pub use geometry::Geometry;
// pub use keymap_key::KeymapKey;
// pub use rectangle::Rectangle;
// pub use rgba::{RgbaParseError, RGBA};
// pub use time_coord::TimeCoord;
// pub use window::WindowAttr;

// #[allow(non_camel_case_types)]
// pub type key = i32;

// /// The primary button. This is typically the left mouse button, or the right button in a left-handed setup.
// pub const BUTTON_PRIMARY: u32 = gdk_sys::GDK_BUTTON_PRIMARY as u32;

// /// The middle button.
// pub const BUTTON_MIDDLE: u32 = gdk_sys::GDK_BUTTON_MIDDLE as u32;

// /// The secondary button. This is typically the right mouse button, or the left button in a left-handed setup.
// pub const BUTTON_SECONDARY: u32 = gdk_sys::GDK_BUTTON_SECONDARY as u32;

// // Used as the return value for stopping the propagation of an event handler.
// pub const EVENT_STOP: u32 = gdk_sys::GDK_EVENT_STOP as u32;

// // Used as the return value for continuing the propagation of an event handler.
// pub const EVENT_PROPAGATE: u32 = gdk_sys::GDK_EVENT_PROPAGATE as u32;
