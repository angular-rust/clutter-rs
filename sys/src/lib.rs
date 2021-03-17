#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]

extern crate atk_sys as atk;
extern crate cairo_sys as cairo;
extern crate cogl_sys as cogl;
extern crate gio_sys as gio;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
// extern crate json_sys as json;
extern crate libc;
extern crate pango_sys as pango;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type ClutterActorAlign = c_int;
pub const CLUTTER_ACTOR_ALIGN_FILL: ClutterActorAlign = 0;
pub const CLUTTER_ACTOR_ALIGN_START: ClutterActorAlign = 1;
pub const CLUTTER_ACTOR_ALIGN_CENTER: ClutterActorAlign = 2;
pub const CLUTTER_ACTOR_ALIGN_END: ClutterActorAlign = 3;

pub type ClutterAlignAxis = c_int;
pub const CLUTTER_ALIGN_X_AXIS: ClutterAlignAxis = 0;
pub const CLUTTER_ALIGN_Y_AXIS: ClutterAlignAxis = 1;
pub const CLUTTER_ALIGN_BOTH: ClutterAlignAxis = 2;

pub type ClutterAnimationMode = c_int;
pub const CLUTTER_CUSTOM_MODE: ClutterAnimationMode = 0;
pub const CLUTTER_LINEAR: ClutterAnimationMode = 1;
pub const CLUTTER_EASE_IN_QUAD: ClutterAnimationMode = 2;
pub const CLUTTER_EASE_OUT_QUAD: ClutterAnimationMode = 3;
pub const CLUTTER_EASE_IN_OUT_QUAD: ClutterAnimationMode = 4;
pub const CLUTTER_EASE_IN_CUBIC: ClutterAnimationMode = 5;
pub const CLUTTER_EASE_OUT_CUBIC: ClutterAnimationMode = 6;
pub const CLUTTER_EASE_IN_OUT_CUBIC: ClutterAnimationMode = 7;
pub const CLUTTER_EASE_IN_QUART: ClutterAnimationMode = 8;
pub const CLUTTER_EASE_OUT_QUART: ClutterAnimationMode = 9;
pub const CLUTTER_EASE_IN_OUT_QUART: ClutterAnimationMode = 10;
pub const CLUTTER_EASE_IN_QUINT: ClutterAnimationMode = 11;
pub const CLUTTER_EASE_OUT_QUINT: ClutterAnimationMode = 12;
pub const CLUTTER_EASE_IN_OUT_QUINT: ClutterAnimationMode = 13;
pub const CLUTTER_EASE_IN_SINE: ClutterAnimationMode = 14;
pub const CLUTTER_EASE_OUT_SINE: ClutterAnimationMode = 15;
pub const CLUTTER_EASE_IN_OUT_SINE: ClutterAnimationMode = 16;
pub const CLUTTER_EASE_IN_EXPO: ClutterAnimationMode = 17;
pub const CLUTTER_EASE_OUT_EXPO: ClutterAnimationMode = 18;
pub const CLUTTER_EASE_IN_OUT_EXPO: ClutterAnimationMode = 19;
pub const CLUTTER_EASE_IN_CIRC: ClutterAnimationMode = 20;
pub const CLUTTER_EASE_OUT_CIRC: ClutterAnimationMode = 21;
pub const CLUTTER_EASE_IN_OUT_CIRC: ClutterAnimationMode = 22;
pub const CLUTTER_EASE_IN_ELASTIC: ClutterAnimationMode = 23;
pub const CLUTTER_EASE_OUT_ELASTIC: ClutterAnimationMode = 24;
pub const CLUTTER_EASE_IN_OUT_ELASTIC: ClutterAnimationMode = 25;
pub const CLUTTER_EASE_IN_BACK: ClutterAnimationMode = 26;
pub const CLUTTER_EASE_OUT_BACK: ClutterAnimationMode = 27;
pub const CLUTTER_EASE_IN_OUT_BACK: ClutterAnimationMode = 28;
pub const CLUTTER_EASE_IN_BOUNCE: ClutterAnimationMode = 29;
pub const CLUTTER_EASE_OUT_BOUNCE: ClutterAnimationMode = 30;
pub const CLUTTER_EASE_IN_OUT_BOUNCE: ClutterAnimationMode = 31;
pub const CLUTTER_STEPS: ClutterAnimationMode = 32;
pub const CLUTTER_STEP_START: ClutterAnimationMode = 33;
pub const CLUTTER_STEP_END: ClutterAnimationMode = 34;
pub const CLUTTER_CUBIC_BEZIER: ClutterAnimationMode = 35;
pub const CLUTTER_EASE: ClutterAnimationMode = 36;
pub const CLUTTER_EASE_IN: ClutterAnimationMode = 37;
pub const CLUTTER_EASE_OUT: ClutterAnimationMode = 38;
pub const CLUTTER_EASE_IN_OUT: ClutterAnimationMode = 39;
pub const CLUTTER_ANIMATION_LAST: ClutterAnimationMode = 40;

pub type ClutterBinAlignment = c_int;
pub const CLUTTER_BIN_ALIGNMENT_FIXED: ClutterBinAlignment = 0;
pub const CLUTTER_BIN_ALIGNMENT_FILL: ClutterBinAlignment = 1;
pub const CLUTTER_BIN_ALIGNMENT_START: ClutterBinAlignment = 2;
pub const CLUTTER_BIN_ALIGNMENT_END: ClutterBinAlignment = 3;
pub const CLUTTER_BIN_ALIGNMENT_CENTER: ClutterBinAlignment = 4;

pub type ClutterBindCoordinate = c_int;
pub const CLUTTER_BIND_X: ClutterBindCoordinate = 0;
pub const CLUTTER_BIND_Y: ClutterBindCoordinate = 1;
pub const CLUTTER_BIND_WIDTH: ClutterBindCoordinate = 2;
pub const CLUTTER_BIND_HEIGHT: ClutterBindCoordinate = 3;
pub const CLUTTER_BIND_POSITION: ClutterBindCoordinate = 4;
pub const CLUTTER_BIND_SIZE: ClutterBindCoordinate = 5;
pub const CLUTTER_BIND_ALL: ClutterBindCoordinate = 6;

pub type ClutterBoxAlignment = c_int;
pub const CLUTTER_BOX_ALIGNMENT_START: ClutterBoxAlignment = 0;
pub const CLUTTER_BOX_ALIGNMENT_END: ClutterBoxAlignment = 1;
pub const CLUTTER_BOX_ALIGNMENT_CENTER: ClutterBoxAlignment = 2;

pub type ClutterContentGravity = c_int;
pub const CLUTTER_CONTENT_GRAVITY_TOP_LEFT: ClutterContentGravity = 0;
pub const CLUTTER_CONTENT_GRAVITY_TOP: ClutterContentGravity = 1;
pub const CLUTTER_CONTENT_GRAVITY_TOP_RIGHT: ClutterContentGravity = 2;
pub const CLUTTER_CONTENT_GRAVITY_LEFT: ClutterContentGravity = 3;
pub const CLUTTER_CONTENT_GRAVITY_CENTER: ClutterContentGravity = 4;
pub const CLUTTER_CONTENT_GRAVITY_RIGHT: ClutterContentGravity = 5;
pub const CLUTTER_CONTENT_GRAVITY_BOTTOM_LEFT: ClutterContentGravity = 6;
pub const CLUTTER_CONTENT_GRAVITY_BOTTOM: ClutterContentGravity = 7;
pub const CLUTTER_CONTENT_GRAVITY_BOTTOM_RIGHT: ClutterContentGravity = 8;
pub const CLUTTER_CONTENT_GRAVITY_RESIZE_FILL: ClutterContentGravity = 9;
pub const CLUTTER_CONTENT_GRAVITY_RESIZE_ASPECT: ClutterContentGravity = 10;

pub type ClutterDragAxis = c_int;
pub const CLUTTER_DRAG_AXIS_NONE: ClutterDragAxis = 0;
pub const CLUTTER_DRAG_X_AXIS: ClutterDragAxis = 1;
pub const CLUTTER_DRAG_Y_AXIS: ClutterDragAxis = 2;

pub type ClutterEventType = c_int;
pub const CLUTTER_NOTHING: ClutterEventType = 0;
pub const CLUTTER_KEY_PRESS: ClutterEventType = 1;
pub const CLUTTER_KEY_RELEASE: ClutterEventType = 2;
pub const CLUTTER_MOTION: ClutterEventType = 3;
pub const CLUTTER_ENTER: ClutterEventType = 4;
pub const CLUTTER_LEAVE: ClutterEventType = 5;
pub const CLUTTER_BUTTON_PRESS: ClutterEventType = 6;
pub const CLUTTER_BUTTON_RELEASE: ClutterEventType = 7;
pub const CLUTTER_SCROLL: ClutterEventType = 8;
pub const CLUTTER_STAGE_STATE: ClutterEventType = 9;
pub const CLUTTER_DESTROY_NOTIFY: ClutterEventType = 10;
pub const CLUTTER_CLIENT_MESSAGE: ClutterEventType = 11;
pub const CLUTTER_DELETE: ClutterEventType = 12;
pub const CLUTTER_TOUCH_BEGIN: ClutterEventType = 13;
pub const CLUTTER_TOUCH_UPDATE: ClutterEventType = 14;
pub const CLUTTER_TOUCH_END: ClutterEventType = 15;
pub const CLUTTER_TOUCH_CANCEL: ClutterEventType = 16;
pub const CLUTTER_TOUCHPAD_PINCH: ClutterEventType = 17;
pub const CLUTTER_TOUCHPAD_SWIPE: ClutterEventType = 18;
pub const CLUTTER_EVENT_LAST: ClutterEventType = 19;

pub type ClutterFlowOrientation = c_int;
pub const CLUTTER_FLOW_HORIZONTAL: ClutterFlowOrientation = 0;
pub const CLUTTER_FLOW_VERTICAL: ClutterFlowOrientation = 1;

pub type ClutterGestureTriggerEdge = c_int;
pub const CLUTTER_GESTURE_TRIGGER_EDGE_NONE: ClutterGestureTriggerEdge = 0;
pub const CLUTTER_GESTURE_TRIGGER_EDGE_AFTER: ClutterGestureTriggerEdge = 1;
pub const CLUTTER_GESTURE_TRIGGER_EDGE_BEFORE: ClutterGestureTriggerEdge = 2;

pub type ClutterGravity = c_int;
pub const CLUTTER_GRAVITY_NONE: ClutterGravity = 0;
pub const CLUTTER_GRAVITY_NORTH: ClutterGravity = 1;
pub const CLUTTER_GRAVITY_NORTH_EAST: ClutterGravity = 2;
pub const CLUTTER_GRAVITY_EAST: ClutterGravity = 3;
pub const CLUTTER_GRAVITY_SOUTH_EAST: ClutterGravity = 4;
pub const CLUTTER_GRAVITY_SOUTH: ClutterGravity = 5;
pub const CLUTTER_GRAVITY_SOUTH_WEST: ClutterGravity = 6;
pub const CLUTTER_GRAVITY_WEST: ClutterGravity = 7;
pub const CLUTTER_GRAVITY_NORTH_WEST: ClutterGravity = 8;
pub const CLUTTER_GRAVITY_CENTER: ClutterGravity = 9;

pub type ClutterGridPosition = c_int;
pub const CLUTTER_GRID_POSITION_LEFT: ClutterGridPosition = 0;
pub const CLUTTER_GRID_POSITION_RIGHT: ClutterGridPosition = 1;
pub const CLUTTER_GRID_POSITION_TOP: ClutterGridPosition = 2;
pub const CLUTTER_GRID_POSITION_BOTTOM: ClutterGridPosition = 3;

pub type ClutterImageError = c_int;
pub const CLUTTER_IMAGE_ERROR_INVALID_DATA: ClutterImageError = 0;

pub type ClutterInitError = c_int;
pub const CLUTTER_INIT_SUCCESS: ClutterInitError = 1;
pub const CLUTTER_INIT_ERROR_UNKNOWN: ClutterInitError = 0;
pub const CLUTTER_INIT_ERROR_THREADS: ClutterInitError = -1;
pub const CLUTTER_INIT_ERROR_BACKEND: ClutterInitError = -2;
pub const CLUTTER_INIT_ERROR_INTERNAL: ClutterInitError = -3;

pub type ClutterInputAxis = c_int;
pub const CLUTTER_INPUT_AXIS_IGNORE: ClutterInputAxis = 0;
pub const CLUTTER_INPUT_AXIS_X: ClutterInputAxis = 1;
pub const CLUTTER_INPUT_AXIS_Y: ClutterInputAxis = 2;
pub const CLUTTER_INPUT_AXIS_PRESSURE: ClutterInputAxis = 3;
pub const CLUTTER_INPUT_AXIS_XTILT: ClutterInputAxis = 4;
pub const CLUTTER_INPUT_AXIS_YTILT: ClutterInputAxis = 5;
pub const CLUTTER_INPUT_AXIS_WHEEL: ClutterInputAxis = 6;
pub const CLUTTER_INPUT_AXIS_DISTANCE: ClutterInputAxis = 7;
pub const CLUTTER_INPUT_AXIS_LAST: ClutterInputAxis = 8;

pub type ClutterInputDeviceType = c_int;
pub const CLUTTER_POINTER_DEVICE: ClutterInputDeviceType = 0;
pub const CLUTTER_KEYBOARD_DEVICE: ClutterInputDeviceType = 1;
pub const CLUTTER_EXTENSION_DEVICE: ClutterInputDeviceType = 2;
pub const CLUTTER_JOYSTICK_DEVICE: ClutterInputDeviceType = 3;
pub const CLUTTER_TABLET_DEVICE: ClutterInputDeviceType = 4;
pub const CLUTTER_TOUCHPAD_DEVICE: ClutterInputDeviceType = 5;
pub const CLUTTER_TOUCHSCREEN_DEVICE: ClutterInputDeviceType = 6;
pub const CLUTTER_PEN_DEVICE: ClutterInputDeviceType = 7;
pub const CLUTTER_ERASER_DEVICE: ClutterInputDeviceType = 8;
pub const CLUTTER_CURSOR_DEVICE: ClutterInputDeviceType = 9;
pub const CLUTTER_N_DEVICE_TYPES: ClutterInputDeviceType = 10;

pub type ClutterInputMode = c_int;
pub const CLUTTER_INPUT_MODE_MASTER: ClutterInputMode = 0;
pub const CLUTTER_INPUT_MODE_SLAVE: ClutterInputMode = 1;
pub const CLUTTER_INPUT_MODE_FLOATING: ClutterInputMode = 2;

pub type ClutterInterpolation = c_int;
pub const CLUTTER_INTERPOLATION_LINEAR: ClutterInterpolation = 0;
pub const CLUTTER_INTERPOLATION_CUBIC: ClutterInterpolation = 1;

pub type ClutterLongPressState = c_int;
pub const CLUTTER_LONG_PRESS_QUERY: ClutterLongPressState = 0;
pub const CLUTTER_LONG_PRESS_ACTIVATE: ClutterLongPressState = 1;
pub const CLUTTER_LONG_PRESS_CANCEL: ClutterLongPressState = 2;

pub type ClutterOrientation = c_int;
pub const CLUTTER_ORIENTATION_HORIZONTAL: ClutterOrientation = 0;
pub const CLUTTER_ORIENTATION_VERTICAL: ClutterOrientation = 1;

pub type ClutterPanAxis = c_int;
pub const CLUTTER_PAN_AXIS_NONE: ClutterPanAxis = 0;
pub const CLUTTER_PAN_X_AXIS: ClutterPanAxis = 1;
pub const CLUTTER_PAN_Y_AXIS: ClutterPanAxis = 2;
pub const CLUTTER_PAN_AXIS_AUTO: ClutterPanAxis = 3;

pub type ClutterPathNodeType = c_int;
pub const CLUTTER_PATH_MOVE_TO: ClutterPathNodeType = 0;
pub const CLUTTER_PATH_LINE_TO: ClutterPathNodeType = 1;
pub const CLUTTER_PATH_CURVE_TO: ClutterPathNodeType = 2;
pub const CLUTTER_PATH_CLOSE: ClutterPathNodeType = 3;
pub const CLUTTER_PATH_REL_MOVE_TO: ClutterPathNodeType = 32;
pub const CLUTTER_PATH_REL_LINE_TO: ClutterPathNodeType = 33;
pub const CLUTTER_PATH_REL_CURVE_TO: ClutterPathNodeType = 34;

pub type ClutterPickMode = c_int;
pub const CLUTTER_PICK_NONE: ClutterPickMode = 0;
pub const CLUTTER_PICK_REACTIVE: ClutterPickMode = 1;
pub const CLUTTER_PICK_ALL: ClutterPickMode = 2;

pub type ClutterRequestMode = c_int;
pub const CLUTTER_REQUEST_HEIGHT_FOR_WIDTH: ClutterRequestMode = 0;
pub const CLUTTER_REQUEST_WIDTH_FOR_HEIGHT: ClutterRequestMode = 1;
pub const CLUTTER_REQUEST_CONTENT_SIZE: ClutterRequestMode = 2;

pub type ClutterRotateAxis = c_int;
pub const CLUTTER_X_AXIS: ClutterRotateAxis = 0;
pub const CLUTTER_Y_AXIS: ClutterRotateAxis = 1;
pub const CLUTTER_Z_AXIS: ClutterRotateAxis = 2;

pub type ClutterRotateDirection = c_int;
pub const CLUTTER_ROTATE_CW: ClutterRotateDirection = 0;
pub const CLUTTER_ROTATE_CCW: ClutterRotateDirection = 1;

pub type ClutterScalingFilter = c_int;
pub const CLUTTER_SCALING_FILTER_LINEAR: ClutterScalingFilter = 0;
pub const CLUTTER_SCALING_FILTER_NEAREST: ClutterScalingFilter = 1;
pub const CLUTTER_SCALING_FILTER_TRILINEAR: ClutterScalingFilter = 2;

pub type ClutterScriptError = c_int;
pub const CLUTTER_SCRIPT_ERROR_INVALID_TYPE_FUNCTION: ClutterScriptError = 0;
pub const CLUTTER_SCRIPT_ERROR_INVALID_PROPERTY: ClutterScriptError = 1;
pub const CLUTTER_SCRIPT_ERROR_INVALID_VALUE: ClutterScriptError = 2;

pub type ClutterScrollDirection = c_int;
pub const CLUTTER_SCROLL_UP: ClutterScrollDirection = 0;
pub const CLUTTER_SCROLL_DOWN: ClutterScrollDirection = 1;
pub const CLUTTER_SCROLL_LEFT: ClutterScrollDirection = 2;
pub const CLUTTER_SCROLL_RIGHT: ClutterScrollDirection = 3;
pub const CLUTTER_SCROLL_SMOOTH: ClutterScrollDirection = 4;

pub type ClutterScrollSource = c_int;
pub const CLUTTER_SCROLL_SOURCE_UNKNOWN: ClutterScrollSource = 0;
pub const CLUTTER_SCROLL_SOURCE_WHEEL: ClutterScrollSource = 1;
pub const CLUTTER_SCROLL_SOURCE_FINGER: ClutterScrollSource = 2;
pub const CLUTTER_SCROLL_SOURCE_CONTINUOUS: ClutterScrollSource = 3;

pub type ClutterShaderError = c_int;
pub const CLUTTER_SHADER_ERROR_NO_ASM: ClutterShaderError = 0;
pub const CLUTTER_SHADER_ERROR_NO_GLSL: ClutterShaderError = 1;
pub const CLUTTER_SHADER_ERROR_COMPILE: ClutterShaderError = 2;

pub type ClutterShaderType = c_int;
pub const CLUTTER_VERTEX_SHADER: ClutterShaderType = 0;
pub const CLUTTER_FRAGMENT_SHADER: ClutterShaderType = 1;

pub type ClutterSnapEdge = c_int;
pub const CLUTTER_SNAP_EDGE_TOP: ClutterSnapEdge = 0;
pub const CLUTTER_SNAP_EDGE_RIGHT: ClutterSnapEdge = 1;
pub const CLUTTER_SNAP_EDGE_BOTTOM: ClutterSnapEdge = 2;
pub const CLUTTER_SNAP_EDGE_LEFT: ClutterSnapEdge = 3;

pub type ClutterStaticColor = c_int;
pub const CLUTTER_COLOR_WHITE: ClutterStaticColor = 0;
pub const CLUTTER_COLOR_BLACK: ClutterStaticColor = 1;
pub const CLUTTER_COLOR_RED: ClutterStaticColor = 2;
pub const CLUTTER_COLOR_DARK_RED: ClutterStaticColor = 3;
pub const CLUTTER_COLOR_GREEN: ClutterStaticColor = 4;
pub const CLUTTER_COLOR_DARK_GREEN: ClutterStaticColor = 5;
pub const CLUTTER_COLOR_BLUE: ClutterStaticColor = 6;
pub const CLUTTER_COLOR_DARK_BLUE: ClutterStaticColor = 7;
pub const CLUTTER_COLOR_CYAN: ClutterStaticColor = 8;
pub const CLUTTER_COLOR_DARK_CYAN: ClutterStaticColor = 9;
pub const CLUTTER_COLOR_MAGENTA: ClutterStaticColor = 10;
pub const CLUTTER_COLOR_DARK_MAGENTA: ClutterStaticColor = 11;
pub const CLUTTER_COLOR_YELLOW: ClutterStaticColor = 12;
pub const CLUTTER_COLOR_DARK_YELLOW: ClutterStaticColor = 13;
pub const CLUTTER_COLOR_GRAY: ClutterStaticColor = 14;
pub const CLUTTER_COLOR_DARK_GRAY: ClutterStaticColor = 15;
pub const CLUTTER_COLOR_LIGHT_GRAY: ClutterStaticColor = 16;
pub const CLUTTER_COLOR_BUTTER: ClutterStaticColor = 17;
pub const CLUTTER_COLOR_BUTTER_LIGHT: ClutterStaticColor = 18;
pub const CLUTTER_COLOR_BUTTER_DARK: ClutterStaticColor = 19;
pub const CLUTTER_COLOR_ORANGE: ClutterStaticColor = 20;
pub const CLUTTER_COLOR_ORANGE_LIGHT: ClutterStaticColor = 21;
pub const CLUTTER_COLOR_ORANGE_DARK: ClutterStaticColor = 22;
pub const CLUTTER_COLOR_CHOCOLATE: ClutterStaticColor = 23;
pub const CLUTTER_COLOR_CHOCOLATE_LIGHT: ClutterStaticColor = 24;
pub const CLUTTER_COLOR_CHOCOLATE_DARK: ClutterStaticColor = 25;
pub const CLUTTER_COLOR_CHAMELEON: ClutterStaticColor = 26;
pub const CLUTTER_COLOR_CHAMELEON_LIGHT: ClutterStaticColor = 27;
pub const CLUTTER_COLOR_CHAMELEON_DARK: ClutterStaticColor = 28;
pub const CLUTTER_COLOR_SKY_BLUE: ClutterStaticColor = 29;
pub const CLUTTER_COLOR_SKY_BLUE_LIGHT: ClutterStaticColor = 30;
pub const CLUTTER_COLOR_SKY_BLUE_DARK: ClutterStaticColor = 31;
pub const CLUTTER_COLOR_PLUM: ClutterStaticColor = 32;
pub const CLUTTER_COLOR_PLUM_LIGHT: ClutterStaticColor = 33;
pub const CLUTTER_COLOR_PLUM_DARK: ClutterStaticColor = 34;
pub const CLUTTER_COLOR_SCARLET_RED: ClutterStaticColor = 35;
pub const CLUTTER_COLOR_SCARLET_RED_LIGHT: ClutterStaticColor = 36;
pub const CLUTTER_COLOR_SCARLET_RED_DARK: ClutterStaticColor = 37;
pub const CLUTTER_COLOR_ALUMINIUM_1: ClutterStaticColor = 38;
pub const CLUTTER_COLOR_ALUMINIUM_2: ClutterStaticColor = 39;
pub const CLUTTER_COLOR_ALUMINIUM_3: ClutterStaticColor = 40;
pub const CLUTTER_COLOR_ALUMINIUM_4: ClutterStaticColor = 41;
pub const CLUTTER_COLOR_ALUMINIUM_5: ClutterStaticColor = 42;
pub const CLUTTER_COLOR_ALUMINIUM_6: ClutterStaticColor = 43;
pub const CLUTTER_COLOR_TRANSPARENT: ClutterStaticColor = 44;

pub type ClutterStepMode = c_int;
pub const CLUTTER_STEP_MODE_START: ClutterStepMode = 0;
pub const CLUTTER_STEP_MODE_END: ClutterStepMode = 1;

pub type ClutterTableAlignment = c_int;
pub const CLUTTER_TABLE_ALIGNMENT_START: ClutterTableAlignment = 0;
pub const CLUTTER_TABLE_ALIGNMENT_CENTER: ClutterTableAlignment = 1;
pub const CLUTTER_TABLE_ALIGNMENT_END: ClutterTableAlignment = 2;

pub type ClutterTextDirection = c_int;
pub const CLUTTER_TEXT_DIRECTION_DEFAULT: ClutterTextDirection = 0;
pub const CLUTTER_TEXT_DIRECTION_LTR: ClutterTextDirection = 1;
pub const CLUTTER_TEXT_DIRECTION_RTL: ClutterTextDirection = 2;

pub type ClutterTextureError = c_int;
pub const CLUTTER_TEXTURE_ERROR_OUT_OF_MEMORY: ClutterTextureError = 0;
pub const CLUTTER_TEXTURE_ERROR_NO_YUV: ClutterTextureError = 1;
pub const CLUTTER_TEXTURE_ERROR_BAD_FORMAT: ClutterTextureError = 2;

pub type ClutterTextureQuality = c_int;
pub const CLUTTER_TEXTURE_QUALITY_LOW: ClutterTextureQuality = 0;
pub const CLUTTER_TEXTURE_QUALITY_MEDIUM: ClutterTextureQuality = 1;
pub const CLUTTER_TEXTURE_QUALITY_HIGH: ClutterTextureQuality = 2;

pub type ClutterTimelineDirection = c_int;
pub const CLUTTER_TIMELINE_FORWARD: ClutterTimelineDirection = 0;
pub const CLUTTER_TIMELINE_BACKWARD: ClutterTimelineDirection = 1;

pub type ClutterTouchpadGesturePhase = c_int;
pub const CLUTTER_TOUCHPAD_GESTURE_PHASE_BEGIN: ClutterTouchpadGesturePhase = 0;
pub const CLUTTER_TOUCHPAD_GESTURE_PHASE_UPDATE: ClutterTouchpadGesturePhase = 1;
pub const CLUTTER_TOUCHPAD_GESTURE_PHASE_END: ClutterTouchpadGesturePhase = 2;
pub const CLUTTER_TOUCHPAD_GESTURE_PHASE_CANCEL: ClutterTouchpadGesturePhase = 3;

pub type ClutterUnitType = c_int;
pub const CLUTTER_UNIT_PIXEL: ClutterUnitType = 0;
pub const CLUTTER_UNIT_EM: ClutterUnitType = 1;
pub const CLUTTER_UNIT_MM: ClutterUnitType = 2;
pub const CLUTTER_UNIT_POINT: ClutterUnitType = 3;
pub const CLUTTER_UNIT_CM: ClutterUnitType = 4;

pub type ClutterZoomAxis = c_int;
pub const CLUTTER_ZOOM_X_AXIS: ClutterZoomAxis = 0;
pub const CLUTTER_ZOOM_Y_AXIS: ClutterZoomAxis = 1;
pub const CLUTTER_ZOOM_BOTH: ClutterZoomAxis = 2;

// Constants
pub const CLUTTER_0: c_int = 48;
pub const CLUTTER_1: c_int = 49;
pub const CLUTTER_2: c_int = 50;
pub const CLUTTER_3: c_int = 51;
pub const CLUTTER_3270_AltCursor: c_int = 64784;
pub const CLUTTER_3270_Attn: c_int = 64782;
pub const CLUTTER_3270_BackTab: c_int = 64773;
pub const CLUTTER_3270_ChangeScreen: c_int = 64793;
pub const CLUTTER_3270_Copy: c_int = 64789;
pub const CLUTTER_3270_CursorBlink: c_int = 64783;
pub const CLUTTER_3270_CursorSelect: c_int = 64796;
pub const CLUTTER_3270_DeleteWord: c_int = 64794;
pub const CLUTTER_3270_Duplicate: c_int = 64769;
pub const CLUTTER_3270_Enter: c_int = 64798;
pub const CLUTTER_3270_EraseEOF: c_int = 64774;
pub const CLUTTER_3270_EraseInput: c_int = 64775;
pub const CLUTTER_3270_ExSelect: c_int = 64795;
pub const CLUTTER_3270_FieldMark: c_int = 64770;
pub const CLUTTER_3270_Ident: c_int = 64787;
pub const CLUTTER_3270_Jump: c_int = 64786;
pub const CLUTTER_3270_KeyClick: c_int = 64785;
pub const CLUTTER_3270_Left2: c_int = 64772;
pub const CLUTTER_3270_PA1: c_int = 64778;
pub const CLUTTER_3270_PA2: c_int = 64779;
pub const CLUTTER_3270_PA3: c_int = 64780;
pub const CLUTTER_3270_Play: c_int = 64790;
pub const CLUTTER_3270_PrintScreen: c_int = 64797;
pub const CLUTTER_3270_Quit: c_int = 64777;
pub const CLUTTER_3270_Record: c_int = 64792;
pub const CLUTTER_3270_Reset: c_int = 64776;
pub const CLUTTER_3270_Right2: c_int = 64771;
pub const CLUTTER_3270_Rule: c_int = 64788;
pub const CLUTTER_3270_Setup: c_int = 64791;
pub const CLUTTER_3270_Test: c_int = 64781;
pub const CLUTTER_4: c_int = 52;
pub const CLUTTER_5: c_int = 53;
pub const CLUTTER_6: c_int = 54;
pub const CLUTTER_7: c_int = 55;
pub const CLUTTER_8: c_int = 56;
pub const CLUTTER_9: c_int = 57;
pub const CLUTTER_A: c_int = 65;
pub const CLUTTER_AE: c_int = 198;
pub const CLUTTER_Aacute: c_int = 193;
pub const CLUTTER_Abelowdot: c_int = 16785056;
pub const CLUTTER_Abreve: c_int = 451;
pub const CLUTTER_Abreveacute: c_int = 16785070;
pub const CLUTTER_Abrevebelowdot: c_int = 16785078;
pub const CLUTTER_Abrevegrave: c_int = 16785072;
pub const CLUTTER_Abrevehook: c_int = 16785074;
pub const CLUTTER_Abrevetilde: c_int = 16785076;
pub const CLUTTER_AccessX_Enable: c_int = 65136;
pub const CLUTTER_AccessX_Feedback_Enable: c_int = 65137;
pub const CLUTTER_Acircumflex: c_int = 194;
pub const CLUTTER_Acircumflexacute: c_int = 16785060;
pub const CLUTTER_Acircumflexbelowdot: c_int = 16785068;
pub const CLUTTER_Acircumflexgrave: c_int = 16785062;
pub const CLUTTER_Acircumflexhook: c_int = 16785064;
pub const CLUTTER_Acircumflextilde: c_int = 16785066;
pub const CLUTTER_AddFavorite: c_int = 269025081;
pub const CLUTTER_Adiaeresis: c_int = 196;
pub const CLUTTER_Agrave: c_int = 192;
pub const CLUTTER_Ahook: c_int = 16785058;
pub const CLUTTER_Alt_L: c_int = 65513;
pub const CLUTTER_Alt_R: c_int = 65514;
pub const CLUTTER_Amacron: c_int = 960;
pub const CLUTTER_Aogonek: c_int = 417;
pub const CLUTTER_ApplicationLeft: c_int = 269025104;
pub const CLUTTER_ApplicationRight: c_int = 269025105;
pub const CLUTTER_Arabic_0: c_int = 16778848;
pub const CLUTTER_Arabic_1: c_int = 16778849;
pub const CLUTTER_Arabic_2: c_int = 16778850;
pub const CLUTTER_Arabic_3: c_int = 16778851;
pub const CLUTTER_Arabic_4: c_int = 16778852;
pub const CLUTTER_Arabic_5: c_int = 16778853;
pub const CLUTTER_Arabic_6: c_int = 16778854;
pub const CLUTTER_Arabic_7: c_int = 16778855;
pub const CLUTTER_Arabic_8: c_int = 16778856;
pub const CLUTTER_Arabic_9: c_int = 16778857;
pub const CLUTTER_Arabic_ain: c_int = 1497;
pub const CLUTTER_Arabic_alef: c_int = 1479;
pub const CLUTTER_Arabic_alefmaksura: c_int = 1513;
pub const CLUTTER_Arabic_beh: c_int = 1480;
pub const CLUTTER_Arabic_comma: c_int = 1452;
pub const CLUTTER_Arabic_dad: c_int = 1494;
pub const CLUTTER_Arabic_dal: c_int = 1487;
pub const CLUTTER_Arabic_damma: c_int = 1519;
pub const CLUTTER_Arabic_dammatan: c_int = 1516;
pub const CLUTTER_Arabic_ddal: c_int = 16778888;
pub const CLUTTER_Arabic_farsi_yeh: c_int = 16778956;
pub const CLUTTER_Arabic_fatha: c_int = 1518;
pub const CLUTTER_Arabic_fathatan: c_int = 1515;
pub const CLUTTER_Arabic_feh: c_int = 1505;
pub const CLUTTER_Arabic_fullstop: c_int = 16778964;
pub const CLUTTER_Arabic_gaf: c_int = 16778927;
pub const CLUTTER_Arabic_ghain: c_int = 1498;
pub const CLUTTER_Arabic_ha: c_int = 1511;
pub const CLUTTER_Arabic_hah: c_int = 1485;
pub const CLUTTER_Arabic_hamza: c_int = 1473;
pub const CLUTTER_Arabic_hamza_above: c_int = 16778836;
pub const CLUTTER_Arabic_hamza_below: c_int = 16778837;
pub const CLUTTER_Arabic_hamzaonalef: c_int = 1475;
pub const CLUTTER_Arabic_hamzaonwaw: c_int = 1476;
pub const CLUTTER_Arabic_hamzaonyeh: c_int = 1478;
pub const CLUTTER_Arabic_hamzaunderalef: c_int = 1477;
pub const CLUTTER_Arabic_heh: c_int = 1511;
pub const CLUTTER_Arabic_heh_doachashmee: c_int = 16778942;
pub const CLUTTER_Arabic_heh_goal: c_int = 16778945;
pub const CLUTTER_Arabic_jeem: c_int = 1484;
pub const CLUTTER_Arabic_jeh: c_int = 16778904;
pub const CLUTTER_Arabic_kaf: c_int = 1507;
pub const CLUTTER_Arabic_kasra: c_int = 1520;
pub const CLUTTER_Arabic_kasratan: c_int = 1517;
pub const CLUTTER_Arabic_keheh: c_int = 16778921;
pub const CLUTTER_Arabic_khah: c_int = 1486;
pub const CLUTTER_Arabic_lam: c_int = 1508;
pub const CLUTTER_Arabic_madda_above: c_int = 16778835;
pub const CLUTTER_Arabic_maddaonalef: c_int = 1474;
pub const CLUTTER_Arabic_meem: c_int = 1509;
pub const CLUTTER_Arabic_noon: c_int = 1510;
pub const CLUTTER_Arabic_noon_ghunna: c_int = 16778938;
pub const CLUTTER_Arabic_peh: c_int = 16778878;
pub const CLUTTER_Arabic_percent: c_int = 16778858;
pub const CLUTTER_Arabic_qaf: c_int = 1506;
pub const CLUTTER_Arabic_question_mark: c_int = 1471;
pub const CLUTTER_Arabic_ra: c_int = 1489;
pub const CLUTTER_Arabic_rreh: c_int = 16778897;
pub const CLUTTER_Arabic_sad: c_int = 1493;
pub const CLUTTER_Arabic_seen: c_int = 1491;
pub const CLUTTER_Arabic_semicolon: c_int = 1467;
pub const CLUTTER_Arabic_shadda: c_int = 1521;
pub const CLUTTER_Arabic_sheen: c_int = 1492;
pub const CLUTTER_Arabic_sukun: c_int = 1522;
pub const CLUTTER_Arabic_superscript_alef: c_int = 16778864;
pub const CLUTTER_Arabic_switch: c_int = 65406;
pub const CLUTTER_Arabic_tah: c_int = 1495;
pub const CLUTTER_Arabic_tatweel: c_int = 1504;
pub const CLUTTER_Arabic_tcheh: c_int = 16778886;
pub const CLUTTER_Arabic_teh: c_int = 1482;
pub const CLUTTER_Arabic_tehmarbuta: c_int = 1481;
pub const CLUTTER_Arabic_thal: c_int = 1488;
pub const CLUTTER_Arabic_theh: c_int = 1483;
pub const CLUTTER_Arabic_tteh: c_int = 16778873;
pub const CLUTTER_Arabic_veh: c_int = 16778916;
pub const CLUTTER_Arabic_waw: c_int = 1512;
pub const CLUTTER_Arabic_yeh: c_int = 1514;
pub const CLUTTER_Arabic_yeh_baree: c_int = 16778962;
pub const CLUTTER_Arabic_zah: c_int = 1496;
pub const CLUTTER_Arabic_zain: c_int = 1490;
pub const CLUTTER_Aring: c_int = 197;
pub const CLUTTER_Armenian_AT: c_int = 16778552;
pub const CLUTTER_Armenian_AYB: c_int = 16778545;
pub const CLUTTER_Armenian_BEN: c_int = 16778546;
pub const CLUTTER_Armenian_CHA: c_int = 16778569;
pub const CLUTTER_Armenian_DA: c_int = 16778548;
pub const CLUTTER_Armenian_DZA: c_int = 16778561;
pub const CLUTTER_Armenian_E: c_int = 16778551;
pub const CLUTTER_Armenian_FE: c_int = 16778582;
pub const CLUTTER_Armenian_GHAT: c_int = 16778562;
pub const CLUTTER_Armenian_GIM: c_int = 16778547;
pub const CLUTTER_Armenian_HI: c_int = 16778565;
pub const CLUTTER_Armenian_HO: c_int = 16778560;
pub const CLUTTER_Armenian_INI: c_int = 16778555;
pub const CLUTTER_Armenian_JE: c_int = 16778571;
pub const CLUTTER_Armenian_KE: c_int = 16778580;
pub const CLUTTER_Armenian_KEN: c_int = 16778559;
pub const CLUTTER_Armenian_KHE: c_int = 16778557;
pub const CLUTTER_Armenian_LYUN: c_int = 16778556;
pub const CLUTTER_Armenian_MEN: c_int = 16778564;
pub const CLUTTER_Armenian_NU: c_int = 16778566;
pub const CLUTTER_Armenian_O: c_int = 16778581;
pub const CLUTTER_Armenian_PE: c_int = 16778570;
pub const CLUTTER_Armenian_PYUR: c_int = 16778579;
pub const CLUTTER_Armenian_RA: c_int = 16778572;
pub const CLUTTER_Armenian_RE: c_int = 16778576;
pub const CLUTTER_Armenian_SE: c_int = 16778573;
pub const CLUTTER_Armenian_SHA: c_int = 16778567;
pub const CLUTTER_Armenian_TCHE: c_int = 16778563;
pub const CLUTTER_Armenian_TO: c_int = 16778553;
pub const CLUTTER_Armenian_TSA: c_int = 16778558;
pub const CLUTTER_Armenian_TSO: c_int = 16778577;
pub const CLUTTER_Armenian_TYUN: c_int = 16778575;
pub const CLUTTER_Armenian_VEV: c_int = 16778574;
pub const CLUTTER_Armenian_VO: c_int = 16778568;
pub const CLUTTER_Armenian_VYUN: c_int = 16778578;
pub const CLUTTER_Armenian_YECH: c_int = 16778549;
pub const CLUTTER_Armenian_ZA: c_int = 16778550;
pub const CLUTTER_Armenian_ZHE: c_int = 16778554;
pub const CLUTTER_Armenian_accent: c_int = 16778587;
pub const CLUTTER_Armenian_amanak: c_int = 16778588;
pub const CLUTTER_Armenian_apostrophe: c_int = 16778586;
pub const CLUTTER_Armenian_at: c_int = 16778600;
pub const CLUTTER_Armenian_ayb: c_int = 16778593;
pub const CLUTTER_Armenian_ben: c_int = 16778594;
pub const CLUTTER_Armenian_but: c_int = 16778589;
pub const CLUTTER_Armenian_cha: c_int = 16778617;
pub const CLUTTER_Armenian_da: c_int = 16778596;
pub const CLUTTER_Armenian_dza: c_int = 16778609;
pub const CLUTTER_Armenian_e: c_int = 16778599;
pub const CLUTTER_Armenian_exclam: c_int = 16778588;
pub const CLUTTER_Armenian_fe: c_int = 16778630;
pub const CLUTTER_Armenian_full_stop: c_int = 16778633;
pub const CLUTTER_Armenian_ghat: c_int = 16778610;
pub const CLUTTER_Armenian_gim: c_int = 16778595;
pub const CLUTTER_Armenian_hi: c_int = 16778613;
pub const CLUTTER_Armenian_ho: c_int = 16778608;
pub const CLUTTER_Armenian_hyphen: c_int = 16778634;
pub const CLUTTER_Armenian_ini: c_int = 16778603;
pub const CLUTTER_Armenian_je: c_int = 16778619;
pub const CLUTTER_Armenian_ke: c_int = 16778628;
pub const CLUTTER_Armenian_ken: c_int = 16778607;
pub const CLUTTER_Armenian_khe: c_int = 16778605;
pub const CLUTTER_Armenian_ligature_ew: c_int = 16778631;
pub const CLUTTER_Armenian_lyun: c_int = 16778604;
pub const CLUTTER_Armenian_men: c_int = 16778612;
pub const CLUTTER_Armenian_nu: c_int = 16778614;
pub const CLUTTER_Armenian_o: c_int = 16778629;
pub const CLUTTER_Armenian_paruyk: c_int = 16778590;
pub const CLUTTER_Armenian_pe: c_int = 16778618;
pub const CLUTTER_Armenian_pyur: c_int = 16778627;
pub const CLUTTER_Armenian_question: c_int = 16778590;
pub const CLUTTER_Armenian_ra: c_int = 16778620;
pub const CLUTTER_Armenian_re: c_int = 16778624;
pub const CLUTTER_Armenian_se: c_int = 16778621;
pub const CLUTTER_Armenian_separation_mark: c_int = 16778589;
pub const CLUTTER_Armenian_sha: c_int = 16778615;
pub const CLUTTER_Armenian_shesht: c_int = 16778587;
pub const CLUTTER_Armenian_tche: c_int = 16778611;
pub const CLUTTER_Armenian_to: c_int = 16778601;
pub const CLUTTER_Armenian_tsa: c_int = 16778606;
pub const CLUTTER_Armenian_tso: c_int = 16778625;
pub const CLUTTER_Armenian_tyun: c_int = 16778623;
pub const CLUTTER_Armenian_verjaket: c_int = 16778633;
pub const CLUTTER_Armenian_vev: c_int = 16778622;
pub const CLUTTER_Armenian_vo: c_int = 16778616;
pub const CLUTTER_Armenian_vyun: c_int = 16778626;
pub const CLUTTER_Armenian_yech: c_int = 16778597;
pub const CLUTTER_Armenian_yentamna: c_int = 16778634;
pub const CLUTTER_Armenian_za: c_int = 16778598;
pub const CLUTTER_Armenian_zhe: c_int = 16778602;
pub const CLUTTER_Atilde: c_int = 195;
pub const CLUTTER_AudibleBell_Enable: c_int = 65146;
pub const CLUTTER_AudioCycleTrack: c_int = 269025179;
pub const CLUTTER_AudioForward: c_int = 269025175;
pub const CLUTTER_AudioLowerVolume: c_int = 269025041;
pub const CLUTTER_AudioMedia: c_int = 269025074;
pub const CLUTTER_AudioMicMute: c_int = 269025202;
pub const CLUTTER_AudioMute: c_int = 269025042;
pub const CLUTTER_AudioNext: c_int = 269025047;
pub const CLUTTER_AudioPause: c_int = 269025073;
pub const CLUTTER_AudioPlay: c_int = 269025044;
pub const CLUTTER_AudioPrev: c_int = 269025046;
pub const CLUTTER_AudioRaiseVolume: c_int = 269025043;
pub const CLUTTER_AudioRandomPlay: c_int = 269025177;
pub const CLUTTER_AudioRecord: c_int = 269025052;
pub const CLUTTER_AudioRepeat: c_int = 269025176;
pub const CLUTTER_AudioRewind: c_int = 269025086;
pub const CLUTTER_AudioStop: c_int = 269025045;
pub const CLUTTER_Away: c_int = 269025165;
pub const CLUTTER_B: c_int = 66;
pub const CLUTTER_BUTTON_MIDDLE: c_int = 2;
pub const CLUTTER_BUTTON_PRIMARY: c_int = 1;
pub const CLUTTER_BUTTON_SECONDARY: c_int = 3;
pub const CLUTTER_Babovedot: c_int = 16784898;
pub const CLUTTER_Back: c_int = 269025062;
pub const CLUTTER_BackForward: c_int = 269025087;
pub const CLUTTER_BackSpace: c_int = 65288;
pub const CLUTTER_Battery: c_int = 269025171;
pub const CLUTTER_Begin: c_int = 65368;
pub const CLUTTER_Blue: c_int = 269025190;
pub const CLUTTER_Bluetooth: c_int = 269025172;
pub const CLUTTER_Book: c_int = 269025106;
pub const CLUTTER_BounceKeys_Enable: c_int = 65140;
pub const CLUTTER_Break: c_int = 65387;
pub const CLUTTER_BrightnessAdjust: c_int = 269025083;
pub const CLUTTER_Byelorussian_SHORTU: c_int = 1726;
pub const CLUTTER_Byelorussian_shortu: c_int = 1710;
pub const CLUTTER_C: c_int = 67;
pub const CLUTTER_CD: c_int = 269025107;
pub const CLUTTER_CH: c_int = 65186;
pub const CLUTTER_COGL: *const c_char = b"deprecated\0" as *const u8 as *const c_char;
pub const CLUTTER_CURRENT_TIME: c_int = 0;
pub const CLUTTER_C_H: c_int = 65189;
pub const CLUTTER_C_h: c_int = 65188;
pub const CLUTTER_Cabovedot: c_int = 709;
pub const CLUTTER_Cacute: c_int = 454;
pub const CLUTTER_Calculator: c_int = 269025053;
pub const CLUTTER_Calendar: c_int = 269025056;
pub const CLUTTER_Cancel: c_int = 65385;
pub const CLUTTER_Caps_Lock: c_int = 65509;
pub const CLUTTER_Ccaron: c_int = 456;
pub const CLUTTER_Ccedilla: c_int = 199;
pub const CLUTTER_Ccircumflex: c_int = 710;
pub const CLUTTER_Ch: c_int = 65185;
pub const CLUTTER_Clear: c_int = 65291;
pub const CLUTTER_ClearGrab: c_int = 269024801;
pub const CLUTTER_Close: c_int = 269025110;
pub const CLUTTER_Codeinput: c_int = 65335;
pub const CLUTTER_ColonSign: c_int = 16785569;
pub const CLUTTER_Community: c_int = 269025085;
pub const CLUTTER_ContrastAdjust: c_int = 269025058;
pub const CLUTTER_Control_L: c_int = 65507;
pub const CLUTTER_Control_R: c_int = 65508;
pub const CLUTTER_Copy: c_int = 269025111;
pub const CLUTTER_CruzeiroSign: c_int = 16785570;
pub const CLUTTER_Cut: c_int = 269025112;
pub const CLUTTER_CycleAngle: c_int = 269025180;
pub const CLUTTER_Cyrillic_A: c_int = 1761;
pub const CLUTTER_Cyrillic_BE: c_int = 1762;
pub const CLUTTER_Cyrillic_CHE: c_int = 1790;
pub const CLUTTER_Cyrillic_CHE_descender: c_int = 16778422;
pub const CLUTTER_Cyrillic_CHE_vertstroke: c_int = 16778424;
pub const CLUTTER_Cyrillic_DE: c_int = 1764;
pub const CLUTTER_Cyrillic_DZHE: c_int = 1727;
pub const CLUTTER_Cyrillic_E: c_int = 1788;
pub const CLUTTER_Cyrillic_EF: c_int = 1766;
pub const CLUTTER_Cyrillic_EL: c_int = 1772;
pub const CLUTTER_Cyrillic_EM: c_int = 1773;
pub const CLUTTER_Cyrillic_EN: c_int = 1774;
pub const CLUTTER_Cyrillic_EN_descender: c_int = 16778402;
pub const CLUTTER_Cyrillic_ER: c_int = 1778;
pub const CLUTTER_Cyrillic_ES: c_int = 1779;
pub const CLUTTER_Cyrillic_GHE: c_int = 1767;
pub const CLUTTER_Cyrillic_GHE_bar: c_int = 16778386;
pub const CLUTTER_Cyrillic_HA: c_int = 1768;
pub const CLUTTER_Cyrillic_HARDSIGN: c_int = 1791;
pub const CLUTTER_Cyrillic_HA_descender: c_int = 16778418;
pub const CLUTTER_Cyrillic_I: c_int = 1769;
pub const CLUTTER_Cyrillic_IE: c_int = 1765;
pub const CLUTTER_Cyrillic_IO: c_int = 1715;
pub const CLUTTER_Cyrillic_I_macron: c_int = 16778466;
pub const CLUTTER_Cyrillic_JE: c_int = 1720;
pub const CLUTTER_Cyrillic_KA: c_int = 1771;
pub const CLUTTER_Cyrillic_KA_descender: c_int = 16778394;
pub const CLUTTER_Cyrillic_KA_vertstroke: c_int = 16778396;
pub const CLUTTER_Cyrillic_LJE: c_int = 1721;
pub const CLUTTER_Cyrillic_NJE: c_int = 1722;
pub const CLUTTER_Cyrillic_O: c_int = 1775;
pub const CLUTTER_Cyrillic_O_bar: c_int = 16778472;
pub const CLUTTER_Cyrillic_PE: c_int = 1776;
pub const CLUTTER_Cyrillic_SCHWA: c_int = 16778456;
pub const CLUTTER_Cyrillic_SHA: c_int = 1787;
pub const CLUTTER_Cyrillic_SHCHA: c_int = 1789;
pub const CLUTTER_Cyrillic_SHHA: c_int = 16778426;
pub const CLUTTER_Cyrillic_SHORTI: c_int = 1770;
pub const CLUTTER_Cyrillic_SOFTSIGN: c_int = 1784;
pub const CLUTTER_Cyrillic_TE: c_int = 1780;
pub const CLUTTER_Cyrillic_TSE: c_int = 1763;
pub const CLUTTER_Cyrillic_U: c_int = 1781;
pub const CLUTTER_Cyrillic_U_macron: c_int = 16778478;
pub const CLUTTER_Cyrillic_U_straight: c_int = 16778414;
pub const CLUTTER_Cyrillic_U_straight_bar: c_int = 16778416;
pub const CLUTTER_Cyrillic_VE: c_int = 1783;
pub const CLUTTER_Cyrillic_YA: c_int = 1777;
pub const CLUTTER_Cyrillic_YERU: c_int = 1785;
pub const CLUTTER_Cyrillic_YU: c_int = 1760;
pub const CLUTTER_Cyrillic_ZE: c_int = 1786;
pub const CLUTTER_Cyrillic_ZHE: c_int = 1782;
pub const CLUTTER_Cyrillic_ZHE_descender: c_int = 16778390;
pub const CLUTTER_Cyrillic_a: c_int = 1729;
pub const CLUTTER_Cyrillic_be: c_int = 1730;
pub const CLUTTER_Cyrillic_che: c_int = 1758;
pub const CLUTTER_Cyrillic_che_descender: c_int = 16778423;
pub const CLUTTER_Cyrillic_che_vertstroke: c_int = 16778425;
pub const CLUTTER_Cyrillic_de: c_int = 1732;
pub const CLUTTER_Cyrillic_dzhe: c_int = 1711;
pub const CLUTTER_Cyrillic_e: c_int = 1756;
pub const CLUTTER_Cyrillic_ef: c_int = 1734;
pub const CLUTTER_Cyrillic_el: c_int = 1740;
pub const CLUTTER_Cyrillic_em: c_int = 1741;
pub const CLUTTER_Cyrillic_en: c_int = 1742;
pub const CLUTTER_Cyrillic_en_descender: c_int = 16778403;
pub const CLUTTER_Cyrillic_er: c_int = 1746;
pub const CLUTTER_Cyrillic_es: c_int = 1747;
pub const CLUTTER_Cyrillic_ghe: c_int = 1735;
pub const CLUTTER_Cyrillic_ghe_bar: c_int = 16778387;
pub const CLUTTER_Cyrillic_ha: c_int = 1736;
pub const CLUTTER_Cyrillic_ha_descender: c_int = 16778419;
pub const CLUTTER_Cyrillic_hardsign: c_int = 1759;
pub const CLUTTER_Cyrillic_i: c_int = 1737;
pub const CLUTTER_Cyrillic_i_macron: c_int = 16778467;
pub const CLUTTER_Cyrillic_ie: c_int = 1733;
pub const CLUTTER_Cyrillic_io: c_int = 1699;
pub const CLUTTER_Cyrillic_je: c_int = 1704;
pub const CLUTTER_Cyrillic_ka: c_int = 1739;
pub const CLUTTER_Cyrillic_ka_descender: c_int = 16778395;
pub const CLUTTER_Cyrillic_ka_vertstroke: c_int = 16778397;
pub const CLUTTER_Cyrillic_lje: c_int = 1705;
pub const CLUTTER_Cyrillic_nje: c_int = 1706;
pub const CLUTTER_Cyrillic_o: c_int = 1743;
pub const CLUTTER_Cyrillic_o_bar: c_int = 16778473;
pub const CLUTTER_Cyrillic_pe: c_int = 1744;
pub const CLUTTER_Cyrillic_schwa: c_int = 16778457;
pub const CLUTTER_Cyrillic_sha: c_int = 1755;
pub const CLUTTER_Cyrillic_shcha: c_int = 1757;
pub const CLUTTER_Cyrillic_shha: c_int = 16778427;
pub const CLUTTER_Cyrillic_shorti: c_int = 1738;
pub const CLUTTER_Cyrillic_softsign: c_int = 1752;
pub const CLUTTER_Cyrillic_te: c_int = 1748;
pub const CLUTTER_Cyrillic_tse: c_int = 1731;
pub const CLUTTER_Cyrillic_u: c_int = 1749;
pub const CLUTTER_Cyrillic_u_macron: c_int = 16778479;
pub const CLUTTER_Cyrillic_u_straight: c_int = 16778415;
pub const CLUTTER_Cyrillic_u_straight_bar: c_int = 16778417;
pub const CLUTTER_Cyrillic_ve: c_int = 1751;
pub const CLUTTER_Cyrillic_ya: c_int = 1745;
pub const CLUTTER_Cyrillic_yeru: c_int = 1753;
pub const CLUTTER_Cyrillic_yu: c_int = 1728;
pub const CLUTTER_Cyrillic_ze: c_int = 1754;
pub const CLUTTER_Cyrillic_zhe: c_int = 1750;
pub const CLUTTER_Cyrillic_zhe_descender: c_int = 16778391;
pub const CLUTTER_D: c_int = 68;
pub const CLUTTER_DOS: c_int = 269025114;
pub const CLUTTER_Dabovedot: c_int = 16784906;
pub const CLUTTER_Dcaron: c_int = 463;
pub const CLUTTER_Delete: c_int = 65535;
pub const CLUTTER_Display: c_int = 269025113;
pub const CLUTTER_Documents: c_int = 269025115;
pub const CLUTTER_DongSign: c_int = 16785579;
pub const CLUTTER_Down: c_int = 65364;
pub const CLUTTER_Dstroke: c_int = 464;
pub const CLUTTER_E: c_int = 69;
pub const CLUTTER_ENG: c_int = 957;
pub const CLUTTER_ETH: c_int = 208;
pub const CLUTTER_EVENT_PROPAGATE: gboolean = glib::GFALSE;
pub const CLUTTER_EVENT_STOP: gboolean = glib::GTRUE;
pub const CLUTTER_EZH: c_int = 16777655;
pub const CLUTTER_Eabovedot: c_int = 972;
pub const CLUTTER_Eacute: c_int = 201;
pub const CLUTTER_Ebelowdot: c_int = 16785080;
pub const CLUTTER_Ecaron: c_int = 460;
pub const CLUTTER_Ecircumflex: c_int = 202;
pub const CLUTTER_Ecircumflexacute: c_int = 16785086;
pub const CLUTTER_Ecircumflexbelowdot: c_int = 16785094;
pub const CLUTTER_Ecircumflexgrave: c_int = 16785088;
pub const CLUTTER_Ecircumflexhook: c_int = 16785090;
pub const CLUTTER_Ecircumflextilde: c_int = 16785092;
pub const CLUTTER_EcuSign: c_int = 16785568;
pub const CLUTTER_Ediaeresis: c_int = 203;
pub const CLUTTER_Egrave: c_int = 200;
pub const CLUTTER_Ehook: c_int = 16785082;
pub const CLUTTER_Eisu_Shift: c_int = 65327;
pub const CLUTTER_Eisu_toggle: c_int = 65328;
pub const CLUTTER_Eject: c_int = 269025068;
pub const CLUTTER_Emacron: c_int = 938;
pub const CLUTTER_End: c_int = 65367;
pub const CLUTTER_Eogonek: c_int = 458;
pub const CLUTTER_Escape: c_int = 65307;
pub const CLUTTER_Eth: c_int = 208;
pub const CLUTTER_Etilde: c_int = 16785084;
pub const CLUTTER_EuroSign: c_int = 8364;
pub const CLUTTER_Excel: c_int = 269025116;
pub const CLUTTER_Execute: c_int = 65378;
pub const CLUTTER_Explorer: c_int = 269025117;
pub const CLUTTER_F: c_int = 70;
pub const CLUTTER_F1: c_int = 65470;
pub const CLUTTER_F10: c_int = 65479;
pub const CLUTTER_F11: c_int = 65480;
pub const CLUTTER_F12: c_int = 65481;
pub const CLUTTER_F13: c_int = 65482;
pub const CLUTTER_F14: c_int = 65483;
pub const CLUTTER_F15: c_int = 65484;
pub const CLUTTER_F16: c_int = 65485;
pub const CLUTTER_F17: c_int = 65486;
pub const CLUTTER_F18: c_int = 65487;
pub const CLUTTER_F19: c_int = 65488;
pub const CLUTTER_F2: c_int = 65471;
pub const CLUTTER_F20: c_int = 65489;
pub const CLUTTER_F21: c_int = 65490;
pub const CLUTTER_F22: c_int = 65491;
pub const CLUTTER_F23: c_int = 65492;
pub const CLUTTER_F24: c_int = 65493;
pub const CLUTTER_F25: c_int = 65494;
pub const CLUTTER_F26: c_int = 65495;
pub const CLUTTER_F27: c_int = 65496;
pub const CLUTTER_F28: c_int = 65497;
pub const CLUTTER_F29: c_int = 65498;
pub const CLUTTER_F3: c_int = 65472;
pub const CLUTTER_F30: c_int = 65499;
pub const CLUTTER_F31: c_int = 65500;
pub const CLUTTER_F32: c_int = 65501;
pub const CLUTTER_F33: c_int = 65502;
pub const CLUTTER_F34: c_int = 65503;
pub const CLUTTER_F35: c_int = 65504;
pub const CLUTTER_F4: c_int = 65473;
pub const CLUTTER_F5: c_int = 65474;
pub const CLUTTER_F6: c_int = 65475;
pub const CLUTTER_F7: c_int = 65476;
pub const CLUTTER_F8: c_int = 65477;
pub const CLUTTER_F9: c_int = 65478;
pub const CLUTTER_FFrancSign: c_int = 16785571;
pub const CLUTTER_FLAVOUR: *const c_char = b"deprecated\0" as *const u8 as *const c_char;
pub const CLUTTER_Fabovedot: c_int = 16784926;
pub const CLUTTER_Farsi_0: c_int = 16778992;
pub const CLUTTER_Farsi_1: c_int = 16778993;
pub const CLUTTER_Farsi_2: c_int = 16778994;
pub const CLUTTER_Farsi_3: c_int = 16778995;
pub const CLUTTER_Farsi_4: c_int = 16778996;
pub const CLUTTER_Farsi_5: c_int = 16778997;
pub const CLUTTER_Farsi_6: c_int = 16778998;
pub const CLUTTER_Farsi_7: c_int = 16778999;
pub const CLUTTER_Farsi_8: c_int = 16779000;
pub const CLUTTER_Farsi_9: c_int = 16779001;
pub const CLUTTER_Farsi_yeh: c_int = 16778956;
pub const CLUTTER_Favorites: c_int = 269025072;
pub const CLUTTER_Finance: c_int = 269025084;
pub const CLUTTER_Find: c_int = 65384;
pub const CLUTTER_First_Virtual_Screen: c_int = 65232;
pub const CLUTTER_Forward: c_int = 269025063;
pub const CLUTTER_FrameBack: c_int = 269025181;
pub const CLUTTER_FrameForward: c_int = 269025182;
pub const CLUTTER_G: c_int = 71;
pub const CLUTTER_Gabovedot: c_int = 725;
pub const CLUTTER_Game: c_int = 269025118;
pub const CLUTTER_Gbreve: c_int = 683;
pub const CLUTTER_Gcaron: c_int = 16777702;
pub const CLUTTER_Gcedilla: c_int = 939;
pub const CLUTTER_Gcircumflex: c_int = 728;
pub const CLUTTER_Georgian_an: c_int = 16781520;
pub const CLUTTER_Georgian_ban: c_int = 16781521;
pub const CLUTTER_Georgian_can: c_int = 16781546;
pub const CLUTTER_Georgian_char: c_int = 16781549;
pub const CLUTTER_Georgian_chin: c_int = 16781545;
pub const CLUTTER_Georgian_cil: c_int = 16781548;
pub const CLUTTER_Georgian_don: c_int = 16781523;
pub const CLUTTER_Georgian_en: c_int = 16781524;
pub const CLUTTER_Georgian_fi: c_int = 16781558;
pub const CLUTTER_Georgian_gan: c_int = 16781522;
pub const CLUTTER_Georgian_ghan: c_int = 16781542;
pub const CLUTTER_Georgian_hae: c_int = 16781552;
pub const CLUTTER_Georgian_har: c_int = 16781556;
pub const CLUTTER_Georgian_he: c_int = 16781553;
pub const CLUTTER_Georgian_hie: c_int = 16781554;
pub const CLUTTER_Georgian_hoe: c_int = 16781557;
pub const CLUTTER_Georgian_in: c_int = 16781528;
pub const CLUTTER_Georgian_jhan: c_int = 16781551;
pub const CLUTTER_Georgian_jil: c_int = 16781547;
pub const CLUTTER_Georgian_kan: c_int = 16781529;
pub const CLUTTER_Georgian_khar: c_int = 16781541;
pub const CLUTTER_Georgian_las: c_int = 16781530;
pub const CLUTTER_Georgian_man: c_int = 16781531;
pub const CLUTTER_Georgian_nar: c_int = 16781532;
pub const CLUTTER_Georgian_on: c_int = 16781533;
pub const CLUTTER_Georgian_par: c_int = 16781534;
pub const CLUTTER_Georgian_phar: c_int = 16781540;
pub const CLUTTER_Georgian_qar: c_int = 16781543;
pub const CLUTTER_Georgian_rae: c_int = 16781536;
pub const CLUTTER_Georgian_san: c_int = 16781537;
pub const CLUTTER_Georgian_shin: c_int = 16781544;
pub const CLUTTER_Georgian_tan: c_int = 16781527;
pub const CLUTTER_Georgian_tar: c_int = 16781538;
pub const CLUTTER_Georgian_un: c_int = 16781539;
pub const CLUTTER_Georgian_vin: c_int = 16781525;
pub const CLUTTER_Georgian_we: c_int = 16781555;
pub const CLUTTER_Georgian_xan: c_int = 16781550;
pub const CLUTTER_Georgian_zen: c_int = 16781526;
pub const CLUTTER_Georgian_zhar: c_int = 16781535;
pub const CLUTTER_Go: c_int = 269025119;
pub const CLUTTER_Greek_ALPHA: c_int = 1985;
pub const CLUTTER_Greek_ALPHAaccent: c_int = 1953;
pub const CLUTTER_Greek_BETA: c_int = 1986;
pub const CLUTTER_Greek_CHI: c_int = 2007;
pub const CLUTTER_Greek_DELTA: c_int = 1988;
pub const CLUTTER_Greek_EPSILON: c_int = 1989;
pub const CLUTTER_Greek_EPSILONaccent: c_int = 1954;
pub const CLUTTER_Greek_ETA: c_int = 1991;
pub const CLUTTER_Greek_ETAaccent: c_int = 1955;
pub const CLUTTER_Greek_GAMMA: c_int = 1987;
pub const CLUTTER_Greek_IOTA: c_int = 1993;
pub const CLUTTER_Greek_IOTAaccent: c_int = 1956;
pub const CLUTTER_Greek_IOTAdiaeresis: c_int = 1957;
pub const CLUTTER_Greek_IOTAdieresis: c_int = 1957;
pub const CLUTTER_Greek_KAPPA: c_int = 1994;
pub const CLUTTER_Greek_LAMBDA: c_int = 1995;
pub const CLUTTER_Greek_LAMDA: c_int = 1995;
pub const CLUTTER_Greek_MU: c_int = 1996;
pub const CLUTTER_Greek_NU: c_int = 1997;
pub const CLUTTER_Greek_OMEGA: c_int = 2009;
pub const CLUTTER_Greek_OMEGAaccent: c_int = 1963;
pub const CLUTTER_Greek_OMICRON: c_int = 1999;
pub const CLUTTER_Greek_OMICRONaccent: c_int = 1959;
pub const CLUTTER_Greek_PHI: c_int = 2006;
pub const CLUTTER_Greek_PI: c_int = 2000;
pub const CLUTTER_Greek_PSI: c_int = 2008;
pub const CLUTTER_Greek_RHO: c_int = 2001;
pub const CLUTTER_Greek_SIGMA: c_int = 2002;
pub const CLUTTER_Greek_TAU: c_int = 2004;
pub const CLUTTER_Greek_THETA: c_int = 1992;
pub const CLUTTER_Greek_UPSILON: c_int = 2005;
pub const CLUTTER_Greek_UPSILONaccent: c_int = 1960;
pub const CLUTTER_Greek_UPSILONdieresis: c_int = 1961;
pub const CLUTTER_Greek_XI: c_int = 1998;
pub const CLUTTER_Greek_ZETA: c_int = 1990;
pub const CLUTTER_Greek_accentdieresis: c_int = 1966;
pub const CLUTTER_Greek_alpha: c_int = 2017;
pub const CLUTTER_Greek_alphaaccent: c_int = 1969;
pub const CLUTTER_Greek_beta: c_int = 2018;
pub const CLUTTER_Greek_chi: c_int = 2039;
pub const CLUTTER_Greek_delta: c_int = 2020;
pub const CLUTTER_Greek_epsilon: c_int = 2021;
pub const CLUTTER_Greek_epsilonaccent: c_int = 1970;
pub const CLUTTER_Greek_eta: c_int = 2023;
pub const CLUTTER_Greek_etaaccent: c_int = 1971;
pub const CLUTTER_Greek_finalsmallsigma: c_int = 2035;
pub const CLUTTER_Greek_gamma: c_int = 2019;
pub const CLUTTER_Greek_horizbar: c_int = 1967;
pub const CLUTTER_Greek_iota: c_int = 2025;
pub const CLUTTER_Greek_iotaaccent: c_int = 1972;
pub const CLUTTER_Greek_iotaaccentdieresis: c_int = 1974;
pub const CLUTTER_Greek_iotadieresis: c_int = 1973;
pub const CLUTTER_Greek_kappa: c_int = 2026;
pub const CLUTTER_Greek_lambda: c_int = 2027;
pub const CLUTTER_Greek_lamda: c_int = 2027;
pub const CLUTTER_Greek_mu: c_int = 2028;
pub const CLUTTER_Greek_nu: c_int = 2029;
pub const CLUTTER_Greek_omega: c_int = 2041;
pub const CLUTTER_Greek_omegaaccent: c_int = 1979;
pub const CLUTTER_Greek_omicron: c_int = 2031;
pub const CLUTTER_Greek_omicronaccent: c_int = 1975;
pub const CLUTTER_Greek_phi: c_int = 2038;
pub const CLUTTER_Greek_pi: c_int = 2032;
pub const CLUTTER_Greek_psi: c_int = 2040;
pub const CLUTTER_Greek_rho: c_int = 2033;
pub const CLUTTER_Greek_sigma: c_int = 2034;
pub const CLUTTER_Greek_switch: c_int = 65406;
pub const CLUTTER_Greek_tau: c_int = 2036;
pub const CLUTTER_Greek_theta: c_int = 2024;
pub const CLUTTER_Greek_upsilon: c_int = 2037;
pub const CLUTTER_Greek_upsilonaccent: c_int = 1976;
pub const CLUTTER_Greek_upsilonaccentdieresis: c_int = 1978;
pub const CLUTTER_Greek_upsilondieresis: c_int = 1977;
pub const CLUTTER_Greek_xi: c_int = 2030;
pub const CLUTTER_Greek_zeta: c_int = 2022;
pub const CLUTTER_Green: c_int = 269025188;
pub const CLUTTER_H: c_int = 72;
pub const CLUTTER_HAS_WAYLAND_COMPOSITOR_SUPPORT: c_int = 1;
pub const CLUTTER_Hangul: c_int = 65329;
pub const CLUTTER_Hangul_A: c_int = 3775;
pub const CLUTTER_Hangul_AE: c_int = 3776;
pub const CLUTTER_Hangul_AraeA: c_int = 3830;
pub const CLUTTER_Hangul_AraeAE: c_int = 3831;
pub const CLUTTER_Hangul_Banja: c_int = 65337;
pub const CLUTTER_Hangul_Cieuc: c_int = 3770;
pub const CLUTTER_Hangul_Codeinput: c_int = 65335;
pub const CLUTTER_Hangul_Dikeud: c_int = 3751;
pub const CLUTTER_Hangul_E: c_int = 3780;
pub const CLUTTER_Hangul_EO: c_int = 3779;
pub const CLUTTER_Hangul_EU: c_int = 3793;
pub const CLUTTER_Hangul_End: c_int = 65331;
pub const CLUTTER_Hangul_Hanja: c_int = 65332;
pub const CLUTTER_Hangul_Hieuh: c_int = 3774;
pub const CLUTTER_Hangul_I: c_int = 3795;
pub const CLUTTER_Hangul_Ieung: c_int = 3767;
pub const CLUTTER_Hangul_J_Cieuc: c_int = 3818;
pub const CLUTTER_Hangul_J_Dikeud: c_int = 3802;
pub const CLUTTER_Hangul_J_Hieuh: c_int = 3822;
pub const CLUTTER_Hangul_J_Ieung: c_int = 3816;
pub const CLUTTER_Hangul_J_Jieuj: c_int = 3817;
pub const CLUTTER_Hangul_J_Khieuq: c_int = 3819;
pub const CLUTTER_Hangul_J_Kiyeog: c_int = 3796;
pub const CLUTTER_Hangul_J_KiyeogSios: c_int = 3798;
pub const CLUTTER_Hangul_J_KkogjiDalrinIeung: c_int = 3833;
pub const CLUTTER_Hangul_J_Mieum: c_int = 3811;
pub const CLUTTER_Hangul_J_Nieun: c_int = 3799;
pub const CLUTTER_Hangul_J_NieunHieuh: c_int = 3801;
pub const CLUTTER_Hangul_J_NieunJieuj: c_int = 3800;
pub const CLUTTER_Hangul_J_PanSios: c_int = 3832;
pub const CLUTTER_Hangul_J_Phieuf: c_int = 3821;
pub const CLUTTER_Hangul_J_Pieub: c_int = 3812;
pub const CLUTTER_Hangul_J_PieubSios: c_int = 3813;
pub const CLUTTER_Hangul_J_Rieul: c_int = 3803;
pub const CLUTTER_Hangul_J_RieulHieuh: c_int = 3810;
pub const CLUTTER_Hangul_J_RieulKiyeog: c_int = 3804;
pub const CLUTTER_Hangul_J_RieulMieum: c_int = 3805;
pub const CLUTTER_Hangul_J_RieulPhieuf: c_int = 3809;
pub const CLUTTER_Hangul_J_RieulPieub: c_int = 3806;
pub const CLUTTER_Hangul_J_RieulSios: c_int = 3807;
pub const CLUTTER_Hangul_J_RieulTieut: c_int = 3808;
pub const CLUTTER_Hangul_J_Sios: c_int = 3814;
pub const CLUTTER_Hangul_J_SsangKiyeog: c_int = 3797;
pub const CLUTTER_Hangul_J_SsangSios: c_int = 3815;
pub const CLUTTER_Hangul_J_Tieut: c_int = 3820;
pub const CLUTTER_Hangul_J_YeorinHieuh: c_int = 3834;
pub const CLUTTER_Hangul_Jamo: c_int = 65333;
pub const CLUTTER_Hangul_Jeonja: c_int = 65336;
pub const CLUTTER_Hangul_Jieuj: c_int = 3768;
pub const CLUTTER_Hangul_Khieuq: c_int = 3771;
pub const CLUTTER_Hangul_Kiyeog: c_int = 3745;
pub const CLUTTER_Hangul_KiyeogSios: c_int = 3747;
pub const CLUTTER_Hangul_KkogjiDalrinIeung: c_int = 3827;
pub const CLUTTER_Hangul_Mieum: c_int = 3761;
pub const CLUTTER_Hangul_MultipleCandidate: c_int = 65341;
pub const CLUTTER_Hangul_Nieun: c_int = 3748;
pub const CLUTTER_Hangul_NieunHieuh: c_int = 3750;
pub const CLUTTER_Hangul_NieunJieuj: c_int = 3749;
pub const CLUTTER_Hangul_O: c_int = 3783;
pub const CLUTTER_Hangul_OE: c_int = 3786;
pub const CLUTTER_Hangul_PanSios: c_int = 3826;
pub const CLUTTER_Hangul_Phieuf: c_int = 3773;
pub const CLUTTER_Hangul_Pieub: c_int = 3762;
pub const CLUTTER_Hangul_PieubSios: c_int = 3764;
pub const CLUTTER_Hangul_PostHanja: c_int = 65339;
pub const CLUTTER_Hangul_PreHanja: c_int = 65338;
pub const CLUTTER_Hangul_PreviousCandidate: c_int = 65342;
pub const CLUTTER_Hangul_Rieul: c_int = 3753;
pub const CLUTTER_Hangul_RieulHieuh: c_int = 3760;
pub const CLUTTER_Hangul_RieulKiyeog: c_int = 3754;
pub const CLUTTER_Hangul_RieulMieum: c_int = 3755;
pub const CLUTTER_Hangul_RieulPhieuf: c_int = 3759;
pub const CLUTTER_Hangul_RieulPieub: c_int = 3756;
pub const CLUTTER_Hangul_RieulSios: c_int = 3757;
pub const CLUTTER_Hangul_RieulTieut: c_int = 3758;
pub const CLUTTER_Hangul_RieulYeorinHieuh: c_int = 3823;
pub const CLUTTER_Hangul_Romaja: c_int = 65334;
pub const CLUTTER_Hangul_SingleCandidate: c_int = 65340;
pub const CLUTTER_Hangul_Sios: c_int = 3765;
pub const CLUTTER_Hangul_Special: c_int = 65343;
pub const CLUTTER_Hangul_SsangDikeud: c_int = 3752;
pub const CLUTTER_Hangul_SsangJieuj: c_int = 3769;
pub const CLUTTER_Hangul_SsangKiyeog: c_int = 3746;
pub const CLUTTER_Hangul_SsangPieub: c_int = 3763;
pub const CLUTTER_Hangul_SsangSios: c_int = 3766;
pub const CLUTTER_Hangul_Start: c_int = 65330;
pub const CLUTTER_Hangul_SunkyeongeumMieum: c_int = 3824;
pub const CLUTTER_Hangul_SunkyeongeumPhieuf: c_int = 3828;
pub const CLUTTER_Hangul_SunkyeongeumPieub: c_int = 3825;
pub const CLUTTER_Hangul_Tieut: c_int = 3772;
pub const CLUTTER_Hangul_U: c_int = 3788;
pub const CLUTTER_Hangul_WA: c_int = 3784;
pub const CLUTTER_Hangul_WAE: c_int = 3785;
pub const CLUTTER_Hangul_WE: c_int = 3790;
pub const CLUTTER_Hangul_WEO: c_int = 3789;
pub const CLUTTER_Hangul_WI: c_int = 3791;
pub const CLUTTER_Hangul_YA: c_int = 3777;
pub const CLUTTER_Hangul_YAE: c_int = 3778;
pub const CLUTTER_Hangul_YE: c_int = 3782;
pub const CLUTTER_Hangul_YEO: c_int = 3781;
pub const CLUTTER_Hangul_YI: c_int = 3794;
pub const CLUTTER_Hangul_YO: c_int = 3787;
pub const CLUTTER_Hangul_YU: c_int = 3792;
pub const CLUTTER_Hangul_YeorinHieuh: c_int = 3829;
pub const CLUTTER_Hangul_switch: c_int = 65406;
pub const CLUTTER_Hankaku: c_int = 65321;
pub const CLUTTER_Hcircumflex: c_int = 678;
pub const CLUTTER_Hebrew_switch: c_int = 65406;
pub const CLUTTER_Help: c_int = 65386;
pub const CLUTTER_Henkan: c_int = 65315;
pub const CLUTTER_Henkan_Mode: c_int = 65315;
pub const CLUTTER_Hibernate: c_int = 269025192;
pub const CLUTTER_Hiragana: c_int = 65317;
pub const CLUTTER_Hiragana_Katakana: c_int = 65319;
pub const CLUTTER_History: c_int = 269025079;
pub const CLUTTER_Home: c_int = 65360;
pub const CLUTTER_HomePage: c_int = 269025048;
pub const CLUTTER_HotLinks: c_int = 269025082;
pub const CLUTTER_Hstroke: c_int = 673;
pub const CLUTTER_Hyper_L: c_int = 65517;
pub const CLUTTER_Hyper_R: c_int = 65518;
pub const CLUTTER_I: c_int = 73;
pub const CLUTTER_INPUT_EVDEV: *const c_char = b"evdev\0" as *const u8 as *const c_char;
pub const CLUTTER_INPUT_GDK: *const c_char = b"gdk\0" as *const u8 as *const c_char;
pub const CLUTTER_INPUT_NULL: *const c_char = b"null\0" as *const u8 as *const c_char;
pub const CLUTTER_INPUT_WAYLAND: *const c_char = b"wayland\0" as *const u8 as *const c_char;
pub const CLUTTER_INPUT_X11: *const c_char = b"x11\0" as *const u8 as *const c_char;
pub const CLUTTER_ISO_Center_Object: c_int = 65075;
pub const CLUTTER_ISO_Continuous_Underline: c_int = 65072;
pub const CLUTTER_ISO_Discontinuous_Underline: c_int = 65073;
pub const CLUTTER_ISO_Emphasize: c_int = 65074;
pub const CLUTTER_ISO_Enter: c_int = 65076;
pub const CLUTTER_ISO_Fast_Cursor_Down: c_int = 65071;
pub const CLUTTER_ISO_Fast_Cursor_Left: c_int = 65068;
pub const CLUTTER_ISO_Fast_Cursor_Right: c_int = 65069;
pub const CLUTTER_ISO_Fast_Cursor_Up: c_int = 65070;
pub const CLUTTER_ISO_First_Group: c_int = 65036;
pub const CLUTTER_ISO_First_Group_Lock: c_int = 65037;
pub const CLUTTER_ISO_Group_Latch: c_int = 65030;
pub const CLUTTER_ISO_Group_Lock: c_int = 65031;
pub const CLUTTER_ISO_Group_Shift: c_int = 65406;
pub const CLUTTER_ISO_Last_Group: c_int = 65038;
pub const CLUTTER_ISO_Last_Group_Lock: c_int = 65039;
pub const CLUTTER_ISO_Left_Tab: c_int = 65056;
pub const CLUTTER_ISO_Level2_Latch: c_int = 65026;
pub const CLUTTER_ISO_Level3_Latch: c_int = 65028;
pub const CLUTTER_ISO_Level3_Lock: c_int = 65029;
pub const CLUTTER_ISO_Level3_Shift: c_int = 65027;
pub const CLUTTER_ISO_Level5_Latch: c_int = 65042;
pub const CLUTTER_ISO_Level5_Lock: c_int = 65043;
pub const CLUTTER_ISO_Level5_Shift: c_int = 65041;
pub const CLUTTER_ISO_Lock: c_int = 65025;
pub const CLUTTER_ISO_Move_Line_Down: c_int = 65058;
pub const CLUTTER_ISO_Move_Line_Up: c_int = 65057;
pub const CLUTTER_ISO_Next_Group: c_int = 65032;
pub const CLUTTER_ISO_Next_Group_Lock: c_int = 65033;
pub const CLUTTER_ISO_Partial_Line_Down: c_int = 65060;
pub const CLUTTER_ISO_Partial_Line_Up: c_int = 65059;
pub const CLUTTER_ISO_Partial_Space_Left: c_int = 65061;
pub const CLUTTER_ISO_Partial_Space_Right: c_int = 65062;
pub const CLUTTER_ISO_Prev_Group: c_int = 65034;
pub const CLUTTER_ISO_Prev_Group_Lock: c_int = 65035;
pub const CLUTTER_ISO_Release_Both_Margins: c_int = 65067;
pub const CLUTTER_ISO_Release_Margin_Left: c_int = 65065;
pub const CLUTTER_ISO_Release_Margin_Right: c_int = 65066;
pub const CLUTTER_ISO_Set_Margin_Left: c_int = 65063;
pub const CLUTTER_ISO_Set_Margin_Right: c_int = 65064;
pub const CLUTTER_Iabovedot: c_int = 681;
pub const CLUTTER_Iacute: c_int = 205;
pub const CLUTTER_Ibelowdot: c_int = 16785098;
pub const CLUTTER_Ibreve: c_int = 16777516;
pub const CLUTTER_Icircumflex: c_int = 206;
pub const CLUTTER_Idiaeresis: c_int = 207;
pub const CLUTTER_Igrave: c_int = 204;
pub const CLUTTER_Ihook: c_int = 16785096;
pub const CLUTTER_Imacron: c_int = 975;
pub const CLUTTER_Insert: c_int = 65379;
pub const CLUTTER_Iogonek: c_int = 967;
pub const CLUTTER_Itilde: c_int = 933;
pub const CLUTTER_J: c_int = 74;
pub const CLUTTER_Jcircumflex: c_int = 684;
pub const CLUTTER_K: c_int = 75;
pub const CLUTTER_KEY_0: c_int = 48;
pub const CLUTTER_KEY_1: c_int = 49;
pub const CLUTTER_KEY_2: c_int = 50;
pub const CLUTTER_KEY_3: c_int = 51;
pub const CLUTTER_KEY_3270_AltCursor: c_int = 64784;
pub const CLUTTER_KEY_3270_Attn: c_int = 64782;
pub const CLUTTER_KEY_3270_BackTab: c_int = 64773;
pub const CLUTTER_KEY_3270_ChangeScreen: c_int = 64793;
pub const CLUTTER_KEY_3270_Copy: c_int = 64789;
pub const CLUTTER_KEY_3270_CursorBlink: c_int = 64783;
pub const CLUTTER_KEY_3270_CursorSelect: c_int = 64796;
pub const CLUTTER_KEY_3270_DeleteWord: c_int = 64794;
pub const CLUTTER_KEY_3270_Duplicate: c_int = 64769;
pub const CLUTTER_KEY_3270_Enter: c_int = 64798;
pub const CLUTTER_KEY_3270_EraseEOF: c_int = 64774;
pub const CLUTTER_KEY_3270_EraseInput: c_int = 64775;
pub const CLUTTER_KEY_3270_ExSelect: c_int = 64795;
pub const CLUTTER_KEY_3270_FieldMark: c_int = 64770;
pub const CLUTTER_KEY_3270_Ident: c_int = 64787;
pub const CLUTTER_KEY_3270_Jump: c_int = 64786;
pub const CLUTTER_KEY_3270_KeyClick: c_int = 64785;
pub const CLUTTER_KEY_3270_Left2: c_int = 64772;
pub const CLUTTER_KEY_3270_PA1: c_int = 64778;
pub const CLUTTER_KEY_3270_PA2: c_int = 64779;
pub const CLUTTER_KEY_3270_PA3: c_int = 64780;
pub const CLUTTER_KEY_3270_Play: c_int = 64790;
pub const CLUTTER_KEY_3270_PrintScreen: c_int = 64797;
pub const CLUTTER_KEY_3270_Quit: c_int = 64777;
pub const CLUTTER_KEY_3270_Record: c_int = 64792;
pub const CLUTTER_KEY_3270_Reset: c_int = 64776;
pub const CLUTTER_KEY_3270_Right2: c_int = 64771;
pub const CLUTTER_KEY_3270_Rule: c_int = 64788;
pub const CLUTTER_KEY_3270_Setup: c_int = 64791;
pub const CLUTTER_KEY_3270_Test: c_int = 64781;
pub const CLUTTER_KEY_4: c_int = 52;
pub const CLUTTER_KEY_5: c_int = 53;
pub const CLUTTER_KEY_6: c_int = 54;
pub const CLUTTER_KEY_7: c_int = 55;
pub const CLUTTER_KEY_8: c_int = 56;
pub const CLUTTER_KEY_9: c_int = 57;
pub const CLUTTER_KEY_A: c_int = 65;
pub const CLUTTER_KEY_AE: c_int = 198;
pub const CLUTTER_KEY_Aacute: c_int = 193;
pub const CLUTTER_KEY_Abelowdot: c_int = 16785056;
pub const CLUTTER_KEY_Abreve: c_int = 451;
pub const CLUTTER_KEY_Abreveacute: c_int = 16785070;
pub const CLUTTER_KEY_Abrevebelowdot: c_int = 16785078;
pub const CLUTTER_KEY_Abrevegrave: c_int = 16785072;
pub const CLUTTER_KEY_Abrevehook: c_int = 16785074;
pub const CLUTTER_KEY_Abrevetilde: c_int = 16785076;
pub const CLUTTER_KEY_AccessX_Enable: c_int = 65136;
pub const CLUTTER_KEY_AccessX_Feedback_Enable: c_int = 65137;
pub const CLUTTER_KEY_Acircumflex: c_int = 194;
pub const CLUTTER_KEY_Acircumflexacute: c_int = 16785060;
pub const CLUTTER_KEY_Acircumflexbelowdot: c_int = 16785068;
pub const CLUTTER_KEY_Acircumflexgrave: c_int = 16785062;
pub const CLUTTER_KEY_Acircumflexhook: c_int = 16785064;
pub const CLUTTER_KEY_Acircumflextilde: c_int = 16785066;
pub const CLUTTER_KEY_AddFavorite: c_int = 269025081;
pub const CLUTTER_KEY_Adiaeresis: c_int = 196;
pub const CLUTTER_KEY_Agrave: c_int = 192;
pub const CLUTTER_KEY_Ahook: c_int = 16785058;
pub const CLUTTER_KEY_Alt_L: c_int = 65513;
pub const CLUTTER_KEY_Alt_R: c_int = 65514;
pub const CLUTTER_KEY_Amacron: c_int = 960;
pub const CLUTTER_KEY_Aogonek: c_int = 417;
pub const CLUTTER_KEY_ApplicationLeft: c_int = 269025104;
pub const CLUTTER_KEY_ApplicationRight: c_int = 269025105;
pub const CLUTTER_KEY_Arabic_0: c_int = 16778848;
pub const CLUTTER_KEY_Arabic_1: c_int = 16778849;
pub const CLUTTER_KEY_Arabic_2: c_int = 16778850;
pub const CLUTTER_KEY_Arabic_3: c_int = 16778851;
pub const CLUTTER_KEY_Arabic_4: c_int = 16778852;
pub const CLUTTER_KEY_Arabic_5: c_int = 16778853;
pub const CLUTTER_KEY_Arabic_6: c_int = 16778854;
pub const CLUTTER_KEY_Arabic_7: c_int = 16778855;
pub const CLUTTER_KEY_Arabic_8: c_int = 16778856;
pub const CLUTTER_KEY_Arabic_9: c_int = 16778857;
pub const CLUTTER_KEY_Arabic_ain: c_int = 1497;
pub const CLUTTER_KEY_Arabic_alef: c_int = 1479;
pub const CLUTTER_KEY_Arabic_alefmaksura: c_int = 1513;
pub const CLUTTER_KEY_Arabic_beh: c_int = 1480;
pub const CLUTTER_KEY_Arabic_comma: c_int = 1452;
pub const CLUTTER_KEY_Arabic_dad: c_int = 1494;
pub const CLUTTER_KEY_Arabic_dal: c_int = 1487;
pub const CLUTTER_KEY_Arabic_damma: c_int = 1519;
pub const CLUTTER_KEY_Arabic_dammatan: c_int = 1516;
pub const CLUTTER_KEY_Arabic_ddal: c_int = 16778888;
pub const CLUTTER_KEY_Arabic_farsi_yeh: c_int = 16778956;
pub const CLUTTER_KEY_Arabic_fatha: c_int = 1518;
pub const CLUTTER_KEY_Arabic_fathatan: c_int = 1515;
pub const CLUTTER_KEY_Arabic_feh: c_int = 1505;
pub const CLUTTER_KEY_Arabic_fullstop: c_int = 16778964;
pub const CLUTTER_KEY_Arabic_gaf: c_int = 16778927;
pub const CLUTTER_KEY_Arabic_ghain: c_int = 1498;
pub const CLUTTER_KEY_Arabic_ha: c_int = 1511;
pub const CLUTTER_KEY_Arabic_hah: c_int = 1485;
pub const CLUTTER_KEY_Arabic_hamza: c_int = 1473;
pub const CLUTTER_KEY_Arabic_hamza_above: c_int = 16778836;
pub const CLUTTER_KEY_Arabic_hamza_below: c_int = 16778837;
pub const CLUTTER_KEY_Arabic_hamzaonalef: c_int = 1475;
pub const CLUTTER_KEY_Arabic_hamzaonwaw: c_int = 1476;
pub const CLUTTER_KEY_Arabic_hamzaonyeh: c_int = 1478;
pub const CLUTTER_KEY_Arabic_hamzaunderalef: c_int = 1477;
pub const CLUTTER_KEY_Arabic_heh: c_int = 1511;
pub const CLUTTER_KEY_Arabic_heh_doachashmee: c_int = 16778942;
pub const CLUTTER_KEY_Arabic_heh_goal: c_int = 16778945;
pub const CLUTTER_KEY_Arabic_jeem: c_int = 1484;
pub const CLUTTER_KEY_Arabic_jeh: c_int = 16778904;
pub const CLUTTER_KEY_Arabic_kaf: c_int = 1507;
pub const CLUTTER_KEY_Arabic_kasra: c_int = 1520;
pub const CLUTTER_KEY_Arabic_kasratan: c_int = 1517;
pub const CLUTTER_KEY_Arabic_keheh: c_int = 16778921;
pub const CLUTTER_KEY_Arabic_khah: c_int = 1486;
pub const CLUTTER_KEY_Arabic_lam: c_int = 1508;
pub const CLUTTER_KEY_Arabic_madda_above: c_int = 16778835;
pub const CLUTTER_KEY_Arabic_maddaonalef: c_int = 1474;
pub const CLUTTER_KEY_Arabic_meem: c_int = 1509;
pub const CLUTTER_KEY_Arabic_noon: c_int = 1510;
pub const CLUTTER_KEY_Arabic_noon_ghunna: c_int = 16778938;
pub const CLUTTER_KEY_Arabic_peh: c_int = 16778878;
pub const CLUTTER_KEY_Arabic_percent: c_int = 16778858;
pub const CLUTTER_KEY_Arabic_qaf: c_int = 1506;
pub const CLUTTER_KEY_Arabic_question_mark: c_int = 1471;
pub const CLUTTER_KEY_Arabic_ra: c_int = 1489;
pub const CLUTTER_KEY_Arabic_rreh: c_int = 16778897;
pub const CLUTTER_KEY_Arabic_sad: c_int = 1493;
pub const CLUTTER_KEY_Arabic_seen: c_int = 1491;
pub const CLUTTER_KEY_Arabic_semicolon: c_int = 1467;
pub const CLUTTER_KEY_Arabic_shadda: c_int = 1521;
pub const CLUTTER_KEY_Arabic_sheen: c_int = 1492;
pub const CLUTTER_KEY_Arabic_sukun: c_int = 1522;
pub const CLUTTER_KEY_Arabic_superscript_alef: c_int = 16778864;
pub const CLUTTER_KEY_Arabic_switch: c_int = 65406;
pub const CLUTTER_KEY_Arabic_tah: c_int = 1495;
pub const CLUTTER_KEY_Arabic_tatweel: c_int = 1504;
pub const CLUTTER_KEY_Arabic_tcheh: c_int = 16778886;
pub const CLUTTER_KEY_Arabic_teh: c_int = 1482;
pub const CLUTTER_KEY_Arabic_tehmarbuta: c_int = 1481;
pub const CLUTTER_KEY_Arabic_thal: c_int = 1488;
pub const CLUTTER_KEY_Arabic_theh: c_int = 1483;
pub const CLUTTER_KEY_Arabic_tteh: c_int = 16778873;
pub const CLUTTER_KEY_Arabic_veh: c_int = 16778916;
pub const CLUTTER_KEY_Arabic_waw: c_int = 1512;
pub const CLUTTER_KEY_Arabic_yeh: c_int = 1514;
pub const CLUTTER_KEY_Arabic_yeh_baree: c_int = 16778962;
pub const CLUTTER_KEY_Arabic_zah: c_int = 1496;
pub const CLUTTER_KEY_Arabic_zain: c_int = 1490;
pub const CLUTTER_KEY_Aring: c_int = 197;
pub const CLUTTER_KEY_Armenian_AT: c_int = 16778552;
pub const CLUTTER_KEY_Armenian_AYB: c_int = 16778545;
pub const CLUTTER_KEY_Armenian_BEN: c_int = 16778546;
pub const CLUTTER_KEY_Armenian_CHA: c_int = 16778569;
pub const CLUTTER_KEY_Armenian_DA: c_int = 16778548;
pub const CLUTTER_KEY_Armenian_DZA: c_int = 16778561;
pub const CLUTTER_KEY_Armenian_E: c_int = 16778551;
pub const CLUTTER_KEY_Armenian_FE: c_int = 16778582;
pub const CLUTTER_KEY_Armenian_GHAT: c_int = 16778562;
pub const CLUTTER_KEY_Armenian_GIM: c_int = 16778547;
pub const CLUTTER_KEY_Armenian_HI: c_int = 16778565;
pub const CLUTTER_KEY_Armenian_HO: c_int = 16778560;
pub const CLUTTER_KEY_Armenian_INI: c_int = 16778555;
pub const CLUTTER_KEY_Armenian_JE: c_int = 16778571;
pub const CLUTTER_KEY_Armenian_KE: c_int = 16778580;
pub const CLUTTER_KEY_Armenian_KEN: c_int = 16778559;
pub const CLUTTER_KEY_Armenian_KHE: c_int = 16778557;
pub const CLUTTER_KEY_Armenian_LYUN: c_int = 16778556;
pub const CLUTTER_KEY_Armenian_MEN: c_int = 16778564;
pub const CLUTTER_KEY_Armenian_NU: c_int = 16778566;
pub const CLUTTER_KEY_Armenian_O: c_int = 16778581;
pub const CLUTTER_KEY_Armenian_PE: c_int = 16778570;
pub const CLUTTER_KEY_Armenian_PYUR: c_int = 16778579;
pub const CLUTTER_KEY_Armenian_RA: c_int = 16778572;
pub const CLUTTER_KEY_Armenian_RE: c_int = 16778576;
pub const CLUTTER_KEY_Armenian_SE: c_int = 16778573;
pub const CLUTTER_KEY_Armenian_SHA: c_int = 16778567;
pub const CLUTTER_KEY_Armenian_TCHE: c_int = 16778563;
pub const CLUTTER_KEY_Armenian_TO: c_int = 16778553;
pub const CLUTTER_KEY_Armenian_TSA: c_int = 16778558;
pub const CLUTTER_KEY_Armenian_TSO: c_int = 16778577;
pub const CLUTTER_KEY_Armenian_TYUN: c_int = 16778575;
pub const CLUTTER_KEY_Armenian_VEV: c_int = 16778574;
pub const CLUTTER_KEY_Armenian_VO: c_int = 16778568;
pub const CLUTTER_KEY_Armenian_VYUN: c_int = 16778578;
pub const CLUTTER_KEY_Armenian_YECH: c_int = 16778549;
pub const CLUTTER_KEY_Armenian_ZA: c_int = 16778550;
pub const CLUTTER_KEY_Armenian_ZHE: c_int = 16778554;
pub const CLUTTER_KEY_Armenian_accent: c_int = 16778587;
pub const CLUTTER_KEY_Armenian_amanak: c_int = 16778588;
pub const CLUTTER_KEY_Armenian_apostrophe: c_int = 16778586;
pub const CLUTTER_KEY_Armenian_at: c_int = 16778600;
pub const CLUTTER_KEY_Armenian_ayb: c_int = 16778593;
pub const CLUTTER_KEY_Armenian_ben: c_int = 16778594;
pub const CLUTTER_KEY_Armenian_but: c_int = 16778589;
pub const CLUTTER_KEY_Armenian_cha: c_int = 16778617;
pub const CLUTTER_KEY_Armenian_da: c_int = 16778596;
pub const CLUTTER_KEY_Armenian_dza: c_int = 16778609;
pub const CLUTTER_KEY_Armenian_e: c_int = 16778599;
pub const CLUTTER_KEY_Armenian_exclam: c_int = 16778588;
pub const CLUTTER_KEY_Armenian_fe: c_int = 16778630;
pub const CLUTTER_KEY_Armenian_full_stop: c_int = 16778633;
pub const CLUTTER_KEY_Armenian_ghat: c_int = 16778610;
pub const CLUTTER_KEY_Armenian_gim: c_int = 16778595;
pub const CLUTTER_KEY_Armenian_hi: c_int = 16778613;
pub const CLUTTER_KEY_Armenian_ho: c_int = 16778608;
pub const CLUTTER_KEY_Armenian_hyphen: c_int = 16778634;
pub const CLUTTER_KEY_Armenian_ini: c_int = 16778603;
pub const CLUTTER_KEY_Armenian_je: c_int = 16778619;
pub const CLUTTER_KEY_Armenian_ke: c_int = 16778628;
pub const CLUTTER_KEY_Armenian_ken: c_int = 16778607;
pub const CLUTTER_KEY_Armenian_khe: c_int = 16778605;
pub const CLUTTER_KEY_Armenian_ligature_ew: c_int = 16778631;
pub const CLUTTER_KEY_Armenian_lyun: c_int = 16778604;
pub const CLUTTER_KEY_Armenian_men: c_int = 16778612;
pub const CLUTTER_KEY_Armenian_nu: c_int = 16778614;
pub const CLUTTER_KEY_Armenian_o: c_int = 16778629;
pub const CLUTTER_KEY_Armenian_paruyk: c_int = 16778590;
pub const CLUTTER_KEY_Armenian_pe: c_int = 16778618;
pub const CLUTTER_KEY_Armenian_pyur: c_int = 16778627;
pub const CLUTTER_KEY_Armenian_question: c_int = 16778590;
pub const CLUTTER_KEY_Armenian_ra: c_int = 16778620;
pub const CLUTTER_KEY_Armenian_re: c_int = 16778624;
pub const CLUTTER_KEY_Armenian_se: c_int = 16778621;
pub const CLUTTER_KEY_Armenian_separation_mark: c_int = 16778589;
pub const CLUTTER_KEY_Armenian_sha: c_int = 16778615;
pub const CLUTTER_KEY_Armenian_shesht: c_int = 16778587;
pub const CLUTTER_KEY_Armenian_tche: c_int = 16778611;
pub const CLUTTER_KEY_Armenian_to: c_int = 16778601;
pub const CLUTTER_KEY_Armenian_tsa: c_int = 16778606;
pub const CLUTTER_KEY_Armenian_tso: c_int = 16778625;
pub const CLUTTER_KEY_Armenian_tyun: c_int = 16778623;
pub const CLUTTER_KEY_Armenian_verjaket: c_int = 16778633;
pub const CLUTTER_KEY_Armenian_vev: c_int = 16778622;
pub const CLUTTER_KEY_Armenian_vo: c_int = 16778616;
pub const CLUTTER_KEY_Armenian_vyun: c_int = 16778626;
pub const CLUTTER_KEY_Armenian_yech: c_int = 16778597;
pub const CLUTTER_KEY_Armenian_yentamna: c_int = 16778634;
pub const CLUTTER_KEY_Armenian_za: c_int = 16778598;
pub const CLUTTER_KEY_Armenian_zhe: c_int = 16778602;
pub const CLUTTER_KEY_Atilde: c_int = 195;
pub const CLUTTER_KEY_AudibleBell_Enable: c_int = 65146;
pub const CLUTTER_KEY_AudioCycleTrack: c_int = 269025179;
pub const CLUTTER_KEY_AudioForward: c_int = 269025175;
pub const CLUTTER_KEY_AudioLowerVolume: c_int = 269025041;
pub const CLUTTER_KEY_AudioMedia: c_int = 269025074;
pub const CLUTTER_KEY_AudioMicMute: c_int = 269025202;
pub const CLUTTER_KEY_AudioMute: c_int = 269025042;
pub const CLUTTER_KEY_AudioNext: c_int = 269025047;
pub const CLUTTER_KEY_AudioPause: c_int = 269025073;
pub const CLUTTER_KEY_AudioPlay: c_int = 269025044;
pub const CLUTTER_KEY_AudioPrev: c_int = 269025046;
pub const CLUTTER_KEY_AudioRaiseVolume: c_int = 269025043;
pub const CLUTTER_KEY_AudioRandomPlay: c_int = 269025177;
pub const CLUTTER_KEY_AudioRecord: c_int = 269025052;
pub const CLUTTER_KEY_AudioRepeat: c_int = 269025176;
pub const CLUTTER_KEY_AudioRewind: c_int = 269025086;
pub const CLUTTER_KEY_AudioStop: c_int = 269025045;
pub const CLUTTER_KEY_Away: c_int = 269025165;
pub const CLUTTER_KEY_B: c_int = 66;
pub const CLUTTER_KEY_Babovedot: c_int = 16784898;
pub const CLUTTER_KEY_Back: c_int = 269025062;
pub const CLUTTER_KEY_BackForward: c_int = 269025087;
pub const CLUTTER_KEY_BackSpace: c_int = 65288;
pub const CLUTTER_KEY_Battery: c_int = 269025171;
pub const CLUTTER_KEY_Begin: c_int = 65368;
pub const CLUTTER_KEY_Blue: c_int = 269025190;
pub const CLUTTER_KEY_Bluetooth: c_int = 269025172;
pub const CLUTTER_KEY_Book: c_int = 269025106;
pub const CLUTTER_KEY_BounceKeys_Enable: c_int = 65140;
pub const CLUTTER_KEY_Break: c_int = 65387;
pub const CLUTTER_KEY_BrightnessAdjust: c_int = 269025083;
pub const CLUTTER_KEY_Byelorussian_SHORTU: c_int = 1726;
pub const CLUTTER_KEY_Byelorussian_shortu: c_int = 1710;
pub const CLUTTER_KEY_C: c_int = 67;
pub const CLUTTER_KEY_CD: c_int = 269025107;
pub const CLUTTER_KEY_CH: c_int = 65186;
pub const CLUTTER_KEY_C_H: c_int = 65189;
pub const CLUTTER_KEY_C_h: c_int = 65188;
pub const CLUTTER_KEY_Cabovedot: c_int = 709;
pub const CLUTTER_KEY_Cacute: c_int = 454;
pub const CLUTTER_KEY_Calculator: c_int = 269025053;
pub const CLUTTER_KEY_Calendar: c_int = 269025056;
pub const CLUTTER_KEY_Cancel: c_int = 65385;
pub const CLUTTER_KEY_Caps_Lock: c_int = 65509;
pub const CLUTTER_KEY_Ccaron: c_int = 456;
pub const CLUTTER_KEY_Ccedilla: c_int = 199;
pub const CLUTTER_KEY_Ccircumflex: c_int = 710;
pub const CLUTTER_KEY_Ch: c_int = 65185;
pub const CLUTTER_KEY_Clear: c_int = 65291;
pub const CLUTTER_KEY_ClearGrab: c_int = 269024801;
pub const CLUTTER_KEY_Close: c_int = 269025110;
pub const CLUTTER_KEY_Codeinput: c_int = 65335;
pub const CLUTTER_KEY_ColonSign: c_int = 16785569;
pub const CLUTTER_KEY_Community: c_int = 269025085;
pub const CLUTTER_KEY_ContrastAdjust: c_int = 269025058;
pub const CLUTTER_KEY_Control_L: c_int = 65507;
pub const CLUTTER_KEY_Control_R: c_int = 65508;
pub const CLUTTER_KEY_Copy: c_int = 269025111;
pub const CLUTTER_KEY_CruzeiroSign: c_int = 16785570;
pub const CLUTTER_KEY_Cut: c_int = 269025112;
pub const CLUTTER_KEY_CycleAngle: c_int = 269025180;
pub const CLUTTER_KEY_Cyrillic_A: c_int = 1761;
pub const CLUTTER_KEY_Cyrillic_BE: c_int = 1762;
pub const CLUTTER_KEY_Cyrillic_CHE: c_int = 1790;
pub const CLUTTER_KEY_Cyrillic_CHE_descender: c_int = 16778422;
pub const CLUTTER_KEY_Cyrillic_CHE_vertstroke: c_int = 16778424;
pub const CLUTTER_KEY_Cyrillic_DE: c_int = 1764;
pub const CLUTTER_KEY_Cyrillic_DZHE: c_int = 1727;
pub const CLUTTER_KEY_Cyrillic_E: c_int = 1788;
pub const CLUTTER_KEY_Cyrillic_EF: c_int = 1766;
pub const CLUTTER_KEY_Cyrillic_EL: c_int = 1772;
pub const CLUTTER_KEY_Cyrillic_EM: c_int = 1773;
pub const CLUTTER_KEY_Cyrillic_EN: c_int = 1774;
pub const CLUTTER_KEY_Cyrillic_EN_descender: c_int = 16778402;
pub const CLUTTER_KEY_Cyrillic_ER: c_int = 1778;
pub const CLUTTER_KEY_Cyrillic_ES: c_int = 1779;
pub const CLUTTER_KEY_Cyrillic_GHE: c_int = 1767;
pub const CLUTTER_KEY_Cyrillic_GHE_bar: c_int = 16778386;
pub const CLUTTER_KEY_Cyrillic_HA: c_int = 1768;
pub const CLUTTER_KEY_Cyrillic_HARDSIGN: c_int = 1791;
pub const CLUTTER_KEY_Cyrillic_HA_descender: c_int = 16778418;
pub const CLUTTER_KEY_Cyrillic_I: c_int = 1769;
pub const CLUTTER_KEY_Cyrillic_IE: c_int = 1765;
pub const CLUTTER_KEY_Cyrillic_IO: c_int = 1715;
pub const CLUTTER_KEY_Cyrillic_I_macron: c_int = 16778466;
pub const CLUTTER_KEY_Cyrillic_JE: c_int = 1720;
pub const CLUTTER_KEY_Cyrillic_KA: c_int = 1771;
pub const CLUTTER_KEY_Cyrillic_KA_descender: c_int = 16778394;
pub const CLUTTER_KEY_Cyrillic_KA_vertstroke: c_int = 16778396;
pub const CLUTTER_KEY_Cyrillic_LJE: c_int = 1721;
pub const CLUTTER_KEY_Cyrillic_NJE: c_int = 1722;
pub const CLUTTER_KEY_Cyrillic_O: c_int = 1775;
pub const CLUTTER_KEY_Cyrillic_O_bar: c_int = 16778472;
pub const CLUTTER_KEY_Cyrillic_PE: c_int = 1776;
pub const CLUTTER_KEY_Cyrillic_SCHWA: c_int = 16778456;
pub const CLUTTER_KEY_Cyrillic_SHA: c_int = 1787;
pub const CLUTTER_KEY_Cyrillic_SHCHA: c_int = 1789;
pub const CLUTTER_KEY_Cyrillic_SHHA: c_int = 16778426;
pub const CLUTTER_KEY_Cyrillic_SHORTI: c_int = 1770;
pub const CLUTTER_KEY_Cyrillic_SOFTSIGN: c_int = 1784;
pub const CLUTTER_KEY_Cyrillic_TE: c_int = 1780;
pub const CLUTTER_KEY_Cyrillic_TSE: c_int = 1763;
pub const CLUTTER_KEY_Cyrillic_U: c_int = 1781;
pub const CLUTTER_KEY_Cyrillic_U_macron: c_int = 16778478;
pub const CLUTTER_KEY_Cyrillic_U_straight: c_int = 16778414;
pub const CLUTTER_KEY_Cyrillic_U_straight_bar: c_int = 16778416;
pub const CLUTTER_KEY_Cyrillic_VE: c_int = 1783;
pub const CLUTTER_KEY_Cyrillic_YA: c_int = 1777;
pub const CLUTTER_KEY_Cyrillic_YERU: c_int = 1785;
pub const CLUTTER_KEY_Cyrillic_YU: c_int = 1760;
pub const CLUTTER_KEY_Cyrillic_ZE: c_int = 1786;
pub const CLUTTER_KEY_Cyrillic_ZHE: c_int = 1782;
pub const CLUTTER_KEY_Cyrillic_ZHE_descender: c_int = 16778390;
pub const CLUTTER_KEY_Cyrillic_a: c_int = 1729;
pub const CLUTTER_KEY_Cyrillic_be: c_int = 1730;
pub const CLUTTER_KEY_Cyrillic_che: c_int = 1758;
pub const CLUTTER_KEY_Cyrillic_che_descender: c_int = 16778423;
pub const CLUTTER_KEY_Cyrillic_che_vertstroke: c_int = 16778425;
pub const CLUTTER_KEY_Cyrillic_de: c_int = 1732;
pub const CLUTTER_KEY_Cyrillic_dzhe: c_int = 1711;
pub const CLUTTER_KEY_Cyrillic_e: c_int = 1756;
pub const CLUTTER_KEY_Cyrillic_ef: c_int = 1734;
pub const CLUTTER_KEY_Cyrillic_el: c_int = 1740;
pub const CLUTTER_KEY_Cyrillic_em: c_int = 1741;
pub const CLUTTER_KEY_Cyrillic_en: c_int = 1742;
pub const CLUTTER_KEY_Cyrillic_en_descender: c_int = 16778403;
pub const CLUTTER_KEY_Cyrillic_er: c_int = 1746;
pub const CLUTTER_KEY_Cyrillic_es: c_int = 1747;
pub const CLUTTER_KEY_Cyrillic_ghe: c_int = 1735;
pub const CLUTTER_KEY_Cyrillic_ghe_bar: c_int = 16778387;
pub const CLUTTER_KEY_Cyrillic_ha: c_int = 1736;
pub const CLUTTER_KEY_Cyrillic_ha_descender: c_int = 16778419;
pub const CLUTTER_KEY_Cyrillic_hardsign: c_int = 1759;
pub const CLUTTER_KEY_Cyrillic_i: c_int = 1737;
pub const CLUTTER_KEY_Cyrillic_i_macron: c_int = 16778467;
pub const CLUTTER_KEY_Cyrillic_ie: c_int = 1733;
pub const CLUTTER_KEY_Cyrillic_io: c_int = 1699;
pub const CLUTTER_KEY_Cyrillic_je: c_int = 1704;
pub const CLUTTER_KEY_Cyrillic_ka: c_int = 1739;
pub const CLUTTER_KEY_Cyrillic_ka_descender: c_int = 16778395;
pub const CLUTTER_KEY_Cyrillic_ka_vertstroke: c_int = 16778397;
pub const CLUTTER_KEY_Cyrillic_lje: c_int = 1705;
pub const CLUTTER_KEY_Cyrillic_nje: c_int = 1706;
pub const CLUTTER_KEY_Cyrillic_o: c_int = 1743;
pub const CLUTTER_KEY_Cyrillic_o_bar: c_int = 16778473;
pub const CLUTTER_KEY_Cyrillic_pe: c_int = 1744;
pub const CLUTTER_KEY_Cyrillic_schwa: c_int = 16778457;
pub const CLUTTER_KEY_Cyrillic_sha: c_int = 1755;
pub const CLUTTER_KEY_Cyrillic_shcha: c_int = 1757;
pub const CLUTTER_KEY_Cyrillic_shha: c_int = 16778427;
pub const CLUTTER_KEY_Cyrillic_shorti: c_int = 1738;
pub const CLUTTER_KEY_Cyrillic_softsign: c_int = 1752;
pub const CLUTTER_KEY_Cyrillic_te: c_int = 1748;
pub const CLUTTER_KEY_Cyrillic_tse: c_int = 1731;
pub const CLUTTER_KEY_Cyrillic_u: c_int = 1749;
pub const CLUTTER_KEY_Cyrillic_u_macron: c_int = 16778479;
pub const CLUTTER_KEY_Cyrillic_u_straight: c_int = 16778415;
pub const CLUTTER_KEY_Cyrillic_u_straight_bar: c_int = 16778417;
pub const CLUTTER_KEY_Cyrillic_ve: c_int = 1751;
pub const CLUTTER_KEY_Cyrillic_ya: c_int = 1745;
pub const CLUTTER_KEY_Cyrillic_yeru: c_int = 1753;
pub const CLUTTER_KEY_Cyrillic_yu: c_int = 1728;
pub const CLUTTER_KEY_Cyrillic_ze: c_int = 1754;
pub const CLUTTER_KEY_Cyrillic_zhe: c_int = 1750;
pub const CLUTTER_KEY_Cyrillic_zhe_descender: c_int = 16778391;
pub const CLUTTER_KEY_D: c_int = 68;
pub const CLUTTER_KEY_DOS: c_int = 269025114;
pub const CLUTTER_KEY_Dabovedot: c_int = 16784906;
pub const CLUTTER_KEY_Dcaron: c_int = 463;
pub const CLUTTER_KEY_Delete: c_int = 65535;
pub const CLUTTER_KEY_Display: c_int = 269025113;
pub const CLUTTER_KEY_Documents: c_int = 269025115;
pub const CLUTTER_KEY_DongSign: c_int = 16785579;
pub const CLUTTER_KEY_Down: c_int = 65364;
pub const CLUTTER_KEY_Dstroke: c_int = 464;
pub const CLUTTER_KEY_E: c_int = 69;
pub const CLUTTER_KEY_ENG: c_int = 957;
pub const CLUTTER_KEY_ETH: c_int = 208;
pub const CLUTTER_KEY_EZH: c_int = 16777655;
pub const CLUTTER_KEY_Eabovedot: c_int = 972;
pub const CLUTTER_KEY_Eacute: c_int = 201;
pub const CLUTTER_KEY_Ebelowdot: c_int = 16785080;
pub const CLUTTER_KEY_Ecaron: c_int = 460;
pub const CLUTTER_KEY_Ecircumflex: c_int = 202;
pub const CLUTTER_KEY_Ecircumflexacute: c_int = 16785086;
pub const CLUTTER_KEY_Ecircumflexbelowdot: c_int = 16785094;
pub const CLUTTER_KEY_Ecircumflexgrave: c_int = 16785088;
pub const CLUTTER_KEY_Ecircumflexhook: c_int = 16785090;
pub const CLUTTER_KEY_Ecircumflextilde: c_int = 16785092;
pub const CLUTTER_KEY_EcuSign: c_int = 16785568;
pub const CLUTTER_KEY_Ediaeresis: c_int = 203;
pub const CLUTTER_KEY_Egrave: c_int = 200;
pub const CLUTTER_KEY_Ehook: c_int = 16785082;
pub const CLUTTER_KEY_Eisu_Shift: c_int = 65327;
pub const CLUTTER_KEY_Eisu_toggle: c_int = 65328;
pub const CLUTTER_KEY_Eject: c_int = 269025068;
pub const CLUTTER_KEY_Emacron: c_int = 938;
pub const CLUTTER_KEY_End: c_int = 65367;
pub const CLUTTER_KEY_Eogonek: c_int = 458;
pub const CLUTTER_KEY_Escape: c_int = 65307;
pub const CLUTTER_KEY_Eth: c_int = 208;
pub const CLUTTER_KEY_Etilde: c_int = 16785084;
pub const CLUTTER_KEY_EuroSign: c_int = 8364;
pub const CLUTTER_KEY_Excel: c_int = 269025116;
pub const CLUTTER_KEY_Execute: c_int = 65378;
pub const CLUTTER_KEY_Explorer: c_int = 269025117;
pub const CLUTTER_KEY_F: c_int = 70;
pub const CLUTTER_KEY_F1: c_int = 65470;
pub const CLUTTER_KEY_F10: c_int = 65479;
pub const CLUTTER_KEY_F11: c_int = 65480;
pub const CLUTTER_KEY_F12: c_int = 65481;
pub const CLUTTER_KEY_F13: c_int = 65482;
pub const CLUTTER_KEY_F14: c_int = 65483;
pub const CLUTTER_KEY_F15: c_int = 65484;
pub const CLUTTER_KEY_F16: c_int = 65485;
pub const CLUTTER_KEY_F17: c_int = 65486;
pub const CLUTTER_KEY_F18: c_int = 65487;
pub const CLUTTER_KEY_F19: c_int = 65488;
pub const CLUTTER_KEY_F2: c_int = 65471;
pub const CLUTTER_KEY_F20: c_int = 65489;
pub const CLUTTER_KEY_F21: c_int = 65490;
pub const CLUTTER_KEY_F22: c_int = 65491;
pub const CLUTTER_KEY_F23: c_int = 65492;
pub const CLUTTER_KEY_F24: c_int = 65493;
pub const CLUTTER_KEY_F25: c_int = 65494;
pub const CLUTTER_KEY_F26: c_int = 65495;
pub const CLUTTER_KEY_F27: c_int = 65496;
pub const CLUTTER_KEY_F28: c_int = 65497;
pub const CLUTTER_KEY_F29: c_int = 65498;
pub const CLUTTER_KEY_F3: c_int = 65472;
pub const CLUTTER_KEY_F30: c_int = 65499;
pub const CLUTTER_KEY_F31: c_int = 65500;
pub const CLUTTER_KEY_F32: c_int = 65501;
pub const CLUTTER_KEY_F33: c_int = 65502;
pub const CLUTTER_KEY_F34: c_int = 65503;
pub const CLUTTER_KEY_F35: c_int = 65504;
pub const CLUTTER_KEY_F4: c_int = 65473;
pub const CLUTTER_KEY_F5: c_int = 65474;
pub const CLUTTER_KEY_F6: c_int = 65475;
pub const CLUTTER_KEY_F7: c_int = 65476;
pub const CLUTTER_KEY_F8: c_int = 65477;
pub const CLUTTER_KEY_F9: c_int = 65478;
pub const CLUTTER_KEY_FFrancSign: c_int = 16785571;
pub const CLUTTER_KEY_Fabovedot: c_int = 16784926;
pub const CLUTTER_KEY_Farsi_0: c_int = 16778992;
pub const CLUTTER_KEY_Farsi_1: c_int = 16778993;
pub const CLUTTER_KEY_Farsi_2: c_int = 16778994;
pub const CLUTTER_KEY_Farsi_3: c_int = 16778995;
pub const CLUTTER_KEY_Farsi_4: c_int = 16778996;
pub const CLUTTER_KEY_Farsi_5: c_int = 16778997;
pub const CLUTTER_KEY_Farsi_6: c_int = 16778998;
pub const CLUTTER_KEY_Farsi_7: c_int = 16778999;
pub const CLUTTER_KEY_Farsi_8: c_int = 16779000;
pub const CLUTTER_KEY_Farsi_9: c_int = 16779001;
pub const CLUTTER_KEY_Farsi_yeh: c_int = 16778956;
pub const CLUTTER_KEY_Favorites: c_int = 269025072;
pub const CLUTTER_KEY_Finance: c_int = 269025084;
pub const CLUTTER_KEY_Find: c_int = 65384;
pub const CLUTTER_KEY_First_Virtual_Screen: c_int = 65232;
pub const CLUTTER_KEY_Forward: c_int = 269025063;
pub const CLUTTER_KEY_FrameBack: c_int = 269025181;
pub const CLUTTER_KEY_FrameForward: c_int = 269025182;
pub const CLUTTER_KEY_G: c_int = 71;
pub const CLUTTER_KEY_Gabovedot: c_int = 725;
pub const CLUTTER_KEY_Game: c_int = 269025118;
pub const CLUTTER_KEY_Gbreve: c_int = 683;
pub const CLUTTER_KEY_Gcaron: c_int = 16777702;
pub const CLUTTER_KEY_Gcedilla: c_int = 939;
pub const CLUTTER_KEY_Gcircumflex: c_int = 728;
pub const CLUTTER_KEY_Georgian_an: c_int = 16781520;
pub const CLUTTER_KEY_Georgian_ban: c_int = 16781521;
pub const CLUTTER_KEY_Georgian_can: c_int = 16781546;
pub const CLUTTER_KEY_Georgian_char: c_int = 16781549;
pub const CLUTTER_KEY_Georgian_chin: c_int = 16781545;
pub const CLUTTER_KEY_Georgian_cil: c_int = 16781548;
pub const CLUTTER_KEY_Georgian_don: c_int = 16781523;
pub const CLUTTER_KEY_Georgian_en: c_int = 16781524;
pub const CLUTTER_KEY_Georgian_fi: c_int = 16781558;
pub const CLUTTER_KEY_Georgian_gan: c_int = 16781522;
pub const CLUTTER_KEY_Georgian_ghan: c_int = 16781542;
pub const CLUTTER_KEY_Georgian_hae: c_int = 16781552;
pub const CLUTTER_KEY_Georgian_har: c_int = 16781556;
pub const CLUTTER_KEY_Georgian_he: c_int = 16781553;
pub const CLUTTER_KEY_Georgian_hie: c_int = 16781554;
pub const CLUTTER_KEY_Georgian_hoe: c_int = 16781557;
pub const CLUTTER_KEY_Georgian_in: c_int = 16781528;
pub const CLUTTER_KEY_Georgian_jhan: c_int = 16781551;
pub const CLUTTER_KEY_Georgian_jil: c_int = 16781547;
pub const CLUTTER_KEY_Georgian_kan: c_int = 16781529;
pub const CLUTTER_KEY_Georgian_khar: c_int = 16781541;
pub const CLUTTER_KEY_Georgian_las: c_int = 16781530;
pub const CLUTTER_KEY_Georgian_man: c_int = 16781531;
pub const CLUTTER_KEY_Georgian_nar: c_int = 16781532;
pub const CLUTTER_KEY_Georgian_on: c_int = 16781533;
pub const CLUTTER_KEY_Georgian_par: c_int = 16781534;
pub const CLUTTER_KEY_Georgian_phar: c_int = 16781540;
pub const CLUTTER_KEY_Georgian_qar: c_int = 16781543;
pub const CLUTTER_KEY_Georgian_rae: c_int = 16781536;
pub const CLUTTER_KEY_Georgian_san: c_int = 16781537;
pub const CLUTTER_KEY_Georgian_shin: c_int = 16781544;
pub const CLUTTER_KEY_Georgian_tan: c_int = 16781527;
pub const CLUTTER_KEY_Georgian_tar: c_int = 16781538;
pub const CLUTTER_KEY_Georgian_un: c_int = 16781539;
pub const CLUTTER_KEY_Georgian_vin: c_int = 16781525;
pub const CLUTTER_KEY_Georgian_we: c_int = 16781555;
pub const CLUTTER_KEY_Georgian_xan: c_int = 16781550;
pub const CLUTTER_KEY_Georgian_zen: c_int = 16781526;
pub const CLUTTER_KEY_Georgian_zhar: c_int = 16781535;
pub const CLUTTER_KEY_Go: c_int = 269025119;
pub const CLUTTER_KEY_Greek_ALPHA: c_int = 1985;
pub const CLUTTER_KEY_Greek_ALPHAaccent: c_int = 1953;
pub const CLUTTER_KEY_Greek_BETA: c_int = 1986;
pub const CLUTTER_KEY_Greek_CHI: c_int = 2007;
pub const CLUTTER_KEY_Greek_DELTA: c_int = 1988;
pub const CLUTTER_KEY_Greek_EPSILON: c_int = 1989;
pub const CLUTTER_KEY_Greek_EPSILONaccent: c_int = 1954;
pub const CLUTTER_KEY_Greek_ETA: c_int = 1991;
pub const CLUTTER_KEY_Greek_ETAaccent: c_int = 1955;
pub const CLUTTER_KEY_Greek_GAMMA: c_int = 1987;
pub const CLUTTER_KEY_Greek_IOTA: c_int = 1993;
pub const CLUTTER_KEY_Greek_IOTAaccent: c_int = 1956;
pub const CLUTTER_KEY_Greek_IOTAdiaeresis: c_int = 1957;
pub const CLUTTER_KEY_Greek_IOTAdieresis: c_int = 1957;
pub const CLUTTER_KEY_Greek_KAPPA: c_int = 1994;
pub const CLUTTER_KEY_Greek_LAMBDA: c_int = 1995;
pub const CLUTTER_KEY_Greek_LAMDA: c_int = 1995;
pub const CLUTTER_KEY_Greek_MU: c_int = 1996;
pub const CLUTTER_KEY_Greek_NU: c_int = 1997;
pub const CLUTTER_KEY_Greek_OMEGA: c_int = 2009;
pub const CLUTTER_KEY_Greek_OMEGAaccent: c_int = 1963;
pub const CLUTTER_KEY_Greek_OMICRON: c_int = 1999;
pub const CLUTTER_KEY_Greek_OMICRONaccent: c_int = 1959;
pub const CLUTTER_KEY_Greek_PHI: c_int = 2006;
pub const CLUTTER_KEY_Greek_PI: c_int = 2000;
pub const CLUTTER_KEY_Greek_PSI: c_int = 2008;
pub const CLUTTER_KEY_Greek_RHO: c_int = 2001;
pub const CLUTTER_KEY_Greek_SIGMA: c_int = 2002;
pub const CLUTTER_KEY_Greek_TAU: c_int = 2004;
pub const CLUTTER_KEY_Greek_THETA: c_int = 1992;
pub const CLUTTER_KEY_Greek_UPSILON: c_int = 2005;
pub const CLUTTER_KEY_Greek_UPSILONaccent: c_int = 1960;
pub const CLUTTER_KEY_Greek_UPSILONdieresis: c_int = 1961;
pub const CLUTTER_KEY_Greek_XI: c_int = 1998;
pub const CLUTTER_KEY_Greek_ZETA: c_int = 1990;
pub const CLUTTER_KEY_Greek_accentdieresis: c_int = 1966;
pub const CLUTTER_KEY_Greek_alpha: c_int = 2017;
pub const CLUTTER_KEY_Greek_alphaaccent: c_int = 1969;
pub const CLUTTER_KEY_Greek_beta: c_int = 2018;
pub const CLUTTER_KEY_Greek_chi: c_int = 2039;
pub const CLUTTER_KEY_Greek_delta: c_int = 2020;
pub const CLUTTER_KEY_Greek_epsilon: c_int = 2021;
pub const CLUTTER_KEY_Greek_epsilonaccent: c_int = 1970;
pub const CLUTTER_KEY_Greek_eta: c_int = 2023;
pub const CLUTTER_KEY_Greek_etaaccent: c_int = 1971;
pub const CLUTTER_KEY_Greek_finalsmallsigma: c_int = 2035;
pub const CLUTTER_KEY_Greek_gamma: c_int = 2019;
pub const CLUTTER_KEY_Greek_horizbar: c_int = 1967;
pub const CLUTTER_KEY_Greek_iota: c_int = 2025;
pub const CLUTTER_KEY_Greek_iotaaccent: c_int = 1972;
pub const CLUTTER_KEY_Greek_iotaaccentdieresis: c_int = 1974;
pub const CLUTTER_KEY_Greek_iotadieresis: c_int = 1973;
pub const CLUTTER_KEY_Greek_kappa: c_int = 2026;
pub const CLUTTER_KEY_Greek_lambda: c_int = 2027;
pub const CLUTTER_KEY_Greek_lamda: c_int = 2027;
pub const CLUTTER_KEY_Greek_mu: c_int = 2028;
pub const CLUTTER_KEY_Greek_nu: c_int = 2029;
pub const CLUTTER_KEY_Greek_omega: c_int = 2041;
pub const CLUTTER_KEY_Greek_omegaaccent: c_int = 1979;
pub const CLUTTER_KEY_Greek_omicron: c_int = 2031;
pub const CLUTTER_KEY_Greek_omicronaccent: c_int = 1975;
pub const CLUTTER_KEY_Greek_phi: c_int = 2038;
pub const CLUTTER_KEY_Greek_pi: c_int = 2032;
pub const CLUTTER_KEY_Greek_psi: c_int = 2040;
pub const CLUTTER_KEY_Greek_rho: c_int = 2033;
pub const CLUTTER_KEY_Greek_sigma: c_int = 2034;
pub const CLUTTER_KEY_Greek_switch: c_int = 65406;
pub const CLUTTER_KEY_Greek_tau: c_int = 2036;
pub const CLUTTER_KEY_Greek_theta: c_int = 2024;
pub const CLUTTER_KEY_Greek_upsilon: c_int = 2037;
pub const CLUTTER_KEY_Greek_upsilonaccent: c_int = 1976;
pub const CLUTTER_KEY_Greek_upsilonaccentdieresis: c_int = 1978;
pub const CLUTTER_KEY_Greek_upsilondieresis: c_int = 1977;
pub const CLUTTER_KEY_Greek_xi: c_int = 2030;
pub const CLUTTER_KEY_Greek_zeta: c_int = 2022;
pub const CLUTTER_KEY_Green: c_int = 269025188;
pub const CLUTTER_KEY_H: c_int = 72;
pub const CLUTTER_KEY_Hangul: c_int = 65329;
pub const CLUTTER_KEY_Hangul_A: c_int = 3775;
pub const CLUTTER_KEY_Hangul_AE: c_int = 3776;
pub const CLUTTER_KEY_Hangul_AraeA: c_int = 3830;
pub const CLUTTER_KEY_Hangul_AraeAE: c_int = 3831;
pub const CLUTTER_KEY_Hangul_Banja: c_int = 65337;
pub const CLUTTER_KEY_Hangul_Cieuc: c_int = 3770;
pub const CLUTTER_KEY_Hangul_Codeinput: c_int = 65335;
pub const CLUTTER_KEY_Hangul_Dikeud: c_int = 3751;
pub const CLUTTER_KEY_Hangul_E: c_int = 3780;
pub const CLUTTER_KEY_Hangul_EO: c_int = 3779;
pub const CLUTTER_KEY_Hangul_EU: c_int = 3793;
pub const CLUTTER_KEY_Hangul_End: c_int = 65331;
pub const CLUTTER_KEY_Hangul_Hanja: c_int = 65332;
pub const CLUTTER_KEY_Hangul_Hieuh: c_int = 3774;
pub const CLUTTER_KEY_Hangul_I: c_int = 3795;
pub const CLUTTER_KEY_Hangul_Ieung: c_int = 3767;
pub const CLUTTER_KEY_Hangul_J_Cieuc: c_int = 3818;
pub const CLUTTER_KEY_Hangul_J_Dikeud: c_int = 3802;
pub const CLUTTER_KEY_Hangul_J_Hieuh: c_int = 3822;
pub const CLUTTER_KEY_Hangul_J_Ieung: c_int = 3816;
pub const CLUTTER_KEY_Hangul_J_Jieuj: c_int = 3817;
pub const CLUTTER_KEY_Hangul_J_Khieuq: c_int = 3819;
pub const CLUTTER_KEY_Hangul_J_Kiyeog: c_int = 3796;
pub const CLUTTER_KEY_Hangul_J_KiyeogSios: c_int = 3798;
pub const CLUTTER_KEY_Hangul_J_KkogjiDalrinIeung: c_int = 3833;
pub const CLUTTER_KEY_Hangul_J_Mieum: c_int = 3811;
pub const CLUTTER_KEY_Hangul_J_Nieun: c_int = 3799;
pub const CLUTTER_KEY_Hangul_J_NieunHieuh: c_int = 3801;
pub const CLUTTER_KEY_Hangul_J_NieunJieuj: c_int = 3800;
pub const CLUTTER_KEY_Hangul_J_PanSios: c_int = 3832;
pub const CLUTTER_KEY_Hangul_J_Phieuf: c_int = 3821;
pub const CLUTTER_KEY_Hangul_J_Pieub: c_int = 3812;
pub const CLUTTER_KEY_Hangul_J_PieubSios: c_int = 3813;
pub const CLUTTER_KEY_Hangul_J_Rieul: c_int = 3803;
pub const CLUTTER_KEY_Hangul_J_RieulHieuh: c_int = 3810;
pub const CLUTTER_KEY_Hangul_J_RieulKiyeog: c_int = 3804;
pub const CLUTTER_KEY_Hangul_J_RieulMieum: c_int = 3805;
pub const CLUTTER_KEY_Hangul_J_RieulPhieuf: c_int = 3809;
pub const CLUTTER_KEY_Hangul_J_RieulPieub: c_int = 3806;
pub const CLUTTER_KEY_Hangul_J_RieulSios: c_int = 3807;
pub const CLUTTER_KEY_Hangul_J_RieulTieut: c_int = 3808;
pub const CLUTTER_KEY_Hangul_J_Sios: c_int = 3814;
pub const CLUTTER_KEY_Hangul_J_SsangKiyeog: c_int = 3797;
pub const CLUTTER_KEY_Hangul_J_SsangSios: c_int = 3815;
pub const CLUTTER_KEY_Hangul_J_Tieut: c_int = 3820;
pub const CLUTTER_KEY_Hangul_J_YeorinHieuh: c_int = 3834;
pub const CLUTTER_KEY_Hangul_Jamo: c_int = 65333;
pub const CLUTTER_KEY_Hangul_Jeonja: c_int = 65336;
pub const CLUTTER_KEY_Hangul_Jieuj: c_int = 3768;
pub const CLUTTER_KEY_Hangul_Khieuq: c_int = 3771;
pub const CLUTTER_KEY_Hangul_Kiyeog: c_int = 3745;
pub const CLUTTER_KEY_Hangul_KiyeogSios: c_int = 3747;
pub const CLUTTER_KEY_Hangul_KkogjiDalrinIeung: c_int = 3827;
pub const CLUTTER_KEY_Hangul_Mieum: c_int = 3761;
pub const CLUTTER_KEY_Hangul_MultipleCandidate: c_int = 65341;
pub const CLUTTER_KEY_Hangul_Nieun: c_int = 3748;
pub const CLUTTER_KEY_Hangul_NieunHieuh: c_int = 3750;
pub const CLUTTER_KEY_Hangul_NieunJieuj: c_int = 3749;
pub const CLUTTER_KEY_Hangul_O: c_int = 3783;
pub const CLUTTER_KEY_Hangul_OE: c_int = 3786;
pub const CLUTTER_KEY_Hangul_PanSios: c_int = 3826;
pub const CLUTTER_KEY_Hangul_Phieuf: c_int = 3773;
pub const CLUTTER_KEY_Hangul_Pieub: c_int = 3762;
pub const CLUTTER_KEY_Hangul_PieubSios: c_int = 3764;
pub const CLUTTER_KEY_Hangul_PostHanja: c_int = 65339;
pub const CLUTTER_KEY_Hangul_PreHanja: c_int = 65338;
pub const CLUTTER_KEY_Hangul_PreviousCandidate: c_int = 65342;
pub const CLUTTER_KEY_Hangul_Rieul: c_int = 3753;
pub const CLUTTER_KEY_Hangul_RieulHieuh: c_int = 3760;
pub const CLUTTER_KEY_Hangul_RieulKiyeog: c_int = 3754;
pub const CLUTTER_KEY_Hangul_RieulMieum: c_int = 3755;
pub const CLUTTER_KEY_Hangul_RieulPhieuf: c_int = 3759;
pub const CLUTTER_KEY_Hangul_RieulPieub: c_int = 3756;
pub const CLUTTER_KEY_Hangul_RieulSios: c_int = 3757;
pub const CLUTTER_KEY_Hangul_RieulTieut: c_int = 3758;
pub const CLUTTER_KEY_Hangul_RieulYeorinHieuh: c_int = 3823;
pub const CLUTTER_KEY_Hangul_Romaja: c_int = 65334;
pub const CLUTTER_KEY_Hangul_SingleCandidate: c_int = 65340;
pub const CLUTTER_KEY_Hangul_Sios: c_int = 3765;
pub const CLUTTER_KEY_Hangul_Special: c_int = 65343;
pub const CLUTTER_KEY_Hangul_SsangDikeud: c_int = 3752;
pub const CLUTTER_KEY_Hangul_SsangJieuj: c_int = 3769;
pub const CLUTTER_KEY_Hangul_SsangKiyeog: c_int = 3746;
pub const CLUTTER_KEY_Hangul_SsangPieub: c_int = 3763;
pub const CLUTTER_KEY_Hangul_SsangSios: c_int = 3766;
pub const CLUTTER_KEY_Hangul_Start: c_int = 65330;
pub const CLUTTER_KEY_Hangul_SunkyeongeumMieum: c_int = 3824;
pub const CLUTTER_KEY_Hangul_SunkyeongeumPhieuf: c_int = 3828;
pub const CLUTTER_KEY_Hangul_SunkyeongeumPieub: c_int = 3825;
pub const CLUTTER_KEY_Hangul_Tieut: c_int = 3772;
pub const CLUTTER_KEY_Hangul_U: c_int = 3788;
pub const CLUTTER_KEY_Hangul_WA: c_int = 3784;
pub const CLUTTER_KEY_Hangul_WAE: c_int = 3785;
pub const CLUTTER_KEY_Hangul_WE: c_int = 3790;
pub const CLUTTER_KEY_Hangul_WEO: c_int = 3789;
pub const CLUTTER_KEY_Hangul_WI: c_int = 3791;
pub const CLUTTER_KEY_Hangul_YA: c_int = 3777;
pub const CLUTTER_KEY_Hangul_YAE: c_int = 3778;
pub const CLUTTER_KEY_Hangul_YE: c_int = 3782;
pub const CLUTTER_KEY_Hangul_YEO: c_int = 3781;
pub const CLUTTER_KEY_Hangul_YI: c_int = 3794;
pub const CLUTTER_KEY_Hangul_YO: c_int = 3787;
pub const CLUTTER_KEY_Hangul_YU: c_int = 3792;
pub const CLUTTER_KEY_Hangul_YeorinHieuh: c_int = 3829;
pub const CLUTTER_KEY_Hangul_switch: c_int = 65406;
pub const CLUTTER_KEY_Hankaku: c_int = 65321;
pub const CLUTTER_KEY_Hcircumflex: c_int = 678;
pub const CLUTTER_KEY_Hebrew_switch: c_int = 65406;
pub const CLUTTER_KEY_Help: c_int = 65386;
pub const CLUTTER_KEY_Henkan: c_int = 65315;
pub const CLUTTER_KEY_Henkan_Mode: c_int = 65315;
pub const CLUTTER_KEY_Hibernate: c_int = 269025192;
pub const CLUTTER_KEY_Hiragana: c_int = 65317;
pub const CLUTTER_KEY_Hiragana_Katakana: c_int = 65319;
pub const CLUTTER_KEY_History: c_int = 269025079;
pub const CLUTTER_KEY_Home: c_int = 65360;
pub const CLUTTER_KEY_HomePage: c_int = 269025048;
pub const CLUTTER_KEY_HotLinks: c_int = 269025082;
pub const CLUTTER_KEY_Hstroke: c_int = 673;
pub const CLUTTER_KEY_Hyper_L: c_int = 65517;
pub const CLUTTER_KEY_Hyper_R: c_int = 65518;
pub const CLUTTER_KEY_I: c_int = 73;
pub const CLUTTER_KEY_ISO_Center_Object: c_int = 65075;
pub const CLUTTER_KEY_ISO_Continuous_Underline: c_int = 65072;
pub const CLUTTER_KEY_ISO_Discontinuous_Underline: c_int = 65073;
pub const CLUTTER_KEY_ISO_Emphasize: c_int = 65074;
pub const CLUTTER_KEY_ISO_Enter: c_int = 65076;
pub const CLUTTER_KEY_ISO_Fast_Cursor_Down: c_int = 65071;
pub const CLUTTER_KEY_ISO_Fast_Cursor_Left: c_int = 65068;
pub const CLUTTER_KEY_ISO_Fast_Cursor_Right: c_int = 65069;
pub const CLUTTER_KEY_ISO_Fast_Cursor_Up: c_int = 65070;
pub const CLUTTER_KEY_ISO_First_Group: c_int = 65036;
pub const CLUTTER_KEY_ISO_First_Group_Lock: c_int = 65037;
pub const CLUTTER_KEY_ISO_Group_Latch: c_int = 65030;
pub const CLUTTER_KEY_ISO_Group_Lock: c_int = 65031;
pub const CLUTTER_KEY_ISO_Group_Shift: c_int = 65406;
pub const CLUTTER_KEY_ISO_Last_Group: c_int = 65038;
pub const CLUTTER_KEY_ISO_Last_Group_Lock: c_int = 65039;
pub const CLUTTER_KEY_ISO_Left_Tab: c_int = 65056;
pub const CLUTTER_KEY_ISO_Level2_Latch: c_int = 65026;
pub const CLUTTER_KEY_ISO_Level3_Latch: c_int = 65028;
pub const CLUTTER_KEY_ISO_Level3_Lock: c_int = 65029;
pub const CLUTTER_KEY_ISO_Level3_Shift: c_int = 65027;
pub const CLUTTER_KEY_ISO_Level5_Latch: c_int = 65042;
pub const CLUTTER_KEY_ISO_Level5_Lock: c_int = 65043;
pub const CLUTTER_KEY_ISO_Level5_Shift: c_int = 65041;
pub const CLUTTER_KEY_ISO_Lock: c_int = 65025;
pub const CLUTTER_KEY_ISO_Move_Line_Down: c_int = 65058;
pub const CLUTTER_KEY_ISO_Move_Line_Up: c_int = 65057;
pub const CLUTTER_KEY_ISO_Next_Group: c_int = 65032;
pub const CLUTTER_KEY_ISO_Next_Group_Lock: c_int = 65033;
pub const CLUTTER_KEY_ISO_Partial_Line_Down: c_int = 65060;
pub const CLUTTER_KEY_ISO_Partial_Line_Up: c_int = 65059;
pub const CLUTTER_KEY_ISO_Partial_Space_Left: c_int = 65061;
pub const CLUTTER_KEY_ISO_Partial_Space_Right: c_int = 65062;
pub const CLUTTER_KEY_ISO_Prev_Group: c_int = 65034;
pub const CLUTTER_KEY_ISO_Prev_Group_Lock: c_int = 65035;
pub const CLUTTER_KEY_ISO_Release_Both_Margins: c_int = 65067;
pub const CLUTTER_KEY_ISO_Release_Margin_Left: c_int = 65065;
pub const CLUTTER_KEY_ISO_Release_Margin_Right: c_int = 65066;
pub const CLUTTER_KEY_ISO_Set_Margin_Left: c_int = 65063;
pub const CLUTTER_KEY_ISO_Set_Margin_Right: c_int = 65064;
pub const CLUTTER_KEY_Iabovedot: c_int = 681;
pub const CLUTTER_KEY_Iacute: c_int = 205;
pub const CLUTTER_KEY_Ibelowdot: c_int = 16785098;
pub const CLUTTER_KEY_Ibreve: c_int = 16777516;
pub const CLUTTER_KEY_Icircumflex: c_int = 206;
pub const CLUTTER_KEY_Idiaeresis: c_int = 207;
pub const CLUTTER_KEY_Igrave: c_int = 204;
pub const CLUTTER_KEY_Ihook: c_int = 16785096;
pub const CLUTTER_KEY_Imacron: c_int = 975;
pub const CLUTTER_KEY_Insert: c_int = 65379;
pub const CLUTTER_KEY_Iogonek: c_int = 967;
pub const CLUTTER_KEY_Itilde: c_int = 933;
pub const CLUTTER_KEY_J: c_int = 74;
pub const CLUTTER_KEY_Jcircumflex: c_int = 684;
pub const CLUTTER_KEY_K: c_int = 75;
pub const CLUTTER_KEY_KP_0: c_int = 65456;
pub const CLUTTER_KEY_KP_1: c_int = 65457;
pub const CLUTTER_KEY_KP_2: c_int = 65458;
pub const CLUTTER_KEY_KP_3: c_int = 65459;
pub const CLUTTER_KEY_KP_4: c_int = 65460;
pub const CLUTTER_KEY_KP_5: c_int = 65461;
pub const CLUTTER_KEY_KP_6: c_int = 65462;
pub const CLUTTER_KEY_KP_7: c_int = 65463;
pub const CLUTTER_KEY_KP_8: c_int = 65464;
pub const CLUTTER_KEY_KP_9: c_int = 65465;
pub const CLUTTER_KEY_KP_Add: c_int = 65451;
pub const CLUTTER_KEY_KP_Begin: c_int = 65437;
pub const CLUTTER_KEY_KP_Decimal: c_int = 65454;
pub const CLUTTER_KEY_KP_Delete: c_int = 65439;
pub const CLUTTER_KEY_KP_Divide: c_int = 65455;
pub const CLUTTER_KEY_KP_Down: c_int = 65433;
pub const CLUTTER_KEY_KP_End: c_int = 65436;
pub const CLUTTER_KEY_KP_Enter: c_int = 65421;
pub const CLUTTER_KEY_KP_Equal: c_int = 65469;
pub const CLUTTER_KEY_KP_F1: c_int = 65425;
pub const CLUTTER_KEY_KP_F2: c_int = 65426;
pub const CLUTTER_KEY_KP_F3: c_int = 65427;
pub const CLUTTER_KEY_KP_F4: c_int = 65428;
pub const CLUTTER_KEY_KP_Home: c_int = 65429;
pub const CLUTTER_KEY_KP_Insert: c_int = 65438;
pub const CLUTTER_KEY_KP_Left: c_int = 65430;
pub const CLUTTER_KEY_KP_Multiply: c_int = 65450;
pub const CLUTTER_KEY_KP_Next: c_int = 65435;
pub const CLUTTER_KEY_KP_Page_Down: c_int = 65435;
pub const CLUTTER_KEY_KP_Page_Up: c_int = 65434;
pub const CLUTTER_KEY_KP_Prior: c_int = 65434;
pub const CLUTTER_KEY_KP_Right: c_int = 65432;
pub const CLUTTER_KEY_KP_Separator: c_int = 65452;
pub const CLUTTER_KEY_KP_Space: c_int = 65408;
pub const CLUTTER_KEY_KP_Subtract: c_int = 65453;
pub const CLUTTER_KEY_KP_Tab: c_int = 65417;
pub const CLUTTER_KEY_KP_Up: c_int = 65431;
pub const CLUTTER_KEY_Kana_Lock: c_int = 65325;
pub const CLUTTER_KEY_Kana_Shift: c_int = 65326;
pub const CLUTTER_KEY_Kanji: c_int = 65313;
pub const CLUTTER_KEY_Kanji_Bangou: c_int = 65335;
pub const CLUTTER_KEY_Katakana: c_int = 65318;
pub const CLUTTER_KEY_KbdBrightnessDown: c_int = 269025030;
pub const CLUTTER_KEY_KbdBrightnessUp: c_int = 269025029;
pub const CLUTTER_KEY_KbdLightOnOff: c_int = 269025028;
pub const CLUTTER_KEY_Kcedilla: c_int = 979;
pub const CLUTTER_KEY_Korean_Won: c_int = 3839;
pub const CLUTTER_KEY_L: c_int = 76;
pub const CLUTTER_KEY_L1: c_int = 65480;
pub const CLUTTER_KEY_L10: c_int = 65489;
pub const CLUTTER_KEY_L2: c_int = 65481;
pub const CLUTTER_KEY_L3: c_int = 65482;
pub const CLUTTER_KEY_L4: c_int = 65483;
pub const CLUTTER_KEY_L5: c_int = 65484;
pub const CLUTTER_KEY_L6: c_int = 65485;
pub const CLUTTER_KEY_L7: c_int = 65486;
pub const CLUTTER_KEY_L8: c_int = 65487;
pub const CLUTTER_KEY_L9: c_int = 65488;
pub const CLUTTER_KEY_Lacute: c_int = 453;
pub const CLUTTER_KEY_Last_Virtual_Screen: c_int = 65236;
pub const CLUTTER_KEY_Launch0: c_int = 269025088;
pub const CLUTTER_KEY_Launch1: c_int = 269025089;
pub const CLUTTER_KEY_Launch2: c_int = 269025090;
pub const CLUTTER_KEY_Launch3: c_int = 269025091;
pub const CLUTTER_KEY_Launch4: c_int = 269025092;
pub const CLUTTER_KEY_Launch5: c_int = 269025093;
pub const CLUTTER_KEY_Launch6: c_int = 269025094;
pub const CLUTTER_KEY_Launch7: c_int = 269025095;
pub const CLUTTER_KEY_Launch8: c_int = 269025096;
pub const CLUTTER_KEY_Launch9: c_int = 269025097;
pub const CLUTTER_KEY_LaunchA: c_int = 269025098;
pub const CLUTTER_KEY_LaunchB: c_int = 269025099;
pub const CLUTTER_KEY_LaunchC: c_int = 269025100;
pub const CLUTTER_KEY_LaunchD: c_int = 269025101;
pub const CLUTTER_KEY_LaunchE: c_int = 269025102;
pub const CLUTTER_KEY_LaunchF: c_int = 269025103;
pub const CLUTTER_KEY_Lbelowdot: c_int = 16784950;
pub const CLUTTER_KEY_Lcaron: c_int = 421;
pub const CLUTTER_KEY_Lcedilla: c_int = 934;
pub const CLUTTER_KEY_Left: c_int = 65361;
pub const CLUTTER_KEY_LightBulb: c_int = 269025077;
pub const CLUTTER_KEY_Linefeed: c_int = 65290;
pub const CLUTTER_KEY_LiraSign: c_int = 16785572;
pub const CLUTTER_KEY_LogGrabInfo: c_int = 269024805;
pub const CLUTTER_KEY_LogOff: c_int = 269025121;
pub const CLUTTER_KEY_LogWindowTree: c_int = 269024804;
pub const CLUTTER_KEY_Lstroke: c_int = 419;
pub const CLUTTER_KEY_M: c_int = 77;
pub const CLUTTER_KEY_Mabovedot: c_int = 16784960;
pub const CLUTTER_KEY_Macedonia_DSE: c_int = 1717;
pub const CLUTTER_KEY_Macedonia_GJE: c_int = 1714;
pub const CLUTTER_KEY_Macedonia_KJE: c_int = 1724;
pub const CLUTTER_KEY_Macedonia_dse: c_int = 1701;
pub const CLUTTER_KEY_Macedonia_gje: c_int = 1698;
pub const CLUTTER_KEY_Macedonia_kje: c_int = 1708;
pub const CLUTTER_KEY_Mae_Koho: c_int = 65342;
pub const CLUTTER_KEY_Mail: c_int = 269025049;
pub const CLUTTER_KEY_MailForward: c_int = 269025168;
pub const CLUTTER_KEY_Market: c_int = 269025122;
pub const CLUTTER_KEY_Massyo: c_int = 65324;
pub const CLUTTER_KEY_Meeting: c_int = 269025123;
pub const CLUTTER_KEY_Memo: c_int = 269025054;
pub const CLUTTER_KEY_Menu: c_int = 65383;
pub const CLUTTER_KEY_MenuKB: c_int = 269025125;
pub const CLUTTER_KEY_MenuPB: c_int = 269025126;
pub const CLUTTER_KEY_Messenger: c_int = 269025166;
pub const CLUTTER_KEY_Meta_L: c_int = 65511;
pub const CLUTTER_KEY_Meta_R: c_int = 65512;
pub const CLUTTER_KEY_MillSign: c_int = 16785573;
pub const CLUTTER_KEY_ModeLock: c_int = 269025025;
pub const CLUTTER_KEY_Mode_switch: c_int = 65406;
pub const CLUTTER_KEY_MonBrightnessDown: c_int = 269025027;
pub const CLUTTER_KEY_MonBrightnessUp: c_int = 269025026;
pub const CLUTTER_KEY_MouseKeys_Accel_Enable: c_int = 65143;
pub const CLUTTER_KEY_MouseKeys_Enable: c_int = 65142;
pub const CLUTTER_KEY_Muhenkan: c_int = 65314;
pub const CLUTTER_KEY_Multi_key: c_int = 65312;
pub const CLUTTER_KEY_MultipleCandidate: c_int = 65341;
pub const CLUTTER_KEY_Music: c_int = 269025170;
pub const CLUTTER_KEY_MyComputer: c_int = 269025075;
pub const CLUTTER_KEY_MySites: c_int = 269025127;
pub const CLUTTER_KEY_N: c_int = 78;
pub const CLUTTER_KEY_Nacute: c_int = 465;
pub const CLUTTER_KEY_NairaSign: c_int = 16785574;
pub const CLUTTER_KEY_Ncaron: c_int = 466;
pub const CLUTTER_KEY_Ncedilla: c_int = 977;
pub const CLUTTER_KEY_New: c_int = 269025128;
pub const CLUTTER_KEY_NewSheqelSign: c_int = 16785578;
pub const CLUTTER_KEY_News: c_int = 269025129;
pub const CLUTTER_KEY_Next: c_int = 65366;
pub const CLUTTER_KEY_Next_VMode: c_int = 269024802;
pub const CLUTTER_KEY_Next_Virtual_Screen: c_int = 65234;
pub const CLUTTER_KEY_Ntilde: c_int = 209;
pub const CLUTTER_KEY_Num_Lock: c_int = 65407;
pub const CLUTTER_KEY_O: c_int = 79;
pub const CLUTTER_KEY_OE: c_int = 5052;
pub const CLUTTER_KEY_Oacute: c_int = 211;
pub const CLUTTER_KEY_Obarred: c_int = 16777631;
pub const CLUTTER_KEY_Obelowdot: c_int = 16785100;
pub const CLUTTER_KEY_Ocaron: c_int = 16777681;
pub const CLUTTER_KEY_Ocircumflex: c_int = 212;
pub const CLUTTER_KEY_Ocircumflexacute: c_int = 16785104;
pub const CLUTTER_KEY_Ocircumflexbelowdot: c_int = 16785112;
pub const CLUTTER_KEY_Ocircumflexgrave: c_int = 16785106;
pub const CLUTTER_KEY_Ocircumflexhook: c_int = 16785108;
pub const CLUTTER_KEY_Ocircumflextilde: c_int = 16785110;
pub const CLUTTER_KEY_Odiaeresis: c_int = 214;
pub const CLUTTER_KEY_Odoubleacute: c_int = 469;
pub const CLUTTER_KEY_OfficeHome: c_int = 269025130;
pub const CLUTTER_KEY_Ograve: c_int = 210;
pub const CLUTTER_KEY_Ohook: c_int = 16785102;
pub const CLUTTER_KEY_Ohorn: c_int = 16777632;
pub const CLUTTER_KEY_Ohornacute: c_int = 16785114;
pub const CLUTTER_KEY_Ohornbelowdot: c_int = 16785122;
pub const CLUTTER_KEY_Ohorngrave: c_int = 16785116;
pub const CLUTTER_KEY_Ohornhook: c_int = 16785118;
pub const CLUTTER_KEY_Ohorntilde: c_int = 16785120;
pub const CLUTTER_KEY_Omacron: c_int = 978;
pub const CLUTTER_KEY_Ooblique: c_int = 216;
pub const CLUTTER_KEY_Open: c_int = 269025131;
pub const CLUTTER_KEY_OpenURL: c_int = 269025080;
pub const CLUTTER_KEY_Option: c_int = 269025132;
pub const CLUTTER_KEY_Oslash: c_int = 216;
pub const CLUTTER_KEY_Otilde: c_int = 213;
pub const CLUTTER_KEY_Overlay1_Enable: c_int = 65144;
pub const CLUTTER_KEY_Overlay2_Enable: c_int = 65145;
pub const CLUTTER_KEY_P: c_int = 80;
pub const CLUTTER_KEY_Pabovedot: c_int = 16784982;
pub const CLUTTER_KEY_Page_Down: c_int = 65366;
pub const CLUTTER_KEY_Page_Up: c_int = 65365;
pub const CLUTTER_KEY_Paste: c_int = 269025133;
pub const CLUTTER_KEY_Pause: c_int = 65299;
pub const CLUTTER_KEY_PesetaSign: c_int = 16785575;
pub const CLUTTER_KEY_Phone: c_int = 269025134;
pub const CLUTTER_KEY_Pictures: c_int = 269025169;
pub const CLUTTER_KEY_Pointer_Accelerate: c_int = 65274;
pub const CLUTTER_KEY_Pointer_Button1: c_int = 65257;
pub const CLUTTER_KEY_Pointer_Button2: c_int = 65258;
pub const CLUTTER_KEY_Pointer_Button3: c_int = 65259;
pub const CLUTTER_KEY_Pointer_Button4: c_int = 65260;
pub const CLUTTER_KEY_Pointer_Button5: c_int = 65261;
pub const CLUTTER_KEY_Pointer_Button_Dflt: c_int = 65256;
pub const CLUTTER_KEY_Pointer_DblClick1: c_int = 65263;
pub const CLUTTER_KEY_Pointer_DblClick2: c_int = 65264;
pub const CLUTTER_KEY_Pointer_DblClick3: c_int = 65265;
pub const CLUTTER_KEY_Pointer_DblClick4: c_int = 65266;
pub const CLUTTER_KEY_Pointer_DblClick5: c_int = 65267;
pub const CLUTTER_KEY_Pointer_DblClick_Dflt: c_int = 65262;
pub const CLUTTER_KEY_Pointer_DfltBtnNext: c_int = 65275;
pub const CLUTTER_KEY_Pointer_DfltBtnPrev: c_int = 65276;
pub const CLUTTER_KEY_Pointer_Down: c_int = 65251;
pub const CLUTTER_KEY_Pointer_DownLeft: c_int = 65254;
pub const CLUTTER_KEY_Pointer_DownRight: c_int = 65255;
pub const CLUTTER_KEY_Pointer_Drag1: c_int = 65269;
pub const CLUTTER_KEY_Pointer_Drag2: c_int = 65270;
pub const CLUTTER_KEY_Pointer_Drag3: c_int = 65271;
pub const CLUTTER_KEY_Pointer_Drag4: c_int = 65272;
pub const CLUTTER_KEY_Pointer_Drag5: c_int = 65277;
pub const CLUTTER_KEY_Pointer_Drag_Dflt: c_int = 65268;
pub const CLUTTER_KEY_Pointer_EnableKeys: c_int = 65273;
pub const CLUTTER_KEY_Pointer_Left: c_int = 65248;
pub const CLUTTER_KEY_Pointer_Right: c_int = 65249;
pub const CLUTTER_KEY_Pointer_Up: c_int = 65250;
pub const CLUTTER_KEY_Pointer_UpLeft: c_int = 65252;
pub const CLUTTER_KEY_Pointer_UpRight: c_int = 65253;
pub const CLUTTER_KEY_PowerDown: c_int = 269025057;
pub const CLUTTER_KEY_PowerOff: c_int = 269025066;
pub const CLUTTER_KEY_Prev_VMode: c_int = 269024803;
pub const CLUTTER_KEY_Prev_Virtual_Screen: c_int = 65233;
pub const CLUTTER_KEY_PreviousCandidate: c_int = 65342;
pub const CLUTTER_KEY_Print: c_int = 65377;
pub const CLUTTER_KEY_Prior: c_int = 65365;
pub const CLUTTER_KEY_Q: c_int = 81;
pub const CLUTTER_KEY_R: c_int = 82;
pub const CLUTTER_KEY_R1: c_int = 65490;
pub const CLUTTER_KEY_R10: c_int = 65499;
pub const CLUTTER_KEY_R11: c_int = 65500;
pub const CLUTTER_KEY_R12: c_int = 65501;
pub const CLUTTER_KEY_R13: c_int = 65502;
pub const CLUTTER_KEY_R14: c_int = 65503;
pub const CLUTTER_KEY_R15: c_int = 65504;
pub const CLUTTER_KEY_R2: c_int = 65491;
pub const CLUTTER_KEY_R3: c_int = 65492;
pub const CLUTTER_KEY_R4: c_int = 65493;
pub const CLUTTER_KEY_R5: c_int = 65494;
pub const CLUTTER_KEY_R6: c_int = 65495;
pub const CLUTTER_KEY_R7: c_int = 65496;
pub const CLUTTER_KEY_R8: c_int = 65497;
pub const CLUTTER_KEY_R9: c_int = 65498;
pub const CLUTTER_KEY_Racute: c_int = 448;
pub const CLUTTER_KEY_Rcaron: c_int = 472;
pub const CLUTTER_KEY_Rcedilla: c_int = 931;
pub const CLUTTER_KEY_Red: c_int = 269025187;
pub const CLUTTER_KEY_Redo: c_int = 65382;
pub const CLUTTER_KEY_Refresh: c_int = 269025065;
pub const CLUTTER_KEY_Reload: c_int = 269025139;
pub const CLUTTER_KEY_RepeatKeys_Enable: c_int = 65138;
pub const CLUTTER_KEY_Reply: c_int = 269025138;
pub const CLUTTER_KEY_Return: c_int = 65293;
pub const CLUTTER_KEY_Right: c_int = 65363;
pub const CLUTTER_KEY_RockerDown: c_int = 269025060;
pub const CLUTTER_KEY_RockerEnter: c_int = 269025061;
pub const CLUTTER_KEY_RockerUp: c_int = 269025059;
pub const CLUTTER_KEY_Romaji: c_int = 65316;
pub const CLUTTER_KEY_RotateWindows: c_int = 269025140;
pub const CLUTTER_KEY_RotationKB: c_int = 269025142;
pub const CLUTTER_KEY_RotationPB: c_int = 269025141;
pub const CLUTTER_KEY_RupeeSign: c_int = 16785576;
pub const CLUTTER_KEY_S: c_int = 83;
pub const CLUTTER_KEY_SCHWA: c_int = 16777615;
pub const CLUTTER_KEY_Sabovedot: c_int = 16784992;
pub const CLUTTER_KEY_Sacute: c_int = 422;
pub const CLUTTER_KEY_Save: c_int = 269025143;
pub const CLUTTER_KEY_Scaron: c_int = 425;
pub const CLUTTER_KEY_Scedilla: c_int = 426;
pub const CLUTTER_KEY_Scircumflex: c_int = 734;
pub const CLUTTER_KEY_ScreenSaver: c_int = 269025069;
pub const CLUTTER_KEY_ScrollClick: c_int = 269025146;
pub const CLUTTER_KEY_ScrollDown: c_int = 269025145;
pub const CLUTTER_KEY_ScrollUp: c_int = 269025144;
pub const CLUTTER_KEY_Scroll_Lock: c_int = 65300;
pub const CLUTTER_KEY_Search: c_int = 269025051;
pub const CLUTTER_KEY_Select: c_int = 65376;
pub const CLUTTER_KEY_SelectButton: c_int = 269025184;
pub const CLUTTER_KEY_Send: c_int = 269025147;
pub const CLUTTER_KEY_Serbian_DJE: c_int = 1713;
pub const CLUTTER_KEY_Serbian_DZE: c_int = 1727;
pub const CLUTTER_KEY_Serbian_JE: c_int = 1720;
pub const CLUTTER_KEY_Serbian_LJE: c_int = 1721;
pub const CLUTTER_KEY_Serbian_NJE: c_int = 1722;
pub const CLUTTER_KEY_Serbian_TSHE: c_int = 1723;
pub const CLUTTER_KEY_Serbian_dje: c_int = 1697;
pub const CLUTTER_KEY_Serbian_dze: c_int = 1711;
pub const CLUTTER_KEY_Serbian_je: c_int = 1704;
pub const CLUTTER_KEY_Serbian_lje: c_int = 1705;
pub const CLUTTER_KEY_Serbian_nje: c_int = 1706;
pub const CLUTTER_KEY_Serbian_tshe: c_int = 1707;
pub const CLUTTER_KEY_Shift_L: c_int = 65505;
pub const CLUTTER_KEY_Shift_Lock: c_int = 65510;
pub const CLUTTER_KEY_Shift_R: c_int = 65506;
pub const CLUTTER_KEY_Shop: c_int = 269025078;
pub const CLUTTER_KEY_SingleCandidate: c_int = 65340;
pub const CLUTTER_KEY_Sinh_a: c_int = 16780677;
pub const CLUTTER_KEY_Sinh_aa: c_int = 16780678;
pub const CLUTTER_KEY_Sinh_aa2: c_int = 16780751;
pub const CLUTTER_KEY_Sinh_ae: c_int = 16780679;
pub const CLUTTER_KEY_Sinh_ae2: c_int = 16780752;
pub const CLUTTER_KEY_Sinh_aee: c_int = 16780680;
pub const CLUTTER_KEY_Sinh_aee2: c_int = 16780753;
pub const CLUTTER_KEY_Sinh_ai: c_int = 16780691;
pub const CLUTTER_KEY_Sinh_ai2: c_int = 16780763;
pub const CLUTTER_KEY_Sinh_al: c_int = 16780746;
pub const CLUTTER_KEY_Sinh_au: c_int = 16780694;
pub const CLUTTER_KEY_Sinh_au2: c_int = 16780766;
pub const CLUTTER_KEY_Sinh_ba: c_int = 16780726;
pub const CLUTTER_KEY_Sinh_bha: c_int = 16780727;
pub const CLUTTER_KEY_Sinh_ca: c_int = 16780704;
pub const CLUTTER_KEY_Sinh_cha: c_int = 16780705;
pub const CLUTTER_KEY_Sinh_dda: c_int = 16780713;
pub const CLUTTER_KEY_Sinh_ddha: c_int = 16780714;
pub const CLUTTER_KEY_Sinh_dha: c_int = 16780719;
pub const CLUTTER_KEY_Sinh_dhha: c_int = 16780720;
pub const CLUTTER_KEY_Sinh_e: c_int = 16780689;
pub const CLUTTER_KEY_Sinh_e2: c_int = 16780761;
pub const CLUTTER_KEY_Sinh_ee: c_int = 16780690;
pub const CLUTTER_KEY_Sinh_ee2: c_int = 16780762;
pub const CLUTTER_KEY_Sinh_fa: c_int = 16780742;
pub const CLUTTER_KEY_Sinh_ga: c_int = 16780700;
pub const CLUTTER_KEY_Sinh_gha: c_int = 16780701;
pub const CLUTTER_KEY_Sinh_h2: c_int = 16780675;
pub const CLUTTER_KEY_Sinh_ha: c_int = 16780740;
pub const CLUTTER_KEY_Sinh_i: c_int = 16780681;
pub const CLUTTER_KEY_Sinh_i2: c_int = 16780754;
pub const CLUTTER_KEY_Sinh_ii: c_int = 16780682;
pub const CLUTTER_KEY_Sinh_ii2: c_int = 16780755;
pub const CLUTTER_KEY_Sinh_ja: c_int = 16780706;
pub const CLUTTER_KEY_Sinh_jha: c_int = 16780707;
pub const CLUTTER_KEY_Sinh_jnya: c_int = 16780709;
pub const CLUTTER_KEY_Sinh_ka: c_int = 16780698;
pub const CLUTTER_KEY_Sinh_kha: c_int = 16780699;
pub const CLUTTER_KEY_Sinh_kunddaliya: c_int = 16780788;
pub const CLUTTER_KEY_Sinh_la: c_int = 16780733;
pub const CLUTTER_KEY_Sinh_lla: c_int = 16780741;
pub const CLUTTER_KEY_Sinh_lu: c_int = 16780687;
pub const CLUTTER_KEY_Sinh_lu2: c_int = 16780767;
pub const CLUTTER_KEY_Sinh_luu: c_int = 16780688;
pub const CLUTTER_KEY_Sinh_luu2: c_int = 16780787;
pub const CLUTTER_KEY_Sinh_ma: c_int = 16780728;
pub const CLUTTER_KEY_Sinh_mba: c_int = 16780729;
pub const CLUTTER_KEY_Sinh_na: c_int = 16780721;
pub const CLUTTER_KEY_Sinh_ndda: c_int = 16780716;
pub const CLUTTER_KEY_Sinh_ndha: c_int = 16780723;
pub const CLUTTER_KEY_Sinh_ng: c_int = 16780674;
pub const CLUTTER_KEY_Sinh_ng2: c_int = 16780702;
pub const CLUTTER_KEY_Sinh_nga: c_int = 16780703;
pub const CLUTTER_KEY_Sinh_nja: c_int = 16780710;
pub const CLUTTER_KEY_Sinh_nna: c_int = 16780715;
pub const CLUTTER_KEY_Sinh_nya: c_int = 16780708;
pub const CLUTTER_KEY_Sinh_o: c_int = 16780692;
pub const CLUTTER_KEY_Sinh_o2: c_int = 16780764;
pub const CLUTTER_KEY_Sinh_oo: c_int = 16780693;
pub const CLUTTER_KEY_Sinh_oo2: c_int = 16780765;
pub const CLUTTER_KEY_Sinh_pa: c_int = 16780724;
pub const CLUTTER_KEY_Sinh_pha: c_int = 16780725;
pub const CLUTTER_KEY_Sinh_ra: c_int = 16780731;
pub const CLUTTER_KEY_Sinh_ri: c_int = 16780685;
pub const CLUTTER_KEY_Sinh_rii: c_int = 16780686;
pub const CLUTTER_KEY_Sinh_ru2: c_int = 16780760;
pub const CLUTTER_KEY_Sinh_ruu2: c_int = 16780786;
pub const CLUTTER_KEY_Sinh_sa: c_int = 16780739;
pub const CLUTTER_KEY_Sinh_sha: c_int = 16780737;
pub const CLUTTER_KEY_Sinh_ssha: c_int = 16780738;
pub const CLUTTER_KEY_Sinh_tha: c_int = 16780717;
pub const CLUTTER_KEY_Sinh_thha: c_int = 16780718;
pub const CLUTTER_KEY_Sinh_tta: c_int = 16780711;
pub const CLUTTER_KEY_Sinh_ttha: c_int = 16780712;
pub const CLUTTER_KEY_Sinh_u: c_int = 16780683;
pub const CLUTTER_KEY_Sinh_u2: c_int = 16780756;
pub const CLUTTER_KEY_Sinh_uu: c_int = 16780684;
pub const CLUTTER_KEY_Sinh_uu2: c_int = 16780758;
pub const CLUTTER_KEY_Sinh_va: c_int = 16780736;
pub const CLUTTER_KEY_Sinh_ya: c_int = 16780730;
pub const CLUTTER_KEY_Sleep: c_int = 269025071;
pub const CLUTTER_KEY_SlowKeys_Enable: c_int = 65139;
pub const CLUTTER_KEY_Spell: c_int = 269025148;
pub const CLUTTER_KEY_SplitScreen: c_int = 269025149;
pub const CLUTTER_KEY_Standby: c_int = 269025040;
pub const CLUTTER_KEY_Start: c_int = 269025050;
pub const CLUTTER_KEY_StickyKeys_Enable: c_int = 65141;
pub const CLUTTER_KEY_Stop: c_int = 269025064;
pub const CLUTTER_KEY_Subtitle: c_int = 269025178;
pub const CLUTTER_KEY_Super_L: c_int = 65515;
pub const CLUTTER_KEY_Super_R: c_int = 65516;
pub const CLUTTER_KEY_Support: c_int = 269025150;
pub const CLUTTER_KEY_Suspend: c_int = 269025191;
pub const CLUTTER_KEY_Switch_VT_1: c_int = 269024769;
pub const CLUTTER_KEY_Switch_VT_10: c_int = 269024778;
pub const CLUTTER_KEY_Switch_VT_11: c_int = 269024779;
pub const CLUTTER_KEY_Switch_VT_12: c_int = 269024780;
pub const CLUTTER_KEY_Switch_VT_2: c_int = 269024770;
pub const CLUTTER_KEY_Switch_VT_3: c_int = 269024771;
pub const CLUTTER_KEY_Switch_VT_4: c_int = 269024772;
pub const CLUTTER_KEY_Switch_VT_5: c_int = 269024773;
pub const CLUTTER_KEY_Switch_VT_6: c_int = 269024774;
pub const CLUTTER_KEY_Switch_VT_7: c_int = 269024775;
pub const CLUTTER_KEY_Switch_VT_8: c_int = 269024776;
pub const CLUTTER_KEY_Switch_VT_9: c_int = 269024777;
pub const CLUTTER_KEY_Sys_Req: c_int = 65301;
pub const CLUTTER_KEY_T: c_int = 84;
pub const CLUTTER_KEY_THORN: c_int = 222;
pub const CLUTTER_KEY_Tab: c_int = 65289;
pub const CLUTTER_KEY_Tabovedot: c_int = 16785002;
pub const CLUTTER_KEY_TaskPane: c_int = 269025151;
pub const CLUTTER_KEY_Tcaron: c_int = 427;
pub const CLUTTER_KEY_Tcedilla: c_int = 478;
pub const CLUTTER_KEY_Terminal: c_int = 269025152;
pub const CLUTTER_KEY_Terminate_Server: c_int = 65237;
pub const CLUTTER_KEY_Thai_baht: c_int = 3551;
pub const CLUTTER_KEY_Thai_bobaimai: c_int = 3514;
pub const CLUTTER_KEY_Thai_chochan: c_int = 3496;
pub const CLUTTER_KEY_Thai_chochang: c_int = 3498;
pub const CLUTTER_KEY_Thai_choching: c_int = 3497;
pub const CLUTTER_KEY_Thai_chochoe: c_int = 3500;
pub const CLUTTER_KEY_Thai_dochada: c_int = 3502;
pub const CLUTTER_KEY_Thai_dodek: c_int = 3508;
pub const CLUTTER_KEY_Thai_fofa: c_int = 3517;
pub const CLUTTER_KEY_Thai_fofan: c_int = 3519;
pub const CLUTTER_KEY_Thai_hohip: c_int = 3531;
pub const CLUTTER_KEY_Thai_honokhuk: c_int = 3534;
pub const CLUTTER_KEY_Thai_khokhai: c_int = 3490;
pub const CLUTTER_KEY_Thai_khokhon: c_int = 3493;
pub const CLUTTER_KEY_Thai_khokhuat: c_int = 3491;
pub const CLUTTER_KEY_Thai_khokhwai: c_int = 3492;
pub const CLUTTER_KEY_Thai_khorakhang: c_int = 3494;
pub const CLUTTER_KEY_Thai_kokai: c_int = 3489;
pub const CLUTTER_KEY_Thai_lakkhangyao: c_int = 3557;
pub const CLUTTER_KEY_Thai_lekchet: c_int = 3575;
pub const CLUTTER_KEY_Thai_lekha: c_int = 3573;
pub const CLUTTER_KEY_Thai_lekhok: c_int = 3574;
pub const CLUTTER_KEY_Thai_lekkao: c_int = 3577;
pub const CLUTTER_KEY_Thai_leknung: c_int = 3569;
pub const CLUTTER_KEY_Thai_lekpaet: c_int = 3576;
pub const CLUTTER_KEY_Thai_leksam: c_int = 3571;
pub const CLUTTER_KEY_Thai_leksi: c_int = 3572;
pub const CLUTTER_KEY_Thai_leksong: c_int = 3570;
pub const CLUTTER_KEY_Thai_leksun: c_int = 3568;
pub const CLUTTER_KEY_Thai_lochula: c_int = 3532;
pub const CLUTTER_KEY_Thai_loling: c_int = 3525;
pub const CLUTTER_KEY_Thai_lu: c_int = 3526;
pub const CLUTTER_KEY_Thai_maichattawa: c_int = 3563;
pub const CLUTTER_KEY_Thai_maiek: c_int = 3560;
pub const CLUTTER_KEY_Thai_maihanakat: c_int = 3537;
pub const CLUTTER_KEY_Thai_maihanakat_maitho: c_int = 3550;
pub const CLUTTER_KEY_Thai_maitaikhu: c_int = 3559;
pub const CLUTTER_KEY_Thai_maitho: c_int = 3561;
pub const CLUTTER_KEY_Thai_maitri: c_int = 3562;
pub const CLUTTER_KEY_Thai_maiyamok: c_int = 3558;
pub const CLUTTER_KEY_Thai_moma: c_int = 3521;
pub const CLUTTER_KEY_Thai_ngongu: c_int = 3495;
pub const CLUTTER_KEY_Thai_nikhahit: c_int = 3565;
pub const CLUTTER_KEY_Thai_nonen: c_int = 3507;
pub const CLUTTER_KEY_Thai_nonu: c_int = 3513;
pub const CLUTTER_KEY_Thai_oang: c_int = 3533;
pub const CLUTTER_KEY_Thai_paiyannoi: c_int = 3535;
pub const CLUTTER_KEY_Thai_phinthu: c_int = 3546;
pub const CLUTTER_KEY_Thai_phophan: c_int = 3518;
pub const CLUTTER_KEY_Thai_phophung: c_int = 3516;
pub const CLUTTER_KEY_Thai_phosamphao: c_int = 3520;
pub const CLUTTER_KEY_Thai_popla: c_int = 3515;
pub const CLUTTER_KEY_Thai_rorua: c_int = 3523;
pub const CLUTTER_KEY_Thai_ru: c_int = 3524;
pub const CLUTTER_KEY_Thai_saraa: c_int = 3536;
pub const CLUTTER_KEY_Thai_saraaa: c_int = 3538;
pub const CLUTTER_KEY_Thai_saraae: c_int = 3553;
pub const CLUTTER_KEY_Thai_saraaimaimalai: c_int = 3556;
pub const CLUTTER_KEY_Thai_saraaimaimuan: c_int = 3555;
pub const CLUTTER_KEY_Thai_saraam: c_int = 3539;
pub const CLUTTER_KEY_Thai_sarae: c_int = 3552;
pub const CLUTTER_KEY_Thai_sarai: c_int = 3540;
pub const CLUTTER_KEY_Thai_saraii: c_int = 3541;
pub const CLUTTER_KEY_Thai_sarao: c_int = 3554;
pub const CLUTTER_KEY_Thai_sarau: c_int = 3544;
pub const CLUTTER_KEY_Thai_saraue: c_int = 3542;
pub const CLUTTER_KEY_Thai_sarauee: c_int = 3543;
pub const CLUTTER_KEY_Thai_sarauu: c_int = 3545;
pub const CLUTTER_KEY_Thai_sorusi: c_int = 3529;
pub const CLUTTER_KEY_Thai_sosala: c_int = 3528;
pub const CLUTTER_KEY_Thai_soso: c_int = 3499;
pub const CLUTTER_KEY_Thai_sosua: c_int = 3530;
pub const CLUTTER_KEY_Thai_thanthakhat: c_int = 3564;
pub const CLUTTER_KEY_Thai_thonangmontho: c_int = 3505;
pub const CLUTTER_KEY_Thai_thophuthao: c_int = 3506;
pub const CLUTTER_KEY_Thai_thothahan: c_int = 3511;
pub const CLUTTER_KEY_Thai_thothan: c_int = 3504;
pub const CLUTTER_KEY_Thai_thothong: c_int = 3512;
pub const CLUTTER_KEY_Thai_thothung: c_int = 3510;
pub const CLUTTER_KEY_Thai_topatak: c_int = 3503;
pub const CLUTTER_KEY_Thai_totao: c_int = 3509;
pub const CLUTTER_KEY_Thai_wowaen: c_int = 3527;
pub const CLUTTER_KEY_Thai_yoyak: c_int = 3522;
pub const CLUTTER_KEY_Thai_yoying: c_int = 3501;
pub const CLUTTER_KEY_Thorn: c_int = 222;
pub const CLUTTER_KEY_Time: c_int = 269025183;
pub const CLUTTER_KEY_ToDoList: c_int = 269025055;
pub const CLUTTER_KEY_Tools: c_int = 269025153;
pub const CLUTTER_KEY_TopMenu: c_int = 269025186;
pub const CLUTTER_KEY_TouchpadOff: c_int = 269025201;
pub const CLUTTER_KEY_TouchpadOn: c_int = 269025200;
pub const CLUTTER_KEY_TouchpadToggle: c_int = 269025193;
pub const CLUTTER_KEY_Touroku: c_int = 65323;
pub const CLUTTER_KEY_Travel: c_int = 269025154;
pub const CLUTTER_KEY_Tslash: c_int = 940;
pub const CLUTTER_KEY_U: c_int = 85;
pub const CLUTTER_KEY_UWB: c_int = 269025174;
pub const CLUTTER_KEY_Uacute: c_int = 218;
pub const CLUTTER_KEY_Ubelowdot: c_int = 16785124;
pub const CLUTTER_KEY_Ubreve: c_int = 733;
pub const CLUTTER_KEY_Ucircumflex: c_int = 219;
pub const CLUTTER_KEY_Udiaeresis: c_int = 220;
pub const CLUTTER_KEY_Udoubleacute: c_int = 475;
pub const CLUTTER_KEY_Ugrave: c_int = 217;
pub const CLUTTER_KEY_Uhook: c_int = 16785126;
pub const CLUTTER_KEY_Uhorn: c_int = 16777647;
pub const CLUTTER_KEY_Uhornacute: c_int = 16785128;
pub const CLUTTER_KEY_Uhornbelowdot: c_int = 16785136;
pub const CLUTTER_KEY_Uhorngrave: c_int = 16785130;
pub const CLUTTER_KEY_Uhornhook: c_int = 16785132;
pub const CLUTTER_KEY_Uhorntilde: c_int = 16785134;
pub const CLUTTER_KEY_Ukrainian_GHE_WITH_UPTURN: c_int = 1725;
pub const CLUTTER_KEY_Ukrainian_I: c_int = 1718;
pub const CLUTTER_KEY_Ukrainian_IE: c_int = 1716;
pub const CLUTTER_KEY_Ukrainian_YI: c_int = 1719;
pub const CLUTTER_KEY_Ukrainian_ghe_with_upturn: c_int = 1709;
pub const CLUTTER_KEY_Ukrainian_i: c_int = 1702;
pub const CLUTTER_KEY_Ukrainian_ie: c_int = 1700;
pub const CLUTTER_KEY_Ukrainian_yi: c_int = 1703;
pub const CLUTTER_KEY_Ukranian_I: c_int = 1718;
pub const CLUTTER_KEY_Ukranian_JE: c_int = 1716;
pub const CLUTTER_KEY_Ukranian_YI: c_int = 1719;
pub const CLUTTER_KEY_Ukranian_i: c_int = 1702;
pub const CLUTTER_KEY_Ukranian_je: c_int = 1700;
pub const CLUTTER_KEY_Ukranian_yi: c_int = 1703;
pub const CLUTTER_KEY_Umacron: c_int = 990;
pub const CLUTTER_KEY_Undo: c_int = 65381;
pub const CLUTTER_KEY_Ungrab: c_int = 269024800;
pub const CLUTTER_KEY_Uogonek: c_int = 985;
pub const CLUTTER_KEY_Up: c_int = 65362;
pub const CLUTTER_KEY_Uring: c_int = 473;
pub const CLUTTER_KEY_User1KB: c_int = 269025157;
pub const CLUTTER_KEY_User2KB: c_int = 269025158;
pub const CLUTTER_KEY_UserPB: c_int = 269025156;
pub const CLUTTER_KEY_Utilde: c_int = 989;
pub const CLUTTER_KEY_V: c_int = 86;
pub const CLUTTER_KEY_VendorHome: c_int = 269025076;
pub const CLUTTER_KEY_Video: c_int = 269025159;
pub const CLUTTER_KEY_View: c_int = 269025185;
pub const CLUTTER_KEY_VoidSymbol: c_int = 16777215;
pub const CLUTTER_KEY_W: c_int = 87;
pub const CLUTTER_KEY_WLAN: c_int = 269025173;
pub const CLUTTER_KEY_WWW: c_int = 269025070;
pub const CLUTTER_KEY_Wacute: c_int = 16785026;
pub const CLUTTER_KEY_WakeUp: c_int = 269025067;
pub const CLUTTER_KEY_Wcircumflex: c_int = 16777588;
pub const CLUTTER_KEY_Wdiaeresis: c_int = 16785028;
pub const CLUTTER_KEY_WebCam: c_int = 269025167;
pub const CLUTTER_KEY_Wgrave: c_int = 16785024;
pub const CLUTTER_KEY_WheelButton: c_int = 269025160;
pub const CLUTTER_KEY_WindowClear: c_int = 269025109;
pub const CLUTTER_KEY_WonSign: c_int = 16785577;
pub const CLUTTER_KEY_Word: c_int = 269025161;
pub const CLUTTER_KEY_X: c_int = 88;
pub const CLUTTER_KEY_Xabovedot: c_int = 16785034;
pub const CLUTTER_KEY_Xfer: c_int = 269025162;
pub const CLUTTER_KEY_Y: c_int = 89;
pub const CLUTTER_KEY_Yacute: c_int = 221;
pub const CLUTTER_KEY_Ybelowdot: c_int = 16785140;
pub const CLUTTER_KEY_Ycircumflex: c_int = 16777590;
pub const CLUTTER_KEY_Ydiaeresis: c_int = 5054;
pub const CLUTTER_KEY_Yellow: c_int = 269025189;
pub const CLUTTER_KEY_Ygrave: c_int = 16785138;
pub const CLUTTER_KEY_Yhook: c_int = 16785142;
pub const CLUTTER_KEY_Ytilde: c_int = 16785144;
pub const CLUTTER_KEY_Z: c_int = 90;
pub const CLUTTER_KEY_Zabovedot: c_int = 431;
pub const CLUTTER_KEY_Zacute: c_int = 428;
pub const CLUTTER_KEY_Zcaron: c_int = 430;
pub const CLUTTER_KEY_Zen_Koho: c_int = 65341;
pub const CLUTTER_KEY_Zenkaku: c_int = 65320;
pub const CLUTTER_KEY_Zenkaku_Hankaku: c_int = 65322;
pub const CLUTTER_KEY_ZoomIn: c_int = 269025163;
pub const CLUTTER_KEY_ZoomOut: c_int = 269025164;
pub const CLUTTER_KEY_Zstroke: c_int = 16777653;
pub const CLUTTER_KEY_a: c_int = 97;
pub const CLUTTER_KEY_aacute: c_int = 225;
pub const CLUTTER_KEY_abelowdot: c_int = 16785057;
pub const CLUTTER_KEY_abovedot: c_int = 511;
pub const CLUTTER_KEY_abreve: c_int = 483;
pub const CLUTTER_KEY_abreveacute: c_int = 16785071;
pub const CLUTTER_KEY_abrevebelowdot: c_int = 16785079;
pub const CLUTTER_KEY_abrevegrave: c_int = 16785073;
pub const CLUTTER_KEY_abrevehook: c_int = 16785075;
pub const CLUTTER_KEY_abrevetilde: c_int = 16785077;
pub const CLUTTER_KEY_acircumflex: c_int = 226;
pub const CLUTTER_KEY_acircumflexacute: c_int = 16785061;
pub const CLUTTER_KEY_acircumflexbelowdot: c_int = 16785069;
pub const CLUTTER_KEY_acircumflexgrave: c_int = 16785063;
pub const CLUTTER_KEY_acircumflexhook: c_int = 16785065;
pub const CLUTTER_KEY_acircumflextilde: c_int = 16785067;
pub const CLUTTER_KEY_acute: c_int = 180;
pub const CLUTTER_KEY_adiaeresis: c_int = 228;
pub const CLUTTER_KEY_ae: c_int = 230;
pub const CLUTTER_KEY_agrave: c_int = 224;
pub const CLUTTER_KEY_ahook: c_int = 16785059;
pub const CLUTTER_KEY_amacron: c_int = 992;
pub const CLUTTER_KEY_ampersand: c_int = 38;
pub const CLUTTER_KEY_aogonek: c_int = 433;
pub const CLUTTER_KEY_apostrophe: c_int = 39;
pub const CLUTTER_KEY_approxeq: c_int = 16785992;
pub const CLUTTER_KEY_approximate: c_int = 2248;
pub const CLUTTER_KEY_aring: c_int = 229;
pub const CLUTTER_KEY_asciicircum: c_int = 94;
pub const CLUTTER_KEY_asciitilde: c_int = 126;
pub const CLUTTER_KEY_asterisk: c_int = 42;
pub const CLUTTER_KEY_at: c_int = 64;
pub const CLUTTER_KEY_atilde: c_int = 227;
pub const CLUTTER_KEY_b: c_int = 98;
pub const CLUTTER_KEY_babovedot: c_int = 16784899;
pub const CLUTTER_KEY_backslash: c_int = 92;
pub const CLUTTER_KEY_ballotcross: c_int = 2804;
pub const CLUTTER_KEY_bar: c_int = 124;
pub const CLUTTER_KEY_because: c_int = 16785973;
pub const CLUTTER_KEY_blank: c_int = 2527;
pub const CLUTTER_KEY_botintegral: c_int = 2213;
pub const CLUTTER_KEY_botleftparens: c_int = 2220;
pub const CLUTTER_KEY_botleftsqbracket: c_int = 2216;
pub const CLUTTER_KEY_botleftsummation: c_int = 2226;
pub const CLUTTER_KEY_botrightparens: c_int = 2222;
pub const CLUTTER_KEY_botrightsqbracket: c_int = 2218;
pub const CLUTTER_KEY_botrightsummation: c_int = 2230;
pub const CLUTTER_KEY_bott: c_int = 2550;
pub const CLUTTER_KEY_botvertsummationconnector: c_int = 2228;
pub const CLUTTER_KEY_braceleft: c_int = 123;
pub const CLUTTER_KEY_braceright: c_int = 125;
pub const CLUTTER_KEY_bracketleft: c_int = 91;
pub const CLUTTER_KEY_bracketright: c_int = 93;
pub const CLUTTER_KEY_braille_blank: c_int = 16787456;
pub const CLUTTER_KEY_braille_dot_1: c_int = 65521;
pub const CLUTTER_KEY_braille_dot_10: c_int = 65530;
pub const CLUTTER_KEY_braille_dot_2: c_int = 65522;
pub const CLUTTER_KEY_braille_dot_3: c_int = 65523;
pub const CLUTTER_KEY_braille_dot_4: c_int = 65524;
pub const CLUTTER_KEY_braille_dot_5: c_int = 65525;
pub const CLUTTER_KEY_braille_dot_6: c_int = 65526;
pub const CLUTTER_KEY_braille_dot_7: c_int = 65527;
pub const CLUTTER_KEY_braille_dot_8: c_int = 65528;
pub const CLUTTER_KEY_braille_dot_9: c_int = 65529;
pub const CLUTTER_KEY_braille_dots_1: c_int = 16787457;
pub const CLUTTER_KEY_braille_dots_12: c_int = 16787459;
pub const CLUTTER_KEY_braille_dots_123: c_int = 16787463;
pub const CLUTTER_KEY_braille_dots_1234: c_int = 16787471;
pub const CLUTTER_KEY_braille_dots_12345: c_int = 16787487;
pub const CLUTTER_KEY_braille_dots_123456: c_int = 16787519;
pub const CLUTTER_KEY_braille_dots_1234567: c_int = 16787583;
pub const CLUTTER_KEY_braille_dots_12345678: c_int = 16787711;
pub const CLUTTER_KEY_braille_dots_1234568: c_int = 16787647;
pub const CLUTTER_KEY_braille_dots_123457: c_int = 16787551;
pub const CLUTTER_KEY_braille_dots_1234578: c_int = 16787679;
pub const CLUTTER_KEY_braille_dots_123458: c_int = 16787615;
pub const CLUTTER_KEY_braille_dots_12346: c_int = 16787503;
pub const CLUTTER_KEY_braille_dots_123467: c_int = 16787567;
pub const CLUTTER_KEY_braille_dots_1234678: c_int = 16787695;
pub const CLUTTER_KEY_braille_dots_123468: c_int = 16787631;
pub const CLUTTER_KEY_braille_dots_12347: c_int = 16787535;
pub const CLUTTER_KEY_braille_dots_123478: c_int = 16787663;
pub const CLUTTER_KEY_braille_dots_12348: c_int = 16787599;
pub const CLUTTER_KEY_braille_dots_1235: c_int = 16787479;
pub const CLUTTER_KEY_braille_dots_12356: c_int = 16787511;
pub const CLUTTER_KEY_braille_dots_123567: c_int = 16787575;
pub const CLUTTER_KEY_braille_dots_1235678: c_int = 16787703;
pub const CLUTTER_KEY_braille_dots_123568: c_int = 16787639;
pub const CLUTTER_KEY_braille_dots_12357: c_int = 16787543;
pub const CLUTTER_KEY_braille_dots_123578: c_int = 16787671;
pub const CLUTTER_KEY_braille_dots_12358: c_int = 16787607;
pub const CLUTTER_KEY_braille_dots_1236: c_int = 16787495;
pub const CLUTTER_KEY_braille_dots_12367: c_int = 16787559;
pub const CLUTTER_KEY_braille_dots_123678: c_int = 16787687;
pub const CLUTTER_KEY_braille_dots_12368: c_int = 16787623;
pub const CLUTTER_KEY_braille_dots_1237: c_int = 16787527;
pub const CLUTTER_KEY_braille_dots_12378: c_int = 16787655;
pub const CLUTTER_KEY_braille_dots_1238: c_int = 16787591;
pub const CLUTTER_KEY_braille_dots_124: c_int = 16787467;
pub const CLUTTER_KEY_braille_dots_1245: c_int = 16787483;
pub const CLUTTER_KEY_braille_dots_12456: c_int = 16787515;
pub const CLUTTER_KEY_braille_dots_124567: c_int = 16787579;
pub const CLUTTER_KEY_braille_dots_1245678: c_int = 16787707;
pub const CLUTTER_KEY_braille_dots_124568: c_int = 16787643;
pub const CLUTTER_KEY_braille_dots_12457: c_int = 16787547;
pub const CLUTTER_KEY_braille_dots_124578: c_int = 16787675;
pub const CLUTTER_KEY_braille_dots_12458: c_int = 16787611;
pub const CLUTTER_KEY_braille_dots_1246: c_int = 16787499;
pub const CLUTTER_KEY_braille_dots_12467: c_int = 16787563;
pub const CLUTTER_KEY_braille_dots_124678: c_int = 16787691;
pub const CLUTTER_KEY_braille_dots_12468: c_int = 16787627;
pub const CLUTTER_KEY_braille_dots_1247: c_int = 16787531;
pub const CLUTTER_KEY_braille_dots_12478: c_int = 16787659;
pub const CLUTTER_KEY_braille_dots_1248: c_int = 16787595;
pub const CLUTTER_KEY_braille_dots_125: c_int = 16787475;
pub const CLUTTER_KEY_braille_dots_1256: c_int = 16787507;
pub const CLUTTER_KEY_braille_dots_12567: c_int = 16787571;
pub const CLUTTER_KEY_braille_dots_125678: c_int = 16787699;
pub const CLUTTER_KEY_braille_dots_12568: c_int = 16787635;
pub const CLUTTER_KEY_braille_dots_1257: c_int = 16787539;
pub const CLUTTER_KEY_braille_dots_12578: c_int = 16787667;
pub const CLUTTER_KEY_braille_dots_1258: c_int = 16787603;
pub const CLUTTER_KEY_braille_dots_126: c_int = 16787491;
pub const CLUTTER_KEY_braille_dots_1267: c_int = 16787555;
pub const CLUTTER_KEY_braille_dots_12678: c_int = 16787683;
pub const CLUTTER_KEY_braille_dots_1268: c_int = 16787619;
pub const CLUTTER_KEY_braille_dots_127: c_int = 16787523;
pub const CLUTTER_KEY_braille_dots_1278: c_int = 16787651;
pub const CLUTTER_KEY_braille_dots_128: c_int = 16787587;
pub const CLUTTER_KEY_braille_dots_13: c_int = 16787461;
pub const CLUTTER_KEY_braille_dots_134: c_int = 16787469;
pub const CLUTTER_KEY_braille_dots_1345: c_int = 16787485;
pub const CLUTTER_KEY_braille_dots_13456: c_int = 16787517;
pub const CLUTTER_KEY_braille_dots_134567: c_int = 16787581;
pub const CLUTTER_KEY_braille_dots_1345678: c_int = 16787709;
pub const CLUTTER_KEY_braille_dots_134568: c_int = 16787645;
pub const CLUTTER_KEY_braille_dots_13457: c_int = 16787549;
pub const CLUTTER_KEY_braille_dots_134578: c_int = 16787677;
pub const CLUTTER_KEY_braille_dots_13458: c_int = 16787613;
pub const CLUTTER_KEY_braille_dots_1346: c_int = 16787501;
pub const CLUTTER_KEY_braille_dots_13467: c_int = 16787565;
pub const CLUTTER_KEY_braille_dots_134678: c_int = 16787693;
pub const CLUTTER_KEY_braille_dots_13468: c_int = 16787629;
pub const CLUTTER_KEY_braille_dots_1347: c_int = 16787533;
pub const CLUTTER_KEY_braille_dots_13478: c_int = 16787661;
pub const CLUTTER_KEY_braille_dots_1348: c_int = 16787597;
pub const CLUTTER_KEY_braille_dots_135: c_int = 16787477;
pub const CLUTTER_KEY_braille_dots_1356: c_int = 16787509;
pub const CLUTTER_KEY_braille_dots_13567: c_int = 16787573;
pub const CLUTTER_KEY_braille_dots_135678: c_int = 16787701;
pub const CLUTTER_KEY_braille_dots_13568: c_int = 16787637;
pub const CLUTTER_KEY_braille_dots_1357: c_int = 16787541;
pub const CLUTTER_KEY_braille_dots_13578: c_int = 16787669;
pub const CLUTTER_KEY_braille_dots_1358: c_int = 16787605;
pub const CLUTTER_KEY_braille_dots_136: c_int = 16787493;
pub const CLUTTER_KEY_braille_dots_1367: c_int = 16787557;
pub const CLUTTER_KEY_braille_dots_13678: c_int = 16787685;
pub const CLUTTER_KEY_braille_dots_1368: c_int = 16787621;
pub const CLUTTER_KEY_braille_dots_137: c_int = 16787525;
pub const CLUTTER_KEY_braille_dots_1378: c_int = 16787653;
pub const CLUTTER_KEY_braille_dots_138: c_int = 16787589;
pub const CLUTTER_KEY_braille_dots_14: c_int = 16787465;
pub const CLUTTER_KEY_braille_dots_145: c_int = 16787481;
pub const CLUTTER_KEY_braille_dots_1456: c_int = 16787513;
pub const CLUTTER_KEY_braille_dots_14567: c_int = 16787577;
pub const CLUTTER_KEY_braille_dots_145678: c_int = 16787705;
pub const CLUTTER_KEY_braille_dots_14568: c_int = 16787641;
pub const CLUTTER_KEY_braille_dots_1457: c_int = 16787545;
pub const CLUTTER_KEY_braille_dots_14578: c_int = 16787673;
pub const CLUTTER_KEY_braille_dots_1458: c_int = 16787609;
pub const CLUTTER_KEY_braille_dots_146: c_int = 16787497;
pub const CLUTTER_KEY_braille_dots_1467: c_int = 16787561;
pub const CLUTTER_KEY_braille_dots_14678: c_int = 16787689;
pub const CLUTTER_KEY_braille_dots_1468: c_int = 16787625;
pub const CLUTTER_KEY_braille_dots_147: c_int = 16787529;
pub const CLUTTER_KEY_braille_dots_1478: c_int = 16787657;
pub const CLUTTER_KEY_braille_dots_148: c_int = 16787593;
pub const CLUTTER_KEY_braille_dots_15: c_int = 16787473;
pub const CLUTTER_KEY_braille_dots_156: c_int = 16787505;
pub const CLUTTER_KEY_braille_dots_1567: c_int = 16787569;
pub const CLUTTER_KEY_braille_dots_15678: c_int = 16787697;
pub const CLUTTER_KEY_braille_dots_1568: c_int = 16787633;
pub const CLUTTER_KEY_braille_dots_157: c_int = 16787537;
pub const CLUTTER_KEY_braille_dots_1578: c_int = 16787665;
pub const CLUTTER_KEY_braille_dots_158: c_int = 16787601;
pub const CLUTTER_KEY_braille_dots_16: c_int = 16787489;
pub const CLUTTER_KEY_braille_dots_167: c_int = 16787553;
pub const CLUTTER_KEY_braille_dots_1678: c_int = 16787681;
pub const CLUTTER_KEY_braille_dots_168: c_int = 16787617;
pub const CLUTTER_KEY_braille_dots_17: c_int = 16787521;
pub const CLUTTER_KEY_braille_dots_178: c_int = 16787649;
pub const CLUTTER_KEY_braille_dots_18: c_int = 16787585;
pub const CLUTTER_KEY_braille_dots_2: c_int = 16787458;
pub const CLUTTER_KEY_braille_dots_23: c_int = 16787462;
pub const CLUTTER_KEY_braille_dots_234: c_int = 16787470;
pub const CLUTTER_KEY_braille_dots_2345: c_int = 16787486;
pub const CLUTTER_KEY_braille_dots_23456: c_int = 16787518;
pub const CLUTTER_KEY_braille_dots_234567: c_int = 16787582;
pub const CLUTTER_KEY_braille_dots_2345678: c_int = 16787710;
pub const CLUTTER_KEY_braille_dots_234568: c_int = 16787646;
pub const CLUTTER_KEY_braille_dots_23457: c_int = 16787550;
pub const CLUTTER_KEY_braille_dots_234578: c_int = 16787678;
pub const CLUTTER_KEY_braille_dots_23458: c_int = 16787614;
pub const CLUTTER_KEY_braille_dots_2346: c_int = 16787502;
pub const CLUTTER_KEY_braille_dots_23467: c_int = 16787566;
pub const CLUTTER_KEY_braille_dots_234678: c_int = 16787694;
pub const CLUTTER_KEY_braille_dots_23468: c_int = 16787630;
pub const CLUTTER_KEY_braille_dots_2347: c_int = 16787534;
pub const CLUTTER_KEY_braille_dots_23478: c_int = 16787662;
pub const CLUTTER_KEY_braille_dots_2348: c_int = 16787598;
pub const CLUTTER_KEY_braille_dots_235: c_int = 16787478;
pub const CLUTTER_KEY_braille_dots_2356: c_int = 16787510;
pub const CLUTTER_KEY_braille_dots_23567: c_int = 16787574;
pub const CLUTTER_KEY_braille_dots_235678: c_int = 16787702;
pub const CLUTTER_KEY_braille_dots_23568: c_int = 16787638;
pub const CLUTTER_KEY_braille_dots_2357: c_int = 16787542;
pub const CLUTTER_KEY_braille_dots_23578: c_int = 16787670;
pub const CLUTTER_KEY_braille_dots_2358: c_int = 16787606;
pub const CLUTTER_KEY_braille_dots_236: c_int = 16787494;
pub const CLUTTER_KEY_braille_dots_2367: c_int = 16787558;
pub const CLUTTER_KEY_braille_dots_23678: c_int = 16787686;
pub const CLUTTER_KEY_braille_dots_2368: c_int = 16787622;
pub const CLUTTER_KEY_braille_dots_237: c_int = 16787526;
pub const CLUTTER_KEY_braille_dots_2378: c_int = 16787654;
pub const CLUTTER_KEY_braille_dots_238: c_int = 16787590;
pub const CLUTTER_KEY_braille_dots_24: c_int = 16787466;
pub const CLUTTER_KEY_braille_dots_245: c_int = 16787482;
pub const CLUTTER_KEY_braille_dots_2456: c_int = 16787514;
pub const CLUTTER_KEY_braille_dots_24567: c_int = 16787578;
pub const CLUTTER_KEY_braille_dots_245678: c_int = 16787706;
pub const CLUTTER_KEY_braille_dots_24568: c_int = 16787642;
pub const CLUTTER_KEY_braille_dots_2457: c_int = 16787546;
pub const CLUTTER_KEY_braille_dots_24578: c_int = 16787674;
pub const CLUTTER_KEY_braille_dots_2458: c_int = 16787610;
pub const CLUTTER_KEY_braille_dots_246: c_int = 16787498;
pub const CLUTTER_KEY_braille_dots_2467: c_int = 16787562;
pub const CLUTTER_KEY_braille_dots_24678: c_int = 16787690;
pub const CLUTTER_KEY_braille_dots_2468: c_int = 16787626;
pub const CLUTTER_KEY_braille_dots_247: c_int = 16787530;
pub const CLUTTER_KEY_braille_dots_2478: c_int = 16787658;
pub const CLUTTER_KEY_braille_dots_248: c_int = 16787594;
pub const CLUTTER_KEY_braille_dots_25: c_int = 16787474;
pub const CLUTTER_KEY_braille_dots_256: c_int = 16787506;
pub const CLUTTER_KEY_braille_dots_2567: c_int = 16787570;
pub const CLUTTER_KEY_braille_dots_25678: c_int = 16787698;
pub const CLUTTER_KEY_braille_dots_2568: c_int = 16787634;
pub const CLUTTER_KEY_braille_dots_257: c_int = 16787538;
pub const CLUTTER_KEY_braille_dots_2578: c_int = 16787666;
pub const CLUTTER_KEY_braille_dots_258: c_int = 16787602;
pub const CLUTTER_KEY_braille_dots_26: c_int = 16787490;
pub const CLUTTER_KEY_braille_dots_267: c_int = 16787554;
pub const CLUTTER_KEY_braille_dots_2678: c_int = 16787682;
pub const CLUTTER_KEY_braille_dots_268: c_int = 16787618;
pub const CLUTTER_KEY_braille_dots_27: c_int = 16787522;
pub const CLUTTER_KEY_braille_dots_278: c_int = 16787650;
pub const CLUTTER_KEY_braille_dots_28: c_int = 16787586;
pub const CLUTTER_KEY_braille_dots_3: c_int = 16787460;
pub const CLUTTER_KEY_braille_dots_34: c_int = 16787468;
pub const CLUTTER_KEY_braille_dots_345: c_int = 16787484;
pub const CLUTTER_KEY_braille_dots_3456: c_int = 16787516;
pub const CLUTTER_KEY_braille_dots_34567: c_int = 16787580;
pub const CLUTTER_KEY_braille_dots_345678: c_int = 16787708;
pub const CLUTTER_KEY_braille_dots_34568: c_int = 16787644;
pub const CLUTTER_KEY_braille_dots_3457: c_int = 16787548;
pub const CLUTTER_KEY_braille_dots_34578: c_int = 16787676;
pub const CLUTTER_KEY_braille_dots_3458: c_int = 16787612;
pub const CLUTTER_KEY_braille_dots_346: c_int = 16787500;
pub const CLUTTER_KEY_braille_dots_3467: c_int = 16787564;
pub const CLUTTER_KEY_braille_dots_34678: c_int = 16787692;
pub const CLUTTER_KEY_braille_dots_3468: c_int = 16787628;
pub const CLUTTER_KEY_braille_dots_347: c_int = 16787532;
pub const CLUTTER_KEY_braille_dots_3478: c_int = 16787660;
pub const CLUTTER_KEY_braille_dots_348: c_int = 16787596;
pub const CLUTTER_KEY_braille_dots_35: c_int = 16787476;
pub const CLUTTER_KEY_braille_dots_356: c_int = 16787508;
pub const CLUTTER_KEY_braille_dots_3567: c_int = 16787572;
pub const CLUTTER_KEY_braille_dots_35678: c_int = 16787700;
pub const CLUTTER_KEY_braille_dots_3568: c_int = 16787636;
pub const CLUTTER_KEY_braille_dots_357: c_int = 16787540;
pub const CLUTTER_KEY_braille_dots_3578: c_int = 16787668;
pub const CLUTTER_KEY_braille_dots_358: c_int = 16787604;
pub const CLUTTER_KEY_braille_dots_36: c_int = 16787492;
pub const CLUTTER_KEY_braille_dots_367: c_int = 16787556;
pub const CLUTTER_KEY_braille_dots_3678: c_int = 16787684;
pub const CLUTTER_KEY_braille_dots_368: c_int = 16787620;
pub const CLUTTER_KEY_braille_dots_37: c_int = 16787524;
pub const CLUTTER_KEY_braille_dots_378: c_int = 16787652;
pub const CLUTTER_KEY_braille_dots_38: c_int = 16787588;
pub const CLUTTER_KEY_braille_dots_4: c_int = 16787464;
pub const CLUTTER_KEY_braille_dots_45: c_int = 16787480;
pub const CLUTTER_KEY_braille_dots_456: c_int = 16787512;
pub const CLUTTER_KEY_braille_dots_4567: c_int = 16787576;
pub const CLUTTER_KEY_braille_dots_45678: c_int = 16787704;
pub const CLUTTER_KEY_braille_dots_4568: c_int = 16787640;
pub const CLUTTER_KEY_braille_dots_457: c_int = 16787544;
pub const CLUTTER_KEY_braille_dots_4578: c_int = 16787672;
pub const CLUTTER_KEY_braille_dots_458: c_int = 16787608;
pub const CLUTTER_KEY_braille_dots_46: c_int = 16787496;
pub const CLUTTER_KEY_braille_dots_467: c_int = 16787560;
pub const CLUTTER_KEY_braille_dots_4678: c_int = 16787688;
pub const CLUTTER_KEY_braille_dots_468: c_int = 16787624;
pub const CLUTTER_KEY_braille_dots_47: c_int = 16787528;
pub const CLUTTER_KEY_braille_dots_478: c_int = 16787656;
pub const CLUTTER_KEY_braille_dots_48: c_int = 16787592;
pub const CLUTTER_KEY_braille_dots_5: c_int = 16787472;
pub const CLUTTER_KEY_braille_dots_56: c_int = 16787504;
pub const CLUTTER_KEY_braille_dots_567: c_int = 16787568;
pub const CLUTTER_KEY_braille_dots_5678: c_int = 16787696;
pub const CLUTTER_KEY_braille_dots_568: c_int = 16787632;
pub const CLUTTER_KEY_braille_dots_57: c_int = 16787536;
pub const CLUTTER_KEY_braille_dots_578: c_int = 16787664;
pub const CLUTTER_KEY_braille_dots_58: c_int = 16787600;
pub const CLUTTER_KEY_braille_dots_6: c_int = 16787488;
pub const CLUTTER_KEY_braille_dots_67: c_int = 16787552;
pub const CLUTTER_KEY_braille_dots_678: c_int = 16787680;
pub const CLUTTER_KEY_braille_dots_68: c_int = 16787616;
pub const CLUTTER_KEY_braille_dots_7: c_int = 16787520;
pub const CLUTTER_KEY_braille_dots_78: c_int = 16787648;
pub const CLUTTER_KEY_braille_dots_8: c_int = 16787584;
pub const CLUTTER_KEY_breve: c_int = 418;
pub const CLUTTER_KEY_brokenbar: c_int = 166;
pub const CLUTTER_KEY_c: c_int = 99;
pub const CLUTTER_KEY_c_h: c_int = 65187;
pub const CLUTTER_KEY_cabovedot: c_int = 741;
pub const CLUTTER_KEY_cacute: c_int = 486;
pub const CLUTTER_KEY_careof: c_int = 2744;
pub const CLUTTER_KEY_caret: c_int = 2812;
pub const CLUTTER_KEY_caron: c_int = 439;
pub const CLUTTER_KEY_ccaron: c_int = 488;
pub const CLUTTER_KEY_ccedilla: c_int = 231;
pub const CLUTTER_KEY_ccircumflex: c_int = 742;
pub const CLUTTER_KEY_cedilla: c_int = 184;
pub const CLUTTER_KEY_cent: c_int = 162;
pub const CLUTTER_KEY_ch: c_int = 65184;
pub const CLUTTER_KEY_checkerboard: c_int = 2529;
pub const CLUTTER_KEY_checkmark: c_int = 2803;
pub const CLUTTER_KEY_circle: c_int = 3023;
pub const CLUTTER_KEY_club: c_int = 2796;
pub const CLUTTER_KEY_colon: c_int = 58;
pub const CLUTTER_KEY_comma: c_int = 44;
pub const CLUTTER_KEY_containsas: c_int = 16785931;
pub const CLUTTER_KEY_copyright: c_int = 169;
pub const CLUTTER_KEY_cr: c_int = 2532;
pub const CLUTTER_KEY_crossinglines: c_int = 2542;
pub const CLUTTER_KEY_cuberoot: c_int = 16785947;
pub const CLUTTER_KEY_currency: c_int = 164;
pub const CLUTTER_KEY_cursor: c_int = 2815;
pub const CLUTTER_KEY_d: c_int = 100;
pub const CLUTTER_KEY_dabovedot: c_int = 16784907;
pub const CLUTTER_KEY_dagger: c_int = 2801;
pub const CLUTTER_KEY_dcaron: c_int = 495;
pub const CLUTTER_KEY_dead_A: c_int = 65153;
pub const CLUTTER_KEY_dead_E: c_int = 65155;
pub const CLUTTER_KEY_dead_I: c_int = 65157;
pub const CLUTTER_KEY_dead_O: c_int = 65159;
pub const CLUTTER_KEY_dead_U: c_int = 65161;
pub const CLUTTER_KEY_dead_a: c_int = 65152;
pub const CLUTTER_KEY_dead_abovecomma: c_int = 65124;
pub const CLUTTER_KEY_dead_abovedot: c_int = 65110;
pub const CLUTTER_KEY_dead_abovereversedcomma: c_int = 65125;
pub const CLUTTER_KEY_dead_abovering: c_int = 65112;
pub const CLUTTER_KEY_dead_aboveverticalline: c_int = 65169;
pub const CLUTTER_KEY_dead_acute: c_int = 65105;
pub const CLUTTER_KEY_dead_belowbreve: c_int = 65131;
pub const CLUTTER_KEY_dead_belowcircumflex: c_int = 65129;
pub const CLUTTER_KEY_dead_belowcomma: c_int = 65134;
pub const CLUTTER_KEY_dead_belowdiaeresis: c_int = 65132;
pub const CLUTTER_KEY_dead_belowdot: c_int = 65120;
pub const CLUTTER_KEY_dead_belowmacron: c_int = 65128;
pub const CLUTTER_KEY_dead_belowring: c_int = 65127;
pub const CLUTTER_KEY_dead_belowtilde: c_int = 65130;
pub const CLUTTER_KEY_dead_belowverticalline: c_int = 65170;
pub const CLUTTER_KEY_dead_breve: c_int = 65109;
pub const CLUTTER_KEY_dead_capital_schwa: c_int = 65163;
pub const CLUTTER_KEY_dead_caron: c_int = 65114;
pub const CLUTTER_KEY_dead_cedilla: c_int = 65115;
pub const CLUTTER_KEY_dead_circumflex: c_int = 65106;
pub const CLUTTER_KEY_dead_currency: c_int = 65135;
pub const CLUTTER_KEY_dead_dasia: c_int = 65125;
pub const CLUTTER_KEY_dead_diaeresis: c_int = 65111;
pub const CLUTTER_KEY_dead_doubleacute: c_int = 65113;
pub const CLUTTER_KEY_dead_doublegrave: c_int = 65126;
pub const CLUTTER_KEY_dead_e: c_int = 65154;
pub const CLUTTER_KEY_dead_grave: c_int = 65104;
pub const CLUTTER_KEY_dead_greek: c_int = 65164;
pub const CLUTTER_KEY_dead_hook: c_int = 65121;
pub const CLUTTER_KEY_dead_horn: c_int = 65122;
pub const CLUTTER_KEY_dead_i: c_int = 65156;
pub const CLUTTER_KEY_dead_invertedbreve: c_int = 65133;
pub const CLUTTER_KEY_dead_iota: c_int = 65117;
pub const CLUTTER_KEY_dead_longsolidusoverlay: c_int = 65171;
pub const CLUTTER_KEY_dead_lowline: c_int = 65168;
pub const CLUTTER_KEY_dead_macron: c_int = 65108;
pub const CLUTTER_KEY_dead_o: c_int = 65158;
pub const CLUTTER_KEY_dead_ogonek: c_int = 65116;
pub const CLUTTER_KEY_dead_perispomeni: c_int = 65107;
pub const CLUTTER_KEY_dead_psili: c_int = 65124;
pub const CLUTTER_KEY_dead_semivoiced_sound: c_int = 65119;
pub const CLUTTER_KEY_dead_small_schwa: c_int = 65162;
pub const CLUTTER_KEY_dead_stroke: c_int = 65123;
pub const CLUTTER_KEY_dead_tilde: c_int = 65107;
pub const CLUTTER_KEY_dead_u: c_int = 65160;
pub const CLUTTER_KEY_dead_voiced_sound: c_int = 65118;
pub const CLUTTER_KEY_decimalpoint: c_int = 2749;
pub const CLUTTER_KEY_degree: c_int = 176;
pub const CLUTTER_KEY_diaeresis: c_int = 168;
pub const CLUTTER_KEY_diamond: c_int = 2797;
pub const CLUTTER_KEY_digitspace: c_int = 2725;
pub const CLUTTER_KEY_dintegral: c_int = 16785964;
pub const CLUTTER_KEY_division: c_int = 247;
pub const CLUTTER_KEY_dollar: c_int = 36;
pub const CLUTTER_KEY_doubbaselinedot: c_int = 2735;
pub const CLUTTER_KEY_doubleacute: c_int = 445;
pub const CLUTTER_KEY_doubledagger: c_int = 2802;
pub const CLUTTER_KEY_doublelowquotemark: c_int = 2814;
pub const CLUTTER_KEY_downarrow: c_int = 2302;
pub const CLUTTER_KEY_downcaret: c_int = 2984;
pub const CLUTTER_KEY_downshoe: c_int = 3030;
pub const CLUTTER_KEY_downstile: c_int = 3012;
pub const CLUTTER_KEY_downtack: c_int = 3010;
pub const CLUTTER_KEY_dstroke: c_int = 496;
pub const CLUTTER_KEY_e: c_int = 101;
pub const CLUTTER_KEY_eabovedot: c_int = 1004;
pub const CLUTTER_KEY_eacute: c_int = 233;
pub const CLUTTER_KEY_ebelowdot: c_int = 16785081;
pub const CLUTTER_KEY_ecaron: c_int = 492;
pub const CLUTTER_KEY_ecircumflex: c_int = 234;
pub const CLUTTER_KEY_ecircumflexacute: c_int = 16785087;
pub const CLUTTER_KEY_ecircumflexbelowdot: c_int = 16785095;
pub const CLUTTER_KEY_ecircumflexgrave: c_int = 16785089;
pub const CLUTTER_KEY_ecircumflexhook: c_int = 16785091;
pub const CLUTTER_KEY_ecircumflextilde: c_int = 16785093;
pub const CLUTTER_KEY_ediaeresis: c_int = 235;
pub const CLUTTER_KEY_egrave: c_int = 232;
pub const CLUTTER_KEY_ehook: c_int = 16785083;
pub const CLUTTER_KEY_eightsubscript: c_int = 16785544;
pub const CLUTTER_KEY_eightsuperior: c_int = 16785528;
pub const CLUTTER_KEY_elementof: c_int = 16785928;
pub const CLUTTER_KEY_ellipsis: c_int = 2734;
pub const CLUTTER_KEY_em3space: c_int = 2723;
pub const CLUTTER_KEY_em4space: c_int = 2724;
pub const CLUTTER_KEY_emacron: c_int = 954;
pub const CLUTTER_KEY_emdash: c_int = 2729;
pub const CLUTTER_KEY_emfilledcircle: c_int = 2782;
pub const CLUTTER_KEY_emfilledrect: c_int = 2783;
pub const CLUTTER_KEY_emopencircle: c_int = 2766;
pub const CLUTTER_KEY_emopenrectangle: c_int = 2767;
pub const CLUTTER_KEY_emptyset: c_int = 16785925;
pub const CLUTTER_KEY_emspace: c_int = 2721;
pub const CLUTTER_KEY_endash: c_int = 2730;
pub const CLUTTER_KEY_enfilledcircbullet: c_int = 2790;
pub const CLUTTER_KEY_enfilledsqbullet: c_int = 2791;
pub const CLUTTER_KEY_eng: c_int = 959;
pub const CLUTTER_KEY_enopencircbullet: c_int = 2784;
pub const CLUTTER_KEY_enopensquarebullet: c_int = 2785;
pub const CLUTTER_KEY_enspace: c_int = 2722;
pub const CLUTTER_KEY_eogonek: c_int = 490;
pub const CLUTTER_KEY_equal: c_int = 61;
pub const CLUTTER_KEY_eth: c_int = 240;
pub const CLUTTER_KEY_etilde: c_int = 16785085;
pub const CLUTTER_KEY_exclam: c_int = 33;
pub const CLUTTER_KEY_exclamdown: c_int = 161;
pub const CLUTTER_KEY_ezh: c_int = 16777874;
pub const CLUTTER_KEY_f: c_int = 102;
pub const CLUTTER_KEY_fabovedot: c_int = 16784927;
pub const CLUTTER_KEY_femalesymbol: c_int = 2808;
pub const CLUTTER_KEY_ff: c_int = 2531;
pub const CLUTTER_KEY_figdash: c_int = 2747;
pub const CLUTTER_KEY_filledlefttribullet: c_int = 2780;
pub const CLUTTER_KEY_filledrectbullet: c_int = 2779;
pub const CLUTTER_KEY_filledrighttribullet: c_int = 2781;
pub const CLUTTER_KEY_filledtribulletdown: c_int = 2793;
pub const CLUTTER_KEY_filledtribulletup: c_int = 2792;
pub const CLUTTER_KEY_fiveeighths: c_int = 2757;
pub const CLUTTER_KEY_fivesixths: c_int = 2743;
pub const CLUTTER_KEY_fivesubscript: c_int = 16785541;
pub const CLUTTER_KEY_fivesuperior: c_int = 16785525;
pub const CLUTTER_KEY_fourfifths: c_int = 2741;
pub const CLUTTER_KEY_foursubscript: c_int = 16785540;
pub const CLUTTER_KEY_foursuperior: c_int = 16785524;
pub const CLUTTER_KEY_fourthroot: c_int = 16785948;
pub const CLUTTER_KEY_function: c_int = 2294;
pub const CLUTTER_KEY_g: c_int = 103;
pub const CLUTTER_KEY_gabovedot: c_int = 757;
pub const CLUTTER_KEY_gbreve: c_int = 699;
pub const CLUTTER_KEY_gcaron: c_int = 16777703;
pub const CLUTTER_KEY_gcedilla: c_int = 955;
pub const CLUTTER_KEY_gcircumflex: c_int = 760;
pub const CLUTTER_KEY_grave: c_int = 96;
pub const CLUTTER_KEY_greater: c_int = 62;
pub const CLUTTER_KEY_greaterthanequal: c_int = 2238;
pub const CLUTTER_KEY_guillemotleft: c_int = 171;
pub const CLUTTER_KEY_guillemotright: c_int = 187;
pub const CLUTTER_KEY_h: c_int = 104;
pub const CLUTTER_KEY_hairspace: c_int = 2728;
pub const CLUTTER_KEY_hcircumflex: c_int = 694;
pub const CLUTTER_KEY_heart: c_int = 2798;
pub const CLUTTER_KEY_hebrew_aleph: c_int = 3296;
pub const CLUTTER_KEY_hebrew_ayin: c_int = 3314;
pub const CLUTTER_KEY_hebrew_bet: c_int = 3297;
pub const CLUTTER_KEY_hebrew_beth: c_int = 3297;
pub const CLUTTER_KEY_hebrew_chet: c_int = 3303;
pub const CLUTTER_KEY_hebrew_dalet: c_int = 3299;
pub const CLUTTER_KEY_hebrew_daleth: c_int = 3299;
pub const CLUTTER_KEY_hebrew_doublelowline: c_int = 3295;
pub const CLUTTER_KEY_hebrew_finalkaph: c_int = 3306;
pub const CLUTTER_KEY_hebrew_finalmem: c_int = 3309;
pub const CLUTTER_KEY_hebrew_finalnun: c_int = 3311;
pub const CLUTTER_KEY_hebrew_finalpe: c_int = 3315;
pub const CLUTTER_KEY_hebrew_finalzade: c_int = 3317;
pub const CLUTTER_KEY_hebrew_finalzadi: c_int = 3317;
pub const CLUTTER_KEY_hebrew_gimel: c_int = 3298;
pub const CLUTTER_KEY_hebrew_gimmel: c_int = 3298;
pub const CLUTTER_KEY_hebrew_he: c_int = 3300;
pub const CLUTTER_KEY_hebrew_het: c_int = 3303;
pub const CLUTTER_KEY_hebrew_kaph: c_int = 3307;
pub const CLUTTER_KEY_hebrew_kuf: c_int = 3319;
pub const CLUTTER_KEY_hebrew_lamed: c_int = 3308;
pub const CLUTTER_KEY_hebrew_mem: c_int = 3310;
pub const CLUTTER_KEY_hebrew_nun: c_int = 3312;
pub const CLUTTER_KEY_hebrew_pe: c_int = 3316;
pub const CLUTTER_KEY_hebrew_qoph: c_int = 3319;
pub const CLUTTER_KEY_hebrew_resh: c_int = 3320;
pub const CLUTTER_KEY_hebrew_samech: c_int = 3313;
pub const CLUTTER_KEY_hebrew_samekh: c_int = 3313;
pub const CLUTTER_KEY_hebrew_shin: c_int = 3321;
pub const CLUTTER_KEY_hebrew_taf: c_int = 3322;
pub const CLUTTER_KEY_hebrew_taw: c_int = 3322;
pub const CLUTTER_KEY_hebrew_tet: c_int = 3304;
pub const CLUTTER_KEY_hebrew_teth: c_int = 3304;
pub const CLUTTER_KEY_hebrew_waw: c_int = 3301;
pub const CLUTTER_KEY_hebrew_yod: c_int = 3305;
pub const CLUTTER_KEY_hebrew_zade: c_int = 3318;
pub const CLUTTER_KEY_hebrew_zadi: c_int = 3318;
pub const CLUTTER_KEY_hebrew_zain: c_int = 3302;
pub const CLUTTER_KEY_hebrew_zayin: c_int = 3302;
pub const CLUTTER_KEY_hexagram: c_int = 2778;
pub const CLUTTER_KEY_horizconnector: c_int = 2211;
pub const CLUTTER_KEY_horizlinescan1: c_int = 2543;
pub const CLUTTER_KEY_horizlinescan3: c_int = 2544;
pub const CLUTTER_KEY_horizlinescan5: c_int = 2545;
pub const CLUTTER_KEY_horizlinescan7: c_int = 2546;
pub const CLUTTER_KEY_horizlinescan9: c_int = 2547;
pub const CLUTTER_KEY_hstroke: c_int = 689;
pub const CLUTTER_KEY_ht: c_int = 2530;
pub const CLUTTER_KEY_hyphen: c_int = 173;
pub const CLUTTER_KEY_i: c_int = 105;
pub const CLUTTER_KEY_iTouch: c_int = 269025120;
pub const CLUTTER_KEY_iacute: c_int = 237;
pub const CLUTTER_KEY_ibelowdot: c_int = 16785099;
pub const CLUTTER_KEY_ibreve: c_int = 16777517;
pub const CLUTTER_KEY_icircumflex: c_int = 238;
pub const CLUTTER_KEY_identical: c_int = 2255;
pub const CLUTTER_KEY_idiaeresis: c_int = 239;
pub const CLUTTER_KEY_idotless: c_int = 697;
pub const CLUTTER_KEY_ifonlyif: c_int = 2253;
pub const CLUTTER_KEY_igrave: c_int = 236;
pub const CLUTTER_KEY_ihook: c_int = 16785097;
pub const CLUTTER_KEY_imacron: c_int = 1007;
pub const CLUTTER_KEY_implies: c_int = 2254;
pub const CLUTTER_KEY_includedin: c_int = 2266;
pub const CLUTTER_KEY_includes: c_int = 2267;
pub const CLUTTER_KEY_infinity: c_int = 2242;
pub const CLUTTER_KEY_integral: c_int = 2239;
pub const CLUTTER_KEY_intersection: c_int = 2268;
pub const CLUTTER_KEY_iogonek: c_int = 999;
pub const CLUTTER_KEY_itilde: c_int = 949;
pub const CLUTTER_KEY_j: c_int = 106;
pub const CLUTTER_KEY_jcircumflex: c_int = 700;
pub const CLUTTER_KEY_jot: c_int = 3018;
pub const CLUTTER_KEY_k: c_int = 107;
pub const CLUTTER_KEY_kana_A: c_int = 1201;
pub const CLUTTER_KEY_kana_CHI: c_int = 1217;
pub const CLUTTER_KEY_kana_E: c_int = 1204;
pub const CLUTTER_KEY_kana_FU: c_int = 1228;
pub const CLUTTER_KEY_kana_HA: c_int = 1226;
pub const CLUTTER_KEY_kana_HE: c_int = 1229;
pub const CLUTTER_KEY_kana_HI: c_int = 1227;
pub const CLUTTER_KEY_kana_HO: c_int = 1230;
pub const CLUTTER_KEY_kana_HU: c_int = 1228;
pub const CLUTTER_KEY_kana_I: c_int = 1202;
pub const CLUTTER_KEY_kana_KA: c_int = 1206;
pub const CLUTTER_KEY_kana_KE: c_int = 1209;
pub const CLUTTER_KEY_kana_KI: c_int = 1207;
pub const CLUTTER_KEY_kana_KO: c_int = 1210;
pub const CLUTTER_KEY_kana_KU: c_int = 1208;
pub const CLUTTER_KEY_kana_MA: c_int = 1231;
pub const CLUTTER_KEY_kana_ME: c_int = 1234;
pub const CLUTTER_KEY_kana_MI: c_int = 1232;
pub const CLUTTER_KEY_kana_MO: c_int = 1235;
pub const CLUTTER_KEY_kana_MU: c_int = 1233;
pub const CLUTTER_KEY_kana_N: c_int = 1245;
pub const CLUTTER_KEY_kana_NA: c_int = 1221;
pub const CLUTTER_KEY_kana_NE: c_int = 1224;
pub const CLUTTER_KEY_kana_NI: c_int = 1222;
pub const CLUTTER_KEY_kana_NO: c_int = 1225;
pub const CLUTTER_KEY_kana_NU: c_int = 1223;
pub const CLUTTER_KEY_kana_O: c_int = 1205;
pub const CLUTTER_KEY_kana_RA: c_int = 1239;
pub const CLUTTER_KEY_kana_RE: c_int = 1242;
pub const CLUTTER_KEY_kana_RI: c_int = 1240;
pub const CLUTTER_KEY_kana_RO: c_int = 1243;
pub const CLUTTER_KEY_kana_RU: c_int = 1241;
pub const CLUTTER_KEY_kana_SA: c_int = 1211;
pub const CLUTTER_KEY_kana_SE: c_int = 1214;
pub const CLUTTER_KEY_kana_SHI: c_int = 1212;
pub const CLUTTER_KEY_kana_SO: c_int = 1215;
pub const CLUTTER_KEY_kana_SU: c_int = 1213;
pub const CLUTTER_KEY_kana_TA: c_int = 1216;
pub const CLUTTER_KEY_kana_TE: c_int = 1219;
pub const CLUTTER_KEY_kana_TI: c_int = 1217;
pub const CLUTTER_KEY_kana_TO: c_int = 1220;
pub const CLUTTER_KEY_kana_TSU: c_int = 1218;
pub const CLUTTER_KEY_kana_TU: c_int = 1218;
pub const CLUTTER_KEY_kana_U: c_int = 1203;
pub const CLUTTER_KEY_kana_WA: c_int = 1244;
pub const CLUTTER_KEY_kana_WO: c_int = 1190;
pub const CLUTTER_KEY_kana_YA: c_int = 1236;
pub const CLUTTER_KEY_kana_YO: c_int = 1238;
pub const CLUTTER_KEY_kana_YU: c_int = 1237;
pub const CLUTTER_KEY_kana_a: c_int = 1191;
pub const CLUTTER_KEY_kana_closingbracket: c_int = 1187;
pub const CLUTTER_KEY_kana_comma: c_int = 1188;
pub const CLUTTER_KEY_kana_conjunctive: c_int = 1189;
pub const CLUTTER_KEY_kana_e: c_int = 1194;
pub const CLUTTER_KEY_kana_fullstop: c_int = 1185;
pub const CLUTTER_KEY_kana_i: c_int = 1192;
pub const CLUTTER_KEY_kana_middledot: c_int = 1189;
pub const CLUTTER_KEY_kana_o: c_int = 1195;
pub const CLUTTER_KEY_kana_openingbracket: c_int = 1186;
pub const CLUTTER_KEY_kana_switch: c_int = 65406;
pub const CLUTTER_KEY_kana_tsu: c_int = 1199;
pub const CLUTTER_KEY_kana_tu: c_int = 1199;
pub const CLUTTER_KEY_kana_u: c_int = 1193;
pub const CLUTTER_KEY_kana_ya: c_int = 1196;
pub const CLUTTER_KEY_kana_yo: c_int = 1198;
pub const CLUTTER_KEY_kana_yu: c_int = 1197;
pub const CLUTTER_KEY_kappa: c_int = 930;
pub const CLUTTER_KEY_kcedilla: c_int = 1011;
pub const CLUTTER_KEY_kra: c_int = 930;
pub const CLUTTER_KEY_l: c_int = 108;
pub const CLUTTER_KEY_lacute: c_int = 485;
pub const CLUTTER_KEY_latincross: c_int = 2777;
pub const CLUTTER_KEY_lbelowdot: c_int = 16784951;
pub const CLUTTER_KEY_lcaron: c_int = 437;
pub const CLUTTER_KEY_lcedilla: c_int = 950;
pub const CLUTTER_KEY_leftanglebracket: c_int = 2748;
pub const CLUTTER_KEY_leftarrow: c_int = 2299;
pub const CLUTTER_KEY_leftcaret: c_int = 2979;
pub const CLUTTER_KEY_leftdoublequotemark: c_int = 2770;
pub const CLUTTER_KEY_leftmiddlecurlybrace: c_int = 2223;
pub const CLUTTER_KEY_leftopentriangle: c_int = 2764;
pub const CLUTTER_KEY_leftpointer: c_int = 2794;
pub const CLUTTER_KEY_leftradical: c_int = 2209;
pub const CLUTTER_KEY_leftshoe: c_int = 3034;
pub const CLUTTER_KEY_leftsinglequotemark: c_int = 2768;
pub const CLUTTER_KEY_leftt: c_int = 2548;
pub const CLUTTER_KEY_lefttack: c_int = 3036;
pub const CLUTTER_KEY_less: c_int = 60;
pub const CLUTTER_KEY_lessthanequal: c_int = 2236;
pub const CLUTTER_KEY_lf: c_int = 2533;
pub const CLUTTER_KEY_logicaland: c_int = 2270;
pub const CLUTTER_KEY_logicalor: c_int = 2271;
pub const CLUTTER_KEY_lowleftcorner: c_int = 2541;
pub const CLUTTER_KEY_lowrightcorner: c_int = 2538;
pub const CLUTTER_KEY_lstroke: c_int = 435;
pub const CLUTTER_KEY_m: c_int = 109;
pub const CLUTTER_KEY_mabovedot: c_int = 16784961;
pub const CLUTTER_KEY_macron: c_int = 175;
pub const CLUTTER_KEY_malesymbol: c_int = 2807;
pub const CLUTTER_KEY_maltesecross: c_int = 2800;
pub const CLUTTER_KEY_marker: c_int = 2751;
pub const CLUTTER_KEY_masculine: c_int = 186;
pub const CLUTTER_KEY_minus: c_int = 45;
pub const CLUTTER_KEY_minutes: c_int = 2774;
pub const CLUTTER_KEY_mu: c_int = 181;
pub const CLUTTER_KEY_multiply: c_int = 215;
pub const CLUTTER_KEY_musicalflat: c_int = 2806;
pub const CLUTTER_KEY_musicalsharp: c_int = 2805;
pub const CLUTTER_KEY_n: c_int = 110;
pub const CLUTTER_KEY_nabla: c_int = 2245;
pub const CLUTTER_KEY_nacute: c_int = 497;
pub const CLUTTER_KEY_ncaron: c_int = 498;
pub const CLUTTER_KEY_ncedilla: c_int = 1009;
pub const CLUTTER_KEY_ninesubscript: c_int = 16785545;
pub const CLUTTER_KEY_ninesuperior: c_int = 16785529;
pub const CLUTTER_KEY_nl: c_int = 2536;
pub const CLUTTER_KEY_nobreakspace: c_int = 160;
pub const CLUTTER_KEY_notapproxeq: c_int = 16785991;
pub const CLUTTER_KEY_notelementof: c_int = 16785929;
pub const CLUTTER_KEY_notequal: c_int = 2237;
pub const CLUTTER_KEY_notidentical: c_int = 16786018;
pub const CLUTTER_KEY_notsign: c_int = 172;
pub const CLUTTER_KEY_ntilde: c_int = 241;
pub const CLUTTER_KEY_numbersign: c_int = 35;
pub const CLUTTER_KEY_numerosign: c_int = 1712;
pub const CLUTTER_KEY_o: c_int = 111;
pub const CLUTTER_KEY_oacute: c_int = 243;
pub const CLUTTER_KEY_obarred: c_int = 16777845;
pub const CLUTTER_KEY_obelowdot: c_int = 16785101;
pub const CLUTTER_KEY_ocaron: c_int = 16777682;
pub const CLUTTER_KEY_ocircumflex: c_int = 244;
pub const CLUTTER_KEY_ocircumflexacute: c_int = 16785105;
pub const CLUTTER_KEY_ocircumflexbelowdot: c_int = 16785113;
pub const CLUTTER_KEY_ocircumflexgrave: c_int = 16785107;
pub const CLUTTER_KEY_ocircumflexhook: c_int = 16785109;
pub const CLUTTER_KEY_ocircumflextilde: c_int = 16785111;
pub const CLUTTER_KEY_odiaeresis: c_int = 246;
pub const CLUTTER_KEY_odoubleacute: c_int = 501;
pub const CLUTTER_KEY_oe: c_int = 5053;
pub const CLUTTER_KEY_ogonek: c_int = 434;
pub const CLUTTER_KEY_ograve: c_int = 242;
pub const CLUTTER_KEY_ohook: c_int = 16785103;
pub const CLUTTER_KEY_ohorn: c_int = 16777633;
pub const CLUTTER_KEY_ohornacute: c_int = 16785115;
pub const CLUTTER_KEY_ohornbelowdot: c_int = 16785123;
pub const CLUTTER_KEY_ohorngrave: c_int = 16785117;
pub const CLUTTER_KEY_ohornhook: c_int = 16785119;
pub const CLUTTER_KEY_ohorntilde: c_int = 16785121;
pub const CLUTTER_KEY_omacron: c_int = 1010;
pub const CLUTTER_KEY_oneeighth: c_int = 2755;
pub const CLUTTER_KEY_onefifth: c_int = 2738;
pub const CLUTTER_KEY_onehalf: c_int = 189;
pub const CLUTTER_KEY_onequarter: c_int = 188;
pub const CLUTTER_KEY_onesixth: c_int = 2742;
pub const CLUTTER_KEY_onesubscript: c_int = 16785537;
pub const CLUTTER_KEY_onesuperior: c_int = 185;
pub const CLUTTER_KEY_onethird: c_int = 2736;
pub const CLUTTER_KEY_ooblique: c_int = 248;
pub const CLUTTER_KEY_openrectbullet: c_int = 2786;
pub const CLUTTER_KEY_openstar: c_int = 2789;
pub const CLUTTER_KEY_opentribulletdown: c_int = 2788;
pub const CLUTTER_KEY_opentribulletup: c_int = 2787;
pub const CLUTTER_KEY_ordfeminine: c_int = 170;
pub const CLUTTER_KEY_oslash: c_int = 248;
pub const CLUTTER_KEY_otilde: c_int = 245;
pub const CLUTTER_KEY_overbar: c_int = 3008;
pub const CLUTTER_KEY_overline: c_int = 1150;
pub const CLUTTER_KEY_p: c_int = 112;
pub const CLUTTER_KEY_pabovedot: c_int = 16784983;
pub const CLUTTER_KEY_paragraph: c_int = 182;
pub const CLUTTER_KEY_parenleft: c_int = 40;
pub const CLUTTER_KEY_parenright: c_int = 41;
pub const CLUTTER_KEY_partdifferential: c_int = 16785922;
pub const CLUTTER_KEY_partialderivative: c_int = 2287;
pub const CLUTTER_KEY_percent: c_int = 37;
pub const CLUTTER_KEY_period: c_int = 46;
pub const CLUTTER_KEY_periodcentered: c_int = 183;
pub const CLUTTER_KEY_permille: c_int = 2773;
pub const CLUTTER_KEY_phonographcopyright: c_int = 2811;
pub const CLUTTER_KEY_plus: c_int = 43;
pub const CLUTTER_KEY_plusminus: c_int = 177;
pub const CLUTTER_KEY_prescription: c_int = 2772;
pub const CLUTTER_KEY_prolongedsound: c_int = 1200;
pub const CLUTTER_KEY_punctspace: c_int = 2726;
pub const CLUTTER_KEY_q: c_int = 113;
pub const CLUTTER_KEY_quad: c_int = 3020;
pub const CLUTTER_KEY_question: c_int = 63;
pub const CLUTTER_KEY_questiondown: c_int = 191;
pub const CLUTTER_KEY_quotedbl: c_int = 34;
pub const CLUTTER_KEY_quoteleft: c_int = 96;
pub const CLUTTER_KEY_quoteright: c_int = 39;
pub const CLUTTER_KEY_r: c_int = 114;
pub const CLUTTER_KEY_racute: c_int = 480;
pub const CLUTTER_KEY_radical: c_int = 2262;
pub const CLUTTER_KEY_rcaron: c_int = 504;
pub const CLUTTER_KEY_rcedilla: c_int = 947;
pub const CLUTTER_KEY_registered: c_int = 174;
pub const CLUTTER_KEY_rightanglebracket: c_int = 2750;
pub const CLUTTER_KEY_rightarrow: c_int = 2301;
pub const CLUTTER_KEY_rightcaret: c_int = 2982;
pub const CLUTTER_KEY_rightdoublequotemark: c_int = 2771;
pub const CLUTTER_KEY_rightmiddlecurlybrace: c_int = 2224;
pub const CLUTTER_KEY_rightmiddlesummation: c_int = 2231;
pub const CLUTTER_KEY_rightopentriangle: c_int = 2765;
pub const CLUTTER_KEY_rightpointer: c_int = 2795;
pub const CLUTTER_KEY_rightshoe: c_int = 3032;
pub const CLUTTER_KEY_rightsinglequotemark: c_int = 2769;
pub const CLUTTER_KEY_rightt: c_int = 2549;
pub const CLUTTER_KEY_righttack: c_int = 3068;
pub const CLUTTER_KEY_s: c_int = 115;
pub const CLUTTER_KEY_sabovedot: c_int = 16784993;
pub const CLUTTER_KEY_sacute: c_int = 438;
pub const CLUTTER_KEY_scaron: c_int = 441;
pub const CLUTTER_KEY_scedilla: c_int = 442;
pub const CLUTTER_KEY_schwa: c_int = 16777817;
pub const CLUTTER_KEY_scircumflex: c_int = 766;
pub const CLUTTER_KEY_script_switch: c_int = 65406;
pub const CLUTTER_KEY_seconds: c_int = 2775;
pub const CLUTTER_KEY_section: c_int = 167;
pub const CLUTTER_KEY_semicolon: c_int = 59;
pub const CLUTTER_KEY_semivoicedsound: c_int = 1247;
pub const CLUTTER_KEY_seveneighths: c_int = 2758;
pub const CLUTTER_KEY_sevensubscript: c_int = 16785543;
pub const CLUTTER_KEY_sevensuperior: c_int = 16785527;
pub const CLUTTER_KEY_signaturemark: c_int = 2762;
pub const CLUTTER_KEY_signifblank: c_int = 2732;
pub const CLUTTER_KEY_similarequal: c_int = 2249;
pub const CLUTTER_KEY_singlelowquotemark: c_int = 2813;
pub const CLUTTER_KEY_sixsubscript: c_int = 16785542;
pub const CLUTTER_KEY_sixsuperior: c_int = 16785526;
pub const CLUTTER_KEY_slash: c_int = 47;
pub const CLUTTER_KEY_soliddiamond: c_int = 2528;
pub const CLUTTER_KEY_space: c_int = 32;
pub const CLUTTER_KEY_squareroot: c_int = 16785946;
pub const CLUTTER_KEY_ssharp: c_int = 223;
pub const CLUTTER_KEY_sterling: c_int = 163;
pub const CLUTTER_KEY_stricteq: c_int = 16786019;
pub const CLUTTER_KEY_t: c_int = 116;
pub const CLUTTER_KEY_tabovedot: c_int = 16785003;
pub const CLUTTER_KEY_tcaron: c_int = 443;
pub const CLUTTER_KEY_tcedilla: c_int = 510;
pub const CLUTTER_KEY_telephone: c_int = 2809;
pub const CLUTTER_KEY_telephonerecorder: c_int = 2810;
pub const CLUTTER_KEY_therefore: c_int = 2240;
pub const CLUTTER_KEY_thinspace: c_int = 2727;
pub const CLUTTER_KEY_thorn: c_int = 254;
pub const CLUTTER_KEY_threeeighths: c_int = 2756;
pub const CLUTTER_KEY_threefifths: c_int = 2740;
pub const CLUTTER_KEY_threequarters: c_int = 190;
pub const CLUTTER_KEY_threesubscript: c_int = 16785539;
pub const CLUTTER_KEY_threesuperior: c_int = 179;
pub const CLUTTER_KEY_tintegral: c_int = 16785965;
pub const CLUTTER_KEY_topintegral: c_int = 2212;
pub const CLUTTER_KEY_topleftparens: c_int = 2219;
pub const CLUTTER_KEY_topleftradical: c_int = 2210;
pub const CLUTTER_KEY_topleftsqbracket: c_int = 2215;
pub const CLUTTER_KEY_topleftsummation: c_int = 2225;
pub const CLUTTER_KEY_toprightparens: c_int = 2221;
pub const CLUTTER_KEY_toprightsqbracket: c_int = 2217;
pub const CLUTTER_KEY_toprightsummation: c_int = 2229;
pub const CLUTTER_KEY_topt: c_int = 2551;
pub const CLUTTER_KEY_topvertsummationconnector: c_int = 2227;
pub const CLUTTER_KEY_trademark: c_int = 2761;
pub const CLUTTER_KEY_trademarkincircle: c_int = 2763;
pub const CLUTTER_KEY_tslash: c_int = 956;
pub const CLUTTER_KEY_twofifths: c_int = 2739;
pub const CLUTTER_KEY_twosubscript: c_int = 16785538;
pub const CLUTTER_KEY_twosuperior: c_int = 178;
pub const CLUTTER_KEY_twothirds: c_int = 2737;
pub const CLUTTER_KEY_u: c_int = 117;
pub const CLUTTER_KEY_uacute: c_int = 250;
pub const CLUTTER_KEY_ubelowdot: c_int = 16785125;
pub const CLUTTER_KEY_ubreve: c_int = 765;
pub const CLUTTER_KEY_ucircumflex: c_int = 251;
pub const CLUTTER_KEY_udiaeresis: c_int = 252;
pub const CLUTTER_KEY_udoubleacute: c_int = 507;
pub const CLUTTER_KEY_ugrave: c_int = 249;
pub const CLUTTER_KEY_uhook: c_int = 16785127;
pub const CLUTTER_KEY_uhorn: c_int = 16777648;
pub const CLUTTER_KEY_uhornacute: c_int = 16785129;
pub const CLUTTER_KEY_uhornbelowdot: c_int = 16785137;
pub const CLUTTER_KEY_uhorngrave: c_int = 16785131;
pub const CLUTTER_KEY_uhornhook: c_int = 16785133;
pub const CLUTTER_KEY_uhorntilde: c_int = 16785135;
pub const CLUTTER_KEY_umacron: c_int = 1022;
pub const CLUTTER_KEY_underbar: c_int = 3014;
pub const CLUTTER_KEY_underscore: c_int = 95;
pub const CLUTTER_KEY_union: c_int = 2269;
pub const CLUTTER_KEY_uogonek: c_int = 1017;
pub const CLUTTER_KEY_uparrow: c_int = 2300;
pub const CLUTTER_KEY_upcaret: c_int = 2985;
pub const CLUTTER_KEY_upleftcorner: c_int = 2540;
pub const CLUTTER_KEY_uprightcorner: c_int = 2539;
pub const CLUTTER_KEY_upshoe: c_int = 3011;
pub const CLUTTER_KEY_upstile: c_int = 3027;
pub const CLUTTER_KEY_uptack: c_int = 3022;
pub const CLUTTER_KEY_uring: c_int = 505;
pub const CLUTTER_KEY_utilde: c_int = 1021;
pub const CLUTTER_KEY_v: c_int = 118;
pub const CLUTTER_KEY_variation: c_int = 2241;
pub const CLUTTER_KEY_vertbar: c_int = 2552;
pub const CLUTTER_KEY_vertconnector: c_int = 2214;
pub const CLUTTER_KEY_voicedsound: c_int = 1246;
pub const CLUTTER_KEY_vt: c_int = 2537;
pub const CLUTTER_KEY_w: c_int = 119;
pub const CLUTTER_KEY_wacute: c_int = 16785027;
pub const CLUTTER_KEY_wcircumflex: c_int = 16777589;
pub const CLUTTER_KEY_wdiaeresis: c_int = 16785029;
pub const CLUTTER_KEY_wgrave: c_int = 16785025;
pub const CLUTTER_KEY_x: c_int = 120;
pub const CLUTTER_KEY_xabovedot: c_int = 16785035;
pub const CLUTTER_KEY_y: c_int = 121;
pub const CLUTTER_KEY_yacute: c_int = 253;
pub const CLUTTER_KEY_ybelowdot: c_int = 16785141;
pub const CLUTTER_KEY_ycircumflex: c_int = 16777591;
pub const CLUTTER_KEY_ydiaeresis: c_int = 255;
pub const CLUTTER_KEY_yen: c_int = 165;
pub const CLUTTER_KEY_ygrave: c_int = 16785139;
pub const CLUTTER_KEY_yhook: c_int = 16785143;
pub const CLUTTER_KEY_ytilde: c_int = 16785145;
pub const CLUTTER_KEY_z: c_int = 122;
pub const CLUTTER_KEY_zabovedot: c_int = 447;
pub const CLUTTER_KEY_zacute: c_int = 444;
pub const CLUTTER_KEY_zcaron: c_int = 446;
pub const CLUTTER_KEY_zerosubscript: c_int = 16785536;
pub const CLUTTER_KEY_zerosuperior: c_int = 16785520;
pub const CLUTTER_KEY_zstroke: c_int = 16777654;
pub const CLUTTER_KP_0: c_int = 65456;
pub const CLUTTER_KP_1: c_int = 65457;
pub const CLUTTER_KP_2: c_int = 65458;
pub const CLUTTER_KP_3: c_int = 65459;
pub const CLUTTER_KP_4: c_int = 65460;
pub const CLUTTER_KP_5: c_int = 65461;
pub const CLUTTER_KP_6: c_int = 65462;
pub const CLUTTER_KP_7: c_int = 65463;
pub const CLUTTER_KP_8: c_int = 65464;
pub const CLUTTER_KP_9: c_int = 65465;
pub const CLUTTER_KP_Add: c_int = 65451;
pub const CLUTTER_KP_Begin: c_int = 65437;
pub const CLUTTER_KP_Decimal: c_int = 65454;
pub const CLUTTER_KP_Delete: c_int = 65439;
pub const CLUTTER_KP_Divide: c_int = 65455;
pub const CLUTTER_KP_Down: c_int = 65433;
pub const CLUTTER_KP_End: c_int = 65436;
pub const CLUTTER_KP_Enter: c_int = 65421;
pub const CLUTTER_KP_Equal: c_int = 65469;
pub const CLUTTER_KP_F1: c_int = 65425;
pub const CLUTTER_KP_F2: c_int = 65426;
pub const CLUTTER_KP_F3: c_int = 65427;
pub const CLUTTER_KP_F4: c_int = 65428;
pub const CLUTTER_KP_Home: c_int = 65429;
pub const CLUTTER_KP_Insert: c_int = 65438;
pub const CLUTTER_KP_Left: c_int = 65430;
pub const CLUTTER_KP_Multiply: c_int = 65450;
pub const CLUTTER_KP_Next: c_int = 65435;
pub const CLUTTER_KP_Page_Down: c_int = 65435;
pub const CLUTTER_KP_Page_Up: c_int = 65434;
pub const CLUTTER_KP_Prior: c_int = 65434;
pub const CLUTTER_KP_Right: c_int = 65432;
pub const CLUTTER_KP_Separator: c_int = 65452;
pub const CLUTTER_KP_Space: c_int = 65408;
pub const CLUTTER_KP_Subtract: c_int = 65453;
pub const CLUTTER_KP_Tab: c_int = 65417;
pub const CLUTTER_KP_Up: c_int = 65431;
pub const CLUTTER_Kana_Lock: c_int = 65325;
pub const CLUTTER_Kana_Shift: c_int = 65326;
pub const CLUTTER_Kanji: c_int = 65313;
pub const CLUTTER_Kanji_Bangou: c_int = 65335;
pub const CLUTTER_Katakana: c_int = 65318;
pub const CLUTTER_KbdBrightnessDown: c_int = 269025030;
pub const CLUTTER_KbdBrightnessUp: c_int = 269025029;
pub const CLUTTER_KbdLightOnOff: c_int = 269025028;
pub const CLUTTER_Kcedilla: c_int = 979;
pub const CLUTTER_Korean_Won: c_int = 3839;
pub const CLUTTER_L: c_int = 76;
pub const CLUTTER_L1: c_int = 65480;
pub const CLUTTER_L10: c_int = 65489;
pub const CLUTTER_L2: c_int = 65481;
pub const CLUTTER_L3: c_int = 65482;
pub const CLUTTER_L4: c_int = 65483;
pub const CLUTTER_L5: c_int = 65484;
pub const CLUTTER_L6: c_int = 65485;
pub const CLUTTER_L7: c_int = 65486;
pub const CLUTTER_L8: c_int = 65487;
pub const CLUTTER_L9: c_int = 65488;
pub const CLUTTER_Lacute: c_int = 453;
pub const CLUTTER_Last_Virtual_Screen: c_int = 65236;
pub const CLUTTER_Launch0: c_int = 269025088;
pub const CLUTTER_Launch1: c_int = 269025089;
pub const CLUTTER_Launch2: c_int = 269025090;
pub const CLUTTER_Launch3: c_int = 269025091;
pub const CLUTTER_Launch4: c_int = 269025092;
pub const CLUTTER_Launch5: c_int = 269025093;
pub const CLUTTER_Launch6: c_int = 269025094;
pub const CLUTTER_Launch7: c_int = 269025095;
pub const CLUTTER_Launch8: c_int = 269025096;
pub const CLUTTER_Launch9: c_int = 269025097;
pub const CLUTTER_LaunchA: c_int = 269025098;
pub const CLUTTER_LaunchB: c_int = 269025099;
pub const CLUTTER_LaunchC: c_int = 269025100;
pub const CLUTTER_LaunchD: c_int = 269025101;
pub const CLUTTER_LaunchE: c_int = 269025102;
pub const CLUTTER_LaunchF: c_int = 269025103;
pub const CLUTTER_Lbelowdot: c_int = 16784950;
pub const CLUTTER_Lcaron: c_int = 421;
pub const CLUTTER_Lcedilla: c_int = 934;
pub const CLUTTER_Left: c_int = 65361;
pub const CLUTTER_LightBulb: c_int = 269025077;
pub const CLUTTER_Linefeed: c_int = 65290;
pub const CLUTTER_LiraSign: c_int = 16785572;
pub const CLUTTER_LogGrabInfo: c_int = 269024805;
pub const CLUTTER_LogOff: c_int = 269025121;
pub const CLUTTER_LogWindowTree: c_int = 269024804;
pub const CLUTTER_Lstroke: c_int = 419;
pub const CLUTTER_M: c_int = 77;
pub const CLUTTER_MAJOR_VERSION: c_int = 1;
pub const CLUTTER_MICRO_VERSION: c_int = 4;
pub const CLUTTER_MINOR_VERSION: c_int = 26;
pub const CLUTTER_Mabovedot: c_int = 16784960;
pub const CLUTTER_Macedonia_DSE: c_int = 1717;
pub const CLUTTER_Macedonia_GJE: c_int = 1714;
pub const CLUTTER_Macedonia_KJE: c_int = 1724;
pub const CLUTTER_Macedonia_dse: c_int = 1701;
pub const CLUTTER_Macedonia_gje: c_int = 1698;
pub const CLUTTER_Macedonia_kje: c_int = 1708;
pub const CLUTTER_Mae_Koho: c_int = 65342;
pub const CLUTTER_Mail: c_int = 269025049;
pub const CLUTTER_MailForward: c_int = 269025168;
pub const CLUTTER_Market: c_int = 269025122;
pub const CLUTTER_Massyo: c_int = 65324;
pub const CLUTTER_Meeting: c_int = 269025123;
pub const CLUTTER_Memo: c_int = 269025054;
pub const CLUTTER_Menu: c_int = 65383;
pub const CLUTTER_MenuKB: c_int = 269025125;
pub const CLUTTER_MenuPB: c_int = 269025126;
pub const CLUTTER_Messenger: c_int = 269025166;
pub const CLUTTER_Meta_L: c_int = 65511;
pub const CLUTTER_Meta_R: c_int = 65512;
pub const CLUTTER_MillSign: c_int = 16785573;
pub const CLUTTER_ModeLock: c_int = 269025025;
pub const CLUTTER_Mode_switch: c_int = 65406;
pub const CLUTTER_MonBrightnessDown: c_int = 269025027;
pub const CLUTTER_MonBrightnessUp: c_int = 269025026;
pub const CLUTTER_MouseKeys_Accel_Enable: c_int = 65143;
pub const CLUTTER_MouseKeys_Enable: c_int = 65142;
pub const CLUTTER_Muhenkan: c_int = 65314;
pub const CLUTTER_Multi_key: c_int = 65312;
pub const CLUTTER_MultipleCandidate: c_int = 65341;
pub const CLUTTER_Music: c_int = 269025170;
pub const CLUTTER_MyComputer: c_int = 269025075;
pub const CLUTTER_MySites: c_int = 269025127;
pub const CLUTTER_N: c_int = 78;
pub const CLUTTER_NO_FPU: c_int = 0;
pub const CLUTTER_Nacute: c_int = 465;
pub const CLUTTER_NairaSign: c_int = 16785574;
pub const CLUTTER_Ncaron: c_int = 466;
pub const CLUTTER_Ncedilla: c_int = 977;
pub const CLUTTER_New: c_int = 269025128;
pub const CLUTTER_NewSheqelSign: c_int = 16785578;
pub const CLUTTER_News: c_int = 269025129;
pub const CLUTTER_Next: c_int = 65366;
pub const CLUTTER_Next_VMode: c_int = 269024802;
pub const CLUTTER_Next_Virtual_Screen: c_int = 65234;
pub const CLUTTER_Ntilde: c_int = 209;
pub const CLUTTER_Num_Lock: c_int = 65407;
pub const CLUTTER_O: c_int = 79;
pub const CLUTTER_OE: c_int = 5052;
pub const CLUTTER_Oacute: c_int = 211;
pub const CLUTTER_Obarred: c_int = 16777631;
pub const CLUTTER_Obelowdot: c_int = 16785100;
pub const CLUTTER_Ocaron: c_int = 16777681;
pub const CLUTTER_Ocircumflex: c_int = 212;
pub const CLUTTER_Ocircumflexacute: c_int = 16785104;
pub const CLUTTER_Ocircumflexbelowdot: c_int = 16785112;
pub const CLUTTER_Ocircumflexgrave: c_int = 16785106;
pub const CLUTTER_Ocircumflexhook: c_int = 16785108;
pub const CLUTTER_Ocircumflextilde: c_int = 16785110;
pub const CLUTTER_Odiaeresis: c_int = 214;
pub const CLUTTER_Odoubleacute: c_int = 469;
pub const CLUTTER_OfficeHome: c_int = 269025130;
pub const CLUTTER_Ograve: c_int = 210;
pub const CLUTTER_Ohook: c_int = 16785102;
pub const CLUTTER_Ohorn: c_int = 16777632;
pub const CLUTTER_Ohornacute: c_int = 16785114;
pub const CLUTTER_Ohornbelowdot: c_int = 16785122;
pub const CLUTTER_Ohorngrave: c_int = 16785116;
pub const CLUTTER_Ohornhook: c_int = 16785118;
pub const CLUTTER_Ohorntilde: c_int = 16785120;
pub const CLUTTER_Omacron: c_int = 978;
pub const CLUTTER_Ooblique: c_int = 216;
pub const CLUTTER_Open: c_int = 269025131;
pub const CLUTTER_OpenURL: c_int = 269025080;
pub const CLUTTER_Option: c_int = 269025132;
pub const CLUTTER_Oslash: c_int = 216;
pub const CLUTTER_Otilde: c_int = 213;
pub const CLUTTER_Overlay1_Enable: c_int = 65144;
pub const CLUTTER_Overlay2_Enable: c_int = 65145;
pub const CLUTTER_P: c_int = 80;
pub const CLUTTER_PATH_RELATIVE: c_int = 32;
pub const CLUTTER_PRIORITY_REDRAW: c_int = 50;
pub const CLUTTER_Pabovedot: c_int = 16784982;
pub const CLUTTER_Page_Down: c_int = 65366;
pub const CLUTTER_Page_Up: c_int = 65365;
pub const CLUTTER_Paste: c_int = 269025133;
pub const CLUTTER_Pause: c_int = 65299;
pub const CLUTTER_PesetaSign: c_int = 16785575;
pub const CLUTTER_Phone: c_int = 269025134;
pub const CLUTTER_Pictures: c_int = 269025169;
pub const CLUTTER_Pointer_Accelerate: c_int = 65274;
pub const CLUTTER_Pointer_Button1: c_int = 65257;
pub const CLUTTER_Pointer_Button2: c_int = 65258;
pub const CLUTTER_Pointer_Button3: c_int = 65259;
pub const CLUTTER_Pointer_Button4: c_int = 65260;
pub const CLUTTER_Pointer_Button5: c_int = 65261;
pub const CLUTTER_Pointer_Button_Dflt: c_int = 65256;
pub const CLUTTER_Pointer_DblClick1: c_int = 65263;
pub const CLUTTER_Pointer_DblClick2: c_int = 65264;
pub const CLUTTER_Pointer_DblClick3: c_int = 65265;
pub const CLUTTER_Pointer_DblClick4: c_int = 65266;
pub const CLUTTER_Pointer_DblClick5: c_int = 65267;
pub const CLUTTER_Pointer_DblClick_Dflt: c_int = 65262;
pub const CLUTTER_Pointer_DfltBtnNext: c_int = 65275;
pub const CLUTTER_Pointer_DfltBtnPrev: c_int = 65276;
pub const CLUTTER_Pointer_Down: c_int = 65251;
pub const CLUTTER_Pointer_DownLeft: c_int = 65254;
pub const CLUTTER_Pointer_DownRight: c_int = 65255;
pub const CLUTTER_Pointer_Drag1: c_int = 65269;
pub const CLUTTER_Pointer_Drag2: c_int = 65270;
pub const CLUTTER_Pointer_Drag3: c_int = 65271;
pub const CLUTTER_Pointer_Drag4: c_int = 65272;
pub const CLUTTER_Pointer_Drag5: c_int = 65277;
pub const CLUTTER_Pointer_Drag_Dflt: c_int = 65268;
pub const CLUTTER_Pointer_EnableKeys: c_int = 65273;
pub const CLUTTER_Pointer_Left: c_int = 65248;
pub const CLUTTER_Pointer_Right: c_int = 65249;
pub const CLUTTER_Pointer_Up: c_int = 65250;
pub const CLUTTER_Pointer_UpLeft: c_int = 65252;
pub const CLUTTER_Pointer_UpRight: c_int = 65253;
pub const CLUTTER_PowerDown: c_int = 269025057;
pub const CLUTTER_PowerOff: c_int = 269025066;
pub const CLUTTER_Prev_VMode: c_int = 269024803;
pub const CLUTTER_Prev_Virtual_Screen: c_int = 65233;
pub const CLUTTER_PreviousCandidate: c_int = 65342;
pub const CLUTTER_Print: c_int = 65377;
pub const CLUTTER_Prior: c_int = 65365;
pub const CLUTTER_Q: c_int = 81;
pub const CLUTTER_R: c_int = 82;
pub const CLUTTER_R1: c_int = 65490;
pub const CLUTTER_R10: c_int = 65499;
pub const CLUTTER_R11: c_int = 65500;
pub const CLUTTER_R12: c_int = 65501;
pub const CLUTTER_R13: c_int = 65502;
pub const CLUTTER_R14: c_int = 65503;
pub const CLUTTER_R15: c_int = 65504;
pub const CLUTTER_R2: c_int = 65491;
pub const CLUTTER_R3: c_int = 65492;
pub const CLUTTER_R4: c_int = 65493;
pub const CLUTTER_R5: c_int = 65494;
pub const CLUTTER_R6: c_int = 65495;
pub const CLUTTER_R7: c_int = 65496;
pub const CLUTTER_R8: c_int = 65497;
pub const CLUTTER_R9: c_int = 65498;
pub const CLUTTER_Racute: c_int = 448;
pub const CLUTTER_Rcaron: c_int = 472;
pub const CLUTTER_Rcedilla: c_int = 931;
pub const CLUTTER_Red: c_int = 269025187;
pub const CLUTTER_Redo: c_int = 65382;
pub const CLUTTER_Refresh: c_int = 269025065;
pub const CLUTTER_Reload: c_int = 269025139;
pub const CLUTTER_RepeatKeys_Enable: c_int = 65138;
pub const CLUTTER_Reply: c_int = 269025138;
pub const CLUTTER_Return: c_int = 65293;
pub const CLUTTER_Right: c_int = 65363;
pub const CLUTTER_RockerDown: c_int = 269025060;
pub const CLUTTER_RockerEnter: c_int = 269025061;
pub const CLUTTER_RockerUp: c_int = 269025059;
pub const CLUTTER_Romaji: c_int = 65316;
pub const CLUTTER_RotateWindows: c_int = 269025140;
pub const CLUTTER_RotationKB: c_int = 269025142;
pub const CLUTTER_RotationPB: c_int = 269025141;
pub const CLUTTER_RupeeSign: c_int = 16785576;
pub const CLUTTER_S: c_int = 83;
pub const CLUTTER_SCHWA: c_int = 16777615;
pub const CLUTTER_STAGE_TYPE: *const c_char = b"deprecated\0" as *const u8 as *const c_char;
pub const CLUTTER_Sabovedot: c_int = 16784992;
pub const CLUTTER_Sacute: c_int = 422;
pub const CLUTTER_Save: c_int = 269025143;
pub const CLUTTER_Scaron: c_int = 425;
pub const CLUTTER_Scedilla: c_int = 426;
pub const CLUTTER_Scircumflex: c_int = 734;
pub const CLUTTER_ScreenSaver: c_int = 269025069;
pub const CLUTTER_ScrollClick: c_int = 269025146;
pub const CLUTTER_ScrollDown: c_int = 269025145;
pub const CLUTTER_ScrollUp: c_int = 269025144;
pub const CLUTTER_Scroll_Lock: c_int = 65300;
pub const CLUTTER_Search: c_int = 269025051;
pub const CLUTTER_Select: c_int = 65376;
pub const CLUTTER_SelectButton: c_int = 269025184;
pub const CLUTTER_Send: c_int = 269025147;
pub const CLUTTER_Serbian_DJE: c_int = 1713;
pub const CLUTTER_Serbian_DZE: c_int = 1727;
pub const CLUTTER_Serbian_JE: c_int = 1720;
pub const CLUTTER_Serbian_LJE: c_int = 1721;
pub const CLUTTER_Serbian_NJE: c_int = 1722;
pub const CLUTTER_Serbian_TSHE: c_int = 1723;
pub const CLUTTER_Serbian_dje: c_int = 1697;
pub const CLUTTER_Serbian_dze: c_int = 1711;
pub const CLUTTER_Serbian_je: c_int = 1704;
pub const CLUTTER_Serbian_lje: c_int = 1705;
pub const CLUTTER_Serbian_nje: c_int = 1706;
pub const CLUTTER_Serbian_tshe: c_int = 1707;
pub const CLUTTER_Shift_L: c_int = 65505;
pub const CLUTTER_Shift_Lock: c_int = 65510;
pub const CLUTTER_Shift_R: c_int = 65506;
pub const CLUTTER_Shop: c_int = 269025078;
pub const CLUTTER_SingleCandidate: c_int = 65340;
pub const CLUTTER_Sinh_a: c_int = 16780677;
pub const CLUTTER_Sinh_aa: c_int = 16780678;
pub const CLUTTER_Sinh_aa2: c_int = 16780751;
pub const CLUTTER_Sinh_ae: c_int = 16780679;
pub const CLUTTER_Sinh_ae2: c_int = 16780752;
pub const CLUTTER_Sinh_aee: c_int = 16780680;
pub const CLUTTER_Sinh_aee2: c_int = 16780753;
pub const CLUTTER_Sinh_ai: c_int = 16780691;
pub const CLUTTER_Sinh_ai2: c_int = 16780763;
pub const CLUTTER_Sinh_al: c_int = 16780746;
pub const CLUTTER_Sinh_au: c_int = 16780694;
pub const CLUTTER_Sinh_au2: c_int = 16780766;
pub const CLUTTER_Sinh_ba: c_int = 16780726;
pub const CLUTTER_Sinh_bha: c_int = 16780727;
pub const CLUTTER_Sinh_ca: c_int = 16780704;
pub const CLUTTER_Sinh_cha: c_int = 16780705;
pub const CLUTTER_Sinh_dda: c_int = 16780713;
pub const CLUTTER_Sinh_ddha: c_int = 16780714;
pub const CLUTTER_Sinh_dha: c_int = 16780719;
pub const CLUTTER_Sinh_dhha: c_int = 16780720;
pub const CLUTTER_Sinh_e: c_int = 16780689;
pub const CLUTTER_Sinh_e2: c_int = 16780761;
pub const CLUTTER_Sinh_ee: c_int = 16780690;
pub const CLUTTER_Sinh_ee2: c_int = 16780762;
pub const CLUTTER_Sinh_fa: c_int = 16780742;
pub const CLUTTER_Sinh_ga: c_int = 16780700;
pub const CLUTTER_Sinh_gha: c_int = 16780701;
pub const CLUTTER_Sinh_h2: c_int = 16780675;
pub const CLUTTER_Sinh_ha: c_int = 16780740;
pub const CLUTTER_Sinh_i: c_int = 16780681;
pub const CLUTTER_Sinh_i2: c_int = 16780754;
pub const CLUTTER_Sinh_ii: c_int = 16780682;
pub const CLUTTER_Sinh_ii2: c_int = 16780755;
pub const CLUTTER_Sinh_ja: c_int = 16780706;
pub const CLUTTER_Sinh_jha: c_int = 16780707;
pub const CLUTTER_Sinh_jnya: c_int = 16780709;
pub const CLUTTER_Sinh_ka: c_int = 16780698;
pub const CLUTTER_Sinh_kha: c_int = 16780699;
pub const CLUTTER_Sinh_kunddaliya: c_int = 16780788;
pub const CLUTTER_Sinh_la: c_int = 16780733;
pub const CLUTTER_Sinh_lla: c_int = 16780741;
pub const CLUTTER_Sinh_lu: c_int = 16780687;
pub const CLUTTER_Sinh_lu2: c_int = 16780767;
pub const CLUTTER_Sinh_luu: c_int = 16780688;
pub const CLUTTER_Sinh_luu2: c_int = 16780787;
pub const CLUTTER_Sinh_ma: c_int = 16780728;
pub const CLUTTER_Sinh_mba: c_int = 16780729;
pub const CLUTTER_Sinh_na: c_int = 16780721;
pub const CLUTTER_Sinh_ndda: c_int = 16780716;
pub const CLUTTER_Sinh_ndha: c_int = 16780723;
pub const CLUTTER_Sinh_ng: c_int = 16780674;
pub const CLUTTER_Sinh_ng2: c_int = 16780702;
pub const CLUTTER_Sinh_nga: c_int = 16780703;
pub const CLUTTER_Sinh_nja: c_int = 16780710;
pub const CLUTTER_Sinh_nna: c_int = 16780715;
pub const CLUTTER_Sinh_nya: c_int = 16780708;
pub const CLUTTER_Sinh_o: c_int = 16780692;
pub const CLUTTER_Sinh_o2: c_int = 16780764;
pub const CLUTTER_Sinh_oo: c_int = 16780693;
pub const CLUTTER_Sinh_oo2: c_int = 16780765;
pub const CLUTTER_Sinh_pa: c_int = 16780724;
pub const CLUTTER_Sinh_pha: c_int = 16780725;
pub const CLUTTER_Sinh_ra: c_int = 16780731;
pub const CLUTTER_Sinh_ri: c_int = 16780685;
pub const CLUTTER_Sinh_rii: c_int = 16780686;
pub const CLUTTER_Sinh_ru2: c_int = 16780760;
pub const CLUTTER_Sinh_ruu2: c_int = 16780786;
pub const CLUTTER_Sinh_sa: c_int = 16780739;
pub const CLUTTER_Sinh_sha: c_int = 16780737;
pub const CLUTTER_Sinh_ssha: c_int = 16780738;
pub const CLUTTER_Sinh_tha: c_int = 16780717;
pub const CLUTTER_Sinh_thha: c_int = 16780718;
pub const CLUTTER_Sinh_tta: c_int = 16780711;
pub const CLUTTER_Sinh_ttha: c_int = 16780712;
pub const CLUTTER_Sinh_u: c_int = 16780683;
pub const CLUTTER_Sinh_u2: c_int = 16780756;
pub const CLUTTER_Sinh_uu: c_int = 16780684;
pub const CLUTTER_Sinh_uu2: c_int = 16780758;
pub const CLUTTER_Sinh_va: c_int = 16780736;
pub const CLUTTER_Sinh_ya: c_int = 16780730;
pub const CLUTTER_Sleep: c_int = 269025071;
pub const CLUTTER_SlowKeys_Enable: c_int = 65139;
pub const CLUTTER_Spell: c_int = 269025148;
pub const CLUTTER_SplitScreen: c_int = 269025149;
pub const CLUTTER_Standby: c_int = 269025040;
pub const CLUTTER_Start: c_int = 269025050;
pub const CLUTTER_StickyKeys_Enable: c_int = 65141;
pub const CLUTTER_Stop: c_int = 269025064;
pub const CLUTTER_Subtitle: c_int = 269025178;
pub const CLUTTER_Super_L: c_int = 65515;
pub const CLUTTER_Super_R: c_int = 65516;
pub const CLUTTER_Support: c_int = 269025150;
pub const CLUTTER_Suspend: c_int = 269025191;
pub const CLUTTER_Switch_VT_1: c_int = 269024769;
pub const CLUTTER_Switch_VT_10: c_int = 269024778;
pub const CLUTTER_Switch_VT_11: c_int = 269024779;
pub const CLUTTER_Switch_VT_12: c_int = 269024780;
pub const CLUTTER_Switch_VT_2: c_int = 269024770;
pub const CLUTTER_Switch_VT_3: c_int = 269024771;
pub const CLUTTER_Switch_VT_4: c_int = 269024772;
pub const CLUTTER_Switch_VT_5: c_int = 269024773;
pub const CLUTTER_Switch_VT_6: c_int = 269024774;
pub const CLUTTER_Switch_VT_7: c_int = 269024775;
pub const CLUTTER_Switch_VT_8: c_int = 269024776;
pub const CLUTTER_Switch_VT_9: c_int = 269024777;
pub const CLUTTER_Sys_Req: c_int = 65301;
pub const CLUTTER_T: c_int = 84;
pub const CLUTTER_THORN: c_int = 222;
pub const CLUTTER_Tab: c_int = 65289;
pub const CLUTTER_Tabovedot: c_int = 16785002;
pub const CLUTTER_TaskPane: c_int = 269025151;
pub const CLUTTER_Tcaron: c_int = 427;
pub const CLUTTER_Tcedilla: c_int = 478;
pub const CLUTTER_Terminal: c_int = 269025152;
pub const CLUTTER_Terminate_Server: c_int = 65237;
pub const CLUTTER_Thai_baht: c_int = 3551;
pub const CLUTTER_Thai_bobaimai: c_int = 3514;
pub const CLUTTER_Thai_chochan: c_int = 3496;
pub const CLUTTER_Thai_chochang: c_int = 3498;
pub const CLUTTER_Thai_choching: c_int = 3497;
pub const CLUTTER_Thai_chochoe: c_int = 3500;
pub const CLUTTER_Thai_dochada: c_int = 3502;
pub const CLUTTER_Thai_dodek: c_int = 3508;
pub const CLUTTER_Thai_fofa: c_int = 3517;
pub const CLUTTER_Thai_fofan: c_int = 3519;
pub const CLUTTER_Thai_hohip: c_int = 3531;
pub const CLUTTER_Thai_honokhuk: c_int = 3534;
pub const CLUTTER_Thai_khokhai: c_int = 3490;
pub const CLUTTER_Thai_khokhon: c_int = 3493;
pub const CLUTTER_Thai_khokhuat: c_int = 3491;
pub const CLUTTER_Thai_khokhwai: c_int = 3492;
pub const CLUTTER_Thai_khorakhang: c_int = 3494;
pub const CLUTTER_Thai_kokai: c_int = 3489;
pub const CLUTTER_Thai_lakkhangyao: c_int = 3557;
pub const CLUTTER_Thai_lekchet: c_int = 3575;
pub const CLUTTER_Thai_lekha: c_int = 3573;
pub const CLUTTER_Thai_lekhok: c_int = 3574;
pub const CLUTTER_Thai_lekkao: c_int = 3577;
pub const CLUTTER_Thai_leknung: c_int = 3569;
pub const CLUTTER_Thai_lekpaet: c_int = 3576;
pub const CLUTTER_Thai_leksam: c_int = 3571;
pub const CLUTTER_Thai_leksi: c_int = 3572;
pub const CLUTTER_Thai_leksong: c_int = 3570;
pub const CLUTTER_Thai_leksun: c_int = 3568;
pub const CLUTTER_Thai_lochula: c_int = 3532;
pub const CLUTTER_Thai_loling: c_int = 3525;
pub const CLUTTER_Thai_lu: c_int = 3526;
pub const CLUTTER_Thai_maichattawa: c_int = 3563;
pub const CLUTTER_Thai_maiek: c_int = 3560;
pub const CLUTTER_Thai_maihanakat: c_int = 3537;
pub const CLUTTER_Thai_maihanakat_maitho: c_int = 3550;
pub const CLUTTER_Thai_maitaikhu: c_int = 3559;
pub const CLUTTER_Thai_maitho: c_int = 3561;
pub const CLUTTER_Thai_maitri: c_int = 3562;
pub const CLUTTER_Thai_maiyamok: c_int = 3558;
pub const CLUTTER_Thai_moma: c_int = 3521;
pub const CLUTTER_Thai_ngongu: c_int = 3495;
pub const CLUTTER_Thai_nikhahit: c_int = 3565;
pub const CLUTTER_Thai_nonen: c_int = 3507;
pub const CLUTTER_Thai_nonu: c_int = 3513;
pub const CLUTTER_Thai_oang: c_int = 3533;
pub const CLUTTER_Thai_paiyannoi: c_int = 3535;
pub const CLUTTER_Thai_phinthu: c_int = 3546;
pub const CLUTTER_Thai_phophan: c_int = 3518;
pub const CLUTTER_Thai_phophung: c_int = 3516;
pub const CLUTTER_Thai_phosamphao: c_int = 3520;
pub const CLUTTER_Thai_popla: c_int = 3515;
pub const CLUTTER_Thai_rorua: c_int = 3523;
pub const CLUTTER_Thai_ru: c_int = 3524;
pub const CLUTTER_Thai_saraa: c_int = 3536;
pub const CLUTTER_Thai_saraaa: c_int = 3538;
pub const CLUTTER_Thai_saraae: c_int = 3553;
pub const CLUTTER_Thai_saraaimaimalai: c_int = 3556;
pub const CLUTTER_Thai_saraaimaimuan: c_int = 3555;
pub const CLUTTER_Thai_saraam: c_int = 3539;
pub const CLUTTER_Thai_sarae: c_int = 3552;
pub const CLUTTER_Thai_sarai: c_int = 3540;
pub const CLUTTER_Thai_saraii: c_int = 3541;
pub const CLUTTER_Thai_sarao: c_int = 3554;
pub const CLUTTER_Thai_sarau: c_int = 3544;
pub const CLUTTER_Thai_saraue: c_int = 3542;
pub const CLUTTER_Thai_sarauee: c_int = 3543;
pub const CLUTTER_Thai_sarauu: c_int = 3545;
pub const CLUTTER_Thai_sorusi: c_int = 3529;
pub const CLUTTER_Thai_sosala: c_int = 3528;
pub const CLUTTER_Thai_soso: c_int = 3499;
pub const CLUTTER_Thai_sosua: c_int = 3530;
pub const CLUTTER_Thai_thanthakhat: c_int = 3564;
pub const CLUTTER_Thai_thonangmontho: c_int = 3505;
pub const CLUTTER_Thai_thophuthao: c_int = 3506;
pub const CLUTTER_Thai_thothahan: c_int = 3511;
pub const CLUTTER_Thai_thothan: c_int = 3504;
pub const CLUTTER_Thai_thothong: c_int = 3512;
pub const CLUTTER_Thai_thothung: c_int = 3510;
pub const CLUTTER_Thai_topatak: c_int = 3503;
pub const CLUTTER_Thai_totao: c_int = 3509;
pub const CLUTTER_Thai_wowaen: c_int = 3527;
pub const CLUTTER_Thai_yoyak: c_int = 3522;
pub const CLUTTER_Thai_yoying: c_int = 3501;
pub const CLUTTER_Thorn: c_int = 222;
pub const CLUTTER_Time: c_int = 269025183;
pub const CLUTTER_ToDoList: c_int = 269025055;
pub const CLUTTER_Tools: c_int = 269025153;
pub const CLUTTER_TopMenu: c_int = 269025186;
pub const CLUTTER_TouchpadOff: c_int = 269025201;
pub const CLUTTER_TouchpadOn: c_int = 269025200;
pub const CLUTTER_TouchpadToggle: c_int = 269025193;
pub const CLUTTER_Touroku: c_int = 65323;
pub const CLUTTER_Travel: c_int = 269025154;
pub const CLUTTER_Tslash: c_int = 940;
pub const CLUTTER_U: c_int = 85;
pub const CLUTTER_UWB: c_int = 269025174;
pub const CLUTTER_Uacute: c_int = 218;
pub const CLUTTER_Ubelowdot: c_int = 16785124;
pub const CLUTTER_Ubreve: c_int = 733;
pub const CLUTTER_Ucircumflex: c_int = 219;
pub const CLUTTER_Udiaeresis: c_int = 220;
pub const CLUTTER_Udoubleacute: c_int = 475;
pub const CLUTTER_Ugrave: c_int = 217;
pub const CLUTTER_Uhook: c_int = 16785126;
pub const CLUTTER_Uhorn: c_int = 16777647;
pub const CLUTTER_Uhornacute: c_int = 16785128;
pub const CLUTTER_Uhornbelowdot: c_int = 16785136;
pub const CLUTTER_Uhorngrave: c_int = 16785130;
pub const CLUTTER_Uhornhook: c_int = 16785132;
pub const CLUTTER_Uhorntilde: c_int = 16785134;
pub const CLUTTER_Ukrainian_GHE_WITH_UPTURN: c_int = 1725;
pub const CLUTTER_Ukrainian_I: c_int = 1718;
pub const CLUTTER_Ukrainian_IE: c_int = 1716;
pub const CLUTTER_Ukrainian_YI: c_int = 1719;
pub const CLUTTER_Ukrainian_ghe_with_upturn: c_int = 1709;
pub const CLUTTER_Ukrainian_i: c_int = 1702;
pub const CLUTTER_Ukrainian_ie: c_int = 1700;
pub const CLUTTER_Ukrainian_yi: c_int = 1703;
pub const CLUTTER_Ukranian_I: c_int = 1718;
pub const CLUTTER_Ukranian_JE: c_int = 1716;
pub const CLUTTER_Ukranian_YI: c_int = 1719;
pub const CLUTTER_Ukranian_i: c_int = 1702;
pub const CLUTTER_Ukranian_je: c_int = 1700;
pub const CLUTTER_Ukranian_yi: c_int = 1703;
pub const CLUTTER_Umacron: c_int = 990;
pub const CLUTTER_Undo: c_int = 65381;
pub const CLUTTER_Ungrab: c_int = 269024800;
pub const CLUTTER_Uogonek: c_int = 985;
pub const CLUTTER_Up: c_int = 65362;
pub const CLUTTER_Uring: c_int = 473;
pub const CLUTTER_User1KB: c_int = 269025157;
pub const CLUTTER_User2KB: c_int = 269025158;
pub const CLUTTER_UserPB: c_int = 269025156;
pub const CLUTTER_Utilde: c_int = 989;
pub const CLUTTER_V: c_int = 86;
pub const CLUTTER_VERSION: c_double = 1.260000;
pub const CLUTTER_VERSION_HEX: c_int = 0;
pub const CLUTTER_VERSION_S: *const c_char = b"1.26.4\0" as *const u8 as *const c_char;
pub const CLUTTER_VendorHome: c_int = 269025076;
pub const CLUTTER_Video: c_int = 269025159;
pub const CLUTTER_View: c_int = 269025185;
pub const CLUTTER_VoidSymbol: c_int = 16777215;
pub const CLUTTER_W: c_int = 87;
pub const CLUTTER_WINDOWING_EGL: *const c_char = b"eglnative\0" as *const u8 as *const c_char;
pub const CLUTTER_WINDOWING_GDK: *const c_char = b"gdk\0" as *const u8 as *const c_char;
pub const CLUTTER_WINDOWING_GLX: *const c_char = b"glx\0" as *const u8 as *const c_char;
pub const CLUTTER_WINDOWING_WAYLAND: *const c_char = b"wayland\0" as *const u8 as *const c_char;
pub const CLUTTER_WINDOWING_X11: *const c_char = b"x11\0" as *const u8 as *const c_char;
pub const CLUTTER_WLAN: c_int = 269025173;
pub const CLUTTER_WWW: c_int = 269025070;
pub const CLUTTER_Wacute: c_int = 16785026;
pub const CLUTTER_WakeUp: c_int = 269025067;
pub const CLUTTER_Wcircumflex: c_int = 16777588;
pub const CLUTTER_Wdiaeresis: c_int = 16785028;
pub const CLUTTER_WebCam: c_int = 269025167;
pub const CLUTTER_Wgrave: c_int = 16785024;
pub const CLUTTER_WheelButton: c_int = 269025160;
pub const CLUTTER_WindowClear: c_int = 269025109;
pub const CLUTTER_WonSign: c_int = 16785577;
pub const CLUTTER_Word: c_int = 269025161;
pub const CLUTTER_X: c_int = 88;
pub const CLUTTER_Xabovedot: c_int = 16785034;
pub const CLUTTER_Xfer: c_int = 269025162;
pub const CLUTTER_Y: c_int = 89;
pub const CLUTTER_Yacute: c_int = 221;
pub const CLUTTER_Ybelowdot: c_int = 16785140;
pub const CLUTTER_Ycircumflex: c_int = 16777590;
pub const CLUTTER_Ydiaeresis: c_int = 5054;
pub const CLUTTER_Yellow: c_int = 269025189;
pub const CLUTTER_Ygrave: c_int = 16785138;
pub const CLUTTER_Yhook: c_int = 16785142;
pub const CLUTTER_Ytilde: c_int = 16785144;
pub const CLUTTER_Z: c_int = 90;
pub const CLUTTER_Zabovedot: c_int = 431;
pub const CLUTTER_Zacute: c_int = 428;
pub const CLUTTER_Zcaron: c_int = 430;
pub const CLUTTER_Zen_Koho: c_int = 65341;
pub const CLUTTER_Zenkaku: c_int = 65320;
pub const CLUTTER_Zenkaku_Hankaku: c_int = 65322;
pub const CLUTTER_ZoomIn: c_int = 269025163;
pub const CLUTTER_ZoomOut: c_int = 269025164;
pub const CLUTTER_Zstroke: c_int = 16777653;
pub const CLUTTER_a: c_int = 97;
pub const CLUTTER_aacute: c_int = 225;
pub const CLUTTER_abelowdot: c_int = 16785057;
pub const CLUTTER_abovedot: c_int = 511;
pub const CLUTTER_abreve: c_int = 483;
pub const CLUTTER_abreveacute: c_int = 16785071;
pub const CLUTTER_abrevebelowdot: c_int = 16785079;
pub const CLUTTER_abrevegrave: c_int = 16785073;
pub const CLUTTER_abrevehook: c_int = 16785075;
pub const CLUTTER_abrevetilde: c_int = 16785077;
pub const CLUTTER_acircumflex: c_int = 226;
pub const CLUTTER_acircumflexacute: c_int = 16785061;
pub const CLUTTER_acircumflexbelowdot: c_int = 16785069;
pub const CLUTTER_acircumflexgrave: c_int = 16785063;
pub const CLUTTER_acircumflexhook: c_int = 16785065;
pub const CLUTTER_acircumflextilde: c_int = 16785067;
pub const CLUTTER_acute: c_int = 180;
pub const CLUTTER_adiaeresis: c_int = 228;
pub const CLUTTER_ae: c_int = 230;
pub const CLUTTER_agrave: c_int = 224;
pub const CLUTTER_ahook: c_int = 16785059;
pub const CLUTTER_amacron: c_int = 992;
pub const CLUTTER_ampersand: c_int = 38;
pub const CLUTTER_aogonek: c_int = 433;
pub const CLUTTER_apostrophe: c_int = 39;
pub const CLUTTER_approxeq: c_int = 16785992;
pub const CLUTTER_approximate: c_int = 2248;
pub const CLUTTER_aring: c_int = 229;
pub const CLUTTER_asciicircum: c_int = 94;
pub const CLUTTER_asciitilde: c_int = 126;
pub const CLUTTER_asterisk: c_int = 42;
pub const CLUTTER_at: c_int = 64;
pub const CLUTTER_atilde: c_int = 227;
pub const CLUTTER_b: c_int = 98;
pub const CLUTTER_babovedot: c_int = 16784899;
pub const CLUTTER_backslash: c_int = 92;
pub const CLUTTER_ballotcross: c_int = 2804;
pub const CLUTTER_bar: c_int = 124;
pub const CLUTTER_because: c_int = 16785973;
pub const CLUTTER_blank: c_int = 2527;
pub const CLUTTER_botintegral: c_int = 2213;
pub const CLUTTER_botleftparens: c_int = 2220;
pub const CLUTTER_botleftsqbracket: c_int = 2216;
pub const CLUTTER_botleftsummation: c_int = 2226;
pub const CLUTTER_botrightparens: c_int = 2222;
pub const CLUTTER_botrightsqbracket: c_int = 2218;
pub const CLUTTER_botrightsummation: c_int = 2230;
pub const CLUTTER_bott: c_int = 2550;
pub const CLUTTER_botvertsummationconnector: c_int = 2228;
pub const CLUTTER_braceleft: c_int = 123;
pub const CLUTTER_braceright: c_int = 125;
pub const CLUTTER_bracketleft: c_int = 91;
pub const CLUTTER_bracketright: c_int = 93;
pub const CLUTTER_braille_blank: c_int = 16787456;
pub const CLUTTER_braille_dot_1: c_int = 65521;
pub const CLUTTER_braille_dot_10: c_int = 65530;
pub const CLUTTER_braille_dot_2: c_int = 65522;
pub const CLUTTER_braille_dot_3: c_int = 65523;
pub const CLUTTER_braille_dot_4: c_int = 65524;
pub const CLUTTER_braille_dot_5: c_int = 65525;
pub const CLUTTER_braille_dot_6: c_int = 65526;
pub const CLUTTER_braille_dot_7: c_int = 65527;
pub const CLUTTER_braille_dot_8: c_int = 65528;
pub const CLUTTER_braille_dot_9: c_int = 65529;
pub const CLUTTER_braille_dots_1: c_int = 16787457;
pub const CLUTTER_braille_dots_12: c_int = 16787459;
pub const CLUTTER_braille_dots_123: c_int = 16787463;
pub const CLUTTER_braille_dots_1234: c_int = 16787471;
pub const CLUTTER_braille_dots_12345: c_int = 16787487;
pub const CLUTTER_braille_dots_123456: c_int = 16787519;
pub const CLUTTER_braille_dots_1234567: c_int = 16787583;
pub const CLUTTER_braille_dots_12345678: c_int = 16787711;
pub const CLUTTER_braille_dots_1234568: c_int = 16787647;
pub const CLUTTER_braille_dots_123457: c_int = 16787551;
pub const CLUTTER_braille_dots_1234578: c_int = 16787679;
pub const CLUTTER_braille_dots_123458: c_int = 16787615;
pub const CLUTTER_braille_dots_12346: c_int = 16787503;
pub const CLUTTER_braille_dots_123467: c_int = 16787567;
pub const CLUTTER_braille_dots_1234678: c_int = 16787695;
pub const CLUTTER_braille_dots_123468: c_int = 16787631;
pub const CLUTTER_braille_dots_12347: c_int = 16787535;
pub const CLUTTER_braille_dots_123478: c_int = 16787663;
pub const CLUTTER_braille_dots_12348: c_int = 16787599;
pub const CLUTTER_braille_dots_1235: c_int = 16787479;
pub const CLUTTER_braille_dots_12356: c_int = 16787511;
pub const CLUTTER_braille_dots_123567: c_int = 16787575;
pub const CLUTTER_braille_dots_1235678: c_int = 16787703;
pub const CLUTTER_braille_dots_123568: c_int = 16787639;
pub const CLUTTER_braille_dots_12357: c_int = 16787543;
pub const CLUTTER_braille_dots_123578: c_int = 16787671;
pub const CLUTTER_braille_dots_12358: c_int = 16787607;
pub const CLUTTER_braille_dots_1236: c_int = 16787495;
pub const CLUTTER_braille_dots_12367: c_int = 16787559;
pub const CLUTTER_braille_dots_123678: c_int = 16787687;
pub const CLUTTER_braille_dots_12368: c_int = 16787623;
pub const CLUTTER_braille_dots_1237: c_int = 16787527;
pub const CLUTTER_braille_dots_12378: c_int = 16787655;
pub const CLUTTER_braille_dots_1238: c_int = 16787591;
pub const CLUTTER_braille_dots_124: c_int = 16787467;
pub const CLUTTER_braille_dots_1245: c_int = 16787483;
pub const CLUTTER_braille_dots_12456: c_int = 16787515;
pub const CLUTTER_braille_dots_124567: c_int = 16787579;
pub const CLUTTER_braille_dots_1245678: c_int = 16787707;
pub const CLUTTER_braille_dots_124568: c_int = 16787643;
pub const CLUTTER_braille_dots_12457: c_int = 16787547;
pub const CLUTTER_braille_dots_124578: c_int = 16787675;
pub const CLUTTER_braille_dots_12458: c_int = 16787611;
pub const CLUTTER_braille_dots_1246: c_int = 16787499;
pub const CLUTTER_braille_dots_12467: c_int = 16787563;
pub const CLUTTER_braille_dots_124678: c_int = 16787691;
pub const CLUTTER_braille_dots_12468: c_int = 16787627;
pub const CLUTTER_braille_dots_1247: c_int = 16787531;
pub const CLUTTER_braille_dots_12478: c_int = 16787659;
pub const CLUTTER_braille_dots_1248: c_int = 16787595;
pub const CLUTTER_braille_dots_125: c_int = 16787475;
pub const CLUTTER_braille_dots_1256: c_int = 16787507;
pub const CLUTTER_braille_dots_12567: c_int = 16787571;
pub const CLUTTER_braille_dots_125678: c_int = 16787699;
pub const CLUTTER_braille_dots_12568: c_int = 16787635;
pub const CLUTTER_braille_dots_1257: c_int = 16787539;
pub const CLUTTER_braille_dots_12578: c_int = 16787667;
pub const CLUTTER_braille_dots_1258: c_int = 16787603;
pub const CLUTTER_braille_dots_126: c_int = 16787491;
pub const CLUTTER_braille_dots_1267: c_int = 16787555;
pub const CLUTTER_braille_dots_12678: c_int = 16787683;
pub const CLUTTER_braille_dots_1268: c_int = 16787619;
pub const CLUTTER_braille_dots_127: c_int = 16787523;
pub const CLUTTER_braille_dots_1278: c_int = 16787651;
pub const CLUTTER_braille_dots_128: c_int = 16787587;
pub const CLUTTER_braille_dots_13: c_int = 16787461;
pub const CLUTTER_braille_dots_134: c_int = 16787469;
pub const CLUTTER_braille_dots_1345: c_int = 16787485;
pub const CLUTTER_braille_dots_13456: c_int = 16787517;
pub const CLUTTER_braille_dots_134567: c_int = 16787581;
pub const CLUTTER_braille_dots_1345678: c_int = 16787709;
pub const CLUTTER_braille_dots_134568: c_int = 16787645;
pub const CLUTTER_braille_dots_13457: c_int = 16787549;
pub const CLUTTER_braille_dots_134578: c_int = 16787677;
pub const CLUTTER_braille_dots_13458: c_int = 16787613;
pub const CLUTTER_braille_dots_1346: c_int = 16787501;
pub const CLUTTER_braille_dots_13467: c_int = 16787565;
pub const CLUTTER_braille_dots_134678: c_int = 16787693;
pub const CLUTTER_braille_dots_13468: c_int = 16787629;
pub const CLUTTER_braille_dots_1347: c_int = 16787533;
pub const CLUTTER_braille_dots_13478: c_int = 16787661;
pub const CLUTTER_braille_dots_1348: c_int = 16787597;
pub const CLUTTER_braille_dots_135: c_int = 16787477;
pub const CLUTTER_braille_dots_1356: c_int = 16787509;
pub const CLUTTER_braille_dots_13567: c_int = 16787573;
pub const CLUTTER_braille_dots_135678: c_int = 16787701;
pub const CLUTTER_braille_dots_13568: c_int = 16787637;
pub const CLUTTER_braille_dots_1357: c_int = 16787541;
pub const CLUTTER_braille_dots_13578: c_int = 16787669;
pub const CLUTTER_braille_dots_1358: c_int = 16787605;
pub const CLUTTER_braille_dots_136: c_int = 16787493;
pub const CLUTTER_braille_dots_1367: c_int = 16787557;
pub const CLUTTER_braille_dots_13678: c_int = 16787685;
pub const CLUTTER_braille_dots_1368: c_int = 16787621;
pub const CLUTTER_braille_dots_137: c_int = 16787525;
pub const CLUTTER_braille_dots_1378: c_int = 16787653;
pub const CLUTTER_braille_dots_138: c_int = 16787589;
pub const CLUTTER_braille_dots_14: c_int = 16787465;
pub const CLUTTER_braille_dots_145: c_int = 16787481;
pub const CLUTTER_braille_dots_1456: c_int = 16787513;
pub const CLUTTER_braille_dots_14567: c_int = 16787577;
pub const CLUTTER_braille_dots_145678: c_int = 16787705;
pub const CLUTTER_braille_dots_14568: c_int = 16787641;
pub const CLUTTER_braille_dots_1457: c_int = 16787545;
pub const CLUTTER_braille_dots_14578: c_int = 16787673;
pub const CLUTTER_braille_dots_1458: c_int = 16787609;
pub const CLUTTER_braille_dots_146: c_int = 16787497;
pub const CLUTTER_braille_dots_1467: c_int = 16787561;
pub const CLUTTER_braille_dots_14678: c_int = 16787689;
pub const CLUTTER_braille_dots_1468: c_int = 16787625;
pub const CLUTTER_braille_dots_147: c_int = 16787529;
pub const CLUTTER_braille_dots_1478: c_int = 16787657;
pub const CLUTTER_braille_dots_148: c_int = 16787593;
pub const CLUTTER_braille_dots_15: c_int = 16787473;
pub const CLUTTER_braille_dots_156: c_int = 16787505;
pub const CLUTTER_braille_dots_1567: c_int = 16787569;
pub const CLUTTER_braille_dots_15678: c_int = 16787697;
pub const CLUTTER_braille_dots_1568: c_int = 16787633;
pub const CLUTTER_braille_dots_157: c_int = 16787537;
pub const CLUTTER_braille_dots_1578: c_int = 16787665;
pub const CLUTTER_braille_dots_158: c_int = 16787601;
pub const CLUTTER_braille_dots_16: c_int = 16787489;
pub const CLUTTER_braille_dots_167: c_int = 16787553;
pub const CLUTTER_braille_dots_1678: c_int = 16787681;
pub const CLUTTER_braille_dots_168: c_int = 16787617;
pub const CLUTTER_braille_dots_17: c_int = 16787521;
pub const CLUTTER_braille_dots_178: c_int = 16787649;
pub const CLUTTER_braille_dots_18: c_int = 16787585;
pub const CLUTTER_braille_dots_2: c_int = 16787458;
pub const CLUTTER_braille_dots_23: c_int = 16787462;
pub const CLUTTER_braille_dots_234: c_int = 16787470;
pub const CLUTTER_braille_dots_2345: c_int = 16787486;
pub const CLUTTER_braille_dots_23456: c_int = 16787518;
pub const CLUTTER_braille_dots_234567: c_int = 16787582;
pub const CLUTTER_braille_dots_2345678: c_int = 16787710;
pub const CLUTTER_braille_dots_234568: c_int = 16787646;
pub const CLUTTER_braille_dots_23457: c_int = 16787550;
pub const CLUTTER_braille_dots_234578: c_int = 16787678;
pub const CLUTTER_braille_dots_23458: c_int = 16787614;
pub const CLUTTER_braille_dots_2346: c_int = 16787502;
pub const CLUTTER_braille_dots_23467: c_int = 16787566;
pub const CLUTTER_braille_dots_234678: c_int = 16787694;
pub const CLUTTER_braille_dots_23468: c_int = 16787630;
pub const CLUTTER_braille_dots_2347: c_int = 16787534;
pub const CLUTTER_braille_dots_23478: c_int = 16787662;
pub const CLUTTER_braille_dots_2348: c_int = 16787598;
pub const CLUTTER_braille_dots_235: c_int = 16787478;
pub const CLUTTER_braille_dots_2356: c_int = 16787510;
pub const CLUTTER_braille_dots_23567: c_int = 16787574;
pub const CLUTTER_braille_dots_235678: c_int = 16787702;
pub const CLUTTER_braille_dots_23568: c_int = 16787638;
pub const CLUTTER_braille_dots_2357: c_int = 16787542;
pub const CLUTTER_braille_dots_23578: c_int = 16787670;
pub const CLUTTER_braille_dots_2358: c_int = 16787606;
pub const CLUTTER_braille_dots_236: c_int = 16787494;
pub const CLUTTER_braille_dots_2367: c_int = 16787558;
pub const CLUTTER_braille_dots_23678: c_int = 16787686;
pub const CLUTTER_braille_dots_2368: c_int = 16787622;
pub const CLUTTER_braille_dots_237: c_int = 16787526;
pub const CLUTTER_braille_dots_2378: c_int = 16787654;
pub const CLUTTER_braille_dots_238: c_int = 16787590;
pub const CLUTTER_braille_dots_24: c_int = 16787466;
pub const CLUTTER_braille_dots_245: c_int = 16787482;
pub const CLUTTER_braille_dots_2456: c_int = 16787514;
pub const CLUTTER_braille_dots_24567: c_int = 16787578;
pub const CLUTTER_braille_dots_245678: c_int = 16787706;
pub const CLUTTER_braille_dots_24568: c_int = 16787642;
pub const CLUTTER_braille_dots_2457: c_int = 16787546;
pub const CLUTTER_braille_dots_24578: c_int = 16787674;
pub const CLUTTER_braille_dots_2458: c_int = 16787610;
pub const CLUTTER_braille_dots_246: c_int = 16787498;
pub const CLUTTER_braille_dots_2467: c_int = 16787562;
pub const CLUTTER_braille_dots_24678: c_int = 16787690;
pub const CLUTTER_braille_dots_2468: c_int = 16787626;
pub const CLUTTER_braille_dots_247: c_int = 16787530;
pub const CLUTTER_braille_dots_2478: c_int = 16787658;
pub const CLUTTER_braille_dots_248: c_int = 16787594;
pub const CLUTTER_braille_dots_25: c_int = 16787474;
pub const CLUTTER_braille_dots_256: c_int = 16787506;
pub const CLUTTER_braille_dots_2567: c_int = 16787570;
pub const CLUTTER_braille_dots_25678: c_int = 16787698;
pub const CLUTTER_braille_dots_2568: c_int = 16787634;
pub const CLUTTER_braille_dots_257: c_int = 16787538;
pub const CLUTTER_braille_dots_2578: c_int = 16787666;
pub const CLUTTER_braille_dots_258: c_int = 16787602;
pub const CLUTTER_braille_dots_26: c_int = 16787490;
pub const CLUTTER_braille_dots_267: c_int = 16787554;
pub const CLUTTER_braille_dots_2678: c_int = 16787682;
pub const CLUTTER_braille_dots_268: c_int = 16787618;
pub const CLUTTER_braille_dots_27: c_int = 16787522;
pub const CLUTTER_braille_dots_278: c_int = 16787650;
pub const CLUTTER_braille_dots_28: c_int = 16787586;
pub const CLUTTER_braille_dots_3: c_int = 16787460;
pub const CLUTTER_braille_dots_34: c_int = 16787468;
pub const CLUTTER_braille_dots_345: c_int = 16787484;
pub const CLUTTER_braille_dots_3456: c_int = 16787516;
pub const CLUTTER_braille_dots_34567: c_int = 16787580;
pub const CLUTTER_braille_dots_345678: c_int = 16787708;
pub const CLUTTER_braille_dots_34568: c_int = 16787644;
pub const CLUTTER_braille_dots_3457: c_int = 16787548;
pub const CLUTTER_braille_dots_34578: c_int = 16787676;
pub const CLUTTER_braille_dots_3458: c_int = 16787612;
pub const CLUTTER_braille_dots_346: c_int = 16787500;
pub const CLUTTER_braille_dots_3467: c_int = 16787564;
pub const CLUTTER_braille_dots_34678: c_int = 16787692;
pub const CLUTTER_braille_dots_3468: c_int = 16787628;
pub const CLUTTER_braille_dots_347: c_int = 16787532;
pub const CLUTTER_braille_dots_3478: c_int = 16787660;
pub const CLUTTER_braille_dots_348: c_int = 16787596;
pub const CLUTTER_braille_dots_35: c_int = 16787476;
pub const CLUTTER_braille_dots_356: c_int = 16787508;
pub const CLUTTER_braille_dots_3567: c_int = 16787572;
pub const CLUTTER_braille_dots_35678: c_int = 16787700;
pub const CLUTTER_braille_dots_3568: c_int = 16787636;
pub const CLUTTER_braille_dots_357: c_int = 16787540;
pub const CLUTTER_braille_dots_3578: c_int = 16787668;
pub const CLUTTER_braille_dots_358: c_int = 16787604;
pub const CLUTTER_braille_dots_36: c_int = 16787492;
pub const CLUTTER_braille_dots_367: c_int = 16787556;
pub const CLUTTER_braille_dots_3678: c_int = 16787684;
pub const CLUTTER_braille_dots_368: c_int = 16787620;
pub const CLUTTER_braille_dots_37: c_int = 16787524;
pub const CLUTTER_braille_dots_378: c_int = 16787652;
pub const CLUTTER_braille_dots_38: c_int = 16787588;
pub const CLUTTER_braille_dots_4: c_int = 16787464;
pub const CLUTTER_braille_dots_45: c_int = 16787480;
pub const CLUTTER_braille_dots_456: c_int = 16787512;
pub const CLUTTER_braille_dots_4567: c_int = 16787576;
pub const CLUTTER_braille_dots_45678: c_int = 16787704;
pub const CLUTTER_braille_dots_4568: c_int = 16787640;
pub const CLUTTER_braille_dots_457: c_int = 16787544;
pub const CLUTTER_braille_dots_4578: c_int = 16787672;
pub const CLUTTER_braille_dots_458: c_int = 16787608;
pub const CLUTTER_braille_dots_46: c_int = 16787496;
pub const CLUTTER_braille_dots_467: c_int = 16787560;
pub const CLUTTER_braille_dots_4678: c_int = 16787688;
pub const CLUTTER_braille_dots_468: c_int = 16787624;
pub const CLUTTER_braille_dots_47: c_int = 16787528;
pub const CLUTTER_braille_dots_478: c_int = 16787656;
pub const CLUTTER_braille_dots_48: c_int = 16787592;
pub const CLUTTER_braille_dots_5: c_int = 16787472;
pub const CLUTTER_braille_dots_56: c_int = 16787504;
pub const CLUTTER_braille_dots_567: c_int = 16787568;
pub const CLUTTER_braille_dots_5678: c_int = 16787696;
pub const CLUTTER_braille_dots_568: c_int = 16787632;
pub const CLUTTER_braille_dots_57: c_int = 16787536;
pub const CLUTTER_braille_dots_578: c_int = 16787664;
pub const CLUTTER_braille_dots_58: c_int = 16787600;
pub const CLUTTER_braille_dots_6: c_int = 16787488;
pub const CLUTTER_braille_dots_67: c_int = 16787552;
pub const CLUTTER_braille_dots_678: c_int = 16787680;
pub const CLUTTER_braille_dots_68: c_int = 16787616;
pub const CLUTTER_braille_dots_7: c_int = 16787520;
pub const CLUTTER_braille_dots_78: c_int = 16787648;
pub const CLUTTER_braille_dots_8: c_int = 16787584;
pub const CLUTTER_breve: c_int = 418;
pub const CLUTTER_brokenbar: c_int = 166;
pub const CLUTTER_c: c_int = 99;
pub const CLUTTER_c_h: c_int = 65187;
pub const CLUTTER_cabovedot: c_int = 741;
pub const CLUTTER_cacute: c_int = 486;
pub const CLUTTER_careof: c_int = 2744;
pub const CLUTTER_caret: c_int = 2812;
pub const CLUTTER_caron: c_int = 439;
pub const CLUTTER_ccaron: c_int = 488;
pub const CLUTTER_ccedilla: c_int = 231;
pub const CLUTTER_ccircumflex: c_int = 742;
pub const CLUTTER_cedilla: c_int = 184;
pub const CLUTTER_cent: c_int = 162;
pub const CLUTTER_ch: c_int = 65184;
pub const CLUTTER_checkerboard: c_int = 2529;
pub const CLUTTER_checkmark: c_int = 2803;
pub const CLUTTER_circle: c_int = 3023;
pub const CLUTTER_club: c_int = 2796;
pub const CLUTTER_colon: c_int = 58;
pub const CLUTTER_comma: c_int = 44;
pub const CLUTTER_containsas: c_int = 16785931;
pub const CLUTTER_copyright: c_int = 169;
pub const CLUTTER_cr: c_int = 2532;
pub const CLUTTER_crossinglines: c_int = 2542;
pub const CLUTTER_cuberoot: c_int = 16785947;
pub const CLUTTER_currency: c_int = 164;
pub const CLUTTER_cursor: c_int = 2815;
pub const CLUTTER_d: c_int = 100;
pub const CLUTTER_dabovedot: c_int = 16784907;
pub const CLUTTER_dagger: c_int = 2801;
pub const CLUTTER_dcaron: c_int = 495;
pub const CLUTTER_dead_A: c_int = 65153;
pub const CLUTTER_dead_E: c_int = 65155;
pub const CLUTTER_dead_I: c_int = 65157;
pub const CLUTTER_dead_O: c_int = 65159;
pub const CLUTTER_dead_U: c_int = 65161;
pub const CLUTTER_dead_a: c_int = 65152;
pub const CLUTTER_dead_abovecomma: c_int = 65124;
pub const CLUTTER_dead_abovedot: c_int = 65110;
pub const CLUTTER_dead_abovereversedcomma: c_int = 65125;
pub const CLUTTER_dead_abovering: c_int = 65112;
pub const CLUTTER_dead_aboveverticalline: c_int = 65169;
pub const CLUTTER_dead_acute: c_int = 65105;
pub const CLUTTER_dead_belowbreve: c_int = 65131;
pub const CLUTTER_dead_belowcircumflex: c_int = 65129;
pub const CLUTTER_dead_belowcomma: c_int = 65134;
pub const CLUTTER_dead_belowdiaeresis: c_int = 65132;
pub const CLUTTER_dead_belowdot: c_int = 65120;
pub const CLUTTER_dead_belowmacron: c_int = 65128;
pub const CLUTTER_dead_belowring: c_int = 65127;
pub const CLUTTER_dead_belowtilde: c_int = 65130;
pub const CLUTTER_dead_belowverticalline: c_int = 65170;
pub const CLUTTER_dead_breve: c_int = 65109;
pub const CLUTTER_dead_capital_schwa: c_int = 65163;
pub const CLUTTER_dead_caron: c_int = 65114;
pub const CLUTTER_dead_cedilla: c_int = 65115;
pub const CLUTTER_dead_circumflex: c_int = 65106;
pub const CLUTTER_dead_currency: c_int = 65135;
pub const CLUTTER_dead_dasia: c_int = 65125;
pub const CLUTTER_dead_diaeresis: c_int = 65111;
pub const CLUTTER_dead_doubleacute: c_int = 65113;
pub const CLUTTER_dead_doublegrave: c_int = 65126;
pub const CLUTTER_dead_e: c_int = 65154;
pub const CLUTTER_dead_grave: c_int = 65104;
pub const CLUTTER_dead_greek: c_int = 65164;
pub const CLUTTER_dead_hook: c_int = 65121;
pub const CLUTTER_dead_horn: c_int = 65122;
pub const CLUTTER_dead_i: c_int = 65156;
pub const CLUTTER_dead_invertedbreve: c_int = 65133;
pub const CLUTTER_dead_iota: c_int = 65117;
pub const CLUTTER_dead_longsolidusoverlay: c_int = 65171;
pub const CLUTTER_dead_lowline: c_int = 65168;
pub const CLUTTER_dead_macron: c_int = 65108;
pub const CLUTTER_dead_o: c_int = 65158;
pub const CLUTTER_dead_ogonek: c_int = 65116;
pub const CLUTTER_dead_perispomeni: c_int = 65107;
pub const CLUTTER_dead_psili: c_int = 65124;
pub const CLUTTER_dead_semivoiced_sound: c_int = 65119;
pub const CLUTTER_dead_small_schwa: c_int = 65162;
pub const CLUTTER_dead_stroke: c_int = 65123;
pub const CLUTTER_dead_tilde: c_int = 65107;
pub const CLUTTER_dead_u: c_int = 65160;
pub const CLUTTER_dead_voiced_sound: c_int = 65118;
pub const CLUTTER_decimalpoint: c_int = 2749;
pub const CLUTTER_degree: c_int = 176;
pub const CLUTTER_diaeresis: c_int = 168;
pub const CLUTTER_diamond: c_int = 2797;
pub const CLUTTER_digitspace: c_int = 2725;
pub const CLUTTER_dintegral: c_int = 16785964;
pub const CLUTTER_division: c_int = 247;
pub const CLUTTER_dollar: c_int = 36;
pub const CLUTTER_doubbaselinedot: c_int = 2735;
pub const CLUTTER_doubleacute: c_int = 445;
pub const CLUTTER_doubledagger: c_int = 2802;
pub const CLUTTER_doublelowquotemark: c_int = 2814;
pub const CLUTTER_downarrow: c_int = 2302;
pub const CLUTTER_downcaret: c_int = 2984;
pub const CLUTTER_downshoe: c_int = 3030;
pub const CLUTTER_downstile: c_int = 3012;
pub const CLUTTER_downtack: c_int = 3010;
pub const CLUTTER_dstroke: c_int = 496;
pub const CLUTTER_e: c_int = 101;
pub const CLUTTER_eabovedot: c_int = 1004;
pub const CLUTTER_eacute: c_int = 233;
pub const CLUTTER_ebelowdot: c_int = 16785081;
pub const CLUTTER_ecaron: c_int = 492;
pub const CLUTTER_ecircumflex: c_int = 234;
pub const CLUTTER_ecircumflexacute: c_int = 16785087;
pub const CLUTTER_ecircumflexbelowdot: c_int = 16785095;
pub const CLUTTER_ecircumflexgrave: c_int = 16785089;
pub const CLUTTER_ecircumflexhook: c_int = 16785091;
pub const CLUTTER_ecircumflextilde: c_int = 16785093;
pub const CLUTTER_ediaeresis: c_int = 235;
pub const CLUTTER_egrave: c_int = 232;
pub const CLUTTER_ehook: c_int = 16785083;
pub const CLUTTER_eightsubscript: c_int = 16785544;
pub const CLUTTER_eightsuperior: c_int = 16785528;
pub const CLUTTER_elementof: c_int = 16785928;
pub const CLUTTER_ellipsis: c_int = 2734;
pub const CLUTTER_em3space: c_int = 2723;
pub const CLUTTER_em4space: c_int = 2724;
pub const CLUTTER_emacron: c_int = 954;
pub const CLUTTER_emdash: c_int = 2729;
pub const CLUTTER_emfilledcircle: c_int = 2782;
pub const CLUTTER_emfilledrect: c_int = 2783;
pub const CLUTTER_emopencircle: c_int = 2766;
pub const CLUTTER_emopenrectangle: c_int = 2767;
pub const CLUTTER_emptyset: c_int = 16785925;
pub const CLUTTER_emspace: c_int = 2721;
pub const CLUTTER_endash: c_int = 2730;
pub const CLUTTER_enfilledcircbullet: c_int = 2790;
pub const CLUTTER_enfilledsqbullet: c_int = 2791;
pub const CLUTTER_eng: c_int = 959;
pub const CLUTTER_enopencircbullet: c_int = 2784;
pub const CLUTTER_enopensquarebullet: c_int = 2785;
pub const CLUTTER_enspace: c_int = 2722;
pub const CLUTTER_eogonek: c_int = 490;
pub const CLUTTER_equal: c_int = 61;
pub const CLUTTER_eth: c_int = 240;
pub const CLUTTER_etilde: c_int = 16785085;
pub const CLUTTER_exclam: c_int = 33;
pub const CLUTTER_exclamdown: c_int = 161;
pub const CLUTTER_ezh: c_int = 16777874;
pub const CLUTTER_f: c_int = 102;
pub const CLUTTER_fabovedot: c_int = 16784927;
pub const CLUTTER_femalesymbol: c_int = 2808;
pub const CLUTTER_ff: c_int = 2531;
pub const CLUTTER_figdash: c_int = 2747;
pub const CLUTTER_filledlefttribullet: c_int = 2780;
pub const CLUTTER_filledrectbullet: c_int = 2779;
pub const CLUTTER_filledrighttribullet: c_int = 2781;
pub const CLUTTER_filledtribulletdown: c_int = 2793;
pub const CLUTTER_filledtribulletup: c_int = 2792;
pub const CLUTTER_fiveeighths: c_int = 2757;
pub const CLUTTER_fivesixths: c_int = 2743;
pub const CLUTTER_fivesubscript: c_int = 16785541;
pub const CLUTTER_fivesuperior: c_int = 16785525;
pub const CLUTTER_fourfifths: c_int = 2741;
pub const CLUTTER_foursubscript: c_int = 16785540;
pub const CLUTTER_foursuperior: c_int = 16785524;
pub const CLUTTER_fourthroot: c_int = 16785948;
pub const CLUTTER_function: c_int = 2294;
pub const CLUTTER_g: c_int = 103;
pub const CLUTTER_gabovedot: c_int = 757;
pub const CLUTTER_gbreve: c_int = 699;
pub const CLUTTER_gcaron: c_int = 16777703;
pub const CLUTTER_gcedilla: c_int = 955;
pub const CLUTTER_gcircumflex: c_int = 760;
pub const CLUTTER_grave: c_int = 96;
pub const CLUTTER_greater: c_int = 62;
pub const CLUTTER_greaterthanequal: c_int = 2238;
pub const CLUTTER_guillemotleft: c_int = 171;
pub const CLUTTER_guillemotright: c_int = 187;
pub const CLUTTER_h: c_int = 104;
pub const CLUTTER_hairspace: c_int = 2728;
pub const CLUTTER_hcircumflex: c_int = 694;
pub const CLUTTER_heart: c_int = 2798;
pub const CLUTTER_hebrew_aleph: c_int = 3296;
pub const CLUTTER_hebrew_ayin: c_int = 3314;
pub const CLUTTER_hebrew_bet: c_int = 3297;
pub const CLUTTER_hebrew_beth: c_int = 3297;
pub const CLUTTER_hebrew_chet: c_int = 3303;
pub const CLUTTER_hebrew_dalet: c_int = 3299;
pub const CLUTTER_hebrew_daleth: c_int = 3299;
pub const CLUTTER_hebrew_doublelowline: c_int = 3295;
pub const CLUTTER_hebrew_finalkaph: c_int = 3306;
pub const CLUTTER_hebrew_finalmem: c_int = 3309;
pub const CLUTTER_hebrew_finalnun: c_int = 3311;
pub const CLUTTER_hebrew_finalpe: c_int = 3315;
pub const CLUTTER_hebrew_finalzade: c_int = 3317;
pub const CLUTTER_hebrew_finalzadi: c_int = 3317;
pub const CLUTTER_hebrew_gimel: c_int = 3298;
pub const CLUTTER_hebrew_gimmel: c_int = 3298;
pub const CLUTTER_hebrew_he: c_int = 3300;
pub const CLUTTER_hebrew_het: c_int = 3303;
pub const CLUTTER_hebrew_kaph: c_int = 3307;
pub const CLUTTER_hebrew_kuf: c_int = 3319;
pub const CLUTTER_hebrew_lamed: c_int = 3308;
pub const CLUTTER_hebrew_mem: c_int = 3310;
pub const CLUTTER_hebrew_nun: c_int = 3312;
pub const CLUTTER_hebrew_pe: c_int = 3316;
pub const CLUTTER_hebrew_qoph: c_int = 3319;
pub const CLUTTER_hebrew_resh: c_int = 3320;
pub const CLUTTER_hebrew_samech: c_int = 3313;
pub const CLUTTER_hebrew_samekh: c_int = 3313;
pub const CLUTTER_hebrew_shin: c_int = 3321;
pub const CLUTTER_hebrew_taf: c_int = 3322;
pub const CLUTTER_hebrew_taw: c_int = 3322;
pub const CLUTTER_hebrew_tet: c_int = 3304;
pub const CLUTTER_hebrew_teth: c_int = 3304;
pub const CLUTTER_hebrew_waw: c_int = 3301;
pub const CLUTTER_hebrew_yod: c_int = 3305;
pub const CLUTTER_hebrew_zade: c_int = 3318;
pub const CLUTTER_hebrew_zadi: c_int = 3318;
pub const CLUTTER_hebrew_zain: c_int = 3302;
pub const CLUTTER_hebrew_zayin: c_int = 3302;
pub const CLUTTER_hexagram: c_int = 2778;
pub const CLUTTER_horizconnector: c_int = 2211;
pub const CLUTTER_horizlinescan1: c_int = 2543;
pub const CLUTTER_horizlinescan3: c_int = 2544;
pub const CLUTTER_horizlinescan5: c_int = 2545;
pub const CLUTTER_horizlinescan7: c_int = 2546;
pub const CLUTTER_horizlinescan9: c_int = 2547;
pub const CLUTTER_hstroke: c_int = 689;
pub const CLUTTER_ht: c_int = 2530;
pub const CLUTTER_hyphen: c_int = 173;
pub const CLUTTER_i: c_int = 105;
pub const CLUTTER_iTouch: c_int = 269025120;
pub const CLUTTER_iacute: c_int = 237;
pub const CLUTTER_ibelowdot: c_int = 16785099;
pub const CLUTTER_ibreve: c_int = 16777517;
pub const CLUTTER_icircumflex: c_int = 238;
pub const CLUTTER_identical: c_int = 2255;
pub const CLUTTER_idiaeresis: c_int = 239;
pub const CLUTTER_idotless: c_int = 697;
pub const CLUTTER_ifonlyif: c_int = 2253;
pub const CLUTTER_igrave: c_int = 236;
pub const CLUTTER_ihook: c_int = 16785097;
pub const CLUTTER_imacron: c_int = 1007;
pub const CLUTTER_implies: c_int = 2254;
pub const CLUTTER_includedin: c_int = 2266;
pub const CLUTTER_includes: c_int = 2267;
pub const CLUTTER_infinity: c_int = 2242;
pub const CLUTTER_integral: c_int = 2239;
pub const CLUTTER_intersection: c_int = 2268;
pub const CLUTTER_iogonek: c_int = 999;
pub const CLUTTER_itilde: c_int = 949;
pub const CLUTTER_j: c_int = 106;
pub const CLUTTER_jcircumflex: c_int = 700;
pub const CLUTTER_jot: c_int = 3018;
pub const CLUTTER_k: c_int = 107;
pub const CLUTTER_kana_A: c_int = 1201;
pub const CLUTTER_kana_CHI: c_int = 1217;
pub const CLUTTER_kana_E: c_int = 1204;
pub const CLUTTER_kana_FU: c_int = 1228;
pub const CLUTTER_kana_HA: c_int = 1226;
pub const CLUTTER_kana_HE: c_int = 1229;
pub const CLUTTER_kana_HI: c_int = 1227;
pub const CLUTTER_kana_HO: c_int = 1230;
pub const CLUTTER_kana_HU: c_int = 1228;
pub const CLUTTER_kana_I: c_int = 1202;
pub const CLUTTER_kana_KA: c_int = 1206;
pub const CLUTTER_kana_KE: c_int = 1209;
pub const CLUTTER_kana_KI: c_int = 1207;
pub const CLUTTER_kana_KO: c_int = 1210;
pub const CLUTTER_kana_KU: c_int = 1208;
pub const CLUTTER_kana_MA: c_int = 1231;
pub const CLUTTER_kana_ME: c_int = 1234;
pub const CLUTTER_kana_MI: c_int = 1232;
pub const CLUTTER_kana_MO: c_int = 1235;
pub const CLUTTER_kana_MU: c_int = 1233;
pub const CLUTTER_kana_N: c_int = 1245;
pub const CLUTTER_kana_NA: c_int = 1221;
pub const CLUTTER_kana_NE: c_int = 1224;
pub const CLUTTER_kana_NI: c_int = 1222;
pub const CLUTTER_kana_NO: c_int = 1225;
pub const CLUTTER_kana_NU: c_int = 1223;
pub const CLUTTER_kana_O: c_int = 1205;
pub const CLUTTER_kana_RA: c_int = 1239;
pub const CLUTTER_kana_RE: c_int = 1242;
pub const CLUTTER_kana_RI: c_int = 1240;
pub const CLUTTER_kana_RO: c_int = 1243;
pub const CLUTTER_kana_RU: c_int = 1241;
pub const CLUTTER_kana_SA: c_int = 1211;
pub const CLUTTER_kana_SE: c_int = 1214;
pub const CLUTTER_kana_SHI: c_int = 1212;
pub const CLUTTER_kana_SO: c_int = 1215;
pub const CLUTTER_kana_SU: c_int = 1213;
pub const CLUTTER_kana_TA: c_int = 1216;
pub const CLUTTER_kana_TE: c_int = 1219;
pub const CLUTTER_kana_TI: c_int = 1217;
pub const CLUTTER_kana_TO: c_int = 1220;
pub const CLUTTER_kana_TSU: c_int = 1218;
pub const CLUTTER_kana_TU: c_int = 1218;
pub const CLUTTER_kana_U: c_int = 1203;
pub const CLUTTER_kana_WA: c_int = 1244;
pub const CLUTTER_kana_WO: c_int = 1190;
pub const CLUTTER_kana_YA: c_int = 1236;
pub const CLUTTER_kana_YO: c_int = 1238;
pub const CLUTTER_kana_YU: c_int = 1237;
pub const CLUTTER_kana_a: c_int = 1191;
pub const CLUTTER_kana_closingbracket: c_int = 1187;
pub const CLUTTER_kana_comma: c_int = 1188;
pub const CLUTTER_kana_conjunctive: c_int = 1189;
pub const CLUTTER_kana_e: c_int = 1194;
pub const CLUTTER_kana_fullstop: c_int = 1185;
pub const CLUTTER_kana_i: c_int = 1192;
pub const CLUTTER_kana_middledot: c_int = 1189;
pub const CLUTTER_kana_o: c_int = 1195;
pub const CLUTTER_kana_openingbracket: c_int = 1186;
pub const CLUTTER_kana_switch: c_int = 65406;
pub const CLUTTER_kana_tsu: c_int = 1199;
pub const CLUTTER_kana_tu: c_int = 1199;
pub const CLUTTER_kana_u: c_int = 1193;
pub const CLUTTER_kana_ya: c_int = 1196;
pub const CLUTTER_kana_yo: c_int = 1198;
pub const CLUTTER_kana_yu: c_int = 1197;
pub const CLUTTER_kappa: c_int = 930;
pub const CLUTTER_kcedilla: c_int = 1011;
pub const CLUTTER_kra: c_int = 930;
pub const CLUTTER_l: c_int = 108;
pub const CLUTTER_lacute: c_int = 485;
pub const CLUTTER_latincross: c_int = 2777;
pub const CLUTTER_lbelowdot: c_int = 16784951;
pub const CLUTTER_lcaron: c_int = 437;
pub const CLUTTER_lcedilla: c_int = 950;
pub const CLUTTER_leftanglebracket: c_int = 2748;
pub const CLUTTER_leftarrow: c_int = 2299;
pub const CLUTTER_leftcaret: c_int = 2979;
pub const CLUTTER_leftdoublequotemark: c_int = 2770;
pub const CLUTTER_leftmiddlecurlybrace: c_int = 2223;
pub const CLUTTER_leftopentriangle: c_int = 2764;
pub const CLUTTER_leftpointer: c_int = 2794;
pub const CLUTTER_leftradical: c_int = 2209;
pub const CLUTTER_leftshoe: c_int = 3034;
pub const CLUTTER_leftsinglequotemark: c_int = 2768;
pub const CLUTTER_leftt: c_int = 2548;
pub const CLUTTER_lefttack: c_int = 3036;
pub const CLUTTER_less: c_int = 60;
pub const CLUTTER_lessthanequal: c_int = 2236;
pub const CLUTTER_lf: c_int = 2533;
pub const CLUTTER_logicaland: c_int = 2270;
pub const CLUTTER_logicalor: c_int = 2271;
pub const CLUTTER_lowleftcorner: c_int = 2541;
pub const CLUTTER_lowrightcorner: c_int = 2538;
pub const CLUTTER_lstroke: c_int = 435;
pub const CLUTTER_m: c_int = 109;
pub const CLUTTER_mabovedot: c_int = 16784961;
pub const CLUTTER_macron: c_int = 175;
pub const CLUTTER_malesymbol: c_int = 2807;
pub const CLUTTER_maltesecross: c_int = 2800;
pub const CLUTTER_marker: c_int = 2751;
pub const CLUTTER_masculine: c_int = 186;
pub const CLUTTER_minus: c_int = 45;
pub const CLUTTER_minutes: c_int = 2774;
pub const CLUTTER_mu: c_int = 181;
pub const CLUTTER_multiply: c_int = 215;
pub const CLUTTER_musicalflat: c_int = 2806;
pub const CLUTTER_musicalsharp: c_int = 2805;
pub const CLUTTER_n: c_int = 110;
pub const CLUTTER_nabla: c_int = 2245;
pub const CLUTTER_nacute: c_int = 497;
pub const CLUTTER_ncaron: c_int = 498;
pub const CLUTTER_ncedilla: c_int = 1009;
pub const CLUTTER_ninesubscript: c_int = 16785545;
pub const CLUTTER_ninesuperior: c_int = 16785529;
pub const CLUTTER_nl: c_int = 2536;
pub const CLUTTER_nobreakspace: c_int = 160;
pub const CLUTTER_notapproxeq: c_int = 16785991;
pub const CLUTTER_notelementof: c_int = 16785929;
pub const CLUTTER_notequal: c_int = 2237;
pub const CLUTTER_notidentical: c_int = 16786018;
pub const CLUTTER_notsign: c_int = 172;
pub const CLUTTER_ntilde: c_int = 241;
pub const CLUTTER_numbersign: c_int = 35;
pub const CLUTTER_numerosign: c_int = 1712;
pub const CLUTTER_o: c_int = 111;
pub const CLUTTER_oacute: c_int = 243;
pub const CLUTTER_obarred: c_int = 16777845;
pub const CLUTTER_obelowdot: c_int = 16785101;
pub const CLUTTER_ocaron: c_int = 16777682;
pub const CLUTTER_ocircumflex: c_int = 244;
pub const CLUTTER_ocircumflexacute: c_int = 16785105;
pub const CLUTTER_ocircumflexbelowdot: c_int = 16785113;
pub const CLUTTER_ocircumflexgrave: c_int = 16785107;
pub const CLUTTER_ocircumflexhook: c_int = 16785109;
pub const CLUTTER_ocircumflextilde: c_int = 16785111;
pub const CLUTTER_odiaeresis: c_int = 246;
pub const CLUTTER_odoubleacute: c_int = 501;
pub const CLUTTER_oe: c_int = 5053;
pub const CLUTTER_ogonek: c_int = 434;
pub const CLUTTER_ograve: c_int = 242;
pub const CLUTTER_ohook: c_int = 16785103;
pub const CLUTTER_ohorn: c_int = 16777633;
pub const CLUTTER_ohornacute: c_int = 16785115;
pub const CLUTTER_ohornbelowdot: c_int = 16785123;
pub const CLUTTER_ohorngrave: c_int = 16785117;
pub const CLUTTER_ohornhook: c_int = 16785119;
pub const CLUTTER_ohorntilde: c_int = 16785121;
pub const CLUTTER_omacron: c_int = 1010;
pub const CLUTTER_oneeighth: c_int = 2755;
pub const CLUTTER_onefifth: c_int = 2738;
pub const CLUTTER_onehalf: c_int = 189;
pub const CLUTTER_onequarter: c_int = 188;
pub const CLUTTER_onesixth: c_int = 2742;
pub const CLUTTER_onesubscript: c_int = 16785537;
pub const CLUTTER_onesuperior: c_int = 185;
pub const CLUTTER_onethird: c_int = 2736;
pub const CLUTTER_ooblique: c_int = 248;
pub const CLUTTER_openrectbullet: c_int = 2786;
pub const CLUTTER_openstar: c_int = 2789;
pub const CLUTTER_opentribulletdown: c_int = 2788;
pub const CLUTTER_opentribulletup: c_int = 2787;
pub const CLUTTER_ordfeminine: c_int = 170;
pub const CLUTTER_oslash: c_int = 248;
pub const CLUTTER_otilde: c_int = 245;
pub const CLUTTER_overbar: c_int = 3008;
pub const CLUTTER_overline: c_int = 1150;
pub const CLUTTER_p: c_int = 112;
pub const CLUTTER_pabovedot: c_int = 16784983;
pub const CLUTTER_paragraph: c_int = 182;
pub const CLUTTER_parenleft: c_int = 40;
pub const CLUTTER_parenright: c_int = 41;
pub const CLUTTER_partdifferential: c_int = 16785922;
pub const CLUTTER_partialderivative: c_int = 2287;
pub const CLUTTER_percent: c_int = 37;
pub const CLUTTER_period: c_int = 46;
pub const CLUTTER_periodcentered: c_int = 183;
pub const CLUTTER_permille: c_int = 2773;
pub const CLUTTER_phonographcopyright: c_int = 2811;
pub const CLUTTER_plus: c_int = 43;
pub const CLUTTER_plusminus: c_int = 177;
pub const CLUTTER_prescription: c_int = 2772;
pub const CLUTTER_prolongedsound: c_int = 1200;
pub const CLUTTER_punctspace: c_int = 2726;
pub const CLUTTER_q: c_int = 113;
pub const CLUTTER_quad: c_int = 3020;
pub const CLUTTER_question: c_int = 63;
pub const CLUTTER_questiondown: c_int = 191;
pub const CLUTTER_quotedbl: c_int = 34;
pub const CLUTTER_quoteleft: c_int = 96;
pub const CLUTTER_quoteright: c_int = 39;
pub const CLUTTER_r: c_int = 114;
pub const CLUTTER_racute: c_int = 480;
pub const CLUTTER_radical: c_int = 2262;
pub const CLUTTER_rcaron: c_int = 504;
pub const CLUTTER_rcedilla: c_int = 947;
pub const CLUTTER_registered: c_int = 174;
pub const CLUTTER_rightanglebracket: c_int = 2750;
pub const CLUTTER_rightarrow: c_int = 2301;
pub const CLUTTER_rightcaret: c_int = 2982;
pub const CLUTTER_rightdoublequotemark: c_int = 2771;
pub const CLUTTER_rightmiddlecurlybrace: c_int = 2224;
pub const CLUTTER_rightmiddlesummation: c_int = 2231;
pub const CLUTTER_rightopentriangle: c_int = 2765;
pub const CLUTTER_rightpointer: c_int = 2795;
pub const CLUTTER_rightshoe: c_int = 3032;
pub const CLUTTER_rightsinglequotemark: c_int = 2769;
pub const CLUTTER_rightt: c_int = 2549;
pub const CLUTTER_righttack: c_int = 3068;
pub const CLUTTER_s: c_int = 115;
pub const CLUTTER_sabovedot: c_int = 16784993;
pub const CLUTTER_sacute: c_int = 438;
pub const CLUTTER_scaron: c_int = 441;
pub const CLUTTER_scedilla: c_int = 442;
pub const CLUTTER_schwa: c_int = 16777817;
pub const CLUTTER_scircumflex: c_int = 766;
pub const CLUTTER_script_switch: c_int = 65406;
pub const CLUTTER_seconds: c_int = 2775;
pub const CLUTTER_section: c_int = 167;
pub const CLUTTER_semicolon: c_int = 59;
pub const CLUTTER_semivoicedsound: c_int = 1247;
pub const CLUTTER_seveneighths: c_int = 2758;
pub const CLUTTER_sevensubscript: c_int = 16785543;
pub const CLUTTER_sevensuperior: c_int = 16785527;
pub const CLUTTER_signaturemark: c_int = 2762;
pub const CLUTTER_signifblank: c_int = 2732;
pub const CLUTTER_similarequal: c_int = 2249;
pub const CLUTTER_singlelowquotemark: c_int = 2813;
pub const CLUTTER_sixsubscript: c_int = 16785542;
pub const CLUTTER_sixsuperior: c_int = 16785526;
pub const CLUTTER_slash: c_int = 47;
pub const CLUTTER_soliddiamond: c_int = 2528;
pub const CLUTTER_space: c_int = 32;
pub const CLUTTER_squareroot: c_int = 16785946;
pub const CLUTTER_ssharp: c_int = 223;
pub const CLUTTER_sterling: c_int = 163;
pub const CLUTTER_stricteq: c_int = 16786019;
pub const CLUTTER_t: c_int = 116;
pub const CLUTTER_tabovedot: c_int = 16785003;
pub const CLUTTER_tcaron: c_int = 443;
pub const CLUTTER_tcedilla: c_int = 510;
pub const CLUTTER_telephone: c_int = 2809;
pub const CLUTTER_telephonerecorder: c_int = 2810;
pub const CLUTTER_therefore: c_int = 2240;
pub const CLUTTER_thinspace: c_int = 2727;
pub const CLUTTER_thorn: c_int = 254;
pub const CLUTTER_threeeighths: c_int = 2756;
pub const CLUTTER_threefifths: c_int = 2740;
pub const CLUTTER_threequarters: c_int = 190;
pub const CLUTTER_threesubscript: c_int = 16785539;
pub const CLUTTER_threesuperior: c_int = 179;
pub const CLUTTER_tintegral: c_int = 16785965;
pub const CLUTTER_topintegral: c_int = 2212;
pub const CLUTTER_topleftparens: c_int = 2219;
pub const CLUTTER_topleftradical: c_int = 2210;
pub const CLUTTER_topleftsqbracket: c_int = 2215;
pub const CLUTTER_topleftsummation: c_int = 2225;
pub const CLUTTER_toprightparens: c_int = 2221;
pub const CLUTTER_toprightsqbracket: c_int = 2217;
pub const CLUTTER_toprightsummation: c_int = 2229;
pub const CLUTTER_topt: c_int = 2551;
pub const CLUTTER_topvertsummationconnector: c_int = 2227;
pub const CLUTTER_trademark: c_int = 2761;
pub const CLUTTER_trademarkincircle: c_int = 2763;
pub const CLUTTER_tslash: c_int = 956;
pub const CLUTTER_twofifths: c_int = 2739;
pub const CLUTTER_twosubscript: c_int = 16785538;
pub const CLUTTER_twosuperior: c_int = 178;
pub const CLUTTER_twothirds: c_int = 2737;
pub const CLUTTER_u: c_int = 117;
pub const CLUTTER_uacute: c_int = 250;
pub const CLUTTER_ubelowdot: c_int = 16785125;
pub const CLUTTER_ubreve: c_int = 765;
pub const CLUTTER_ucircumflex: c_int = 251;
pub const CLUTTER_udiaeresis: c_int = 252;
pub const CLUTTER_udoubleacute: c_int = 507;
pub const CLUTTER_ugrave: c_int = 249;
pub const CLUTTER_uhook: c_int = 16785127;
pub const CLUTTER_uhorn: c_int = 16777648;
pub const CLUTTER_uhornacute: c_int = 16785129;
pub const CLUTTER_uhornbelowdot: c_int = 16785137;
pub const CLUTTER_uhorngrave: c_int = 16785131;
pub const CLUTTER_uhornhook: c_int = 16785133;
pub const CLUTTER_uhorntilde: c_int = 16785135;
pub const CLUTTER_umacron: c_int = 1022;
pub const CLUTTER_underbar: c_int = 3014;
pub const CLUTTER_underscore: c_int = 95;
pub const CLUTTER_union: c_int = 2269;
pub const CLUTTER_uogonek: c_int = 1017;
pub const CLUTTER_uparrow: c_int = 2300;
pub const CLUTTER_upcaret: c_int = 2985;
pub const CLUTTER_upleftcorner: c_int = 2540;
pub const CLUTTER_uprightcorner: c_int = 2539;
pub const CLUTTER_upshoe: c_int = 3011;
pub const CLUTTER_upstile: c_int = 3027;
pub const CLUTTER_uptack: c_int = 3022;
pub const CLUTTER_uring: c_int = 505;
pub const CLUTTER_utilde: c_int = 1021;
pub const CLUTTER_v: c_int = 118;
pub const CLUTTER_variation: c_int = 2241;
pub const CLUTTER_vertbar: c_int = 2552;
pub const CLUTTER_vertconnector: c_int = 2214;
pub const CLUTTER_voicedsound: c_int = 1246;
pub const CLUTTER_vt: c_int = 2537;
pub const CLUTTER_w: c_int = 119;
pub const CLUTTER_wacute: c_int = 16785027;
pub const CLUTTER_wcircumflex: c_int = 16777589;
pub const CLUTTER_wdiaeresis: c_int = 16785029;
pub const CLUTTER_wgrave: c_int = 16785025;
pub const CLUTTER_x: c_int = 120;
pub const CLUTTER_xabovedot: c_int = 16785035;
pub const CLUTTER_y: c_int = 121;
pub const CLUTTER_yacute: c_int = 253;
pub const CLUTTER_ybelowdot: c_int = 16785141;
pub const CLUTTER_ycircumflex: c_int = 16777591;
pub const CLUTTER_ydiaeresis: c_int = 255;
pub const CLUTTER_yen: c_int = 165;
pub const CLUTTER_ygrave: c_int = 16785139;
pub const CLUTTER_yhook: c_int = 16785143;
pub const CLUTTER_ytilde: c_int = 16785145;
pub const CLUTTER_z: c_int = 122;
pub const CLUTTER_zabovedot: c_int = 447;
pub const CLUTTER_zacute: c_int = 444;
pub const CLUTTER_zcaron: c_int = 446;
pub const CLUTTER_zerosubscript: c_int = 16785536;
pub const CLUTTER_zerosuperior: c_int = 16785520;
pub const CLUTTER_zstroke: c_int = 16777654;

// Flags
pub type ClutterActorFlags = c_uint;
pub const CLUTTER_ACTOR_MAPPED: ClutterActorFlags = 2;
pub const CLUTTER_ACTOR_REALIZED: ClutterActorFlags = 4;
pub const CLUTTER_ACTOR_REACTIVE: ClutterActorFlags = 8;
pub const CLUTTER_ACTOR_VISIBLE: ClutterActorFlags = 16;
pub const CLUTTER_ACTOR_NO_LAYOUT: ClutterActorFlags = 32;

pub type ClutterAllocationFlags = c_uint;
pub const CLUTTER_ALLOCATION_NONE: ClutterAllocationFlags = 0;
pub const CLUTTER_ABSOLUTE_ORIGIN_CHANGED: ClutterAllocationFlags = 2;
pub const CLUTTER_DELEGATE_LAYOUT: ClutterAllocationFlags = 4;

pub type ClutterContentRepeat = c_uint;
pub const CLUTTER_REPEAT_NONE: ClutterContentRepeat = 0;
pub const CLUTTER_REPEAT_X_AXIS: ClutterContentRepeat = 1;
pub const CLUTTER_REPEAT_Y_AXIS: ClutterContentRepeat = 2;
pub const CLUTTER_REPEAT_BOTH: ClutterContentRepeat = 3;

pub type ClutterEffectPaintFlags = c_uint;
pub const CLUTTER_EFFECT_PAINT_ACTOR_DIRTY: ClutterEffectPaintFlags = 1;

pub type ClutterEventFlags = c_uint;
pub const CLUTTER_EVENT_NONE: ClutterEventFlags = 0;
pub const CLUTTER_EVENT_FLAG_SYNTHETIC: ClutterEventFlags = 1;

pub type ClutterFeatureFlags = c_uint;
pub const CLUTTER_FEATURE_TEXTURE_NPOT: ClutterFeatureFlags = 4;
pub const CLUTTER_FEATURE_SYNC_TO_VBLANK: ClutterFeatureFlags = 8;
pub const CLUTTER_FEATURE_TEXTURE_YUV: ClutterFeatureFlags = 16;
pub const CLUTTER_FEATURE_TEXTURE_READ_PIXELS: ClutterFeatureFlags = 32;
pub const CLUTTER_FEATURE_STAGE_STATIC: ClutterFeatureFlags = 64;
pub const CLUTTER_FEATURE_STAGE_USER_RESIZE: ClutterFeatureFlags = 128;
pub const CLUTTER_FEATURE_STAGE_CURSOR: ClutterFeatureFlags = 256;
pub const CLUTTER_FEATURE_SHADERS_GLSL: ClutterFeatureFlags = 512;
pub const CLUTTER_FEATURE_OFFSCREEN: ClutterFeatureFlags = 1024;
pub const CLUTTER_FEATURE_STAGE_MULTIPLE: ClutterFeatureFlags = 2048;
pub const CLUTTER_FEATURE_SWAP_EVENTS: ClutterFeatureFlags = 4096;

pub type ClutterFontFlags = c_uint;
pub const CLUTTER_FONT_MIPMAPPING: ClutterFontFlags = 1;
pub const CLUTTER_FONT_HINTING: ClutterFontFlags = 2;

pub type ClutterModifierType = c_uint;
pub const CLUTTER_SHIFT_MASK: ClutterModifierType = 1;
pub const CLUTTER_LOCK_MASK: ClutterModifierType = 2;
pub const CLUTTER_CONTROL_MASK: ClutterModifierType = 4;
pub const CLUTTER_MOD1_MASK: ClutterModifierType = 8;
pub const CLUTTER_MOD2_MASK: ClutterModifierType = 16;
pub const CLUTTER_MOD3_MASK: ClutterModifierType = 32;
pub const CLUTTER_MOD4_MASK: ClutterModifierType = 64;
pub const CLUTTER_MOD5_MASK: ClutterModifierType = 128;
pub const CLUTTER_BUTTON1_MASK: ClutterModifierType = 256;
pub const CLUTTER_BUTTON2_MASK: ClutterModifierType = 512;
pub const CLUTTER_BUTTON3_MASK: ClutterModifierType = 1024;
pub const CLUTTER_BUTTON4_MASK: ClutterModifierType = 2048;
pub const CLUTTER_BUTTON5_MASK: ClutterModifierType = 4096;
pub const CLUTTER_MODIFIER_RESERVED_13_MASK: ClutterModifierType = 8192;
pub const CLUTTER_MODIFIER_RESERVED_14_MASK: ClutterModifierType = 16384;
pub const CLUTTER_MODIFIER_RESERVED_15_MASK: ClutterModifierType = 32768;
pub const CLUTTER_MODIFIER_RESERVED_16_MASK: ClutterModifierType = 65536;
pub const CLUTTER_MODIFIER_RESERVED_17_MASK: ClutterModifierType = 131072;
pub const CLUTTER_MODIFIER_RESERVED_18_MASK: ClutterModifierType = 262144;
pub const CLUTTER_MODIFIER_RESERVED_19_MASK: ClutterModifierType = 524288;
pub const CLUTTER_MODIFIER_RESERVED_20_MASK: ClutterModifierType = 1048576;
pub const CLUTTER_MODIFIER_RESERVED_21_MASK: ClutterModifierType = 2097152;
pub const CLUTTER_MODIFIER_RESERVED_22_MASK: ClutterModifierType = 4194304;
pub const CLUTTER_MODIFIER_RESERVED_23_MASK: ClutterModifierType = 8388608;
pub const CLUTTER_MODIFIER_RESERVED_24_MASK: ClutterModifierType = 16777216;
pub const CLUTTER_MODIFIER_RESERVED_25_MASK: ClutterModifierType = 33554432;
pub const CLUTTER_SUPER_MASK: ClutterModifierType = 67108864;
pub const CLUTTER_HYPER_MASK: ClutterModifierType = 134217728;
pub const CLUTTER_META_MASK: ClutterModifierType = 268435456;
pub const CLUTTER_MODIFIER_RESERVED_29_MASK: ClutterModifierType = 536870912;
pub const CLUTTER_RELEASE_MASK: ClutterModifierType = 1073741824;
pub const CLUTTER_MODIFIER_MASK: ClutterModifierType = 1543512063;

pub type ClutterOffscreenRedirect = c_uint;
pub const CLUTTER_OFFSCREEN_REDIRECT_AUTOMATIC_FOR_OPACITY: ClutterOffscreenRedirect = 1;
pub const CLUTTER_OFFSCREEN_REDIRECT_ALWAYS: ClutterOffscreenRedirect = 2;

pub type ClutterRepaintFlags = c_uint;
pub const CLUTTER_REPAINT_FLAGS_PRE_PAINT: ClutterRepaintFlags = 1;
pub const CLUTTER_REPAINT_FLAGS_POST_PAINT: ClutterRepaintFlags = 2;
pub const CLUTTER_REPAINT_FLAGS_QUEUE_REDRAW_ON_ADD: ClutterRepaintFlags = 4;

pub type ClutterScrollFinishFlags = c_uint;
pub const CLUTTER_SCROLL_FINISHED_NONE: ClutterScrollFinishFlags = 0;
pub const CLUTTER_SCROLL_FINISHED_HORIZONTAL: ClutterScrollFinishFlags = 1;
pub const CLUTTER_SCROLL_FINISHED_VERTICAL: ClutterScrollFinishFlags = 2;

pub type ClutterScrollMode = c_uint;
pub const CLUTTER_SCROLL_NONE: ClutterScrollMode = 0;
pub const CLUTTER_SCROLL_HORIZONTALLY: ClutterScrollMode = 1;
pub const CLUTTER_SCROLL_VERTICALLY: ClutterScrollMode = 2;
pub const CLUTTER_SCROLL_BOTH: ClutterScrollMode = 3;

pub type ClutterStageState = c_uint;
pub const CLUTTER_STAGE_STATE_FULLSCREEN: ClutterStageState = 2;
pub const CLUTTER_STAGE_STATE_OFFSCREEN: ClutterStageState = 4;
pub const CLUTTER_STAGE_STATE_ACTIVATED: ClutterStageState = 8;

pub type ClutterSwipeDirection = c_uint;
pub const CLUTTER_SWIPE_DIRECTION_UP: ClutterSwipeDirection = 1;
pub const CLUTTER_SWIPE_DIRECTION_DOWN: ClutterSwipeDirection = 2;
pub const CLUTTER_SWIPE_DIRECTION_LEFT: ClutterSwipeDirection = 4;
pub const CLUTTER_SWIPE_DIRECTION_RIGHT: ClutterSwipeDirection = 8;

pub type ClutterTextureFlags = c_uint;
pub const CLUTTER_TEXTURE_NONE: ClutterTextureFlags = 0;
pub const CLUTTER_TEXTURE_RGB_FLAG_BGR: ClutterTextureFlags = 2;
pub const CLUTTER_TEXTURE_RGB_FLAG_PREMULT: ClutterTextureFlags = 4;
pub const CLUTTER_TEXTURE_YUV_FLAG_YUV2: ClutterTextureFlags = 8;

// Unions
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClutterEvent {
    pub type_: ClutterEventType,
    pub any: ClutterAnyEvent,
    pub button: ClutterButtonEvent,
    pub key: ClutterKeyEvent,
    pub motion: ClutterMotionEvent,
    pub scroll: ClutterScrollEvent,
    pub stage_state: ClutterStageStateEvent,
    pub crossing: ClutterCrossingEvent,
    pub touch: ClutterTouchEvent,
    pub touchpad_pinch: ClutterTouchpadPinchEvent,
    pub touchpad_swipe: ClutterTouchpadSwipeEvent,
}

impl ::std::fmt::Debug for ClutterEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterEvent @ {:?}", self as *const _))
            .finish()
    }
}

// Callbacks
pub type ClutterActorCreateChildFunc =
    Option<unsafe extern "C" fn(*mut gobject::GObject, gpointer) -> *mut ClutterActor>;
pub type ClutterAlphaFunc = Option<unsafe extern "C" fn(*mut ClutterAlpha, gpointer) -> c_double>;
pub type ClutterBehaviourForeachFunc =
    Option<unsafe extern "C" fn(*mut ClutterBehaviour, *mut ClutterActor, gpointer)>;
pub type ClutterBindingActionFunc = Option<
    unsafe extern "C" fn(
        *mut gobject::GObject,
        *const c_char,
        c_uint,
        ClutterModifierType,
        gpointer,
    ) -> gboolean,
>;
pub type ClutterCallback = Option<unsafe extern "C" fn(*mut ClutterActor, gpointer)>;
pub type ClutterEventFilterFunc =
    Option<unsafe extern "C" fn(*const ClutterEvent, gpointer) -> gboolean>;
pub type ClutterModelFilterFunc =
    Option<unsafe extern "C" fn(*mut ClutterModel, *mut ClutterModelIter, gpointer) -> gboolean>;
pub type ClutterModelForeachFunc =
    Option<unsafe extern "C" fn(*mut ClutterModel, *mut ClutterModelIter, gpointer) -> gboolean>;
pub type ClutterModelSortFunc = Option<
    unsafe extern "C" fn(
        *mut ClutterModel,
        *const gobject::GValue,
        *const gobject::GValue,
        gpointer,
    ) -> c_int,
>;
pub type ClutterPathCallback = Option<unsafe extern "C" fn(*const ClutterPathNode, gpointer)>;
pub type ClutterProgressFunc = Option<
    unsafe extern "C" fn(
        *const gobject::GValue,
        *const gobject::GValue,
        c_double,
        *mut gobject::GValue,
    ) -> gboolean,
>;
pub type ClutterScriptConnectFunc = Option<
    unsafe extern "C" fn(
        *mut ClutterScript,
        *mut gobject::GObject,
        *const c_char,
        *const c_char,
        *mut gobject::GObject,
        gobject::GConnectFlags,
        gpointer,
    ),
>;
pub type ClutterTimelineProgressFunc =
    Option<unsafe extern "C" fn(*mut ClutterTimeline, c_double, c_double, gpointer) -> c_double>;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterActionClass {
    pub parent_class: ClutterActorMetaClass,
    pub _clutter_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_action4: Option<unsafe extern "C" fn()>,
    pub _clutter_action5: Option<unsafe extern "C" fn()>,
    pub _clutter_action6: Option<unsafe extern "C" fn()>,
    pub _clutter_action7: Option<unsafe extern "C" fn()>,
    pub _clutter_action8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterActionClass @ {:?}", self as *const _))
            .field("_clutter_action1", &self._clutter_action1)
            .field("_clutter_action2", &self._clutter_action2)
            .field("_clutter_action3", &self._clutter_action3)
            .field("_clutter_action4", &self._clutter_action4)
            .field("_clutter_action5", &self._clutter_action5)
            .field("_clutter_action6", &self._clutter_action6)
            .field("_clutter_action7", &self._clutter_action7)
            .field("_clutter_action8", &self._clutter_action8)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterActorBox {
    pub x1: c_float,
    pub y1: c_float,
    pub x2: c_float,
    pub y2: c_float,
}

impl ::std::fmt::Debug for ClutterActorBox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterActorBox @ {:?}", self as *const _))
            .field("x1", &self.x1)
            .field("y1", &self.y1)
            .field("x2", &self.x2)
            .field("y2", &self.y2)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterActorClass {
    pub parent_class: gobject::GInitiallyUnownedClass,
    pub show: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub show_all: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub hide: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub hide_all: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub realize: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub unrealize: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub map: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub unmap: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub paint: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub parent_set: Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterActor)>,
    pub destroy: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub pick: Option<unsafe extern "C" fn(*mut ClutterActor, *const ClutterColor)>,
    pub queue_redraw: Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterActor)>,
    pub get_preferred_width:
        Option<unsafe extern "C" fn(*mut ClutterActor, c_float, *mut c_float, *mut c_float)>,
    pub get_preferred_height:
        Option<unsafe extern "C" fn(*mut ClutterActor, c_float, *mut c_float, *mut c_float)>,
    pub allocate: Option<
        unsafe extern "C" fn(*mut ClutterActor, *const ClutterActorBox, ClutterAllocationFlags),
    >,
    pub apply_transform: Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterMatrix)>,
    pub event: Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterEvent) -> gboolean>,
    pub button_press_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterButtonEvent) -> gboolean>,
    pub button_release_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterButtonEvent) -> gboolean>,
    pub scroll_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterScrollEvent) -> gboolean>,
    pub key_press_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterKeyEvent) -> gboolean>,
    pub key_release_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterKeyEvent) -> gboolean>,
    pub motion_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterMotionEvent) -> gboolean>,
    pub enter_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterCrossingEvent) -> gboolean>,
    pub leave_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterCrossingEvent) -> gboolean>,
    pub captured_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterEvent) -> gboolean>,
    pub key_focus_in: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub key_focus_out: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub queue_relayout: Option<unsafe extern "C" fn(*mut ClutterActor)>,
    pub get_accessible: Option<unsafe extern "C" fn(*mut ClutterActor) -> *mut atk::AtkObject>,
    pub get_paint_volume:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterPaintVolume) -> gboolean>,
    pub has_overlaps: Option<unsafe extern "C" fn(*mut ClutterActor) -> gboolean>,
    pub paint_node: Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterPaintNode)>,
    pub touch_event:
        Option<unsafe extern "C" fn(*mut ClutterActor, *mut ClutterTouchEvent) -> gboolean>,
    pub _padding_dummy: [gpointer; 26],
}

impl ::std::fmt::Debug for ClutterActorClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterActorClass @ {:?}", self as *const _))
            .field("show", &self.show)
            .field("show_all", &self.show_all)
            .field("hide", &self.hide)
            .field("hide_all", &self.hide_all)
            .field("realize", &self.realize)
            .field("unrealize", &self.unrealize)
            .field("map", &self.map)
            .field("unmap", &self.unmap)
            .field("paint", &self.paint)
            .field("parent_set", &self.parent_set)
            .field("destroy", &self.destroy)
            .field("pick", &self.pick)
            .field("queue_redraw", &self.queue_redraw)
            .field("get_preferred_width", &self.get_preferred_width)
            .field("get_preferred_height", &self.get_preferred_height)
            .field("allocate", &self.allocate)
            .field("apply_transform", &self.apply_transform)
            .field("event", &self.event)
            .field("button_press_event", &self.button_press_event)
            .field("button_release_event", &self.button_release_event)
            .field("scroll_event", &self.scroll_event)
            .field("key_press_event", &self.key_press_event)
            .field("key_release_event", &self.key_release_event)
            .field("motion_event", &self.motion_event)
            .field("enter_event", &self.enter_event)
            .field("leave_event", &self.leave_event)
            .field("captured_event", &self.captured_event)
            .field("key_focus_in", &self.key_focus_in)
            .field("key_focus_out", &self.key_focus_out)
            .field("queue_relayout", &self.queue_relayout)
            .field("get_accessible", &self.get_accessible)
            .field("get_paint_volume", &self.get_paint_volume)
            .field("has_overlaps", &self.has_overlaps)
            .field("paint_node", &self.paint_node)
            .field("touch_event", &self.touch_event)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterActorIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: c_int,
    pub dummy5: gpointer,
}

impl ::std::fmt::Debug for ClutterActorIter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterActorIter @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterActorMetaClass {
    pub parent_class: gobject::GInitiallyUnownedClass,
    pub set_actor: Option<unsafe extern "C" fn(*mut ClutterActorMeta, *mut ClutterActor)>,
    pub _clutter_meta1: Option<unsafe extern "C" fn()>,
    pub _clutter_meta2: Option<unsafe extern "C" fn()>,
    pub _clutter_meta3: Option<unsafe extern "C" fn()>,
    pub _clutter_meta4: Option<unsafe extern "C" fn()>,
    pub _clutter_meta5: Option<unsafe extern "C" fn()>,
    pub _clutter_meta6: Option<unsafe extern "C" fn()>,
    pub _clutter_meta7: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterActorMetaClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterActorMetaClass @ {:?}", self as *const _))
            .field("set_actor", &self.set_actor)
            .field("_clutter_meta1", &self._clutter_meta1)
            .field("_clutter_meta2", &self._clutter_meta2)
            .field("_clutter_meta3", &self._clutter_meta3)
            .field("_clutter_meta4", &self._clutter_meta4)
            .field("_clutter_meta5", &self._clutter_meta5)
            .field("_clutter_meta6", &self._clutter_meta6)
            .field("_clutter_meta7", &self._clutter_meta7)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterActorMetaPrivate(c_void);

pub type ClutterActorMetaPrivate = *mut _ClutterActorMetaPrivate;

#[repr(C)]
pub struct _ClutterActorPrivate(c_void);

pub type ClutterActorPrivate = *mut _ClutterActorPrivate;

#[repr(C)]
pub struct _ClutterAlignConstraintClass(c_void);

pub type ClutterAlignConstraintClass = *mut _ClutterAlignConstraintClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterAlphaClass {
    pub parent_class: gobject::GInitiallyUnownedClass,
    pub _clutter_alpha_1: Option<unsafe extern "C" fn()>,
    pub _clutter_alpha_2: Option<unsafe extern "C" fn()>,
    pub _clutter_alpha_3: Option<unsafe extern "C" fn()>,
    pub _clutter_alpha_4: Option<unsafe extern "C" fn()>,
    pub _clutter_alpha_5: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterAlphaClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAlphaClass @ {:?}", self as *const _))
            .field("_clutter_alpha_1", &self._clutter_alpha_1)
            .field("_clutter_alpha_2", &self._clutter_alpha_2)
            .field("_clutter_alpha_3", &self._clutter_alpha_3)
            .field("_clutter_alpha_4", &self._clutter_alpha_4)
            .field("_clutter_alpha_5", &self._clutter_alpha_5)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterAlphaPrivate(c_void);

pub type ClutterAlphaPrivate = *mut _ClutterAlphaPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterAnimatableIface {
    pub parent_iface: gobject::GTypeInterface,
    pub animate_property: Option<
        unsafe extern "C" fn(
            *mut ClutterAnimatable,
            *mut ClutterAnimation,
            *const c_char,
            *const gobject::GValue,
            *const gobject::GValue,
            c_double,
            *mut gobject::GValue,
        ) -> gboolean,
    >,
    pub find_property: Option<
        unsafe extern "C" fn(*mut ClutterAnimatable, *const c_char) -> *mut gobject::GParamSpec,
    >,
    pub get_initial_state:
        Option<unsafe extern "C" fn(*mut ClutterAnimatable, *const c_char, *const gobject::GValue)>,
    pub set_final_state:
        Option<unsafe extern "C" fn(*mut ClutterAnimatable, *const c_char, *const gobject::GValue)>,
    pub interpolate_value: Option<
        unsafe extern "C" fn(
            *mut ClutterAnimatable,
            *const c_char,
            *mut ClutterInterval,
            c_double,
            *mut gobject::GValue,
        ) -> gboolean,
    >,
}

impl ::std::fmt::Debug for ClutterAnimatableIface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAnimatableIface @ {:?}", self as *const _))
            .field("animate_property", &self.animate_property)
            .field("find_property", &self.find_property)
            .field("get_initial_state", &self.get_initial_state)
            .field("set_final_state", &self.set_final_state)
            .field("interpolate_value", &self.interpolate_value)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterAnimationClass {
    pub parent_class: gobject::GObjectClass,
    pub started: Option<unsafe extern "C" fn(*mut ClutterAnimation)>,
    pub completed: Option<unsafe extern "C" fn(*mut ClutterAnimation)>,
    pub _clutter_reserved1: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved2: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved3: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved4: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved5: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved6: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved7: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterAnimationClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAnimationClass @ {:?}", self as *const _))
            .field("started", &self.started)
            .field("completed", &self.completed)
            .field("_clutter_reserved1", &self._clutter_reserved1)
            .field("_clutter_reserved2", &self._clutter_reserved2)
            .field("_clutter_reserved3", &self._clutter_reserved3)
            .field("_clutter_reserved4", &self._clutter_reserved4)
            .field("_clutter_reserved5", &self._clutter_reserved5)
            .field("_clutter_reserved6", &self._clutter_reserved6)
            .field("_clutter_reserved7", &self._clutter_reserved7)
            .field("_clutter_reserved8", &self._clutter_reserved8)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterAnimationPrivate(c_void);

pub type ClutterAnimationPrivate = *mut _ClutterAnimationPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterAnimatorClass {
    pub parent_class: gobject::GObjectClass,
    pub _padding_dummy: [gpointer; 16],
}

impl ::std::fmt::Debug for ClutterAnimatorClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAnimatorClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterAnimatorKey(c_void);

impl ::std::fmt::Debug for ClutterAnimatorKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAnimatorKey @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterAnimatorPrivate(c_void);

pub type ClutterAnimatorPrivate = *mut _ClutterAnimatorPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterAnyEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
}

impl ::std::fmt::Debug for ClutterAnyEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAnyEvent @ {:?}", self as *const _))
            .field("type_", &self.type_)
            .field("time", &self.time)
            .field("flags", &self.flags)
            .field("stage", &self.stage)
            .field("source", &self.source)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterBackendClass(c_void);

pub type ClutterBackendClass = *mut _ClutterBackendClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourClass {
    pub parent_class: gobject::GObjectClass,
    pub alpha_notify: Option<unsafe extern "C" fn(*mut ClutterBehaviour, c_double)>,
    pub applied: Option<unsafe extern "C" fn(*mut ClutterBehaviour, *mut ClutterActor)>,
    pub removed: Option<unsafe extern "C" fn(*mut ClutterBehaviour, *mut ClutterActor)>,
    pub _clutter_behaviour1: Option<unsafe extern "C" fn()>,
    pub _clutter_behaviour2: Option<unsafe extern "C" fn()>,
    pub _clutter_behaviour3: Option<unsafe extern "C" fn()>,
    pub _clutter_behaviour4: Option<unsafe extern "C" fn()>,
    pub _clutter_behaviour5: Option<unsafe extern "C" fn()>,
    pub _clutter_behaviour6: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterBehaviourClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBehaviourClass @ {:?}", self as *const _))
            .field("alpha_notify", &self.alpha_notify)
            .field("applied", &self.applied)
            .field("removed", &self.removed)
            .field("_clutter_behaviour1", &self._clutter_behaviour1)
            .field("_clutter_behaviour2", &self._clutter_behaviour2)
            .field("_clutter_behaviour3", &self._clutter_behaviour3)
            .field("_clutter_behaviour4", &self._clutter_behaviour4)
            .field("_clutter_behaviour5", &self._clutter_behaviour5)
            .field("_clutter_behaviour6", &self._clutter_behaviour6)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourDepthClass {
    pub parent_class: ClutterBehaviourClass,
}

impl ::std::fmt::Debug for ClutterBehaviourDepthClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterBehaviourDepthClass @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterBehaviourDepthPrivate(c_void);

pub type ClutterBehaviourDepthPrivate = *mut _ClutterBehaviourDepthPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourEllipseClass {
    pub parent_class: ClutterBehaviourClass,
}

impl ::std::fmt::Debug for ClutterBehaviourEllipseClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterBehaviourEllipseClass @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterBehaviourEllipsePrivate(c_void);

pub type ClutterBehaviourEllipsePrivate = *mut _ClutterBehaviourEllipsePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourOpacityClass {
    pub parent_class: ClutterBehaviourClass,
}

impl ::std::fmt::Debug for ClutterBehaviourOpacityClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterBehaviourOpacityClass @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterBehaviourOpacityPrivate(c_void);

pub type ClutterBehaviourOpacityPrivate = *mut _ClutterBehaviourOpacityPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourPathClass {
    pub parent_class: ClutterBehaviourClass,
    pub knot_reached: Option<unsafe extern "C" fn(*mut ClutterBehaviourPath, c_uint)>,
    pub _clutter_path_1: Option<unsafe extern "C" fn()>,
    pub _clutter_path_2: Option<unsafe extern "C" fn()>,
    pub _clutter_path_3: Option<unsafe extern "C" fn()>,
    pub _clutter_path_4: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterBehaviourPathClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterBehaviourPathClass @ {:?}",
            self as *const _
        ))
        .field("knot_reached", &self.knot_reached)
        .field("_clutter_path_1", &self._clutter_path_1)
        .field("_clutter_path_2", &self._clutter_path_2)
        .field("_clutter_path_3", &self._clutter_path_3)
        .field("_clutter_path_4", &self._clutter_path_4)
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterBehaviourPathPrivate(c_void);

pub type ClutterBehaviourPathPrivate = *mut _ClutterBehaviourPathPrivate;

#[repr(C)]
pub struct _ClutterBehaviourPrivate(c_void);

pub type ClutterBehaviourPrivate = *mut _ClutterBehaviourPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourRotateClass {
    pub parent_class: ClutterBehaviourClass,
}

impl ::std::fmt::Debug for ClutterBehaviourRotateClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterBehaviourRotateClass @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterBehaviourRotatePrivate(c_void);

pub type ClutterBehaviourRotatePrivate = *mut _ClutterBehaviourRotatePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourScaleClass {
    pub parent_class: ClutterBehaviourClass,
}

impl ::std::fmt::Debug for ClutterBehaviourScaleClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterBehaviourScaleClass @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterBehaviourScalePrivate(c_void);

pub type ClutterBehaviourScalePrivate = *mut _ClutterBehaviourScalePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBinLayoutClass {
    pub parent_class: ClutterLayoutManagerClass,
}

impl ::std::fmt::Debug for ClutterBinLayoutClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBinLayoutClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterBinLayoutPrivate(c_void);

pub type ClutterBinLayoutPrivate = *mut _ClutterBinLayoutPrivate;

#[repr(C)]
pub struct _ClutterBindConstraintClass(c_void);

pub type ClutterBindConstraintClass = *mut _ClutterBindConstraintClass;

#[repr(C)]
pub struct _ClutterBindingPoolClass(c_void);

pub type ClutterBindingPoolClass = *mut _ClutterBindingPoolClass;

#[repr(C)]
pub struct _ClutterBlurEffectClass(c_void);

pub type ClutterBlurEffectClass = *mut _ClutterBlurEffectClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBoxClass {
    pub parent_class: ClutterActorClass,
    pub clutter_padding_1: Option<unsafe extern "C" fn()>,
    pub clutter_padding_2: Option<unsafe extern "C" fn()>,
    pub clutter_padding_3: Option<unsafe extern "C" fn()>,
    pub clutter_padding_4: Option<unsafe extern "C" fn()>,
    pub clutter_padding_5: Option<unsafe extern "C" fn()>,
    pub clutter_padding_6: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterBoxClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBoxClass @ {:?}", self as *const _))
            .field("clutter_padding_1", &self.clutter_padding_1)
            .field("clutter_padding_2", &self.clutter_padding_2)
            .field("clutter_padding_3", &self.clutter_padding_3)
            .field("clutter_padding_4", &self.clutter_padding_4)
            .field("clutter_padding_5", &self.clutter_padding_5)
            .field("clutter_padding_6", &self.clutter_padding_6)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBoxLayoutClass {
    pub parent_class: ClutterLayoutManagerClass,
}

impl ::std::fmt::Debug for ClutterBoxLayoutClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBoxLayoutClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterBoxLayoutPrivate(c_void);

pub type ClutterBoxLayoutPrivate = *mut _ClutterBoxLayoutPrivate;

#[repr(C)]
pub struct _ClutterBoxPrivate(c_void);

pub type ClutterBoxPrivate = *mut _ClutterBoxPrivate;

#[repr(C)]
pub struct _ClutterBrightnessContrastEffectClass(c_void);

pub type ClutterBrightnessContrastEffectClass = *mut _ClutterBrightnessContrastEffectClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterButtonEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
    pub x: c_float,
    pub y: c_float,
    pub modifier_state: ClutterModifierType,
    pub button: u32,
    pub click_count: c_uint,
    pub axes: *mut c_double,
    pub device: *mut ClutterInputDevice,
}

impl ::std::fmt::Debug for ClutterButtonEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterButtonEvent @ {:?}", self as *const _))
            .field("type_", &self.type_)
            .field("time", &self.time)
            .field("flags", &self.flags)
            .field("stage", &self.stage)
            .field("source", &self.source)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("modifier_state", &self.modifier_state)
            .field("button", &self.button)
            .field("click_count", &self.click_count)
            .field("axes", &self.axes)
            .field("device", &self.device)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterCairoTextureClass {
    pub parent_class: ClutterTextureClass,
    pub create_surface: Option<
        unsafe extern "C" fn(
            *mut ClutterCairoTexture,
            c_uint,
            c_uint,
        ) -> *mut cairo::cairo_surface_t,
    >,
    pub draw:
        Option<unsafe extern "C" fn(*mut ClutterCairoTexture, *mut cairo::cairo_t) -> gboolean>,
    pub _clutter_cairo_3: Option<unsafe extern "C" fn()>,
    pub _clutter_cairo_4: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterCairoTextureClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterCairoTextureClass @ {:?}",
            self as *const _
        ))
        .field("create_surface", &self.create_surface)
        .field("draw", &self.draw)
        .field("_clutter_cairo_3", &self._clutter_cairo_3)
        .field("_clutter_cairo_4", &self._clutter_cairo_4)
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterCairoTexturePrivate(c_void);

pub type ClutterCairoTexturePrivate = *mut _ClutterCairoTexturePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterCanvasClass {
    pub parent_class: gobject::GObjectClass,
    pub draw: Option<
        unsafe extern "C" fn(*mut ClutterCanvas, *mut cairo::cairo_t, c_int, c_int) -> gboolean,
    >,
    pub _padding: [gpointer; 16],
}

impl ::std::fmt::Debug for ClutterCanvasClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterCanvasClass @ {:?}", self as *const _))
            .field("draw", &self.draw)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterCanvasPrivate(c_void);

pub type ClutterCanvasPrivate = *mut _ClutterCanvasPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterChildMetaClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ClutterChildMetaClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterChildMetaClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterClickActionClass {
    pub parent_class: ClutterActionClass,
    pub clicked: Option<unsafe extern "C" fn(*mut ClutterClickAction, *mut ClutterActor)>,
    pub long_press: Option<
        unsafe extern "C" fn(
            *mut ClutterClickAction,
            *mut ClutterActor,
            ClutterLongPressState,
        ) -> gboolean,
    >,
    pub _clutter_click_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_click_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_click_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_click_action4: Option<unsafe extern "C" fn()>,
    pub _clutter_click_action5: Option<unsafe extern "C" fn()>,
    pub _clutter_click_action6: Option<unsafe extern "C" fn()>,
    pub _clutter_click_action7: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterClickActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterClickActionClass @ {:?}", self as *const _))
            .field("clicked", &self.clicked)
            .field("long_press", &self.long_press)
            .field("_clutter_click_action1", &self._clutter_click_action1)
            .field("_clutter_click_action2", &self._clutter_click_action2)
            .field("_clutter_click_action3", &self._clutter_click_action3)
            .field("_clutter_click_action4", &self._clutter_click_action4)
            .field("_clutter_click_action5", &self._clutter_click_action5)
            .field("_clutter_click_action6", &self._clutter_click_action6)
            .field("_clutter_click_action7", &self._clutter_click_action7)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterClickActionPrivate(c_void);

pub type ClutterClickActionPrivate = *mut _ClutterClickActionPrivate;

#[repr(C)]
pub struct _ClutterClipNodeClass(c_void);

pub type ClutterClipNodeClass = *mut _ClutterClipNodeClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterCloneClass {
    pub parent_class: ClutterActorClass,
    pub _clutter_actor_clone1: Option<unsafe extern "C" fn()>,
    pub _clutter_actor_clone2: Option<unsafe extern "C" fn()>,
    pub _clutter_actor_clone3: Option<unsafe extern "C" fn()>,
    pub _clutter_actor_clone4: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterCloneClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterCloneClass @ {:?}", self as *const _))
            .field("_clutter_actor_clone1", &self._clutter_actor_clone1)
            .field("_clutter_actor_clone2", &self._clutter_actor_clone2)
            .field("_clutter_actor_clone3", &self._clutter_actor_clone3)
            .field("_clutter_actor_clone4", &self._clutter_actor_clone4)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterClonePrivate(c_void);

pub type ClutterClonePrivate = *mut _ClutterClonePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl ::std::fmt::Debug for ClutterColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterColor @ {:?}", self as *const _))
            .field("red", &self.red)
            .field("green", &self.green)
            .field("blue", &self.blue)
            .field("alpha", &self.alpha)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterColorNodeClass(c_void);

pub type ClutterColorNodeClass = *mut _ClutterColorNodeClass;

#[repr(C)]
pub struct _ClutterColorizeEffectClass(c_void);

pub type ClutterColorizeEffectClass = *mut _ClutterColorizeEffectClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterConstraintClass {
    pub parent_class: ClutterActorMetaClass,
    pub update_allocation: Option<
        unsafe extern "C" fn(*mut ClutterConstraint, *mut ClutterActor, *mut ClutterActorBox),
    >,
    pub update_preferred_size: Option<
        unsafe extern "C" fn(
            *mut ClutterConstraint,
            *mut ClutterActor,
            ClutterOrientation,
            c_float,
            *mut c_float,
            *mut c_float,
        ),
    >,
    pub _clutter_constraint1: Option<unsafe extern "C" fn()>,
    pub _clutter_constraint2: Option<unsafe extern "C" fn()>,
    pub _clutter_constraint3: Option<unsafe extern "C" fn()>,
    pub _clutter_constraint4: Option<unsafe extern "C" fn()>,
    pub _clutter_constraint5: Option<unsafe extern "C" fn()>,
    pub _clutter_constraint6: Option<unsafe extern "C" fn()>,
    pub _clutter_constraint7: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterConstraintClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterConstraintClass @ {:?}", self as *const _))
            .field("update_allocation", &self.update_allocation)
            .field("update_preferred_size", &self.update_preferred_size)
            .field("_clutter_constraint1", &self._clutter_constraint1)
            .field("_clutter_constraint2", &self._clutter_constraint2)
            .field("_clutter_constraint3", &self._clutter_constraint3)
            .field("_clutter_constraint4", &self._clutter_constraint4)
            .field("_clutter_constraint5", &self._clutter_constraint5)
            .field("_clutter_constraint6", &self._clutter_constraint6)
            .field("_clutter_constraint7", &self._clutter_constraint7)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterContainerIface {
    pub g_iface: gobject::GTypeInterface,
    pub add: Option<unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor)>,
    pub remove: Option<unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor)>,
    pub foreach: Option<unsafe extern "C" fn(*mut ClutterContainer, ClutterCallback, gpointer)>,
    pub foreach_with_internals:
        Option<unsafe extern "C" fn(*mut ClutterContainer, ClutterCallback, gpointer)>,
    pub raise:
        Option<unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor, *mut ClutterActor)>,
    pub lower:
        Option<unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor, *mut ClutterActor)>,
    pub sort_depth_order: Option<unsafe extern "C" fn(*mut ClutterContainer)>,
    pub child_meta_type: GType,
    pub create_child_meta: Option<unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor)>,
    pub destroy_child_meta: Option<unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor)>,
    pub get_child_meta: Option<
        unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor) -> *mut ClutterChildMeta,
    >,
    pub actor_added: Option<unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor)>,
    pub actor_removed: Option<unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor)>,
    pub child_notify: Option<
        unsafe extern "C" fn(*mut ClutterContainer, *mut ClutterActor, *mut gobject::GParamSpec),
    >,
}

impl ::std::fmt::Debug for ClutterContainerIface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterContainerIface @ {:?}", self as *const _))
            .field("add", &self.add)
            .field("remove", &self.remove)
            .field("foreach", &self.foreach)
            .field("foreach_with_internals", &self.foreach_with_internals)
            .field("raise", &self.raise)
            .field("lower", &self.lower)
            .field("sort_depth_order", &self.sort_depth_order)
            .field("child_meta_type", &self.child_meta_type)
            .field("create_child_meta", &self.create_child_meta)
            .field("destroy_child_meta", &self.destroy_child_meta)
            .field("get_child_meta", &self.get_child_meta)
            .field("actor_added", &self.actor_added)
            .field("actor_removed", &self.actor_removed)
            .field("child_notify", &self.child_notify)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterContentIface {
    pub g_iface: gobject::GTypeInterface,
    pub get_preferred_size:
        Option<unsafe extern "C" fn(*mut ClutterContent, *mut c_float, *mut c_float) -> gboolean>,
    pub paint_content:
        Option<unsafe extern "C" fn(*mut ClutterContent, *mut ClutterActor, *mut ClutterPaintNode)>,
    pub attached: Option<unsafe extern "C" fn(*mut ClutterContent, *mut ClutterActor)>,
    pub detached: Option<unsafe extern "C" fn(*mut ClutterContent, *mut ClutterActor)>,
    pub invalidate: Option<unsafe extern "C" fn(*mut ClutterContent)>,
}

impl ::std::fmt::Debug for ClutterContentIface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterContentIface @ {:?}", self as *const _))
            .field("get_preferred_size", &self.get_preferred_size)
            .field("paint_content", &self.paint_content)
            .field("attached", &self.attached)
            .field("detached", &self.detached)
            .field("invalidate", &self.invalidate)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterCrossingEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
    pub x: c_float,
    pub y: c_float,
    pub device: *mut ClutterInputDevice,
    pub related: *mut ClutterActor,
}

impl ::std::fmt::Debug for ClutterCrossingEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterCrossingEvent @ {:?}", self as *const _))
            .field("type_", &self.type_)
            .field("time", &self.time)
            .field("flags", &self.flags)
            .field("stage", &self.stage)
            .field("source", &self.source)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("device", &self.device)
            .field("related", &self.related)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterDeformEffectClass {
    pub parent_class: ClutterOffscreenEffectClass,
    pub deform_vertex: Option<
        unsafe extern "C" fn(
            *mut ClutterDeformEffect,
            c_float,
            c_float,
            *mut cogl::CoglTextureVertex,
        ),
    >,
    pub _clutter_deform1: Option<unsafe extern "C" fn()>,
    pub _clutter_deform2: Option<unsafe extern "C" fn()>,
    pub _clutter_deform3: Option<unsafe extern "C" fn()>,
    pub _clutter_deform4: Option<unsafe extern "C" fn()>,
    pub _clutter_deform5: Option<unsafe extern "C" fn()>,
    pub _clutter_deform6: Option<unsafe extern "C" fn()>,
    pub _clutter_deform7: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterDeformEffectClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterDeformEffectClass @ {:?}",
            self as *const _
        ))
        .field("deform_vertex", &self.deform_vertex)
        .field("_clutter_deform1", &self._clutter_deform1)
        .field("_clutter_deform2", &self._clutter_deform2)
        .field("_clutter_deform3", &self._clutter_deform3)
        .field("_clutter_deform4", &self._clutter_deform4)
        .field("_clutter_deform5", &self._clutter_deform5)
        .field("_clutter_deform6", &self._clutter_deform6)
        .field("_clutter_deform7", &self._clutter_deform7)
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterDeformEffectPrivate(c_void);

pub type ClutterDeformEffectPrivate = *mut _ClutterDeformEffectPrivate;

#[repr(C)]
pub struct _ClutterDesaturateEffectClass(c_void);

pub type ClutterDesaturateEffectClass = *mut _ClutterDesaturateEffectClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterDeviceManagerClass {
    pub parent_class: gobject::GObjectClass,
    pub get_devices: Option<unsafe extern "C" fn(*mut ClutterDeviceManager) -> *const glib::GSList>,
    pub get_core_device: Option<
        unsafe extern "C" fn(
            *mut ClutterDeviceManager,
            ClutterInputDeviceType,
        ) -> *mut ClutterInputDevice,
    >,
    pub get_device:
        Option<unsafe extern "C" fn(*mut ClutterDeviceManager, c_int) -> *mut ClutterInputDevice>,
    pub add_device:
        Option<unsafe extern "C" fn(*mut ClutterDeviceManager, *mut ClutterInputDevice)>,
    pub remove_device:
        Option<unsafe extern "C" fn(*mut ClutterDeviceManager, *mut ClutterInputDevice)>,
    pub select_stage_events:
        Option<unsafe extern "C" fn(*mut ClutterDeviceManager, *mut ClutterStage)>,
    pub _padding: [gpointer; 7],
}

impl ::std::fmt::Debug for ClutterDeviceManagerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterDeviceManagerClass @ {:?}",
            self as *const _
        ))
        .field("get_devices", &self.get_devices)
        .field("get_core_device", &self.get_core_device)
        .field("get_device", &self.get_device)
        .field("add_device", &self.add_device)
        .field("remove_device", &self.remove_device)
        .field("select_stage_events", &self.select_stage_events)
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterDeviceManagerPrivate(c_void);

pub type ClutterDeviceManagerPrivate = *mut _ClutterDeviceManagerPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterDragActionClass {
    pub parent_class: ClutterActionClass,
    pub drag_begin: Option<
        unsafe extern "C" fn(
            *mut ClutterDragAction,
            *mut ClutterActor,
            c_float,
            c_float,
            ClutterModifierType,
        ),
    >,
    pub drag_motion:
        Option<unsafe extern "C" fn(*mut ClutterDragAction, *mut ClutterActor, c_float, c_float)>,
    pub drag_end: Option<
        unsafe extern "C" fn(
            *mut ClutterDragAction,
            *mut ClutterActor,
            c_float,
            c_float,
            ClutterModifierType,
        ),
    >,
    pub drag_progress: Option<
        unsafe extern "C" fn(
            *mut ClutterDragAction,
            *mut ClutterActor,
            c_float,
            c_float,
        ) -> gboolean,
    >,
    pub _clutter_drag_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_drag_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_drag_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_drag_action4: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterDragActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterDragActionClass @ {:?}", self as *const _))
            .field("drag_begin", &self.drag_begin)
            .field("drag_motion", &self.drag_motion)
            .field("drag_end", &self.drag_end)
            .field("drag_progress", &self.drag_progress)
            .field("_clutter_drag_action1", &self._clutter_drag_action1)
            .field("_clutter_drag_action2", &self._clutter_drag_action2)
            .field("_clutter_drag_action3", &self._clutter_drag_action3)
            .field("_clutter_drag_action4", &self._clutter_drag_action4)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterDragActionPrivate(c_void);

pub type ClutterDragActionPrivate = *mut _ClutterDragActionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterDropActionClass {
    pub parent_class: ClutterActionClass,
    pub can_drop: Option<
        unsafe extern "C" fn(
            *mut ClutterDropAction,
            *mut ClutterActor,
            c_float,
            c_float,
        ) -> gboolean,
    >,
    pub over_in: Option<unsafe extern "C" fn(*mut ClutterDropAction, *mut ClutterActor)>,
    pub over_out: Option<unsafe extern "C" fn(*mut ClutterDropAction, *mut ClutterActor)>,
    pub drop:
        Option<unsafe extern "C" fn(*mut ClutterDropAction, *mut ClutterActor, c_float, c_float)>,
    pub _clutter_drop_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_drop_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_drop_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_drop_action4: Option<unsafe extern "C" fn()>,
    pub _clutter_drop_action5: Option<unsafe extern "C" fn()>,
    pub _clutter_drop_action6: Option<unsafe extern "C" fn()>,
    pub _clutter_drop_action7: Option<unsafe extern "C" fn()>,
    pub _clutter_drop_action8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterDropActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterDropActionClass @ {:?}", self as *const _))
            .field("can_drop", &self.can_drop)
            .field("over_in", &self.over_in)
            .field("over_out", &self.over_out)
            .field("drop", &self.drop)
            .field("_clutter_drop_action1", &self._clutter_drop_action1)
            .field("_clutter_drop_action2", &self._clutter_drop_action2)
            .field("_clutter_drop_action3", &self._clutter_drop_action3)
            .field("_clutter_drop_action4", &self._clutter_drop_action4)
            .field("_clutter_drop_action5", &self._clutter_drop_action5)
            .field("_clutter_drop_action6", &self._clutter_drop_action6)
            .field("_clutter_drop_action7", &self._clutter_drop_action7)
            .field("_clutter_drop_action8", &self._clutter_drop_action8)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterDropActionPrivate(c_void);

pub type ClutterDropActionPrivate = *mut _ClutterDropActionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterEffectClass {
    pub parent_class: ClutterActorMetaClass,
    pub pre_paint: Option<unsafe extern "C" fn(*mut ClutterEffect) -> gboolean>,
    pub post_paint: Option<unsafe extern "C" fn(*mut ClutterEffect)>,
    pub get_paint_volume:
        Option<unsafe extern "C" fn(*mut ClutterEffect, *mut ClutterPaintVolume) -> gboolean>,
    pub paint: Option<unsafe extern "C" fn(*mut ClutterEffect, ClutterEffectPaintFlags)>,
    pub pick: Option<unsafe extern "C" fn(*mut ClutterEffect, ClutterEffectPaintFlags)>,
    pub _clutter_effect4: Option<unsafe extern "C" fn()>,
    pub _clutter_effect5: Option<unsafe extern "C" fn()>,
    pub _clutter_effect6: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterEffectClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterEffectClass @ {:?}", self as *const _))
            .field("pre_paint", &self.pre_paint)
            .field("post_paint", &self.post_paint)
            .field("get_paint_volume", &self.get_paint_volume)
            .field("paint", &self.paint)
            .field("pick", &self.pick)
            .field("_clutter_effect4", &self._clutter_effect4)
            .field("_clutter_effect5", &self._clutter_effect5)
            .field("_clutter_effect6", &self._clutter_effect6)
            .finish()
    }
}

#[repr(C)]
pub struct ClutterEventSequence(c_void);

impl ::std::fmt::Debug for ClutterEventSequence {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterEventSequence @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterFixedLayoutClass {
    pub parent_class: ClutterLayoutManagerClass,
}

impl ::std::fmt::Debug for ClutterFixedLayoutClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterFixedLayoutClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterFlowLayoutClass {
    pub parent_class: ClutterLayoutManagerClass,
}

impl ::std::fmt::Debug for ClutterFlowLayoutClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterFlowLayoutClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterFlowLayoutPrivate(c_void);

pub type ClutterFlowLayoutPrivate = *mut _ClutterFlowLayoutPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterFog {
    pub z_near: c_float,
    pub z_far: c_float,
}

impl ::std::fmt::Debug for ClutterFog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterFog @ {:?}", self as *const _))
            .field("z_near", &self.z_near)
            .field("z_far", &self.z_far)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterGeometry {
    pub x: c_int,
    pub y: c_int,
    pub width: c_uint,
    pub height: c_uint,
}

impl ::std::fmt::Debug for ClutterGeometry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterGeometry @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterGestureActionClass {
    pub parent_class: ClutterActionClass,
    pub gesture_begin:
        Option<unsafe extern "C" fn(*mut ClutterGestureAction, *mut ClutterActor) -> gboolean>,
    pub gesture_progress:
        Option<unsafe extern "C" fn(*mut ClutterGestureAction, *mut ClutterActor) -> gboolean>,
    pub gesture_end: Option<unsafe extern "C" fn(*mut ClutterGestureAction, *mut ClutterActor)>,
    pub gesture_cancel: Option<unsafe extern "C" fn(*mut ClutterGestureAction, *mut ClutterActor)>,
    pub gesture_prepare:
        Option<unsafe extern "C" fn(*mut ClutterGestureAction, *mut ClutterActor) -> gboolean>,
    pub _clutter_gesture_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_gesture_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_gesture_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_gesture_action4: Option<unsafe extern "C" fn()>,
    pub _clutter_gesture_action5: Option<unsafe extern "C" fn()>,
    pub _clutter_gesture_action6: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterGestureActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterGestureActionClass @ {:?}",
            self as *const _
        ))
        .field("gesture_begin", &self.gesture_begin)
        .field("gesture_progress", &self.gesture_progress)
        .field("gesture_end", &self.gesture_end)
        .field("gesture_cancel", &self.gesture_cancel)
        .field("gesture_prepare", &self.gesture_prepare)
        .field("_clutter_gesture_action1", &self._clutter_gesture_action1)
        .field("_clutter_gesture_action2", &self._clutter_gesture_action2)
        .field("_clutter_gesture_action3", &self._clutter_gesture_action3)
        .field("_clutter_gesture_action4", &self._clutter_gesture_action4)
        .field("_clutter_gesture_action5", &self._clutter_gesture_action5)
        .field("_clutter_gesture_action6", &self._clutter_gesture_action6)
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterGestureActionPrivate(c_void);

pub type ClutterGestureActionPrivate = *mut _ClutterGestureActionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterGridLayoutClass {
    pub parent_class: ClutterLayoutManagerClass,
    pub _padding: [gpointer; 8],
}

impl ::std::fmt::Debug for ClutterGridLayoutClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterGridLayoutClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterGridLayoutPrivate(c_void);

pub type ClutterGridLayoutPrivate = *mut _ClutterGridLayoutPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterGroupClass {
    pub parent_class: ClutterActorClass,
    pub _clutter_reserved1: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved2: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved3: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved4: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved5: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved6: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterGroupClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterGroupClass @ {:?}", self as *const _))
            .field("_clutter_reserved1", &self._clutter_reserved1)
            .field("_clutter_reserved2", &self._clutter_reserved2)
            .field("_clutter_reserved3", &self._clutter_reserved3)
            .field("_clutter_reserved4", &self._clutter_reserved4)
            .field("_clutter_reserved5", &self._clutter_reserved5)
            .field("_clutter_reserved6", &self._clutter_reserved6)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterGroupPrivate(c_void);

pub type ClutterGroupPrivate = *mut _ClutterGroupPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterImageClass {
    pub parent_class: gobject::GObjectClass,
    pub _padding: [gpointer; 16],
}

impl ::std::fmt::Debug for ClutterImageClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterImageClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterImagePrivate(c_void);

pub type ClutterImagePrivate = *mut _ClutterImagePrivate;

#[repr(C)]
pub struct _ClutterInputDeviceClass(c_void);

pub type ClutterInputDeviceClass = *mut _ClutterInputDeviceClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterIntervalClass {
    pub parent_class: gobject::GInitiallyUnownedClass,
    pub validate:
        Option<unsafe extern "C" fn(*mut ClutterInterval, *mut gobject::GParamSpec) -> gboolean>,
    pub compute_value: Option<
        unsafe extern "C" fn(*mut ClutterInterval, c_double, *mut gobject::GValue) -> gboolean,
    >,
    pub _clutter_reserved1: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved2: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved3: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved4: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved5: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved6: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterIntervalClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterIntervalClass @ {:?}", self as *const _))
            .field("validate", &self.validate)
            .field("compute_value", &self.compute_value)
            .field("_clutter_reserved1", &self._clutter_reserved1)
            .field("_clutter_reserved2", &self._clutter_reserved2)
            .field("_clutter_reserved3", &self._clutter_reserved3)
            .field("_clutter_reserved4", &self._clutter_reserved4)
            .field("_clutter_reserved5", &self._clutter_reserved5)
            .field("_clutter_reserved6", &self._clutter_reserved6)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterIntervalPrivate(c_void);

pub type ClutterIntervalPrivate = *mut _ClutterIntervalPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterKeyEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
    pub modifier_state: ClutterModifierType,
    pub keyval: c_uint,
    pub hardware_keycode: u16,
    pub unicode_value: u32,
    pub device: *mut ClutterInputDevice,
}

impl ::std::fmt::Debug for ClutterKeyEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterKeyEvent @ {:?}", self as *const _))
            .field("type_", &self.type_)
            .field("time", &self.time)
            .field("flags", &self.flags)
            .field("stage", &self.stage)
            .field("source", &self.source)
            .field("modifier_state", &self.modifier_state)
            .field("keyval", &self.keyval)
            .field("hardware_keycode", &self.hardware_keycode)
            .field("unicode_value", &self.unicode_value)
            .field("device", &self.device)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterKeyframeTransitionClass {
    pub parent_class: ClutterPropertyTransitionClass,
    pub _padding: [gpointer; 8],
}

impl ::std::fmt::Debug for ClutterKeyframeTransitionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterKeyframeTransitionClass @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterKeyframeTransitionPrivate(c_void);

pub type ClutterKeyframeTransitionPrivate = *mut _ClutterKeyframeTransitionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterKnot {
    pub x: c_int,
    pub y: c_int,
}

impl ::std::fmt::Debug for ClutterKnot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterKnot @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterLayoutManagerClass {
    pub parent_class: gobject::GInitiallyUnownedClass,
    pub get_preferred_width: Option<
        unsafe extern "C" fn(
            *mut ClutterLayoutManager,
            *mut ClutterContainer,
            c_float,
            *mut c_float,
            *mut c_float,
        ),
    >,
    pub get_preferred_height: Option<
        unsafe extern "C" fn(
            *mut ClutterLayoutManager,
            *mut ClutterContainer,
            c_float,
            *mut c_float,
            *mut c_float,
        ),
    >,
    pub allocate: Option<
        unsafe extern "C" fn(
            *mut ClutterLayoutManager,
            *mut ClutterContainer,
            *const ClutterActorBox,
            ClutterAllocationFlags,
        ),
    >,
    pub set_container:
        Option<unsafe extern "C" fn(*mut ClutterLayoutManager, *mut ClutterContainer)>,
    pub get_child_meta_type: Option<unsafe extern "C" fn(*mut ClutterLayoutManager) -> GType>,
    pub create_child_meta: Option<
        unsafe extern "C" fn(
            *mut ClutterLayoutManager,
            *mut ClutterContainer,
            *mut ClutterActor,
        ) -> *mut ClutterLayoutMeta,
    >,
    pub begin_animation: Option<
        unsafe extern "C" fn(*mut ClutterLayoutManager, c_uint, c_ulong) -> *mut ClutterAlpha,
    >,
    pub get_animation_progress: Option<unsafe extern "C" fn(*mut ClutterLayoutManager) -> c_double>,
    pub end_animation: Option<unsafe extern "C" fn(*mut ClutterLayoutManager)>,
    pub layout_changed: Option<unsafe extern "C" fn(*mut ClutterLayoutManager)>,
    pub _clutter_padding_1: Option<unsafe extern "C" fn()>,
    pub _clutter_padding_2: Option<unsafe extern "C" fn()>,
    pub _clutter_padding_3: Option<unsafe extern "C" fn()>,
    pub _clutter_padding_4: Option<unsafe extern "C" fn()>,
    pub _clutter_padding_5: Option<unsafe extern "C" fn()>,
    pub _clutter_padding_6: Option<unsafe extern "C" fn()>,
    pub _clutter_padding_7: Option<unsafe extern "C" fn()>,
    pub _clutter_padding_8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterLayoutManagerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterLayoutManagerClass @ {:?}",
            self as *const _
        ))
        .field("get_preferred_width", &self.get_preferred_width)
        .field("get_preferred_height", &self.get_preferred_height)
        .field("allocate", &self.allocate)
        .field("set_container", &self.set_container)
        .field("get_child_meta_type", &self.get_child_meta_type)
        .field("create_child_meta", &self.create_child_meta)
        .field("begin_animation", &self.begin_animation)
        .field("get_animation_progress", &self.get_animation_progress)
        .field("end_animation", &self.end_animation)
        .field("layout_changed", &self.layout_changed)
        .field("_clutter_padding_1", &self._clutter_padding_1)
        .field("_clutter_padding_2", &self._clutter_padding_2)
        .field("_clutter_padding_3", &self._clutter_padding_3)
        .field("_clutter_padding_4", &self._clutter_padding_4)
        .field("_clutter_padding_5", &self._clutter_padding_5)
        .field("_clutter_padding_6", &self._clutter_padding_6)
        .field("_clutter_padding_7", &self._clutter_padding_7)
        .field("_clutter_padding_8", &self._clutter_padding_8)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterLayoutMetaClass {
    pub parent_class: ClutterChildMetaClass,
    pub _clutter_padding1: Option<unsafe extern "C" fn()>,
    pub _clutter_padding2: Option<unsafe extern "C" fn()>,
    pub _clutter_padding3: Option<unsafe extern "C" fn()>,
    pub _clutter_padding4: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterLayoutMetaClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterLayoutMetaClass @ {:?}", self as *const _))
            .field("_clutter_padding1", &self._clutter_padding1)
            .field("_clutter_padding2", &self._clutter_padding2)
            .field("_clutter_padding3", &self._clutter_padding3)
            .field("_clutter_padding4", &self._clutter_padding4)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterListModelClass {
    pub parent_class: ClutterModelClass,
}

impl ::std::fmt::Debug for ClutterListModelClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterListModelClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterListModelPrivate(c_void);

pub type ClutterListModelPrivate = *mut _ClutterListModelPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterMargin {
    pub left: c_float,
    pub right: c_float,
    pub top: c_float,
    pub bottom: c_float,
}

impl ::std::fmt::Debug for ClutterMargin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterMargin @ {:?}", self as *const _))
            .field("left", &self.left)
            .field("right", &self.right)
            .field("top", &self.top)
            .field("bottom", &self.bottom)
            .finish()
    }
}

#[repr(C)]
pub struct ClutterMatrix(c_void);

impl ::std::fmt::Debug for ClutterMatrix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterMatrix @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterMediaIface {
    pub base_iface: gobject::GTypeInterface,
    pub eos: Option<unsafe extern "C" fn(*mut ClutterMedia)>,
    pub error: Option<unsafe extern "C" fn(*mut ClutterMedia, *const glib::GError)>,
}

impl ::std::fmt::Debug for ClutterMediaIface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterMediaIface @ {:?}", self as *const _))
            .field("eos", &self.eos)
            .field("error", &self.error)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterModelClass {
    pub parent_class: gobject::GObjectClass,
    pub get_n_rows: Option<unsafe extern "C" fn(*mut ClutterModel) -> c_uint>,
    pub get_n_columns: Option<unsafe extern "C" fn(*mut ClutterModel) -> c_uint>,
    pub get_column_name: Option<unsafe extern "C" fn(*mut ClutterModel, c_uint) -> *const c_char>,
    pub get_column_type: Option<unsafe extern "C" fn(*mut ClutterModel, c_uint) -> GType>,
    pub insert_row: Option<unsafe extern "C" fn(*mut ClutterModel, c_int) -> *mut ClutterModelIter>,
    pub remove_row: Option<unsafe extern "C" fn(*mut ClutterModel, c_uint)>,
    pub get_iter_at_row:
        Option<unsafe extern "C" fn(*mut ClutterModel, c_uint) -> *mut ClutterModelIter>,
    pub resort: Option<unsafe extern "C" fn(*mut ClutterModel, ClutterModelSortFunc, gpointer)>,
    pub row_added: Option<unsafe extern "C" fn(*mut ClutterModel, *mut ClutterModelIter)>,
    pub row_removed: Option<unsafe extern "C" fn(*mut ClutterModel, *mut ClutterModelIter)>,
    pub row_changed: Option<unsafe extern "C" fn(*mut ClutterModel, *mut ClutterModelIter)>,
    pub sort_changed: Option<unsafe extern "C" fn(*mut ClutterModel)>,
    pub filter_changed: Option<unsafe extern "C" fn(*mut ClutterModel)>,
    pub _clutter_model_1: Option<unsafe extern "C" fn()>,
    pub _clutter_model_2: Option<unsafe extern "C" fn()>,
    pub _clutter_model_3: Option<unsafe extern "C" fn()>,
    pub _clutter_model_4: Option<unsafe extern "C" fn()>,
    pub _clutter_model_5: Option<unsafe extern "C" fn()>,
    pub _clutter_model_6: Option<unsafe extern "C" fn()>,
    pub _clutter_model_7: Option<unsafe extern "C" fn()>,
    pub _clutter_model_8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterModelClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterModelClass @ {:?}", self as *const _))
            .field("get_n_rows", &self.get_n_rows)
            .field("get_n_columns", &self.get_n_columns)
            .field("get_column_name", &self.get_column_name)
            .field("get_column_type", &self.get_column_type)
            .field("insert_row", &self.insert_row)
            .field("remove_row", &self.remove_row)
            .field("get_iter_at_row", &self.get_iter_at_row)
            .field("resort", &self.resort)
            .field("row_added", &self.row_added)
            .field("row_removed", &self.row_removed)
            .field("row_changed", &self.row_changed)
            .field("sort_changed", &self.sort_changed)
            .field("filter_changed", &self.filter_changed)
            .field("_clutter_model_1", &self._clutter_model_1)
            .field("_clutter_model_2", &self._clutter_model_2)
            .field("_clutter_model_3", &self._clutter_model_3)
            .field("_clutter_model_4", &self._clutter_model_4)
            .field("_clutter_model_5", &self._clutter_model_5)
            .field("_clutter_model_6", &self._clutter_model_6)
            .field("_clutter_model_7", &self._clutter_model_7)
            .field("_clutter_model_8", &self._clutter_model_8)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterModelIterClass {
    pub parent_class: gobject::GObjectClass,
    pub get_value:
        Option<unsafe extern "C" fn(*mut ClutterModelIter, c_uint, *const gobject::GValue)>,
    pub set_value:
        Option<unsafe extern "C" fn(*mut ClutterModelIter, c_uint, *const gobject::GValue)>,
    pub is_first: Option<unsafe extern "C" fn(*mut ClutterModelIter) -> gboolean>,
    pub is_last: Option<unsafe extern "C" fn(*mut ClutterModelIter) -> gboolean>,
    pub next: Option<unsafe extern "C" fn(*mut ClutterModelIter) -> *mut ClutterModelIter>,
    pub prev: Option<unsafe extern "C" fn(*mut ClutterModelIter) -> *mut ClutterModelIter>,
    pub get_model: Option<unsafe extern "C" fn(*mut ClutterModelIter) -> *mut ClutterModel>,
    pub get_row: Option<unsafe extern "C" fn(*mut ClutterModelIter) -> c_uint>,
    pub copy: Option<unsafe extern "C" fn(*mut ClutterModelIter) -> *mut ClutterModelIter>,
    pub _clutter_model_iter_1: Option<unsafe extern "C" fn()>,
    pub _clutter_model_iter_2: Option<unsafe extern "C" fn()>,
    pub _clutter_model_iter_3: Option<unsafe extern "C" fn()>,
    pub _clutter_model_iter_4: Option<unsafe extern "C" fn()>,
    pub _clutter_model_iter_5: Option<unsafe extern "C" fn()>,
    pub _clutter_model_iter_6: Option<unsafe extern "C" fn()>,
    pub _clutter_model_iter_7: Option<unsafe extern "C" fn()>,
    pub _clutter_model_iter_8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterModelIterClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterModelIterClass @ {:?}", self as *const _))
            .field("get_value", &self.get_value)
            .field("set_value", &self.set_value)
            .field("is_first", &self.is_first)
            .field("is_last", &self.is_last)
            .field("next", &self.next)
            .field("prev", &self.prev)
            .field("get_model", &self.get_model)
            .field("get_row", &self.get_row)
            .field("copy", &self.copy)
            .field("_clutter_model_iter_1", &self._clutter_model_iter_1)
            .field("_clutter_model_iter_2", &self._clutter_model_iter_2)
            .field("_clutter_model_iter_3", &self._clutter_model_iter_3)
            .field("_clutter_model_iter_4", &self._clutter_model_iter_4)
            .field("_clutter_model_iter_5", &self._clutter_model_iter_5)
            .field("_clutter_model_iter_6", &self._clutter_model_iter_6)
            .field("_clutter_model_iter_7", &self._clutter_model_iter_7)
            .field("_clutter_model_iter_8", &self._clutter_model_iter_8)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterModelIterPrivate(c_void);

pub type ClutterModelIterPrivate = *mut _ClutterModelIterPrivate;

#[repr(C)]
pub struct _ClutterModelPrivate(c_void);

pub type ClutterModelPrivate = *mut _ClutterModelPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterMotionEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
    pub x: c_float,
    pub y: c_float,
    pub modifier_state: ClutterModifierType,
    pub axes: *mut c_double,
    pub device: *mut ClutterInputDevice,
}

impl ::std::fmt::Debug for ClutterMotionEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterMotionEvent @ {:?}", self as *const _))
            .field("type_", &self.type_)
            .field("time", &self.time)
            .field("flags", &self.flags)
            .field("stage", &self.stage)
            .field("source", &self.source)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("modifier_state", &self.modifier_state)
            .field("axes", &self.axes)
            .field("device", &self.device)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterOffscreenEffectClass {
    pub parent_class: ClutterEffectClass,
    pub create_texture: Option<
        unsafe extern "C" fn(*mut ClutterOffscreenEffect, c_float, c_float) -> cogl::CoglHandle,
    >,
    pub paint_target: Option<unsafe extern "C" fn(*mut ClutterOffscreenEffect)>,
    pub _clutter_offscreen1: Option<unsafe extern "C" fn()>,
    pub _clutter_offscreen2: Option<unsafe extern "C" fn()>,
    pub _clutter_offscreen3: Option<unsafe extern "C" fn()>,
    pub _clutter_offscreen4: Option<unsafe extern "C" fn()>,
    pub _clutter_offscreen5: Option<unsafe extern "C" fn()>,
    pub _clutter_offscreen6: Option<unsafe extern "C" fn()>,
    pub _clutter_offscreen7: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterOffscreenEffectClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterOffscreenEffectClass @ {:?}",
            self as *const _
        ))
        .field("create_texture", &self.create_texture)
        .field("paint_target", &self.paint_target)
        .field("_clutter_offscreen1", &self._clutter_offscreen1)
        .field("_clutter_offscreen2", &self._clutter_offscreen2)
        .field("_clutter_offscreen3", &self._clutter_offscreen3)
        .field("_clutter_offscreen4", &self._clutter_offscreen4)
        .field("_clutter_offscreen5", &self._clutter_offscreen5)
        .field("_clutter_offscreen6", &self._clutter_offscreen6)
        .field("_clutter_offscreen7", &self._clutter_offscreen7)
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterOffscreenEffectPrivate(c_void);

pub type ClutterOffscreenEffectPrivate = *mut _ClutterOffscreenEffectPrivate;

#[repr(C)]
pub struct _ClutterPageTurnEffectClass(c_void);

pub type ClutterPageTurnEffectClass = *mut _ClutterPageTurnEffectClass;

#[repr(C)]
pub struct _ClutterPaintNodeClass(c_void);

pub type ClutterPaintNodeClass = *mut _ClutterPaintNodeClass;

#[repr(C)]
pub struct _ClutterPaintNodePrivate(c_void);

pub type ClutterPaintNodePrivate = *mut _ClutterPaintNodePrivate;

#[repr(C)]
pub struct ClutterPaintVolume(c_void);

impl ::std::fmt::Debug for ClutterPaintVolume {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPaintVolume @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterPanActionClass {
    pub parent_class: ClutterGestureActionClass,
    pub pan: Option<
        unsafe extern "C" fn(*mut ClutterPanAction, *mut ClutterActor, gboolean) -> gboolean,
    >,
    pub pan_stopped: Option<unsafe extern "C" fn(*mut ClutterPanAction, *mut ClutterActor)>,
    pub _clutter_pan_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_pan_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_pan_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_pan_action4: Option<unsafe extern "C" fn()>,
    pub _clutter_pan_action5: Option<unsafe extern "C" fn()>,
    pub _clutter_pan_action6: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterPanActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPanActionClass @ {:?}", self as *const _))
            .field("pan", &self.pan)
            .field("pan_stopped", &self.pan_stopped)
            .field("_clutter_pan_action1", &self._clutter_pan_action1)
            .field("_clutter_pan_action2", &self._clutter_pan_action2)
            .field("_clutter_pan_action3", &self._clutter_pan_action3)
            .field("_clutter_pan_action4", &self._clutter_pan_action4)
            .field("_clutter_pan_action5", &self._clutter_pan_action5)
            .field("_clutter_pan_action6", &self._clutter_pan_action6)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterPanActionPrivate(c_void);

pub type ClutterPanActionPrivate = *mut _ClutterPanActionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterParamSpecUnits {
    pub parent_instance: gobject::GParamSpec,
    pub default_type: ClutterUnitType,
    pub default_value: c_float,
    pub minimum: c_float,
    pub maximum: c_float,
}

impl ::std::fmt::Debug for ClutterParamSpecUnits {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterParamSpecUnits @ {:?}", self as *const _))
            .field("default_type", &self.default_type)
            .field("default_value", &self.default_value)
            .field("minimum", &self.minimum)
            .field("maximum", &self.maximum)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterPathClass {
    pub parent_class: gobject::GInitiallyUnownedClass,
}

impl ::std::fmt::Debug for ClutterPathClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPathClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterPathConstraintClass(c_void);

pub type ClutterPathConstraintClass = *mut _ClutterPathConstraintClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterPathNode {
    pub type_: ClutterPathNodeType,
    pub points: [ClutterKnot; 3],
}

impl ::std::fmt::Debug for ClutterPathNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPathNode @ {:?}", self as *const _))
            .field("type_", &self.type_)
            .field("points", &self.points)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterPathPrivate(c_void);

pub type ClutterPathPrivate = *mut _ClutterPathPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterPerspective {
    pub fovy: c_float,
    pub aspect: c_float,
    pub z_near: c_float,
    pub z_far: c_float,
}

impl ::std::fmt::Debug for ClutterPerspective {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPerspective @ {:?}", self as *const _))
            .field("fovy", &self.fovy)
            .field("aspect", &self.aspect)
            .field("z_near", &self.z_near)
            .field("z_far", &self.z_far)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterPipelineNodeClass(c_void);

pub type ClutterPipelineNodeClass = *mut _ClutterPipelineNodeClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterPoint {
    pub x: c_float,
    pub y: c_float,
}

impl ::std::fmt::Debug for ClutterPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPoint @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterPropertyTransitionClass {
    pub parent_class: ClutterTransitionClass,
    pub _padding: [gpointer; 8],
}

impl ::std::fmt::Debug for ClutterPropertyTransitionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterPropertyTransitionClass @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterPropertyTransitionPrivate(c_void);

pub type ClutterPropertyTransitionPrivate = *mut _ClutterPropertyTransitionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterRect {
    pub origin: ClutterPoint,
    pub size: ClutterSize,
}

impl ::std::fmt::Debug for ClutterRect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterRect @ {:?}", self as *const _))
            .field("origin", &self.origin)
            .field("size", &self.size)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterRectangleClass {
    pub parent_class: ClutterActorClass,
    pub _clutter_rectangle1: Option<unsafe extern "C" fn()>,
    pub _clutter_rectangle2: Option<unsafe extern "C" fn()>,
    pub _clutter_rectangle3: Option<unsafe extern "C" fn()>,
    pub _clutter_rectangle4: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterRectangleClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterRectangleClass @ {:?}", self as *const _))
            .field("_clutter_rectangle1", &self._clutter_rectangle1)
            .field("_clutter_rectangle2", &self._clutter_rectangle2)
            .field("_clutter_rectangle3", &self._clutter_rectangle3)
            .field("_clutter_rectangle4", &self._clutter_rectangle4)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterRectanglePrivate(c_void);

pub type ClutterRectanglePrivate = *mut _ClutterRectanglePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterRotateActionClass {
    pub parent_class: ClutterGestureActionClass,
    pub rotate: Option<
        unsafe extern "C" fn(*mut ClutterRotateAction, *mut ClutterActor, c_double) -> gboolean,
    >,
    pub _clutter_rotate_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_rotate_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_rotate_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_rotate_action4: Option<unsafe extern "C" fn()>,
    pub _clutter_rotate_action5: Option<unsafe extern "C" fn()>,
    pub _clutter_rotate_action6: Option<unsafe extern "C" fn()>,
    pub _clutter_rotate_action7: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterRotateActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterRotateActionClass @ {:?}",
            self as *const _
        ))
        .field("rotate", &self.rotate)
        .field("_clutter_rotate_action1", &self._clutter_rotate_action1)
        .field("_clutter_rotate_action2", &self._clutter_rotate_action2)
        .field("_clutter_rotate_action3", &self._clutter_rotate_action3)
        .field("_clutter_rotate_action4", &self._clutter_rotate_action4)
        .field("_clutter_rotate_action5", &self._clutter_rotate_action5)
        .field("_clutter_rotate_action6", &self._clutter_rotate_action6)
        .field("_clutter_rotate_action7", &self._clutter_rotate_action7)
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterRotateActionPrivate(c_void);

pub type ClutterRotateActionPrivate = *mut _ClutterRotateActionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterScoreClass {
    pub parent_class: gobject::GObjectClass,
    pub timeline_started: Option<unsafe extern "C" fn(*mut ClutterScore, *mut ClutterTimeline)>,
    pub timeline_completed: Option<unsafe extern "C" fn(*mut ClutterScore, *mut ClutterTimeline)>,
    pub started: Option<unsafe extern "C" fn(*mut ClutterScore)>,
    pub completed: Option<unsafe extern "C" fn(*mut ClutterScore)>,
    pub paused: Option<unsafe extern "C" fn(*mut ClutterScore)>,
    pub _clutter_score_1: Option<unsafe extern "C" fn()>,
    pub _clutter_score_2: Option<unsafe extern "C" fn()>,
    pub _clutter_score_3: Option<unsafe extern "C" fn()>,
    pub _clutter_score_4: Option<unsafe extern "C" fn()>,
    pub _clutter_score_5: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterScoreClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterScoreClass @ {:?}", self as *const _))
            .field("timeline_started", &self.timeline_started)
            .field("timeline_completed", &self.timeline_completed)
            .field("started", &self.started)
            .field("completed", &self.completed)
            .field("paused", &self.paused)
            .field("_clutter_score_1", &self._clutter_score_1)
            .field("_clutter_score_2", &self._clutter_score_2)
            .field("_clutter_score_3", &self._clutter_score_3)
            .field("_clutter_score_4", &self._clutter_score_4)
            .field("_clutter_score_5", &self._clutter_score_5)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterScorePrivate(c_void);

pub type ClutterScorePrivate = *mut _ClutterScorePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterScriptClass {
    pub parent_class: gobject::GObjectClass,
    pub get_type_from_name:
        Option<unsafe extern "C" fn(*mut ClutterScript, *const c_char) -> GType>,
    pub _clutter_reserved1: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved2: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved3: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved4: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved5: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved6: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved7: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterScriptClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterScriptClass @ {:?}", self as *const _))
            .field("get_type_from_name", &self.get_type_from_name)
            .field("_clutter_reserved1", &self._clutter_reserved1)
            .field("_clutter_reserved2", &self._clutter_reserved2)
            .field("_clutter_reserved3", &self._clutter_reserved3)
            .field("_clutter_reserved4", &self._clutter_reserved4)
            .field("_clutter_reserved5", &self._clutter_reserved5)
            .field("_clutter_reserved6", &self._clutter_reserved6)
            .field("_clutter_reserved7", &self._clutter_reserved7)
            .field("_clutter_reserved8", &self._clutter_reserved8)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterScriptPrivate(c_void);

pub type ClutterScriptPrivate = *mut _ClutterScriptPrivate;

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct ClutterScriptableIface {
//     pub g_iface: gobject::GTypeInterface,
//     pub set_id: Option<unsafe extern "C" fn(*mut ClutterScriptable, *const c_char)>,
//     pub get_id: Option<unsafe extern "C" fn(*mut ClutterScriptable) -> *const c_char>,
//     pub parse_custom_node: Option<
//         unsafe extern "C" fn(
//             *mut ClutterScriptable,
//             *mut ClutterScript,
//             *mut gobject::GValue,
//             *const c_char,
//             *mut json::JsonNode,
//         ) -> gboolean,
//     >,
//     pub set_custom_property: Option<
//         unsafe extern "C" fn(
//             *mut ClutterScriptable,
//             *mut ClutterScript,
//             *const c_char,
//             *const gobject::GValue,
//         ),
//     >,
// }

// impl ::std::fmt::Debug for ClutterScriptableIface {
//     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//         f.debug_struct(&format!("ClutterScriptableIface @ {:?}", self as *const _))
//             .field("set_id", &self.set_id)
//             .field("get_id", &self.get_id)
//             .field("parse_custom_node", &self.parse_custom_node)
//             .field("set_custom_property", &self.set_custom_property)
//             .finish()
//     }
// }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterScrollActorClass {
    pub parent_instance: ClutterActorClass,
    pub _padding: [gpointer; 8],
}

impl ::std::fmt::Debug for ClutterScrollActorClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterScrollActorClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterScrollActorPrivate(c_void);

pub type ClutterScrollActorPrivate = *mut _ClutterScrollActorPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterScrollEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
    pub x: c_float,
    pub y: c_float,
    pub direction: ClutterScrollDirection,
    pub modifier_state: ClutterModifierType,
    pub axes: *mut c_double,
    pub device: *mut ClutterInputDevice,
    pub scroll_source: ClutterScrollSource,
    pub finish_flags: ClutterScrollFinishFlags,
}

impl ::std::fmt::Debug for ClutterScrollEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterScrollEvent @ {:?}", self as *const _))
            .field("type_", &self.type_)
            .field("time", &self.time)
            .field("flags", &self.flags)
            .field("stage", &self.stage)
            .field("source", &self.source)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("direction", &self.direction)
            .field("modifier_state", &self.modifier_state)
            .field("axes", &self.axes)
            .field("device", &self.device)
            .field("scroll_source", &self.scroll_source)
            .field("finish_flags", &self.finish_flags)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterSettingsClass(c_void);

pub type ClutterSettingsClass = *mut _ClutterSettingsClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterShaderClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ClutterShaderClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterShaderClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterShaderEffectClass {
    pub parent_class: ClutterOffscreenEffectClass,
    pub get_static_shader_source:
        Option<unsafe extern "C" fn(*mut ClutterShaderEffect) -> *mut c_char>,
    pub _clutter_shader1: Option<unsafe extern "C" fn()>,
    pub _clutter_shader2: Option<unsafe extern "C" fn()>,
    pub _clutter_shader3: Option<unsafe extern "C" fn()>,
    pub _clutter_shader4: Option<unsafe extern "C" fn()>,
    pub _clutter_shader5: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterShaderEffectClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterShaderEffectClass @ {:?}",
            self as *const _
        ))
        .field("get_static_shader_source", &self.get_static_shader_source)
        .field("_clutter_shader1", &self._clutter_shader1)
        .field("_clutter_shader2", &self._clutter_shader2)
        .field("_clutter_shader3", &self._clutter_shader3)
        .field("_clutter_shader4", &self._clutter_shader4)
        .field("_clutter_shader5", &self._clutter_shader5)
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterShaderEffectPrivate(c_void);

pub type ClutterShaderEffectPrivate = *mut _ClutterShaderEffectPrivate;

#[repr(C)]
pub struct _ClutterShaderPrivate(c_void);

pub type ClutterShaderPrivate = *mut _ClutterShaderPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterSize {
    pub width: c_float,
    pub height: c_float,
}

impl ::std::fmt::Debug for ClutterSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterSize @ {:?}", self as *const _))
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterSnapConstraintClass(c_void);

pub type ClutterSnapConstraintClass = *mut _ClutterSnapConstraintClass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterStageClass {
    pub parent_class: ClutterGroupClass,
    pub fullscreen: Option<unsafe extern "C" fn(*mut ClutterStage)>,
    pub unfullscreen: Option<unsafe extern "C" fn(*mut ClutterStage)>,
    pub activate: Option<unsafe extern "C" fn(*mut ClutterStage)>,
    pub deactivate: Option<unsafe extern "C" fn(*mut ClutterStage)>,
    pub delete_event:
        Option<unsafe extern "C" fn(*mut ClutterStage, *mut ClutterEvent) -> gboolean>,
    pub _padding_dummy: [gpointer; 31],
}

impl ::std::fmt::Debug for ClutterStageClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterStageClass @ {:?}", self as *const _))
            .field("fullscreen", &self.fullscreen)
            .field("unfullscreen", &self.unfullscreen)
            .field("activate", &self.activate)
            .field("deactivate", &self.deactivate)
            .field("delete_event", &self.delete_event)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterStageManagerClass {
    pub parent_class: gobject::GObjectClass,
    pub stage_added: Option<unsafe extern "C" fn(*mut ClutterStageManager, *mut ClutterStage)>,
    pub stage_removed: Option<unsafe extern "C" fn(*mut ClutterStageManager, *mut ClutterStage)>,
}

impl ::std::fmt::Debug for ClutterStageManagerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterStageManagerClass @ {:?}",
            self as *const _
        ))
        .field("stage_added", &self.stage_added)
        .field("stage_removed", &self.stage_removed)
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterStagePrivate(c_void);

pub type ClutterStagePrivate = *mut _ClutterStagePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterStageStateEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
    pub changed_mask: ClutterStageState,
    pub new_state: ClutterStageState,
}

impl ::std::fmt::Debug for ClutterStageStateEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterStageStateEvent @ {:?}", self as *const _))
            .field("type_", &self.type_)
            .field("time", &self.time)
            .field("flags", &self.flags)
            .field("stage", &self.stage)
            .field("source", &self.source)
            .field("changed_mask", &self.changed_mask)
            .field("new_state", &self.new_state)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterStateClass {
    pub parent_class: gobject::GObjectClass,
    pub completed: Option<unsafe extern "C" fn(*mut ClutterState)>,
    pub _padding_dummy: [gpointer; 8],
}

impl ::std::fmt::Debug for ClutterStateClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterStateClass @ {:?}", self as *const _))
            .field("completed", &self.completed)
            .finish()
    }
}

#[repr(C)]
pub struct ClutterStateKey(c_void);

impl ::std::fmt::Debug for ClutterStateKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterStateKey @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterStatePrivate(c_void);

pub type ClutterStatePrivate = *mut _ClutterStatePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterSwipeActionClass {
    pub parent_class: ClutterGestureActionClass,
    pub swept: Option<
        unsafe extern "C" fn(*mut ClutterSwipeAction, *mut ClutterActor, ClutterSwipeDirection),
    >,
    pub swipe: Option<
        unsafe extern "C" fn(
            *mut ClutterSwipeAction,
            *mut ClutterActor,
            ClutterSwipeDirection,
        ) -> gboolean,
    >,
    pub _clutter_swipe_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_swipe_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_swipe_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_swipe_action4: Option<unsafe extern "C" fn()>,
    pub _clutter_swipe_action5: Option<unsafe extern "C" fn()>,
    pub _clutter_swipe_action6: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterSwipeActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterSwipeActionClass @ {:?}", self as *const _))
            .field("swept", &self.swept)
            .field("swipe", &self.swipe)
            .field("_clutter_swipe_action1", &self._clutter_swipe_action1)
            .field("_clutter_swipe_action2", &self._clutter_swipe_action2)
            .field("_clutter_swipe_action3", &self._clutter_swipe_action3)
            .field("_clutter_swipe_action4", &self._clutter_swipe_action4)
            .field("_clutter_swipe_action5", &self._clutter_swipe_action5)
            .field("_clutter_swipe_action6", &self._clutter_swipe_action6)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterSwipeActionPrivate(c_void);

pub type ClutterSwipeActionPrivate = *mut _ClutterSwipeActionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTableLayoutClass {
    pub parent_class: ClutterLayoutManagerClass,
}

impl ::std::fmt::Debug for ClutterTableLayoutClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTableLayoutClass @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterTableLayoutPrivate(c_void);

pub type ClutterTableLayoutPrivate = *mut _ClutterTableLayoutPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTapActionClass {
    pub parent_class: ClutterGestureActionClass,
    pub tap: Option<unsafe extern "C" fn(*mut ClutterTapAction, *mut ClutterActor) -> gboolean>,
    pub _clutter_tap_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_tap_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_tap_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_tap_action4: Option<unsafe extern "C" fn()>,
    pub _clutter_tap_action5: Option<unsafe extern "C" fn()>,
    pub _clutter_tap_action6: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterTapActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTapActionClass @ {:?}", self as *const _))
            .field("tap", &self.tap)
            .field("_clutter_tap_action1", &self._clutter_tap_action1)
            .field("_clutter_tap_action2", &self._clutter_tap_action2)
            .field("_clutter_tap_action3", &self._clutter_tap_action3)
            .field("_clutter_tap_action4", &self._clutter_tap_action4)
            .field("_clutter_tap_action5", &self._clutter_tap_action5)
            .field("_clutter_tap_action6", &self._clutter_tap_action6)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterTapActionPrivate(c_void);

pub type ClutterTapActionPrivate = *mut _ClutterTapActionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTextBufferClass {
    pub parent_class: gobject::GObjectClass,
    pub inserted_text:
        Option<unsafe extern "C" fn(*mut ClutterTextBuffer, c_uint, *const c_char, c_uint)>,
    pub deleted_text: Option<unsafe extern "C" fn(*mut ClutterTextBuffer, c_uint, c_uint)>,
    pub get_text:
        Option<unsafe extern "C" fn(*mut ClutterTextBuffer, *mut size_t) -> *const c_char>,
    pub get_length: Option<unsafe extern "C" fn(*mut ClutterTextBuffer) -> c_uint>,
    pub insert_text: Option<
        unsafe extern "C" fn(*mut ClutterTextBuffer, c_uint, *const c_char, c_uint) -> c_uint,
    >,
    pub delete_text: Option<unsafe extern "C" fn(*mut ClutterTextBuffer, c_uint, c_uint) -> c_uint>,
    pub _clutter_reserved1: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved2: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved3: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved4: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved5: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved6: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved7: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterTextBufferClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTextBufferClass @ {:?}", self as *const _))
            .field("inserted_text", &self.inserted_text)
            .field("deleted_text", &self.deleted_text)
            .field("get_text", &self.get_text)
            .field("get_length", &self.get_length)
            .field("insert_text", &self.insert_text)
            .field("delete_text", &self.delete_text)
            .field("_clutter_reserved1", &self._clutter_reserved1)
            .field("_clutter_reserved2", &self._clutter_reserved2)
            .field("_clutter_reserved3", &self._clutter_reserved3)
            .field("_clutter_reserved4", &self._clutter_reserved4)
            .field("_clutter_reserved5", &self._clutter_reserved5)
            .field("_clutter_reserved6", &self._clutter_reserved6)
            .field("_clutter_reserved7", &self._clutter_reserved7)
            .field("_clutter_reserved8", &self._clutter_reserved8)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterTextBufferPrivate(c_void);

pub type ClutterTextBufferPrivate = *mut _ClutterTextBufferPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTextClass {
    pub parent_class: ClutterActorClass,
    pub text_changed: Option<unsafe extern "C" fn(*mut ClutterText)>,
    pub activate: Option<unsafe extern "C" fn(*mut ClutterText)>,
    pub cursor_event: Option<unsafe extern "C" fn(*mut ClutterText, *const ClutterGeometry)>,
    pub cursor_changed: Option<unsafe extern "C" fn(*mut ClutterText)>,
    pub _clutter_reserved1: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved2: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved3: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved4: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved5: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved6: Option<unsafe extern "C" fn()>,
    pub _clutter_reserved7: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterTextClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTextClass @ {:?}", self as *const _))
            .field("text_changed", &self.text_changed)
            .field("activate", &self.activate)
            .field("cursor_event", &self.cursor_event)
            .field("cursor_changed", &self.cursor_changed)
            .field("_clutter_reserved1", &self._clutter_reserved1)
            .field("_clutter_reserved2", &self._clutter_reserved2)
            .field("_clutter_reserved3", &self._clutter_reserved3)
            .field("_clutter_reserved4", &self._clutter_reserved4)
            .field("_clutter_reserved5", &self._clutter_reserved5)
            .field("_clutter_reserved6", &self._clutter_reserved6)
            .field("_clutter_reserved7", &self._clutter_reserved7)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterTextNodeClass(c_void);

pub type ClutterTextNodeClass = *mut _ClutterTextNodeClass;

#[repr(C)]
pub struct _ClutterTextPrivate(c_void);

pub type ClutterTextPrivate = *mut _ClutterTextPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTextureClass {
    pub parent_class: ClutterActorClass,
    pub size_change: Option<unsafe extern "C" fn(*mut ClutterTexture, c_int, c_int)>,
    pub pixbuf_change: Option<unsafe extern "C" fn(*mut ClutterTexture)>,
    pub load_finished: Option<unsafe extern "C" fn(*mut ClutterTexture, *const glib::GError)>,
    pub _clutter_texture1: Option<unsafe extern "C" fn()>,
    pub _clutter_texture2: Option<unsafe extern "C" fn()>,
    pub _clutter_texture3: Option<unsafe extern "C" fn()>,
    pub _clutter_texture4: Option<unsafe extern "C" fn()>,
    pub _clutter_texture5: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterTextureClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTextureClass @ {:?}", self as *const _))
            .field("size_change", &self.size_change)
            .field("pixbuf_change", &self.pixbuf_change)
            .field("load_finished", &self.load_finished)
            .field("_clutter_texture1", &self._clutter_texture1)
            .field("_clutter_texture2", &self._clutter_texture2)
            .field("_clutter_texture3", &self._clutter_texture3)
            .field("_clutter_texture4", &self._clutter_texture4)
            .field("_clutter_texture5", &self._clutter_texture5)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterTextureNodeClass(c_void);

pub type ClutterTextureNodeClass = *mut _ClutterTextureNodeClass;

#[repr(C)]
pub struct _ClutterTexturePrivate(c_void);

pub type ClutterTexturePrivate = *mut _ClutterTexturePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTimelineClass {
    pub parent_class: gobject::GObjectClass,
    pub started: Option<unsafe extern "C" fn(*mut ClutterTimeline)>,
    pub completed: Option<unsafe extern "C" fn(*mut ClutterTimeline)>,
    pub paused: Option<unsafe extern "C" fn(*mut ClutterTimeline)>,
    pub new_frame: Option<unsafe extern "C" fn(*mut ClutterTimeline, c_int)>,
    pub marker_reached: Option<unsafe extern "C" fn(*mut ClutterTimeline, *const c_char, c_int)>,
    pub stopped: Option<unsafe extern "C" fn(*mut ClutterTimeline, gboolean)>,
    pub _clutter_timeline_1: Option<unsafe extern "C" fn()>,
    pub _clutter_timeline_2: Option<unsafe extern "C" fn()>,
    pub _clutter_timeline_3: Option<unsafe extern "C" fn()>,
    pub _clutter_timeline_4: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterTimelineClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTimelineClass @ {:?}", self as *const _))
            .field("started", &self.started)
            .field("completed", &self.completed)
            .field("paused", &self.paused)
            .field("new_frame", &self.new_frame)
            .field("marker_reached", &self.marker_reached)
            .field("stopped", &self.stopped)
            .field("_clutter_timeline_1", &self._clutter_timeline_1)
            .field("_clutter_timeline_2", &self._clutter_timeline_2)
            .field("_clutter_timeline_3", &self._clutter_timeline_3)
            .field("_clutter_timeline_4", &self._clutter_timeline_4)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterTimelinePrivate(c_void);

pub type ClutterTimelinePrivate = *mut _ClutterTimelinePrivate;

#[repr(C)]
pub struct _ClutterTimeoutPool(c_void);

pub type ClutterTimeoutPool = *mut _ClutterTimeoutPool;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTouchEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
    pub x: c_float,
    pub y: c_float,
    pub sequence: *mut ClutterEventSequence,
    pub modifier_state: ClutterModifierType,
    pub axes: *mut c_double,
    pub device: *mut ClutterInputDevice,
}

impl ::std::fmt::Debug for ClutterTouchEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTouchEvent @ {:?}", self as *const _))
            .field("type_", &self.type_)
            .field("time", &self.time)
            .field("flags", &self.flags)
            .field("stage", &self.stage)
            .field("source", &self.source)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("sequence", &self.sequence)
            .field("modifier_state", &self.modifier_state)
            .field("axes", &self.axes)
            .field("device", &self.device)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTouchpadPinchEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
    pub phase: ClutterTouchpadGesturePhase,
    pub x: c_float,
    pub y: c_float,
    pub dx: c_float,
    pub dy: c_float,
    pub angle_delta: c_float,
    pub scale: c_float,
}

impl ::std::fmt::Debug for ClutterTouchpadPinchEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterTouchpadPinchEvent @ {:?}",
            self as *const _
        ))
        .field("type_", &self.type_)
        .field("time", &self.time)
        .field("flags", &self.flags)
        .field("stage", &self.stage)
        .field("source", &self.source)
        .field("phase", &self.phase)
        .field("x", &self.x)
        .field("y", &self.y)
        .field("dx", &self.dx)
        .field("dy", &self.dy)
        .field("angle_delta", &self.angle_delta)
        .field("scale", &self.scale)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTouchpadSwipeEvent {
    pub type_: ClutterEventType,
    pub time: u32,
    pub flags: ClutterEventFlags,
    pub stage: *mut ClutterStage,
    pub source: *mut ClutterActor,
    pub phase: ClutterTouchpadGesturePhase,
    pub n_fingers: c_uint,
    pub x: c_float,
    pub y: c_float,
    pub dx: c_float,
    pub dy: c_float,
}

impl ::std::fmt::Debug for ClutterTouchpadSwipeEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterTouchpadSwipeEvent @ {:?}",
            self as *const _
        ))
        .field("type_", &self.type_)
        .field("time", &self.time)
        .field("flags", &self.flags)
        .field("stage", &self.stage)
        .field("source", &self.source)
        .field("phase", &self.phase)
        .field("n_fingers", &self.n_fingers)
        .field("x", &self.x)
        .field("y", &self.y)
        .field("dx", &self.dx)
        .field("dy", &self.dy)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTransitionClass {
    pub parent_class: ClutterTimelineClass,
    pub attached: Option<unsafe extern "C" fn(*mut ClutterTransition, *mut ClutterAnimatable)>,
    pub detached: Option<unsafe extern "C" fn(*mut ClutterTransition, *mut ClutterAnimatable)>,
    pub compute_value: Option<
        unsafe extern "C" fn(
            *mut ClutterTransition,
            *mut ClutterAnimatable,
            *mut ClutterInterval,
            c_double,
        ),
    >,
    pub _padding: [gpointer; 8],
}

impl ::std::fmt::Debug for ClutterTransitionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTransitionClass @ {:?}", self as *const _))
            .field("attached", &self.attached)
            .field("detached", &self.detached)
            .field("compute_value", &self.compute_value)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTransitionGroupClass {
    pub parent_class: ClutterTransitionClass,
    pub _padding: [gpointer; 8],
}

impl ::std::fmt::Debug for ClutterTransitionGroupClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterTransitionGroupClass @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
pub struct _ClutterTransitionGroupPrivate(c_void);

pub type ClutterTransitionGroupPrivate = *mut _ClutterTransitionGroupPrivate;

#[repr(C)]
pub struct _ClutterTransitionPrivate(c_void);

pub type ClutterTransitionPrivate = *mut _ClutterTransitionPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterUnits {
    pub unit_type: ClutterUnitType,
    pub value: c_float,
    pub pixels: c_float,
    pub pixels_set: c_uint,
    pub serial: i32,
    pub __padding_1: i32,
    pub __padding_2: i64,
}

impl ::std::fmt::Debug for ClutterUnits {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterUnits @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterVertex {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

impl ::std::fmt::Debug for ClutterVertex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterVertex @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterZoomActionClass {
    pub parent_class: ClutterGestureActionClass,
    pub zoom: Option<
        unsafe extern "C" fn(
            *mut ClutterZoomAction,
            *mut ClutterActor,
            *mut ClutterPoint,
            c_double,
        ) -> gboolean,
    >,
    pub _clutter_zoom_action1: Option<unsafe extern "C" fn()>,
    pub _clutter_zoom_action2: Option<unsafe extern "C" fn()>,
    pub _clutter_zoom_action3: Option<unsafe extern "C" fn()>,
    pub _clutter_zoom_action4: Option<unsafe extern "C" fn()>,
    pub _clutter_zoom_action5: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for ClutterZoomActionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterZoomActionClass @ {:?}", self as *const _))
            .field("zoom", &self.zoom)
            .field("_clutter_zoom_action1", &self._clutter_zoom_action1)
            .field("_clutter_zoom_action2", &self._clutter_zoom_action2)
            .field("_clutter_zoom_action3", &self._clutter_zoom_action3)
            .field("_clutter_zoom_action4", &self._clutter_zoom_action4)
            .field("_clutter_zoom_action5", &self._clutter_zoom_action5)
            .finish()
    }
}

#[repr(C)]
pub struct _ClutterZoomActionPrivate(c_void);

pub type ClutterZoomActionPrivate = *mut _ClutterZoomActionPrivate;

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterAction {
    pub parent_instance: ClutterActorMeta,
}

impl ::std::fmt::Debug for ClutterAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAction @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterActor {
    pub parent_instance: gobject::GInitiallyUnowned,
    pub flags: u32,
    pub private_flags: u32,
    pub priv_: *mut ClutterActorPrivate,
}

impl ::std::fmt::Debug for ClutterActor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterActor @ {:?}", self as *const _))
            .field("flags", &self.flags)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterActorMeta {
    pub parent_instance: gobject::GInitiallyUnowned,
    pub priv_: *mut ClutterActorMetaPrivate,
}

impl ::std::fmt::Debug for ClutterActorMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterActorMeta @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterAlignConstraint(c_void);

impl ::std::fmt::Debug for ClutterAlignConstraint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAlignConstraint @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterAlpha {
    pub parent: gobject::GInitiallyUnowned,
    pub priv_: *mut ClutterAlphaPrivate,
}

impl ::std::fmt::Debug for ClutterAlpha {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAlpha @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterAnimation {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterAnimationPrivate,
}

impl ::std::fmt::Debug for ClutterAnimation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAnimation @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterAnimator {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterAnimatorPrivate,
}

impl ::std::fmt::Debug for ClutterAnimator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterAnimator @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterBackend(c_void);

impl ::std::fmt::Debug for ClutterBackend {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBackend @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviour {
    pub parent: gobject::GObject,
    pub priv_: *mut ClutterBehaviourPrivate,
}

impl ::std::fmt::Debug for ClutterBehaviour {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBehaviour @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourDepth {
    pub parent_instance: ClutterBehaviour,
    pub priv_: *mut ClutterBehaviourDepthPrivate,
}

impl ::std::fmt::Debug for ClutterBehaviourDepth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBehaviourDepth @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourEllipse {
    pub parent_instance: ClutterBehaviour,
    pub priv_: *mut ClutterBehaviourEllipsePrivate,
}

impl ::std::fmt::Debug for ClutterBehaviourEllipse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBehaviourEllipse @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourOpacity {
    pub parent: ClutterBehaviour,
    pub priv_: *mut ClutterBehaviourOpacityPrivate,
}

impl ::std::fmt::Debug for ClutterBehaviourOpacity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBehaviourOpacity @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourPath {
    pub parent: ClutterBehaviour,
    pub priv_: *mut ClutterBehaviourPathPrivate,
}

impl ::std::fmt::Debug for ClutterBehaviourPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBehaviourPath @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourRotate {
    pub parent_instance: ClutterBehaviour,
    pub priv_: *mut ClutterBehaviourRotatePrivate,
}

impl ::std::fmt::Debug for ClutterBehaviourRotate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBehaviourRotate @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBehaviourScale {
    pub parent_instance: ClutterBehaviour,
    pub priv_: *mut ClutterBehaviourScalePrivate,
}

impl ::std::fmt::Debug for ClutterBehaviourScale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBehaviourScale @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBinLayout {
    pub parent_instance: ClutterLayoutManager,
    pub priv_: *mut ClutterBinLayoutPrivate,
}

impl ::std::fmt::Debug for ClutterBinLayout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBinLayout @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterBindConstraint(c_void);

impl ::std::fmt::Debug for ClutterBindConstraint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBindConstraint @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterBindingPool(c_void);

impl ::std::fmt::Debug for ClutterBindingPool {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBindingPool @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterBlurEffect(c_void);

impl ::std::fmt::Debug for ClutterBlurEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBlurEffect @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBox {
    pub parent_instance: ClutterActor,
    pub priv_: *mut ClutterBoxPrivate,
}

impl ::std::fmt::Debug for ClutterBox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBox @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterBoxLayout {
    pub parent_instance: ClutterLayoutManager,
    pub priv_: *mut ClutterBoxLayoutPrivate,
}

impl ::std::fmt::Debug for ClutterBoxLayout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterBoxLayout @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterBrightnessContrastEffect(c_void);

impl ::std::fmt::Debug for ClutterBrightnessContrastEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterBrightnessContrastEffect @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterCairoTexture {
    pub parent_instance: ClutterTexture,
    pub priv_: *mut ClutterCairoTexturePrivate,
}

impl ::std::fmt::Debug for ClutterCairoTexture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterCairoTexture @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterCanvas {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterCanvasPrivate,
}

impl ::std::fmt::Debug for ClutterCanvas {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterCanvas @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterChildMeta {
    pub parent_instance: gobject::GObject,
    pub container: *mut ClutterContainer,
    pub actor: *mut ClutterActor,
}

impl ::std::fmt::Debug for ClutterChildMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterChildMeta @ {:?}", self as *const _))
            .field("container", &self.container)
            .field("actor", &self.actor)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterClickAction {
    pub parent_instance: ClutterAction,
    pub priv_: *mut ClutterClickActionPrivate,
}

impl ::std::fmt::Debug for ClutterClickAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterClickAction @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterClipNode(c_void);

impl ::std::fmt::Debug for ClutterClipNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterClipNode @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterClone {
    pub parent_instance: ClutterActor,
    pub priv_: *mut ClutterClonePrivate,
}

impl ::std::fmt::Debug for ClutterClone {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterClone @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterColorNode(c_void);

impl ::std::fmt::Debug for ClutterColorNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterColorNode @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterColorizeEffect(c_void);

impl ::std::fmt::Debug for ClutterColorizeEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterColorizeEffect @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterConstraint {
    pub parent_instance: ClutterActorMeta,
}

impl ::std::fmt::Debug for ClutterConstraint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterConstraint @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterDeformEffect {
    pub parent_instance: ClutterOffscreenEffect,
    pub priv_: *mut ClutterDeformEffectPrivate,
}

impl ::std::fmt::Debug for ClutterDeformEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterDeformEffect @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterDesaturateEffect(c_void);

impl ::std::fmt::Debug for ClutterDesaturateEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterDesaturateEffect @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterDeviceManager {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterDeviceManagerPrivate,
}

impl ::std::fmt::Debug for ClutterDeviceManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterDeviceManager @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterDragAction {
    pub parent_instance: ClutterAction,
    pub priv_: *mut ClutterDragActionPrivate,
}

impl ::std::fmt::Debug for ClutterDragAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterDragAction @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterDropAction {
    pub parent_instance: ClutterAction,
    pub priv_: *mut ClutterDropActionPrivate,
}

impl ::std::fmt::Debug for ClutterDropAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterDropAction @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterEffect {
    pub parent_instance: ClutterActorMeta,
}

impl ::std::fmt::Debug for ClutterEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterEffect @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterFixedLayout {
    pub parent_instance: ClutterLayoutManager,
}

impl ::std::fmt::Debug for ClutterFixedLayout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterFixedLayout @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterFlowLayout {
    pub parent_instance: ClutterLayoutManager,
    pub priv_: *mut ClutterFlowLayoutPrivate,
}

impl ::std::fmt::Debug for ClutterFlowLayout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterFlowLayout @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterGestureAction {
    pub parent_instance: ClutterAction,
    pub priv_: *mut ClutterGestureActionPrivate,
}

impl ::std::fmt::Debug for ClutterGestureAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterGestureAction @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterGridLayout {
    pub parent_instance: ClutterLayoutManager,
    pub priv_: *mut ClutterGridLayoutPrivate,
}

impl ::std::fmt::Debug for ClutterGridLayout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterGridLayout @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterGroup {
    pub parent_instance: ClutterActor,
    pub priv_: *mut ClutterGroupPrivate,
}

impl ::std::fmt::Debug for ClutterGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterGroup @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterImage {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterImagePrivate,
}

impl ::std::fmt::Debug for ClutterImage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterImage @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterInputDevice(c_void);

impl ::std::fmt::Debug for ClutterInputDevice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterInputDevice @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterInterval {
    pub parent_instance: gobject::GInitiallyUnowned,
    pub priv_: *mut ClutterIntervalPrivate,
}

impl ::std::fmt::Debug for ClutterInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterInterval @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterKeyframeTransition {
    pub parent_instance: ClutterPropertyTransition,
    pub priv_: *mut ClutterKeyframeTransitionPrivate,
}

impl ::std::fmt::Debug for ClutterKeyframeTransition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterKeyframeTransition @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterLayoutManager {
    pub parent_instance: gobject::GInitiallyUnowned,
    pub dummy: gpointer,
}

impl ::std::fmt::Debug for ClutterLayoutManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterLayoutManager @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterLayoutMeta {
    pub parent_instance: ClutterChildMeta,
    pub manager: *mut ClutterLayoutManager,
    pub dummy0: i32,
    pub dummy1: gpointer,
}

impl ::std::fmt::Debug for ClutterLayoutMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterLayoutMeta @ {:?}", self as *const _))
            .field("manager", &self.manager)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterListModel {
    pub parent_instance: ClutterModel,
    pub priv_: *mut ClutterListModelPrivate,
}

impl ::std::fmt::Debug for ClutterListModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterListModel @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterModel {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterModelPrivate,
}

impl ::std::fmt::Debug for ClutterModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterModel @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterModelIter {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterModelIterPrivate,
}

impl ::std::fmt::Debug for ClutterModelIter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterModelIter @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterOffscreenEffect {
    pub parent_instance: ClutterEffect,
    pub priv_: *mut ClutterOffscreenEffectPrivate,
}

impl ::std::fmt::Debug for ClutterOffscreenEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterOffscreenEffect @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterPageTurnEffect(c_void);

impl ::std::fmt::Debug for ClutterPageTurnEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPageTurnEffect @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterPaintNode(c_void);

impl ::std::fmt::Debug for ClutterPaintNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPaintNode @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterPanAction {
    pub parent_instance: ClutterGestureAction,
    pub priv_: *mut ClutterPanActionPrivate,
}

impl ::std::fmt::Debug for ClutterPanAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPanAction @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterParamSpecColor {
    pub parent_instance: gobject::GParamSpec,
    pub default_value: *mut ClutterColor,
}

impl ::std::fmt::Debug for ClutterParamSpecColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterParamSpecColor @ {:?}", self as *const _))
            .field("default_value", &self.default_value)
            .finish()
    }
}

#[repr(C)]
pub struct ClutterParamSpecFixed {
    pub parent_instance: gobject::GParamSpec,
    _truncated_record_marker: c_void,
    // /*Ignored*/field minimum has incomplete type
}

impl ::std::fmt::Debug for ClutterParamSpecFixed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterParamSpecFixed @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterParamSpecUnit(c_void);

impl ::std::fmt::Debug for ClutterParamSpecUnit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterParamSpecUnit @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterPath {
    pub parent: gobject::GInitiallyUnowned,
    pub priv_: *mut ClutterPathPrivate,
}

impl ::std::fmt::Debug for ClutterPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPath @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterPathConstraint(c_void);

impl ::std::fmt::Debug for ClutterPathConstraint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPathConstraint @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterPipelineNode(c_void);

impl ::std::fmt::Debug for ClutterPipelineNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterPipelineNode @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterPropertyTransition {
    pub parent_instance: ClutterTransition,
    pub priv_: *mut ClutterPropertyTransitionPrivate,
}

impl ::std::fmt::Debug for ClutterPropertyTransition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ClutterPropertyTransition @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterRectangle {
    pub parent: ClutterActor,
    pub priv_: *mut ClutterRectanglePrivate,
}

impl ::std::fmt::Debug for ClutterRectangle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterRectangle @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterRotateAction {
    pub parent_instance: ClutterGestureAction,
    pub priv_: *mut ClutterRotateActionPrivate,
}

impl ::std::fmt::Debug for ClutterRotateAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterRotateAction @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterScore {
    pub parent: gobject::GObject,
    pub priv_: *mut ClutterScorePrivate,
}

impl ::std::fmt::Debug for ClutterScore {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterScore @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterScript {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterScriptPrivate,
}

impl ::std::fmt::Debug for ClutterScript {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterScript @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterScrollActor {
    pub parent_instance: ClutterActor,
    pub priv_: *mut ClutterScrollActorPrivate,
}

impl ::std::fmt::Debug for ClutterScrollActor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterScrollActor @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterSettings(c_void);

impl ::std::fmt::Debug for ClutterSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterSettings @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterShader {
    pub parent: gobject::GObject,
    pub priv_: *mut ClutterShaderPrivate,
}

impl ::std::fmt::Debug for ClutterShader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterShader @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterShaderEffect {
    pub parent_instance: ClutterOffscreenEffect,
    pub priv_: *mut ClutterShaderEffectPrivate,
}

impl ::std::fmt::Debug for ClutterShaderEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterShaderEffect @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterShaderFloat(c_void);

impl ::std::fmt::Debug for ClutterShaderFloat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterShaderFloat @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterShaderInt(c_void);

impl ::std::fmt::Debug for ClutterShaderInt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterShaderInt @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterShaderMatrix(c_void);

impl ::std::fmt::Debug for ClutterShaderMatrix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterShaderMatrix @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterSnapConstraint(c_void);

impl ::std::fmt::Debug for ClutterSnapConstraint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterSnapConstraint @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterStage {
    pub parent_instance: ClutterGroup,
    pub priv_: *mut ClutterStagePrivate,
}

impl ::std::fmt::Debug for ClutterStage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterStage @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterStageManager(c_void);

impl ::std::fmt::Debug for ClutterStageManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterStageManager @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterState {
    pub parent: gobject::GObject,
    pub priv_: *mut ClutterStatePrivate,
}

impl ::std::fmt::Debug for ClutterState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterState @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterSwipeAction {
    pub parent_instance: ClutterGestureAction,
    pub priv_: *mut ClutterSwipeActionPrivate,
}

impl ::std::fmt::Debug for ClutterSwipeAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterSwipeAction @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTableLayout {
    pub parent_instance: ClutterLayoutManager,
    pub priv_: *mut ClutterTableLayoutPrivate,
}

impl ::std::fmt::Debug for ClutterTableLayout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTableLayout @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTapAction {
    pub parent_instance: ClutterGestureAction,
}

impl ::std::fmt::Debug for ClutterTapAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTapAction @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterText {
    pub parent_instance: ClutterActor,
    pub priv_: *mut ClutterTextPrivate,
}

impl ::std::fmt::Debug for ClutterText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterText @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTextBuffer {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterTextBufferPrivate,
}

impl ::std::fmt::Debug for ClutterTextBuffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTextBuffer @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterTextNode(c_void);

impl ::std::fmt::Debug for ClutterTextNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTextNode @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTexture {
    pub parent: ClutterActor,
    pub priv_: *mut ClutterTexturePrivate,
}

impl ::std::fmt::Debug for ClutterTexture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTexture @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ClutterTextureNode(c_void);

impl ::std::fmt::Debug for ClutterTextureNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTextureNode @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTimeline {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ClutterTimelinePrivate,
}

impl ::std::fmt::Debug for ClutterTimeline {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTimeline @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTransition {
    pub parent_instance: ClutterTimeline,
    pub priv_: *mut ClutterTransitionPrivate,
}

impl ::std::fmt::Debug for ClutterTransition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTransition @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterTransitionGroup {
    pub parent_instance: ClutterTransition,
    pub priv_: *mut ClutterTransitionGroupPrivate,
}

impl ::std::fmt::Debug for ClutterTransitionGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterTransitionGroup @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClutterZoomAction {
    pub parent_instance: ClutterGestureAction,
    pub priv_: *mut ClutterZoomActionPrivate,
}

impl ::std::fmt::Debug for ClutterZoomAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClutterZoomAction @ {:?}", self as *const _))
            .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct ClutterAnimatable(c_void);

impl ::std::fmt::Debug for ClutterAnimatable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "ClutterAnimatable @ {:?}", self as *const _)
    }
}

#[repr(C)]
pub struct ClutterContainer(c_void);

impl ::std::fmt::Debug for ClutterContainer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "ClutterContainer @ {:?}", self as *const _)
    }
}

#[repr(C)]
pub struct ClutterContent(c_void);

impl ::std::fmt::Debug for ClutterContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "ClutterContent @ {:?}", self as *const _)
    }
}

#[repr(C)]
pub struct ClutterMedia(c_void);

impl ::std::fmt::Debug for ClutterMedia {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "ClutterMedia @ {:?}", self as *const _)
    }
}

#[repr(C)]
pub struct ClutterScriptable(c_void);

impl ::std::fmt::Debug for ClutterScriptable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "ClutterScriptable @ {:?}", self as *const _)
    }
}

#[link(name = "clutter-1.0")]
extern "C" {

    //=========================================================================
    // ClutterActorAlign
    //=========================================================================
    pub fn clutter_actor_align_get_type() -> GType;

    //=========================================================================
    // ClutterAlignAxis
    //=========================================================================
    pub fn clutter_align_axis_get_type() -> GType;

    //=========================================================================
    // ClutterAnimationMode
    //=========================================================================
    pub fn clutter_animation_mode_get_type() -> GType;

    //=========================================================================
    // ClutterBinAlignment
    //=========================================================================
    pub fn clutter_bin_alignment_get_type() -> GType;

    //=========================================================================
    // ClutterBindCoordinate
    //=========================================================================
    pub fn clutter_bind_coordinate_get_type() -> GType;

    //=========================================================================
    // ClutterBoxAlignment
    //=========================================================================
    pub fn clutter_box_alignment_get_type() -> GType;

    //=========================================================================
    // ClutterContentGravity
    //=========================================================================
    pub fn clutter_content_gravity_get_type() -> GType;

    //=========================================================================
    // ClutterDragAxis
    //=========================================================================
    pub fn clutter_drag_axis_get_type() -> GType;

    //=========================================================================
    // ClutterEventType
    //=========================================================================
    pub fn clutter_event_type_get_type() -> GType;

    //=========================================================================
    // ClutterFlowOrientation
    //=========================================================================
    pub fn clutter_flow_orientation_get_type() -> GType;

    //=========================================================================
    // ClutterGestureTriggerEdge
    //=========================================================================
    pub fn clutter_gesture_trigger_edge_get_type() -> GType;

    //=========================================================================
    // ClutterGravity
    //=========================================================================
    pub fn clutter_gravity_get_type() -> GType;

    //=========================================================================
    // ClutterGridPosition
    //=========================================================================
    pub fn clutter_grid_position_get_type() -> GType;

    //=========================================================================
    // ClutterImageError
    //=========================================================================
    pub fn clutter_image_error_get_type() -> GType;
    pub fn clutter_image_error_quark() -> glib::GQuark;

    //=========================================================================
    // ClutterInitError
    //=========================================================================
    pub fn clutter_init_error_get_type() -> GType;
    pub fn clutter_init_error_quark() -> glib::GQuark;

    //=========================================================================
    // ClutterInputAxis
    //=========================================================================
    pub fn clutter_input_axis_get_type() -> GType;

    //=========================================================================
    // ClutterInputDeviceType
    //=========================================================================
    pub fn clutter_input_device_type_get_type() -> GType;

    //=========================================================================
    // ClutterInputMode
    //=========================================================================
    pub fn clutter_input_mode_get_type() -> GType;

    //=========================================================================
    // ClutterInterpolation
    //=========================================================================
    pub fn clutter_interpolation_get_type() -> GType;

    //=========================================================================
    // ClutterLongPressState
    //=========================================================================
    pub fn clutter_long_press_state_get_type() -> GType;

    //=========================================================================
    // ClutterOrientation
    //=========================================================================
    pub fn clutter_orientation_get_type() -> GType;

    //=========================================================================
    // ClutterPanAxis
    //=========================================================================
    pub fn clutter_pan_axis_get_type() -> GType;

    //=========================================================================
    // ClutterPathNodeType
    //=========================================================================
    pub fn clutter_path_node_type_get_type() -> GType;

    //=========================================================================
    // ClutterPickMode
    //=========================================================================
    pub fn clutter_pick_mode_get_type() -> GType;

    //=========================================================================
    // ClutterRequestMode
    //=========================================================================
    pub fn clutter_request_mode_get_type() -> GType;

    //=========================================================================
    // ClutterRotateAxis
    //=========================================================================
    pub fn clutter_rotate_axis_get_type() -> GType;

    //=========================================================================
    // ClutterRotateDirection
    //=========================================================================
    pub fn clutter_rotate_direction_get_type() -> GType;

    //=========================================================================
    // ClutterScalingFilter
    //=========================================================================
    pub fn clutter_scaling_filter_get_type() -> GType;

    //=========================================================================
    // ClutterScriptError
    //=========================================================================
    pub fn clutter_script_error_get_type() -> GType;
    pub fn clutter_script_error_quark() -> glib::GQuark;

    //=========================================================================
    // ClutterScrollDirection
    //=========================================================================
    pub fn clutter_scroll_direction_get_type() -> GType;

    //=========================================================================
    // ClutterScrollSource
    //=========================================================================
    pub fn clutter_scroll_source_get_type() -> GType;

    //=========================================================================
    // ClutterShaderError
    //=========================================================================
    pub fn clutter_shader_error_get_type() -> GType;
    pub fn clutter_shader_error_quark() -> glib::GQuark;

    //=========================================================================
    // ClutterShaderType
    //=========================================================================
    pub fn clutter_shader_type_get_type() -> GType;

    //=========================================================================
    // ClutterSnapEdge
    //=========================================================================
    pub fn clutter_snap_edge_get_type() -> GType;

    //=========================================================================
    // ClutterStaticColor
    //=========================================================================
    pub fn clutter_static_color_get_type() -> GType;

    //=========================================================================
    // ClutterStepMode
    //=========================================================================
    pub fn clutter_step_mode_get_type() -> GType;

    //=========================================================================
    // ClutterTableAlignment
    //=========================================================================
    pub fn clutter_table_alignment_get_type() -> GType;

    //=========================================================================
    // ClutterTextDirection
    //=========================================================================
    pub fn clutter_text_direction_get_type() -> GType;

    //=========================================================================
    // ClutterTextureError
    //=========================================================================
    pub fn clutter_texture_error_get_type() -> GType;
    pub fn clutter_texture_error_quark() -> glib::GQuark;

    //=========================================================================
    // ClutterTextureQuality
    //=========================================================================
    pub fn clutter_texture_quality_get_type() -> GType;

    //=========================================================================
    // ClutterTimelineDirection
    //=========================================================================
    pub fn clutter_timeline_direction_get_type() -> GType;

    //=========================================================================
    // ClutterTouchpadGesturePhase
    //=========================================================================
    pub fn clutter_touchpad_gesture_phase_get_type() -> GType;

    //=========================================================================
    // ClutterUnitType
    //=========================================================================
    pub fn clutter_unit_type_get_type() -> GType;

    //=========================================================================
    // ClutterZoomAxis
    //=========================================================================
    pub fn clutter_zoom_axis_get_type() -> GType;

    //=========================================================================
    // ClutterActorFlags
    //=========================================================================
    pub fn clutter_actor_flags_get_type() -> GType;

    //=========================================================================
    // ClutterAllocationFlags
    //=========================================================================
    pub fn clutter_allocation_flags_get_type() -> GType;

    //=========================================================================
    // ClutterContentRepeat
    //=========================================================================
    pub fn clutter_content_repeat_get_type() -> GType;

    //=========================================================================
    // ClutterEffectPaintFlags
    //=========================================================================
    pub fn clutter_effect_paint_flags_get_type() -> GType;

    //=========================================================================
    // ClutterEventFlags
    //=========================================================================
    pub fn clutter_event_flags_get_type() -> GType;

    //=========================================================================
    // ClutterFeatureFlags
    //=========================================================================
    pub fn clutter_feature_flags_get_type() -> GType;

    //=========================================================================
    // ClutterFontFlags
    //=========================================================================
    pub fn clutter_font_flags_get_type() -> GType;

    //=========================================================================
    // ClutterModifierType
    //=========================================================================
    pub fn clutter_modifier_type_get_type() -> GType;

    //=========================================================================
    // ClutterOffscreenRedirect
    //=========================================================================
    pub fn clutter_offscreen_redirect_get_type() -> GType;

    //=========================================================================
    // ClutterRepaintFlags
    //=========================================================================
    pub fn clutter_repaint_flags_get_type() -> GType;

    //=========================================================================
    // ClutterScrollFinishFlags
    //=========================================================================
    pub fn clutter_scroll_finish_flags_get_type() -> GType;

    //=========================================================================
    // ClutterScrollMode
    //=========================================================================
    pub fn clutter_scroll_mode_get_type() -> GType;

    //=========================================================================
    // ClutterStageState
    //=========================================================================
    pub fn clutter_stage_state_get_type() -> GType;

    //=========================================================================
    // ClutterSwipeDirection
    //=========================================================================
    pub fn clutter_swipe_direction_get_type() -> GType;

    //=========================================================================
    // ClutterTextureFlags
    //=========================================================================
    pub fn clutter_texture_flags_get_type() -> GType;

    //=========================================================================
    // ClutterEvent
    //=========================================================================
    pub fn clutter_event_get_type() -> GType;
    pub fn clutter_event_new(type_: ClutterEventType) -> *mut ClutterEvent;
    pub fn clutter_event_copy(event: *const ClutterEvent) -> *mut ClutterEvent;
    pub fn clutter_event_free(event: *mut ClutterEvent);
    pub fn clutter_event_get_angle(
        source: *const ClutterEvent,
        target: *const ClutterEvent,
    ) -> c_double;
    pub fn clutter_event_get_axes(event: *const ClutterEvent, n_axes: *mut c_uint)
        -> *mut c_double;
    pub fn clutter_event_get_button(event: *const ClutterEvent) -> u32;
    pub fn clutter_event_get_click_count(event: *const ClutterEvent) -> c_uint;
    pub fn clutter_event_get_coords(event: *const ClutterEvent, x: *mut c_float, y: *mut c_float);
    pub fn clutter_event_get_device(event: *const ClutterEvent) -> *mut ClutterInputDevice;
    pub fn clutter_event_get_device_id(event: *const ClutterEvent) -> c_int;
    pub fn clutter_event_get_device_type(event: *const ClutterEvent) -> ClutterInputDeviceType;
    pub fn clutter_event_get_distance(
        source: *const ClutterEvent,
        target: *const ClutterEvent,
    ) -> c_float;
    pub fn clutter_event_get_event_sequence(
        event: *const ClutterEvent,
    ) -> *mut ClutterEventSequence;
    pub fn clutter_event_get_flags(event: *const ClutterEvent) -> ClutterEventFlags;
    pub fn clutter_event_get_gesture_motion_delta(
        event: *const ClutterEvent,
        dx: *mut c_double,
        dy: *mut c_double,
    );
    pub fn clutter_event_get_gesture_phase(
        event: *const ClutterEvent,
    ) -> ClutterTouchpadGesturePhase;
    pub fn clutter_event_get_gesture_pinch_angle_delta(event: *const ClutterEvent) -> c_double;
    pub fn clutter_event_get_gesture_pinch_scale(event: *const ClutterEvent) -> c_double;
    pub fn clutter_event_get_gesture_swipe_finger_count(event: *const ClutterEvent) -> c_uint;
    pub fn clutter_event_get_key_code(event: *const ClutterEvent) -> u16;
    pub fn clutter_event_get_key_symbol(event: *const ClutterEvent) -> c_uint;
    pub fn clutter_event_get_key_unicode(event: *const ClutterEvent) -> u32;
    pub fn clutter_event_get_position(event: *const ClutterEvent, position: *mut ClutterPoint);
    pub fn clutter_event_get_related(event: *const ClutterEvent) -> *mut ClutterActor;
    pub fn clutter_event_get_scroll_delta(
        event: *const ClutterEvent,
        dx: *mut c_double,
        dy: *mut c_double,
    );
    pub fn clutter_event_get_scroll_direction(event: *const ClutterEvent)
        -> ClutterScrollDirection;
    pub fn clutter_event_get_scroll_finish_flags(
        event: *const ClutterEvent,
    ) -> ClutterScrollFinishFlags;
    pub fn clutter_event_get_scroll_source(event: *const ClutterEvent) -> ClutterScrollSource;
    pub fn clutter_event_get_source(event: *const ClutterEvent) -> *mut ClutterActor;
    pub fn clutter_event_get_source_device(event: *const ClutterEvent) -> *mut ClutterInputDevice;
    pub fn clutter_event_get_stage(event: *const ClutterEvent) -> *mut ClutterStage;
    pub fn clutter_event_get_state(event: *const ClutterEvent) -> ClutterModifierType;
    pub fn clutter_event_get_state_full(
        event: *const ClutterEvent,
        button_state: *mut ClutterModifierType,
        base_state: *mut ClutterModifierType,
        latched_state: *mut ClutterModifierType,
        locked_state: *mut ClutterModifierType,
        effective_state: *mut ClutterModifierType,
    );
    pub fn clutter_event_get_time(event: *const ClutterEvent) -> u32;
    pub fn clutter_event_has_control_modifier(event: *const ClutterEvent) -> gboolean;
    pub fn clutter_event_has_shift_modifier(event: *const ClutterEvent) -> gboolean;
    pub fn clutter_event_is_pointer_emulated(event: *const ClutterEvent) -> gboolean;
    pub fn clutter_event_put(event: *const ClutterEvent);
    pub fn clutter_event_set_button(event: *mut ClutterEvent, button: u32);
    pub fn clutter_event_set_coords(event: *mut ClutterEvent, x: c_float, y: c_float);
    pub fn clutter_event_set_device(event: *mut ClutterEvent, device: *mut ClutterInputDevice);
    pub fn clutter_event_set_flags(event: *mut ClutterEvent, flags: ClutterEventFlags);
    pub fn clutter_event_set_key_code(event: *mut ClutterEvent, key_code: u16);
    pub fn clutter_event_set_key_symbol(event: *mut ClutterEvent, key_sym: c_uint);
    pub fn clutter_event_set_key_unicode(event: *mut ClutterEvent, key_unicode: u32);
    pub fn clutter_event_set_related(event: *mut ClutterEvent, actor: *mut ClutterActor);
    pub fn clutter_event_set_scroll_delta(event: *mut ClutterEvent, dx: c_double, dy: c_double);
    pub fn clutter_event_set_scroll_direction(
        event: *mut ClutterEvent,
        direction: ClutterScrollDirection,
    );
    pub fn clutter_event_set_source(event: *mut ClutterEvent, actor: *mut ClutterActor);
    pub fn clutter_event_set_source_device(
        event: *mut ClutterEvent,
        device: *mut ClutterInputDevice,
    );
    pub fn clutter_event_set_stage(event: *mut ClutterEvent, stage: *mut ClutterStage);
    pub fn clutter_event_set_state(event: *mut ClutterEvent, state: ClutterModifierType);
    pub fn clutter_event_set_time(event: *mut ClutterEvent, time_: u32);
    pub fn clutter_event_type(event: *const ClutterEvent) -> ClutterEventType;
    pub fn clutter_event_add_filter(
        stage: *mut ClutterStage,
        func: ClutterEventFilterFunc,
        notify: glib::GDestroyNotify,
        user_data: gpointer,
    ) -> c_uint;
    pub fn clutter_event_get() -> *mut ClutterEvent;
    pub fn clutter_event_peek() -> *mut ClutterEvent;
    pub fn clutter_event_remove_filter(id: c_uint);

    //=========================================================================
    // ClutterActorBox
    //=========================================================================
    pub fn clutter_actor_box_get_type() -> GType;
    pub fn clutter_actor_box_new(
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
    ) -> *mut ClutterActorBox;
    pub fn clutter_actor_box_clamp_to_pixel(box_: *mut ClutterActorBox);
    pub fn clutter_actor_box_contains(
        box_: *const ClutterActorBox,
        x: c_float,
        y: c_float,
    ) -> gboolean;
    pub fn clutter_actor_box_copy(box_: *const ClutterActorBox) -> *mut ClutterActorBox;
    pub fn clutter_actor_box_equal(
        box_a: *const ClutterActorBox,
        box_b: *const ClutterActorBox,
    ) -> gboolean;
    pub fn clutter_actor_box_free(box_: *mut ClutterActorBox);
    pub fn clutter_actor_box_from_vertices(
        box_: *mut ClutterActorBox,
        verts: *const [ClutterVertex; 4],
    );
    pub fn clutter_actor_box_get_area(box_: *const ClutterActorBox) -> c_float;
    pub fn clutter_actor_box_get_height(box_: *const ClutterActorBox) -> c_float;
    pub fn clutter_actor_box_get_origin(
        box_: *const ClutterActorBox,
        x: *mut c_float,
        y: *mut c_float,
    );
    pub fn clutter_actor_box_get_size(
        box_: *const ClutterActorBox,
        width: *mut c_float,
        height: *mut c_float,
    );
    pub fn clutter_actor_box_get_width(box_: *const ClutterActorBox) -> c_float;
    pub fn clutter_actor_box_get_x(box_: *const ClutterActorBox) -> c_float;
    pub fn clutter_actor_box_get_y(box_: *const ClutterActorBox) -> c_float;
    pub fn clutter_actor_box_init(
        box_: *mut ClutterActorBox,
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
    ) -> *mut ClutterActorBox;
    pub fn clutter_actor_box_init_rect(
        box_: *mut ClutterActorBox,
        x: c_float,
        y: c_float,
        width: c_float,
        height: c_float,
    );
    pub fn clutter_actor_box_interpolate(
        initial: *const ClutterActorBox,
        final_: *const ClutterActorBox,
        progress: c_double,
        result: *mut ClutterActorBox,
    );
    pub fn clutter_actor_box_set_origin(box_: *mut ClutterActorBox, x: c_float, y: c_float);
    pub fn clutter_actor_box_set_size(box_: *mut ClutterActorBox, width: c_float, height: c_float);
    pub fn clutter_actor_box_union(
        a: *const ClutterActorBox,
        b: *const ClutterActorBox,
        result: *mut ClutterActorBox,
    );
    pub fn clutter_actor_box_alloc() -> *mut ClutterActorBox;

    //=========================================================================
    // ClutterActorIter
    //=========================================================================
    pub fn clutter_actor_iter_destroy(iter: *mut ClutterActorIter);
    pub fn clutter_actor_iter_init(iter: *mut ClutterActorIter, root: *mut ClutterActor);
    pub fn clutter_actor_iter_is_valid(iter: *const ClutterActorIter) -> gboolean;
    pub fn clutter_actor_iter_next(
        iter: *mut ClutterActorIter,
        child: *mut *mut ClutterActor,
    ) -> gboolean;
    pub fn clutter_actor_iter_prev(
        iter: *mut ClutterActorIter,
        child: *mut *mut ClutterActor,
    ) -> gboolean;
    pub fn clutter_actor_iter_remove(iter: *mut ClutterActorIter);

    //=========================================================================
    // ClutterAnimatorKey
    //=========================================================================
    pub fn clutter_animator_key_get_type() -> GType;
    pub fn clutter_animator_key_get_mode(key: *const ClutterAnimatorKey) -> c_ulong;
    pub fn clutter_animator_key_get_object(key: *const ClutterAnimatorKey)
        -> *mut gobject::GObject;
    pub fn clutter_animator_key_get_progress(key: *const ClutterAnimatorKey) -> c_double;
    pub fn clutter_animator_key_get_property_name(key: *const ClutterAnimatorKey) -> *const c_char;
    pub fn clutter_animator_key_get_property_type(key: *const ClutterAnimatorKey) -> GType;
    pub fn clutter_animator_key_get_value(
        key: *const ClutterAnimatorKey,
        value: *mut gobject::GValue,
    ) -> gboolean;

    //=========================================================================
    // ClutterColor
    //=========================================================================
    pub fn clutter_color_get_type() -> GType;
    pub fn clutter_color_alloc() -> *mut ClutterColor;
    pub fn clutter_color_new(red: u8, green: u8, blue: u8, alpha: u8) -> *mut ClutterColor;
    pub fn clutter_color_add(
        a: *const ClutterColor,
        b: *const ClutterColor,
        result: *mut ClutterColor,
    );
    pub fn clutter_color_copy(color: *const ClutterColor) -> *mut ClutterColor;
    pub fn clutter_color_darken(color: *const ClutterColor, result: *mut ClutterColor);
    pub fn clutter_color_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn clutter_color_free(color: *mut ClutterColor);
    pub fn clutter_color_hash(v: gconstpointer) -> c_uint;
    pub fn clutter_color_init(
        color: *mut ClutterColor,
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    ) -> *mut ClutterColor;
    pub fn clutter_color_interpolate(
        initial: *const ClutterColor,
        final_: *const ClutterColor,
        progress: c_double,
        result: *mut ClutterColor,
    );
    pub fn clutter_color_lighten(color: *const ClutterColor, result: *mut ClutterColor);
    pub fn clutter_color_shade(
        color: *const ClutterColor,
        factor: c_double,
        result: *mut ClutterColor,
    );
    pub fn clutter_color_subtract(
        a: *const ClutterColor,
        b: *const ClutterColor,
        result: *mut ClutterColor,
    );
    pub fn clutter_color_to_hls(
        color: *const ClutterColor,
        hue: *mut c_float,
        luminance: *mut c_float,
        saturation: *mut c_float,
    );
    pub fn clutter_color_to_pixel(color: *const ClutterColor) -> u32;
    pub fn clutter_color_to_string(color: *const ClutterColor) -> *mut c_char;
    pub fn clutter_color_from_hls(
        color: *mut ClutterColor,
        hue: c_float,
        luminance: c_float,
        saturation: c_float,
    );
    pub fn clutter_color_from_pixel(color: *mut ClutterColor, pixel: u32);
    pub fn clutter_color_from_string(color: *mut ClutterColor, str: *const c_char) -> gboolean;
    pub fn clutter_color_get_static(color: ClutterStaticColor) -> *const ClutterColor;

    //=========================================================================
    // ClutterEventSequence
    //=========================================================================
    pub fn clutter_event_sequence_get_type() -> GType;

    //=========================================================================
    // ClutterFog
    //=========================================================================
    pub fn clutter_fog_get_type() -> GType;

    //=========================================================================
    // ClutterGeometry
    //=========================================================================
    pub fn clutter_geometry_get_type() -> GType;
    pub fn clutter_geometry_intersects(
        geometry0: *const ClutterGeometry,
        geometry1: *const ClutterGeometry,
    ) -> gboolean;
    pub fn clutter_geometry_union(
        geometry_a: *const ClutterGeometry,
        geometry_b: *const ClutterGeometry,
        result: *mut ClutterGeometry,
    );

    //=========================================================================
    // ClutterKnot
    //=========================================================================
    pub fn clutter_knot_get_type() -> GType;
    pub fn clutter_knot_copy(knot: *const ClutterKnot) -> *mut ClutterKnot;
    pub fn clutter_knot_equal(knot_a: *const ClutterKnot, knot_b: *const ClutterKnot) -> gboolean;
    pub fn clutter_knot_free(knot: *mut ClutterKnot);

    //=========================================================================
    // ClutterMargin
    //=========================================================================
    pub fn clutter_margin_get_type() -> GType;
    pub fn clutter_margin_new() -> *mut ClutterMargin;
    pub fn clutter_margin_copy(margin_: *const ClutterMargin) -> *mut ClutterMargin;
    pub fn clutter_margin_free(margin_: *mut ClutterMargin);

    //=========================================================================
    // ClutterMatrix
    //=========================================================================
    pub fn clutter_matrix_get_type() -> GType;
    pub fn clutter_matrix_free(matrix: *mut ClutterMatrix);
    pub fn clutter_matrix_init_from_array(
        matrix: *mut ClutterMatrix,
        values: *const [c_float; 16],
    ) -> *mut ClutterMatrix;
    pub fn clutter_matrix_init_from_matrix(
        a: *mut ClutterMatrix,
        b: *const ClutterMatrix,
    ) -> *mut ClutterMatrix;
    pub fn clutter_matrix_init_identity(matrix: *mut ClutterMatrix) -> *mut ClutterMatrix;
    pub fn clutter_matrix_alloc() -> *mut ClutterMatrix;

    //=========================================================================
    // ClutterPaintVolume
    //=========================================================================
    pub fn clutter_paint_volume_get_type() -> GType;
    pub fn clutter_paint_volume_copy(pv: *const ClutterPaintVolume) -> *mut ClutterPaintVolume;
    pub fn clutter_paint_volume_free(pv: *mut ClutterPaintVolume);
    pub fn clutter_paint_volume_get_depth(pv: *const ClutterPaintVolume) -> c_float;
    pub fn clutter_paint_volume_get_height(pv: *const ClutterPaintVolume) -> c_float;
    pub fn clutter_paint_volume_get_origin(
        pv: *const ClutterPaintVolume,
        vertex: *mut ClutterVertex,
    );
    pub fn clutter_paint_volume_get_width(pv: *const ClutterPaintVolume) -> c_float;
    pub fn clutter_paint_volume_set_depth(pv: *mut ClutterPaintVolume, depth: c_float);
    pub fn clutter_paint_volume_set_from_allocation(
        pv: *mut ClutterPaintVolume,
        actor: *mut ClutterActor,
    ) -> gboolean;
    pub fn clutter_paint_volume_set_height(pv: *mut ClutterPaintVolume, height: c_float);
    pub fn clutter_paint_volume_set_origin(
        pv: *mut ClutterPaintVolume,
        origin: *const ClutterVertex,
    );
    pub fn clutter_paint_volume_set_width(pv: *mut ClutterPaintVolume, width: c_float);
    pub fn clutter_paint_volume_union(
        pv: *mut ClutterPaintVolume,
        another_pv: *const ClutterPaintVolume,
    );
    pub fn clutter_paint_volume_union_box(
        pv: *mut ClutterPaintVolume,
        box_: *const ClutterActorBox,
    );

    //=========================================================================
    // ClutterPathNode
    //=========================================================================
    pub fn clutter_path_node_get_type() -> GType;
    pub fn clutter_path_node_copy(node: *const ClutterPathNode) -> *mut ClutterPathNode;
    pub fn clutter_path_node_equal(
        node_a: *const ClutterPathNode,
        node_b: *const ClutterPathNode,
    ) -> gboolean;
    pub fn clutter_path_node_free(node: *mut ClutterPathNode);

    //=========================================================================
    // ClutterPerspective
    //=========================================================================
    pub fn clutter_perspective_get_type() -> GType;

    //=========================================================================
    // ClutterPoint
    //=========================================================================
    pub fn clutter_point_get_type() -> GType;
    pub fn clutter_point_alloc() -> *mut ClutterPoint;
    pub fn clutter_point_copy(point: *const ClutterPoint) -> *mut ClutterPoint;
    pub fn clutter_point_distance(
        a: *const ClutterPoint,
        b: *const ClutterPoint,
        x_distance: *mut c_float,
        y_distance: *mut c_float,
    ) -> c_float;
    pub fn clutter_point_equals(a: *const ClutterPoint, b: *const ClutterPoint) -> gboolean;
    pub fn clutter_point_free(point: *mut ClutterPoint);
    pub fn clutter_point_init(
        point: *mut ClutterPoint,
        x: c_float,
        y: c_float,
    ) -> *mut ClutterPoint;
    pub fn clutter_point_zero() -> *const ClutterPoint;

    //=========================================================================
    // ClutterRect
    //=========================================================================
    pub fn clutter_rect_get_type() -> GType;
    pub fn clutter_rect_alloc() -> *mut ClutterRect;
    pub fn clutter_rect_clamp_to_pixel(rect: *mut ClutterRect);
    pub fn clutter_rect_contains_point(
        rect: *mut ClutterRect,
        point: *mut ClutterPoint,
    ) -> gboolean;
    pub fn clutter_rect_contains_rect(a: *mut ClutterRect, b: *mut ClutterRect) -> gboolean;
    pub fn clutter_rect_copy(rect: *const ClutterRect) -> *mut ClutterRect;
    pub fn clutter_rect_equals(a: *mut ClutterRect, b: *mut ClutterRect) -> gboolean;
    pub fn clutter_rect_free(rect: *mut ClutterRect);
    pub fn clutter_rect_get_center(rect: *mut ClutterRect, center: *mut ClutterPoint);
    pub fn clutter_rect_get_height(rect: *mut ClutterRect) -> c_float;
    pub fn clutter_rect_get_width(rect: *mut ClutterRect) -> c_float;
    pub fn clutter_rect_get_x(rect: *mut ClutterRect) -> c_float;
    pub fn clutter_rect_get_y(rect: *mut ClutterRect) -> c_float;
    pub fn clutter_rect_init(
        rect: *mut ClutterRect,
        x: c_float,
        y: c_float,
        width: c_float,
        height: c_float,
    ) -> *mut ClutterRect;
    pub fn clutter_rect_inset(rect: *mut ClutterRect, d_x: c_float, d_y: c_float);
    pub fn clutter_rect_intersection(
        a: *mut ClutterRect,
        b: *mut ClutterRect,
        res: *mut ClutterRect,
    ) -> gboolean;
    pub fn clutter_rect_normalize(rect: *mut ClutterRect) -> *mut ClutterRect;
    pub fn clutter_rect_offset(rect: *mut ClutterRect, d_x: c_float, d_y: c_float);
    pub fn clutter_rect_union(a: *mut ClutterRect, b: *mut ClutterRect, res: *mut ClutterRect);
    pub fn clutter_rect_zero() -> *const ClutterRect;

    //=========================================================================
    // ClutterSize
    //=========================================================================
    pub fn clutter_size_get_type() -> GType;
    pub fn clutter_size_alloc() -> *mut ClutterSize;
    pub fn clutter_size_copy(size: *const ClutterSize) -> *mut ClutterSize;
    pub fn clutter_size_equals(a: *const ClutterSize, b: *const ClutterSize) -> gboolean;
    pub fn clutter_size_free(size: *mut ClutterSize);
    pub fn clutter_size_init(
        size: *mut ClutterSize,
        width: c_float,
        height: c_float,
    ) -> *mut ClutterSize;

    //=========================================================================
    // ClutterStateKey
    //=========================================================================
    pub fn clutter_state_key_get_type() -> GType;
    pub fn clutter_state_key_get_mode(state_key: *const ClutterStateKey) -> c_ulong;
    pub fn clutter_state_key_get_object(state_key: *const ClutterStateKey)
        -> *mut gobject::GObject;
    pub fn clutter_state_key_get_post_delay(state_key: *const ClutterStateKey) -> c_double;
    pub fn clutter_state_key_get_pre_delay(state_key: *const ClutterStateKey) -> c_double;
    pub fn clutter_state_key_get_property_name(state_key: *const ClutterStateKey) -> *const c_char;
    pub fn clutter_state_key_get_property_type(key: *const ClutterStateKey) -> GType;
    pub fn clutter_state_key_get_source_state_name(
        state_key: *const ClutterStateKey,
    ) -> *const c_char;
    pub fn clutter_state_key_get_target_state_name(
        state_key: *const ClutterStateKey,
    ) -> *const c_char;
    pub fn clutter_state_key_get_value(
        state_key: *const ClutterStateKey,
        value: *mut gobject::GValue,
    ) -> gboolean;

    //=========================================================================
    // ClutterTimeoutPool
    //=========================================================================
    pub fn clutter_timeout_pool_add(
        pool: *mut ClutterTimeoutPool,
        fps: c_uint,
        func: glib::GSourceFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
    ) -> c_uint;
    pub fn clutter_timeout_pool_remove(pool: *mut ClutterTimeoutPool, id_: c_uint);
    pub fn clutter_timeout_pool_new(priority: c_int) -> *mut ClutterTimeoutPool;

    //=========================================================================
    // ClutterUnits
    //=========================================================================
    pub fn clutter_units_get_type() -> GType;
    pub fn clutter_units_copy(units: *const ClutterUnits) -> *mut ClutterUnits;
    pub fn clutter_units_free(units: *mut ClutterUnits);
    pub fn clutter_units_get_unit_type(units: *const ClutterUnits) -> ClutterUnitType;
    pub fn clutter_units_get_unit_value(units: *const ClutterUnits) -> c_float;
    pub fn clutter_units_to_pixels(units: *mut ClutterUnits) -> c_float;
    pub fn clutter_units_to_string(units: *const ClutterUnits) -> *mut c_char;
    pub fn clutter_units_from_cm(units: *mut ClutterUnits, cm: c_float);
    pub fn clutter_units_from_em(units: *mut ClutterUnits, em: c_float);
    pub fn clutter_units_from_em_for_font(
        units: *mut ClutterUnits,
        font_name: *const c_char,
        em: c_float,
    );
    pub fn clutter_units_from_mm(units: *mut ClutterUnits, mm: c_float);
    pub fn clutter_units_from_pixels(units: *mut ClutterUnits, px: c_int);
    pub fn clutter_units_from_pt(units: *mut ClutterUnits, pt: c_float);
    pub fn clutter_units_from_string(units: *mut ClutterUnits, str: *const c_char) -> gboolean;

    //=========================================================================
    // ClutterVertex
    //=========================================================================
    pub fn clutter_vertex_get_type() -> GType;
    pub fn clutter_vertex_alloc() -> *mut ClutterVertex;
    pub fn clutter_vertex_new(x: c_float, y: c_float, z: c_float) -> *mut ClutterVertex;
    pub fn clutter_vertex_copy(vertex: *const ClutterVertex) -> *mut ClutterVertex;
    pub fn clutter_vertex_equal(
        vertex_a: *const ClutterVertex,
        vertex_b: *const ClutterVertex,
    ) -> gboolean;
    pub fn clutter_vertex_free(vertex: *mut ClutterVertex);
    pub fn clutter_vertex_init(
        vertex: *mut ClutterVertex,
        x: c_float,
        y: c_float,
        z: c_float,
    ) -> *mut ClutterVertex;

    //=========================================================================
    // ClutterAction
    //=========================================================================
    pub fn clutter_action_get_type() -> GType;

    //=========================================================================
    // ClutterActor
    //=========================================================================
    pub fn clutter_actor_get_type() -> GType;
    pub fn clutter_actor_new() -> *mut ClutterActor;
    pub fn clutter_actor_add_action(self_: *mut ClutterActor, action: *mut ClutterAction);
    pub fn clutter_actor_add_action_with_name(
        self_: *mut ClutterActor,
        name: *const c_char,
        action: *mut ClutterAction,
    );
    pub fn clutter_actor_add_child(self_: *mut ClutterActor, child: *mut ClutterActor);
    pub fn clutter_actor_add_constraint(
        self_: *mut ClutterActor,
        constraint: *mut ClutterConstraint,
    );
    pub fn clutter_actor_add_constraint_with_name(
        self_: *mut ClutterActor,
        name: *const c_char,
        constraint: *mut ClutterConstraint,
    );
    pub fn clutter_actor_add_effect(self_: *mut ClutterActor, effect: *mut ClutterEffect);
    pub fn clutter_actor_add_effect_with_name(
        self_: *mut ClutterActor,
        name: *const c_char,
        effect: *mut ClutterEffect,
    );
    pub fn clutter_actor_add_transition(
        self_: *mut ClutterActor,
        name: *const c_char,
        transition: *mut ClutterTransition,
    );
    pub fn clutter_actor_allocate(
        self_: *mut ClutterActor,
        box_: *const ClutterActorBox,
        flags: ClutterAllocationFlags,
    );
    pub fn clutter_actor_allocate_align_fill(
        self_: *mut ClutterActor,
        box_: *const ClutterActorBox,
        x_align: c_double,
        y_align: c_double,
        x_fill: gboolean,
        y_fill: gboolean,
        flags: ClutterAllocationFlags,
    );
    pub fn clutter_actor_allocate_available_size(
        self_: *mut ClutterActor,
        x: c_float,
        y: c_float,
        available_width: c_float,
        available_height: c_float,
        flags: ClutterAllocationFlags,
    );
    pub fn clutter_actor_allocate_preferred_size(
        self_: *mut ClutterActor,
        flags: ClutterAllocationFlags,
    );
    pub fn clutter_actor_animate(
        actor: *mut ClutterActor,
        mode: c_ulong,
        duration: c_uint,
        first_property_name: *const c_char,
        ...
    ) -> *mut ClutterAnimation;
    pub fn clutter_actor_animate_with_alpha(
        actor: *mut ClutterActor,
        alpha: *mut ClutterAlpha,
        first_property_name: *const c_char,
        ...
    ) -> *mut ClutterAnimation;
    pub fn clutter_actor_animate_with_alphav(
        actor: *mut ClutterActor,
        alpha: *mut ClutterAlpha,
        n_properties: c_int,
        properties: *const *const c_char,
        values: *const gobject::GValue,
    ) -> *mut ClutterAnimation;
    pub fn clutter_actor_animate_with_timeline(
        actor: *mut ClutterActor,
        mode: c_ulong,
        timeline: *mut ClutterTimeline,
        first_property_name: *const c_char,
        ...
    ) -> *mut ClutterAnimation;
    pub fn clutter_actor_animate_with_timelinev(
        actor: *mut ClutterActor,
        mode: c_ulong,
        timeline: *mut ClutterTimeline,
        n_properties: c_int,
        properties: *const *const c_char,
        values: *const gobject::GValue,
    ) -> *mut ClutterAnimation;
    pub fn clutter_actor_animatev(
        actor: *mut ClutterActor,
        mode: c_ulong,
        duration: c_uint,
        n_properties: c_int,
        properties: *const *const c_char,
        values: *const gobject::GValue,
    ) -> *mut ClutterAnimation;
    pub fn clutter_actor_apply_relative_transform_to_point(
        self_: *mut ClutterActor,
        ancestor: *mut ClutterActor,
        point: *const ClutterVertex,
        vertex: *mut ClutterVertex,
    );
    pub fn clutter_actor_apply_transform_to_point(
        self_: *mut ClutterActor,
        point: *const ClutterVertex,
        vertex: *mut ClutterVertex,
    );
    pub fn clutter_actor_bind_model(
        self_: *mut ClutterActor,
        model: *mut gio::GListModel,
        create_child_func: ClutterActorCreateChildFunc,
        user_data: gpointer,
        notify: glib::GDestroyNotify,
    );
    pub fn clutter_actor_bind_model_with_properties(
        self_: *mut ClutterActor,
        model: *mut gio::GListModel,
        child_type: GType,
        first_model_property: *const c_char,
        ...
    );
    pub fn clutter_actor_clear_actions(self_: *mut ClutterActor);
    pub fn clutter_actor_clear_constraints(self_: *mut ClutterActor);
    pub fn clutter_actor_clear_effects(self_: *mut ClutterActor);
    pub fn clutter_actor_contains(
        self_: *mut ClutterActor,
        descendant: *mut ClutterActor,
    ) -> gboolean;
    pub fn clutter_actor_continue_paint(self_: *mut ClutterActor);
    pub fn clutter_actor_create_pango_context(self_: *mut ClutterActor)
        -> *mut pango::PangoContext;
    pub fn clutter_actor_create_pango_layout(
        self_: *mut ClutterActor,
        text: *const c_char,
    ) -> *mut pango::PangoLayout;
    pub fn clutter_actor_destroy(self_: *mut ClutterActor);
    pub fn clutter_actor_destroy_all_children(self_: *mut ClutterActor);
    pub fn clutter_actor_detach_animation(actor: *mut ClutterActor);
    pub fn clutter_actor_event(
        actor: *mut ClutterActor,
        event: *const ClutterEvent,
        capture: gboolean,
    ) -> gboolean;
    pub fn clutter_actor_get_abs_allocation_vertices(
        self_: *mut ClutterActor,
        verts: *mut [ClutterVertex; 4],
    );
    pub fn clutter_actor_get_accessible(self_: *mut ClutterActor) -> *mut atk::AtkObject;
    pub fn clutter_actor_get_action(
        self_: *mut ClutterActor,
        name: *const c_char,
    ) -> *mut ClutterAction;
    pub fn clutter_actor_get_actions(self_: *mut ClutterActor) -> *mut glib::GList;
    pub fn clutter_actor_get_allocation_box(self_: *mut ClutterActor, box_: *mut ClutterActorBox);
    pub fn clutter_actor_get_allocation_geometry(
        self_: *mut ClutterActor,
        geom: *mut ClutterGeometry,
    );
    pub fn clutter_actor_get_allocation_vertices(
        self_: *mut ClutterActor,
        ancestor: *mut ClutterActor,
        verts: *mut [ClutterVertex; 4],
    );
    pub fn clutter_actor_get_anchor_point(
        self_: *mut ClutterActor,
        anchor_x: *mut c_float,
        anchor_y: *mut c_float,
    );
    pub fn clutter_actor_get_anchor_point_gravity(self_: *mut ClutterActor) -> ClutterGravity;
    pub fn clutter_actor_get_animation(actor: *mut ClutterActor) -> *mut ClutterAnimation;
    pub fn clutter_actor_get_background_color(self_: *mut ClutterActor, color: *mut ClutterColor);
    pub fn clutter_actor_get_child_at_index(
        self_: *mut ClutterActor,
        index_: c_int,
    ) -> *mut ClutterActor;
    pub fn clutter_actor_get_child_transform(
        self_: *mut ClutterActor,
        transform: *mut ClutterMatrix,
    );
    pub fn clutter_actor_get_children(self_: *mut ClutterActor) -> *mut glib::GList;
    pub fn clutter_actor_get_clip(
        self_: *mut ClutterActor,
        xoff: *mut c_float,
        yoff: *mut c_float,
        width: *mut c_float,
        height: *mut c_float,
    );
    pub fn clutter_actor_get_clip_to_allocation(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_get_constraint(
        self_: *mut ClutterActor,
        name: *const c_char,
    ) -> *mut ClutterConstraint;
    pub fn clutter_actor_get_constraints(self_: *mut ClutterActor) -> *mut glib::GList;
    pub fn clutter_actor_get_content(self_: *mut ClutterActor) -> *mut ClutterContent;
    pub fn clutter_actor_get_content_box(self_: *mut ClutterActor, box_: *mut ClutterActorBox);
    pub fn clutter_actor_get_content_gravity(self_: *mut ClutterActor) -> ClutterContentGravity;
    pub fn clutter_actor_get_content_repeat(self_: *mut ClutterActor) -> ClutterContentRepeat;
    pub fn clutter_actor_get_content_scaling_filters(
        self_: *mut ClutterActor,
        min_filter: *mut ClutterScalingFilter,
        mag_filter: *mut ClutterScalingFilter,
    );
    pub fn clutter_actor_get_default_paint_volume(
        self_: *mut ClutterActor,
    ) -> *const ClutterPaintVolume;
    pub fn clutter_actor_get_depth(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_easing_delay(self_: *mut ClutterActor) -> c_uint;
    pub fn clutter_actor_get_easing_duration(self_: *mut ClutterActor) -> c_uint;
    pub fn clutter_actor_get_easing_mode(self_: *mut ClutterActor) -> ClutterAnimationMode;
    pub fn clutter_actor_get_effect(
        self_: *mut ClutterActor,
        name: *const c_char,
    ) -> *mut ClutterEffect;
    pub fn clutter_actor_get_effects(self_: *mut ClutterActor) -> *mut glib::GList;
    pub fn clutter_actor_get_first_child(self_: *mut ClutterActor) -> *mut ClutterActor;
    pub fn clutter_actor_get_fixed_position_set(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_get_flags(self_: *mut ClutterActor) -> ClutterActorFlags;
    pub fn clutter_actor_get_geometry(self_: *mut ClutterActor, geometry: *mut ClutterGeometry);
    pub fn clutter_actor_get_gid(self_: *mut ClutterActor) -> u32;
    pub fn clutter_actor_get_height(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_last_child(self_: *mut ClutterActor) -> *mut ClutterActor;
    pub fn clutter_actor_get_layout_manager(self_: *mut ClutterActor) -> *mut ClutterLayoutManager;
    pub fn clutter_actor_get_margin(self_: *mut ClutterActor, margin: *mut ClutterMargin);
    pub fn clutter_actor_get_margin_bottom(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_margin_left(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_margin_right(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_margin_top(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_n_children(self_: *mut ClutterActor) -> c_int;
    pub fn clutter_actor_get_name(self_: *mut ClutterActor) -> *const c_char;
    pub fn clutter_actor_get_next_sibling(self_: *mut ClutterActor) -> *mut ClutterActor;
    pub fn clutter_actor_get_offscreen_redirect(
        self_: *mut ClutterActor,
    ) -> ClutterOffscreenRedirect;
    pub fn clutter_actor_get_opacity(self_: *mut ClutterActor) -> u8;
    pub fn clutter_actor_get_paint_box(
        self_: *mut ClutterActor,
        box_: *mut ClutterActorBox,
    ) -> gboolean;
    pub fn clutter_actor_get_paint_opacity(self_: *mut ClutterActor) -> u8;
    pub fn clutter_actor_get_paint_visibility(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_get_paint_volume(self_: *mut ClutterActor) -> *const ClutterPaintVolume;
    pub fn clutter_actor_get_pango_context(self_: *mut ClutterActor) -> *mut pango::PangoContext;
    pub fn clutter_actor_get_parent(self_: *mut ClutterActor) -> *mut ClutterActor;
    pub fn clutter_actor_get_pivot_point(
        self_: *mut ClutterActor,
        pivot_x: *mut c_float,
        pivot_y: *mut c_float,
    );
    pub fn clutter_actor_get_pivot_point_z(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_position(self_: *mut ClutterActor, x: *mut c_float, y: *mut c_float);
    pub fn clutter_actor_get_preferred_height(
        self_: *mut ClutterActor,
        for_width: c_float,
        min_height_p: *mut c_float,
        natural_height_p: *mut c_float,
    );
    pub fn clutter_actor_get_preferred_size(
        self_: *mut ClutterActor,
        min_width_p: *mut c_float,
        min_height_p: *mut c_float,
        natural_width_p: *mut c_float,
        natural_height_p: *mut c_float,
    );
    pub fn clutter_actor_get_preferred_width(
        self_: *mut ClutterActor,
        for_height: c_float,
        min_width_p: *mut c_float,
        natural_width_p: *mut c_float,
    );
    pub fn clutter_actor_get_previous_sibling(self_: *mut ClutterActor) -> *mut ClutterActor;
    pub fn clutter_actor_get_reactive(actor: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_get_request_mode(self_: *mut ClutterActor) -> ClutterRequestMode;
    pub fn clutter_actor_get_rotation(
        self_: *mut ClutterActor,
        axis: ClutterRotateAxis,
        x: *mut c_float,
        y: *mut c_float,
        z: *mut c_float,
    ) -> c_double;
    pub fn clutter_actor_get_rotation_angle(
        self_: *mut ClutterActor,
        axis: ClutterRotateAxis,
    ) -> c_double;
    pub fn clutter_actor_get_scale(
        self_: *mut ClutterActor,
        scale_x: *mut c_double,
        scale_y: *mut c_double,
    );
    pub fn clutter_actor_get_scale_center(
        self_: *mut ClutterActor,
        center_x: *mut c_float,
        center_y: *mut c_float,
    );
    pub fn clutter_actor_get_scale_gravity(self_: *mut ClutterActor) -> ClutterGravity;
    pub fn clutter_actor_get_scale_z(self_: *mut ClutterActor) -> c_double;
    pub fn clutter_actor_get_shader(self_: *mut ClutterActor) -> *mut ClutterShader;
    pub fn clutter_actor_get_size(
        self_: *mut ClutterActor,
        width: *mut c_float,
        height: *mut c_float,
    );
    pub fn clutter_actor_get_stage(actor: *mut ClutterActor) -> *mut ClutterStage;
    pub fn clutter_actor_get_text_direction(self_: *mut ClutterActor) -> ClutterTextDirection;
    pub fn clutter_actor_get_transform(self_: *mut ClutterActor, transform: *mut ClutterMatrix);
    pub fn clutter_actor_get_transformation_matrix(
        self_: *mut ClutterActor,
        matrix: *mut ClutterMatrix,
    );
    pub fn clutter_actor_get_transformed_paint_volume(
        self_: *mut ClutterActor,
        relative_to_ancestor: *mut ClutterActor,
    ) -> *const ClutterPaintVolume;
    pub fn clutter_actor_get_transformed_position(
        self_: *mut ClutterActor,
        x: *mut c_float,
        y: *mut c_float,
    );
    pub fn clutter_actor_get_transformed_size(
        self_: *mut ClutterActor,
        width: *mut c_float,
        height: *mut c_float,
    );
    pub fn clutter_actor_get_transition(
        self_: *mut ClutterActor,
        name: *const c_char,
    ) -> *mut ClutterTransition;
    pub fn clutter_actor_get_translation(
        self_: *mut ClutterActor,
        translate_x: *mut c_float,
        translate_y: *mut c_float,
        translate_z: *mut c_float,
    );
    pub fn clutter_actor_get_width(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_x(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_x_align(self_: *mut ClutterActor) -> ClutterActorAlign;
    pub fn clutter_actor_get_x_expand(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_get_y(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_y_align(self_: *mut ClutterActor) -> ClutterActorAlign;
    pub fn clutter_actor_get_y_expand(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_get_z_position(self_: *mut ClutterActor) -> c_float;
    pub fn clutter_actor_get_z_rotation_gravity(self_: *mut ClutterActor) -> ClutterGravity;
    pub fn clutter_actor_grab_key_focus(self_: *mut ClutterActor);
    pub fn clutter_actor_has_actions(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_has_allocation(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_has_clip(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_has_constraints(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_has_effects(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_has_key_focus(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_has_overlaps(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_has_pointer(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_hide(self_: *mut ClutterActor);
    pub fn clutter_actor_hide_all(self_: *mut ClutterActor);
    pub fn clutter_actor_insert_child_above(
        self_: *mut ClutterActor,
        child: *mut ClutterActor,
        sibling: *mut ClutterActor,
    );
    pub fn clutter_actor_insert_child_at_index(
        self_: *mut ClutterActor,
        child: *mut ClutterActor,
        index_: c_int,
    );
    pub fn clutter_actor_insert_child_below(
        self_: *mut ClutterActor,
        child: *mut ClutterActor,
        sibling: *mut ClutterActor,
    );
    pub fn clutter_actor_is_in_clone_paint(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_is_mapped(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_is_realized(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_is_rotated(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_is_scaled(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_is_visible(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_lower(self_: *mut ClutterActor, above: *mut ClutterActor);
    pub fn clutter_actor_lower_bottom(self_: *mut ClutterActor);
    pub fn clutter_actor_map(self_: *mut ClutterActor);
    pub fn clutter_actor_move_anchor_point(
        self_: *mut ClutterActor,
        anchor_x: c_float,
        anchor_y: c_float,
    );
    pub fn clutter_actor_move_anchor_point_from_gravity(
        self_: *mut ClutterActor,
        gravity: ClutterGravity,
    );
    pub fn clutter_actor_move_by(self_: *mut ClutterActor, dx: c_float, dy: c_float);
    pub fn clutter_actor_needs_expand(
        self_: *mut ClutterActor,
        orientation: ClutterOrientation,
    ) -> gboolean;
    pub fn clutter_actor_paint(self_: *mut ClutterActor);
    pub fn clutter_actor_pop_internal(self_: *mut ClutterActor);
    pub fn clutter_actor_push_internal(self_: *mut ClutterActor);
    pub fn clutter_actor_queue_redraw(self_: *mut ClutterActor);
    pub fn clutter_actor_queue_redraw_with_clip(
        self_: *mut ClutterActor,
        clip: *const cairo::cairo_rectangle_int_t,
    );
    pub fn clutter_actor_queue_relayout(self_: *mut ClutterActor);
    pub fn clutter_actor_raise(self_: *mut ClutterActor, below: *mut ClutterActor);
    pub fn clutter_actor_raise_top(self_: *mut ClutterActor);
    pub fn clutter_actor_realize(self_: *mut ClutterActor);
    pub fn clutter_actor_remove_action(self_: *mut ClutterActor, action: *mut ClutterAction);
    pub fn clutter_actor_remove_action_by_name(self_: *mut ClutterActor, name: *const c_char);
    pub fn clutter_actor_remove_all_children(self_: *mut ClutterActor);
    pub fn clutter_actor_remove_all_transitions(self_: *mut ClutterActor);
    pub fn clutter_actor_remove_child(self_: *mut ClutterActor, child: *mut ClutterActor);
    pub fn clutter_actor_remove_clip(self_: *mut ClutterActor);
    pub fn clutter_actor_remove_constraint(
        self_: *mut ClutterActor,
        constraint: *mut ClutterConstraint,
    );
    pub fn clutter_actor_remove_constraint_by_name(self_: *mut ClutterActor, name: *const c_char);
    pub fn clutter_actor_remove_effect(self_: *mut ClutterActor, effect: *mut ClutterEffect);
    pub fn clutter_actor_remove_effect_by_name(self_: *mut ClutterActor, name: *const c_char);
    pub fn clutter_actor_remove_transition(self_: *mut ClutterActor, name: *const c_char);
    pub fn clutter_actor_reparent(self_: *mut ClutterActor, new_parent: *mut ClutterActor);
    pub fn clutter_actor_replace_child(
        self_: *mut ClutterActor,
        old_child: *mut ClutterActor,
        new_child: *mut ClutterActor,
    );
    pub fn clutter_actor_restore_easing_state(self_: *mut ClutterActor);
    pub fn clutter_actor_save_easing_state(self_: *mut ClutterActor);
    pub fn clutter_actor_set_allocation(
        self_: *mut ClutterActor,
        box_: *const ClutterActorBox,
        flags: ClutterAllocationFlags,
    );
    pub fn clutter_actor_set_anchor_point(
        self_: *mut ClutterActor,
        anchor_x: c_float,
        anchor_y: c_float,
    );
    pub fn clutter_actor_set_anchor_point_from_gravity(
        self_: *mut ClutterActor,
        gravity: ClutterGravity,
    );
    pub fn clutter_actor_set_background_color(self_: *mut ClutterActor, color: *const ClutterColor);
    pub fn clutter_actor_set_child_above_sibling(
        self_: *mut ClutterActor,
        child: *mut ClutterActor,
        sibling: *mut ClutterActor,
    );
    pub fn clutter_actor_set_child_at_index(
        self_: *mut ClutterActor,
        child: *mut ClutterActor,
        index_: c_int,
    );
    pub fn clutter_actor_set_child_below_sibling(
        self_: *mut ClutterActor,
        child: *mut ClutterActor,
        sibling: *mut ClutterActor,
    );
    pub fn clutter_actor_set_child_transform(
        self_: *mut ClutterActor,
        transform: *const ClutterMatrix,
    );
    pub fn clutter_actor_set_clip(
        self_: *mut ClutterActor,
        xoff: c_float,
        yoff: c_float,
        width: c_float,
        height: c_float,
    );
    pub fn clutter_actor_set_clip_to_allocation(self_: *mut ClutterActor, clip_set: gboolean);
    pub fn clutter_actor_set_content(self_: *mut ClutterActor, content: *mut ClutterContent);
    pub fn clutter_actor_set_content_gravity(
        self_: *mut ClutterActor,
        gravity: ClutterContentGravity,
    );
    pub fn clutter_actor_set_content_repeat(self_: *mut ClutterActor, repeat: ClutterContentRepeat);
    pub fn clutter_actor_set_content_scaling_filters(
        self_: *mut ClutterActor,
        min_filter: ClutterScalingFilter,
        mag_filter: ClutterScalingFilter,
    );
    pub fn clutter_actor_set_depth(self_: *mut ClutterActor, depth: c_float);
    pub fn clutter_actor_set_easing_delay(self_: *mut ClutterActor, msecs: c_uint);
    pub fn clutter_actor_set_easing_duration(self_: *mut ClutterActor, msecs: c_uint);
    pub fn clutter_actor_set_easing_mode(self_: *mut ClutterActor, mode: ClutterAnimationMode);
    pub fn clutter_actor_set_fixed_position_set(self_: *mut ClutterActor, is_set: gboolean);
    pub fn clutter_actor_set_flags(self_: *mut ClutterActor, flags: ClutterActorFlags);
    pub fn clutter_actor_set_geometry(self_: *mut ClutterActor, geometry: *const ClutterGeometry);
    pub fn clutter_actor_set_height(self_: *mut ClutterActor, height: c_float);
    pub fn clutter_actor_set_layout_manager(
        self_: *mut ClutterActor,
        manager: *mut ClutterLayoutManager,
    );
    pub fn clutter_actor_set_margin(self_: *mut ClutterActor, margin: *const ClutterMargin);
    pub fn clutter_actor_set_margin_bottom(self_: *mut ClutterActor, margin: c_float);
    pub fn clutter_actor_set_margin_left(self_: *mut ClutterActor, margin: c_float);
    pub fn clutter_actor_set_margin_right(self_: *mut ClutterActor, margin: c_float);
    pub fn clutter_actor_set_margin_top(self_: *mut ClutterActor, margin: c_float);
    pub fn clutter_actor_set_name(self_: *mut ClutterActor, name: *const c_char);
    pub fn clutter_actor_set_offscreen_redirect(
        self_: *mut ClutterActor,
        redirect: ClutterOffscreenRedirect,
    );
    pub fn clutter_actor_set_opacity(self_: *mut ClutterActor, opacity: u8);
    pub fn clutter_actor_set_parent(self_: *mut ClutterActor, parent: *mut ClutterActor);
    pub fn clutter_actor_set_pivot_point(
        self_: *mut ClutterActor,
        pivot_x: c_float,
        pivot_y: c_float,
    );
    pub fn clutter_actor_set_pivot_point_z(self_: *mut ClutterActor, pivot_z: c_float);
    pub fn clutter_actor_set_position(self_: *mut ClutterActor, x: c_float, y: c_float);
    pub fn clutter_actor_set_reactive(actor: *mut ClutterActor, reactive: gboolean);
    pub fn clutter_actor_set_request_mode(self_: *mut ClutterActor, mode: ClutterRequestMode);
    pub fn clutter_actor_set_rotation(
        self_: *mut ClutterActor,
        axis: ClutterRotateAxis,
        angle: c_double,
        x: c_float,
        y: c_float,
        z: c_float,
    );
    pub fn clutter_actor_set_rotation_angle(
        self_: *mut ClutterActor,
        axis: ClutterRotateAxis,
        angle: c_double,
    );
    pub fn clutter_actor_set_scale(self_: *mut ClutterActor, scale_x: c_double, scale_y: c_double);
    pub fn clutter_actor_set_scale_full(
        self_: *mut ClutterActor,
        scale_x: c_double,
        scale_y: c_double,
        center_x: c_float,
        center_y: c_float,
    );
    pub fn clutter_actor_set_scale_with_gravity(
        self_: *mut ClutterActor,
        scale_x: c_double,
        scale_y: c_double,
        gravity: ClutterGravity,
    );
    pub fn clutter_actor_set_scale_z(self_: *mut ClutterActor, scale_z: c_double);
    pub fn clutter_actor_set_shader(
        self_: *mut ClutterActor,
        shader: *mut ClutterShader,
    ) -> gboolean;
    pub fn clutter_actor_set_shader_param(
        self_: *mut ClutterActor,
        param: *const c_char,
        value: *const gobject::GValue,
    );
    pub fn clutter_actor_set_shader_param_float(
        self_: *mut ClutterActor,
        param: *const c_char,
        value: c_float,
    );
    pub fn clutter_actor_set_shader_param_int(
        self_: *mut ClutterActor,
        param: *const c_char,
        value: c_int,
    );
    pub fn clutter_actor_set_size(self_: *mut ClutterActor, width: c_float, height: c_float);
    pub fn clutter_actor_set_text_direction(
        self_: *mut ClutterActor,
        text_dir: ClutterTextDirection,
    );
    pub fn clutter_actor_set_transform(self_: *mut ClutterActor, transform: *const ClutterMatrix);
    pub fn clutter_actor_set_translation(
        self_: *mut ClutterActor,
        translate_x: c_float,
        translate_y: c_float,
        translate_z: c_float,
    );
    pub fn clutter_actor_set_width(self_: *mut ClutterActor, width: c_float);
    pub fn clutter_actor_set_x(self_: *mut ClutterActor, x: c_float);
    pub fn clutter_actor_set_x_align(self_: *mut ClutterActor, x_align: ClutterActorAlign);
    pub fn clutter_actor_set_x_expand(self_: *mut ClutterActor, expand: gboolean);
    pub fn clutter_actor_set_y(self_: *mut ClutterActor, y: c_float);
    pub fn clutter_actor_set_y_align(self_: *mut ClutterActor, y_align: ClutterActorAlign);
    pub fn clutter_actor_set_y_expand(self_: *mut ClutterActor, expand: gboolean);
    pub fn clutter_actor_set_z_position(self_: *mut ClutterActor, z_position: c_float);
    pub fn clutter_actor_set_z_rotation_from_gravity(
        self_: *mut ClutterActor,
        angle: c_double,
        gravity: ClutterGravity,
    );
    pub fn clutter_actor_should_pick_paint(self_: *mut ClutterActor) -> gboolean;
    pub fn clutter_actor_show(self_: *mut ClutterActor);
    pub fn clutter_actor_show_all(self_: *mut ClutterActor);
    pub fn clutter_actor_transform_stage_point(
        self_: *mut ClutterActor,
        x: c_float,
        y: c_float,
        x_out: *mut c_float,
        y_out: *mut c_float,
    ) -> gboolean;
    pub fn clutter_actor_unmap(self_: *mut ClutterActor);
    pub fn clutter_actor_unparent(self_: *mut ClutterActor);
    pub fn clutter_actor_unrealize(self_: *mut ClutterActor);
    pub fn clutter_actor_unset_flags(self_: *mut ClutterActor, flags: ClutterActorFlags);

    //=========================================================================
    // ClutterActorMeta
    //=========================================================================
    pub fn clutter_actor_meta_get_type() -> GType;
    pub fn clutter_actor_meta_get_actor(meta: *mut ClutterActorMeta) -> *mut ClutterActor;
    pub fn clutter_actor_meta_get_enabled(meta: *mut ClutterActorMeta) -> gboolean;
    pub fn clutter_actor_meta_get_name(meta: *mut ClutterActorMeta) -> *const c_char;
    pub fn clutter_actor_meta_set_enabled(meta: *mut ClutterActorMeta, is_enabled: gboolean);
    pub fn clutter_actor_meta_set_name(meta: *mut ClutterActorMeta, name: *const c_char);

    //=========================================================================
    // ClutterAlignConstraint
    //=========================================================================
    pub fn clutter_align_constraint_get_type() -> GType;
    pub fn clutter_align_constraint_new(
        source: *mut ClutterActor,
        axis: ClutterAlignAxis,
        factor: c_float,
    ) -> *mut ClutterConstraint;
    pub fn clutter_align_constraint_get_align_axis(
        align: *mut ClutterAlignConstraint,
    ) -> ClutterAlignAxis;
    pub fn clutter_align_constraint_get_factor(align: *mut ClutterAlignConstraint) -> c_float;
    pub fn clutter_align_constraint_get_source(
        align: *mut ClutterAlignConstraint,
    ) -> *mut ClutterActor;
    pub fn clutter_align_constraint_set_align_axis(
        align: *mut ClutterAlignConstraint,
        axis: ClutterAlignAxis,
    );
    pub fn clutter_align_constraint_set_factor(align: *mut ClutterAlignConstraint, factor: c_float);
    pub fn clutter_align_constraint_set_source(
        align: *mut ClutterAlignConstraint,
        source: *mut ClutterActor,
    );

    //=========================================================================
    // ClutterAlpha
    //=========================================================================
    pub fn clutter_alpha_get_type() -> GType;
    pub fn clutter_alpha_new() -> *mut ClutterAlpha;
    pub fn clutter_alpha_new_full(
        timeline: *mut ClutterTimeline,
        mode: c_ulong,
    ) -> *mut ClutterAlpha;
    pub fn clutter_alpha_new_with_func(
        timeline: *mut ClutterTimeline,
        func: ClutterAlphaFunc,
        data: gpointer,
        destroy: glib::GDestroyNotify,
    ) -> *mut ClutterAlpha;
    pub fn clutter_alpha_register_closure(closure: *mut gobject::GClosure) -> c_ulong;
    pub fn clutter_alpha_register_func(func: ClutterAlphaFunc, data: gpointer) -> c_ulong;
    pub fn clutter_alpha_get_alpha(alpha: *mut ClutterAlpha) -> c_double;
    pub fn clutter_alpha_get_mode(alpha: *mut ClutterAlpha) -> c_ulong;
    pub fn clutter_alpha_get_timeline(alpha: *mut ClutterAlpha) -> *mut ClutterTimeline;
    pub fn clutter_alpha_set_closure(alpha: *mut ClutterAlpha, closure: *mut gobject::GClosure);
    pub fn clutter_alpha_set_func(
        alpha: *mut ClutterAlpha,
        func: ClutterAlphaFunc,
        data: gpointer,
        destroy: glib::GDestroyNotify,
    );
    pub fn clutter_alpha_set_mode(alpha: *mut ClutterAlpha, mode: c_ulong);
    pub fn clutter_alpha_set_timeline(alpha: *mut ClutterAlpha, timeline: *mut ClutterTimeline);

    //=========================================================================
    // ClutterAnimation
    //=========================================================================
    pub fn clutter_animation_get_type() -> GType;
    pub fn clutter_animation_new() -> *mut ClutterAnimation;
    pub fn clutter_animation_bind(
        animation: *mut ClutterAnimation,
        property_name: *const c_char,
        final_: *const gobject::GValue,
    ) -> *mut ClutterAnimation;
    pub fn clutter_animation_bind_interval(
        animation: *mut ClutterAnimation,
        property_name: *const c_char,
        interval: *mut ClutterInterval,
    ) -> *mut ClutterAnimation;
    pub fn clutter_animation_completed(animation: *mut ClutterAnimation);
    pub fn clutter_animation_get_alpha(animation: *mut ClutterAnimation) -> *mut ClutterAlpha;
    pub fn clutter_animation_get_duration(animation: *mut ClutterAnimation) -> c_uint;
    pub fn clutter_animation_get_interval(
        animation: *mut ClutterAnimation,
        property_name: *const c_char,
    ) -> *mut ClutterInterval;
    pub fn clutter_animation_get_loop(animation: *mut ClutterAnimation) -> gboolean;
    pub fn clutter_animation_get_mode(animation: *mut ClutterAnimation) -> c_ulong;
    pub fn clutter_animation_get_object(animation: *mut ClutterAnimation) -> *mut gobject::GObject;
    pub fn clutter_animation_get_timeline(animation: *mut ClutterAnimation)
        -> *mut ClutterTimeline;
    pub fn clutter_animation_has_property(
        animation: *mut ClutterAnimation,
        property_name: *const c_char,
    ) -> gboolean;
    pub fn clutter_animation_set_alpha(animation: *mut ClutterAnimation, alpha: *mut ClutterAlpha);
    pub fn clutter_animation_set_duration(animation: *mut ClutterAnimation, msecs: c_uint);
    pub fn clutter_animation_set_loop(animation: *mut ClutterAnimation, loop_: gboolean);
    pub fn clutter_animation_set_mode(animation: *mut ClutterAnimation, mode: c_ulong);
    pub fn clutter_animation_set_object(
        animation: *mut ClutterAnimation,
        object: *mut gobject::GObject,
    );
    pub fn clutter_animation_set_timeline(
        animation: *mut ClutterAnimation,
        timeline: *mut ClutterTimeline,
    );
    pub fn clutter_animation_unbind_property(
        animation: *mut ClutterAnimation,
        property_name: *const c_char,
    );
    pub fn clutter_animation_update(
        animation: *mut ClutterAnimation,
        property_name: *const c_char,
        final_: *const gobject::GValue,
    ) -> *mut ClutterAnimation;
    pub fn clutter_animation_update_interval(
        animation: *mut ClutterAnimation,
        property_name: *const c_char,
        interval: *mut ClutterInterval,
    );

    //=========================================================================
    // ClutterAnimator
    //=========================================================================
    pub fn clutter_animator_get_type() -> GType;
    pub fn clutter_animator_new() -> *mut ClutterAnimator;
    pub fn clutter_animator_compute_value(
        animator: *mut ClutterAnimator,
        object: *mut gobject::GObject,
        property_name: *const c_char,
        progress: c_double,
        value: *mut gobject::GValue,
    ) -> gboolean;
    pub fn clutter_animator_get_duration(animator: *mut ClutterAnimator) -> c_uint;
    pub fn clutter_animator_get_keys(
        animator: *mut ClutterAnimator,
        object: *mut gobject::GObject,
        property_name: *const c_char,
        progress: c_double,
    ) -> *mut glib::GList;
    pub fn clutter_animator_get_timeline(animator: *mut ClutterAnimator) -> *mut ClutterTimeline;
    pub fn clutter_animator_property_get_ease_in(
        animator: *mut ClutterAnimator,
        object: *mut gobject::GObject,
        property_name: *const c_char,
    ) -> gboolean;
    pub fn clutter_animator_property_get_interpolation(
        animator: *mut ClutterAnimator,
        object: *mut gobject::GObject,
        property_name: *const c_char,
    ) -> ClutterInterpolation;
    pub fn clutter_animator_property_set_ease_in(
        animator: *mut ClutterAnimator,
        object: *mut gobject::GObject,
        property_name: *const c_char,
        ease_in: gboolean,
    );
    pub fn clutter_animator_property_set_interpolation(
        animator: *mut ClutterAnimator,
        object: *mut gobject::GObject,
        property_name: *const c_char,
        interpolation: ClutterInterpolation,
    );
    pub fn clutter_animator_remove_key(
        animator: *mut ClutterAnimator,
        object: *mut gobject::GObject,
        property_name: *const c_char,
        progress: c_double,
    );
    pub fn clutter_animator_set(
        animator: *mut ClutterAnimator,
        first_object: gpointer,
        first_property_name: *const c_char,
        first_mode: c_uint,
        first_progress: c_double,
        ...
    );
    pub fn clutter_animator_set_duration(animator: *mut ClutterAnimator, duration: c_uint);
    pub fn clutter_animator_set_key(
        animator: *mut ClutterAnimator,
        object: *mut gobject::GObject,
        property_name: *const c_char,
        mode: c_uint,
        progress: c_double,
        value: *const gobject::GValue,
    ) -> *mut ClutterAnimator;
    pub fn clutter_animator_set_timeline(
        animator: *mut ClutterAnimator,
        timeline: *mut ClutterTimeline,
    );
    pub fn clutter_animator_start(animator: *mut ClutterAnimator) -> *mut ClutterTimeline;

    //=========================================================================
    // ClutterBackend
    //=========================================================================
    pub fn clutter_backend_get_type() -> GType;
    pub fn clutter_backend_get_double_click_distance(backend: *mut ClutterBackend) -> c_uint;
    pub fn clutter_backend_get_double_click_time(backend: *mut ClutterBackend) -> c_uint;
    pub fn clutter_backend_get_font_name(backend: *mut ClutterBackend) -> *const c_char;
    pub fn clutter_backend_get_font_options(
        backend: *mut ClutterBackend,
    ) -> *const cairo::cairo_font_options_t;
    pub fn clutter_backend_get_resolution(backend: *mut ClutterBackend) -> c_double;
    pub fn clutter_backend_set_double_click_distance(
        backend: *mut ClutterBackend,
        distance: c_uint,
    );
    pub fn clutter_backend_set_double_click_time(backend: *mut ClutterBackend, msec: c_uint);
    pub fn clutter_backend_set_font_name(backend: *mut ClutterBackend, font_name: *const c_char);
    pub fn clutter_backend_set_font_options(
        backend: *mut ClutterBackend,
        options: *const cairo::cairo_font_options_t,
    );
    pub fn clutter_backend_set_resolution(backend: *mut ClutterBackend, dpi: c_double);

    //=========================================================================
    // ClutterBehaviour
    //=========================================================================
    pub fn clutter_behaviour_get_type() -> GType;
    pub fn clutter_behaviour_actors_foreach(
        behave: *mut ClutterBehaviour,
        func: ClutterBehaviourForeachFunc,
        data: gpointer,
    );
    pub fn clutter_behaviour_apply(behave: *mut ClutterBehaviour, actor: *mut ClutterActor);
    pub fn clutter_behaviour_get_actors(behave: *mut ClutterBehaviour) -> *mut glib::GSList;
    pub fn clutter_behaviour_get_alpha(behave: *mut ClutterBehaviour) -> *mut ClutterAlpha;
    pub fn clutter_behaviour_get_n_actors(behave: *mut ClutterBehaviour) -> c_int;
    pub fn clutter_behaviour_get_nth_actor(
        behave: *mut ClutterBehaviour,
        index_: c_int,
    ) -> *mut ClutterActor;
    pub fn clutter_behaviour_is_applied(
        behave: *mut ClutterBehaviour,
        actor: *mut ClutterActor,
    ) -> gboolean;
    pub fn clutter_behaviour_remove(behave: *mut ClutterBehaviour, actor: *mut ClutterActor);
    pub fn clutter_behaviour_remove_all(behave: *mut ClutterBehaviour);
    pub fn clutter_behaviour_set_alpha(behave: *mut ClutterBehaviour, alpha: *mut ClutterAlpha);

    //=========================================================================
    // ClutterBehaviourDepth
    //=========================================================================
    pub fn clutter_behaviour_depth_get_type() -> GType;
    pub fn clutter_behaviour_depth_new(
        alpha: *mut ClutterAlpha,
        depth_start: c_int,
        depth_end: c_int,
    ) -> *mut ClutterBehaviour;
    pub fn clutter_behaviour_depth_get_bounds(
        behaviour: *mut ClutterBehaviourDepth,
        depth_start: *mut c_int,
        depth_end: *mut c_int,
    );
    pub fn clutter_behaviour_depth_set_bounds(
        behaviour: *mut ClutterBehaviourDepth,
        depth_start: c_int,
        depth_end: c_int,
    );

    //=========================================================================
    // ClutterBehaviourEllipse
    //=========================================================================
    pub fn clutter_behaviour_ellipse_get_type() -> GType;
    pub fn clutter_behaviour_ellipse_new(
        alpha: *mut ClutterAlpha,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        direction: ClutterRotateDirection,
        start: c_double,
        end: c_double,
    ) -> *mut ClutterBehaviour;
    pub fn clutter_behaviour_ellipse_get_angle_end(self_: *mut ClutterBehaviourEllipse)
        -> c_double;
    pub fn clutter_behaviour_ellipse_get_angle_start(
        self_: *mut ClutterBehaviourEllipse,
    ) -> c_double;
    pub fn clutter_behaviour_ellipse_get_angle_tilt(
        self_: *mut ClutterBehaviourEllipse,
        axis: ClutterRotateAxis,
    ) -> c_double;
    pub fn clutter_behaviour_ellipse_get_center(
        self_: *mut ClutterBehaviourEllipse,
        x: *mut c_int,
        y: *mut c_int,
    );
    pub fn clutter_behaviour_ellipse_get_direction(
        self_: *mut ClutterBehaviourEllipse,
    ) -> ClutterRotateDirection;
    pub fn clutter_behaviour_ellipse_get_height(self_: *mut ClutterBehaviourEllipse) -> c_int;
    pub fn clutter_behaviour_ellipse_get_tilt(
        self_: *mut ClutterBehaviourEllipse,
        angle_tilt_x: *mut c_double,
        angle_tilt_y: *mut c_double,
        angle_tilt_z: *mut c_double,
    );
    pub fn clutter_behaviour_ellipse_get_width(self_: *mut ClutterBehaviourEllipse) -> c_int;
    pub fn clutter_behaviour_ellipse_set_angle_end(
        self_: *mut ClutterBehaviourEllipse,
        angle_end: c_double,
    );
    pub fn clutter_behaviour_ellipse_set_angle_start(
        self_: *mut ClutterBehaviourEllipse,
        angle_start: c_double,
    );
    pub fn clutter_behaviour_ellipse_set_angle_tilt(
        self_: *mut ClutterBehaviourEllipse,
        axis: ClutterRotateAxis,
        angle_tilt: c_double,
    );
    pub fn clutter_behaviour_ellipse_set_center(
        self_: *mut ClutterBehaviourEllipse,
        x: c_int,
        y: c_int,
    );
    pub fn clutter_behaviour_ellipse_set_direction(
        self_: *mut ClutterBehaviourEllipse,
        direction: ClutterRotateDirection,
    );
    pub fn clutter_behaviour_ellipse_set_height(self_: *mut ClutterBehaviourEllipse, height: c_int);
    pub fn clutter_behaviour_ellipse_set_tilt(
        self_: *mut ClutterBehaviourEllipse,
        angle_tilt_x: c_double,
        angle_tilt_y: c_double,
        angle_tilt_z: c_double,
    );
    pub fn clutter_behaviour_ellipse_set_width(self_: *mut ClutterBehaviourEllipse, width: c_int);

    //=========================================================================
    // ClutterBehaviourOpacity
    //=========================================================================
    pub fn clutter_behaviour_opacity_get_type() -> GType;
    pub fn clutter_behaviour_opacity_new(
        alpha: *mut ClutterAlpha,
        opacity_start: u8,
        opacity_end: u8,
    ) -> *mut ClutterBehaviour;
    pub fn clutter_behaviour_opacity_get_bounds(
        behaviour: *mut ClutterBehaviourOpacity,
        opacity_start: *mut u8,
        opacity_end: *mut u8,
    );
    pub fn clutter_behaviour_opacity_set_bounds(
        behaviour: *mut ClutterBehaviourOpacity,
        opacity_start: u8,
        opacity_end: u8,
    );

    //=========================================================================
    // ClutterBehaviourPath
    //=========================================================================
    pub fn clutter_behaviour_path_get_type() -> GType;
    pub fn clutter_behaviour_path_new(
        alpha: *mut ClutterAlpha,
        path: *mut ClutterPath,
    ) -> *mut ClutterBehaviour;
    pub fn clutter_behaviour_path_new_with_description(
        alpha: *mut ClutterAlpha,
        desc: *const c_char,
    ) -> *mut ClutterBehaviour;
    pub fn clutter_behaviour_path_new_with_knots(
        alpha: *mut ClutterAlpha,
        knots: *const ClutterKnot,
        n_knots: c_uint,
    ) -> *mut ClutterBehaviour;
    pub fn clutter_behaviour_path_get_path(pathb: *mut ClutterBehaviourPath) -> *mut ClutterPath;
    pub fn clutter_behaviour_path_set_path(
        pathb: *mut ClutterBehaviourPath,
        path: *mut ClutterPath,
    );

    //=========================================================================
    // ClutterBehaviourRotate
    //=========================================================================
    pub fn clutter_behaviour_rotate_get_type() -> GType;
    pub fn clutter_behaviour_rotate_new(
        alpha: *mut ClutterAlpha,
        axis: ClutterRotateAxis,
        direction: ClutterRotateDirection,
        angle_start: c_double,
        angle_end: c_double,
    ) -> *mut ClutterBehaviour;
    pub fn clutter_behaviour_rotate_get_axis(
        rotate: *mut ClutterBehaviourRotate,
    ) -> ClutterRotateAxis;
    pub fn clutter_behaviour_rotate_get_bounds(
        rotate: *mut ClutterBehaviourRotate,
        angle_start: *mut c_double,
        angle_end: *mut c_double,
    );
    pub fn clutter_behaviour_rotate_get_center(
        rotate: *mut ClutterBehaviourRotate,
        x: *mut c_int,
        y: *mut c_int,
        z: *mut c_int,
    );
    pub fn clutter_behaviour_rotate_get_direction(
        rotate: *mut ClutterBehaviourRotate,
    ) -> ClutterRotateDirection;
    pub fn clutter_behaviour_rotate_set_axis(
        rotate: *mut ClutterBehaviourRotate,
        axis: ClutterRotateAxis,
    );
    pub fn clutter_behaviour_rotate_set_bounds(
        rotate: *mut ClutterBehaviourRotate,
        angle_start: c_double,
        angle_end: c_double,
    );
    pub fn clutter_behaviour_rotate_set_center(
        rotate: *mut ClutterBehaviourRotate,
        x: c_int,
        y: c_int,
        z: c_int,
    );
    pub fn clutter_behaviour_rotate_set_direction(
        rotate: *mut ClutterBehaviourRotate,
        direction: ClutterRotateDirection,
    );

    //=========================================================================
    // ClutterBehaviourScale
    //=========================================================================
    pub fn clutter_behaviour_scale_get_type() -> GType;
    pub fn clutter_behaviour_scale_new(
        alpha: *mut ClutterAlpha,
        x_scale_start: c_double,
        y_scale_start: c_double,
        x_scale_end: c_double,
        y_scale_end: c_double,
    ) -> *mut ClutterBehaviour;
    pub fn clutter_behaviour_scale_get_bounds(
        scale: *mut ClutterBehaviourScale,
        x_scale_start: *mut c_double,
        y_scale_start: *mut c_double,
        x_scale_end: *mut c_double,
        y_scale_end: *mut c_double,
    );
    pub fn clutter_behaviour_scale_set_bounds(
        scale: *mut ClutterBehaviourScale,
        x_scale_start: c_double,
        y_scale_start: c_double,
        x_scale_end: c_double,
        y_scale_end: c_double,
    );

    //=========================================================================
    // ClutterBinLayout
    //=========================================================================
    pub fn clutter_bin_layout_get_type() -> GType;
    pub fn clutter_bin_layout_new(
        x_align: ClutterBinAlignment,
        y_align: ClutterBinAlignment,
    ) -> *mut ClutterLayoutManager;
    pub fn clutter_bin_layout_add(
        self_: *mut ClutterBinLayout,
        child: *mut ClutterActor,
        x_align: ClutterBinAlignment,
        y_align: ClutterBinAlignment,
    );
    pub fn clutter_bin_layout_get_alignment(
        self_: *mut ClutterBinLayout,
        child: *mut ClutterActor,
        x_align: *mut ClutterBinAlignment,
        y_align: *mut ClutterBinAlignment,
    );
    pub fn clutter_bin_layout_set_alignment(
        self_: *mut ClutterBinLayout,
        child: *mut ClutterActor,
        x_align: ClutterBinAlignment,
        y_align: ClutterBinAlignment,
    );

    //=========================================================================
    // ClutterBindConstraint
    //=========================================================================
    pub fn clutter_bind_constraint_get_type() -> GType;
    pub fn clutter_bind_constraint_new(
        source: *mut ClutterActor,
        coordinate: ClutterBindCoordinate,
        offset: c_float,
    ) -> *mut ClutterConstraint;
    pub fn clutter_bind_constraint_get_coordinate(
        constraint: *mut ClutterBindConstraint,
    ) -> ClutterBindCoordinate;
    pub fn clutter_bind_constraint_get_offset(constraint: *mut ClutterBindConstraint) -> c_float;
    pub fn clutter_bind_constraint_get_source(
        constraint: *mut ClutterBindConstraint,
    ) -> *mut ClutterActor;
    pub fn clutter_bind_constraint_set_coordinate(
        constraint: *mut ClutterBindConstraint,
        coordinate: ClutterBindCoordinate,
    );
    pub fn clutter_bind_constraint_set_offset(
        constraint: *mut ClutterBindConstraint,
        offset: c_float,
    );
    pub fn clutter_bind_constraint_set_source(
        constraint: *mut ClutterBindConstraint,
        source: *mut ClutterActor,
    );

    //=========================================================================
    // ClutterBindingPool
    //=========================================================================
    pub fn clutter_binding_pool_get_type() -> GType;
    pub fn clutter_binding_pool_new(name: *const c_char) -> *mut ClutterBindingPool;
    pub fn clutter_binding_pool_find(name: *const c_char) -> *mut ClutterBindingPool;
    pub fn clutter_binding_pool_get_for_class(klass: gpointer) -> *mut ClutterBindingPool;
    pub fn clutter_binding_pool_activate(
        pool: *mut ClutterBindingPool,
        key_val: c_uint,
        modifiers: ClutterModifierType,
        gobject: *mut gobject::GObject,
    ) -> gboolean;
    pub fn clutter_binding_pool_block_action(
        pool: *mut ClutterBindingPool,
        action_name: *const c_char,
    );
    pub fn clutter_binding_pool_find_action(
        pool: *mut ClutterBindingPool,
        key_val: c_uint,
        modifiers: ClutterModifierType,
    ) -> *const c_char;
    //pub fn clutter_binding_pool_install_action(pool: *mut ClutterBindingPool, action_name: *const c_char, key_val: c_uint, modifiers: ClutterModifierType, callback: /*Metadata mismatch*/[c:type mismatch `GCallback` != `ClutterBindingActionFunc` of `BindingActionFunc`], data: gpointer, notify: glib::GDestroyNotify);
    pub fn clutter_binding_pool_install_closure(
        pool: *mut ClutterBindingPool,
        action_name: *const c_char,
        key_val: c_uint,
        modifiers: ClutterModifierType,
        closure: *mut gobject::GClosure,
    );
    //pub fn clutter_binding_pool_override_action(pool: *mut ClutterBindingPool, key_val: c_uint, modifiers: ClutterModifierType, callback: /*Metadata mismatch*/[c:type mismatch `GCallback` != `ClutterBindingActionFunc` of `BindingActionFunc`], data: gpointer, notify: glib::GDestroyNotify);
    pub fn clutter_binding_pool_override_closure(
        pool: *mut ClutterBindingPool,
        key_val: c_uint,
        modifiers: ClutterModifierType,
        closure: *mut gobject::GClosure,
    );
    pub fn clutter_binding_pool_remove_action(
        pool: *mut ClutterBindingPool,
        key_val: c_uint,
        modifiers: ClutterModifierType,
    );
    pub fn clutter_binding_pool_unblock_action(
        pool: *mut ClutterBindingPool,
        action_name: *const c_char,
    );

    //=========================================================================
    // ClutterBlurEffect
    //=========================================================================
    pub fn clutter_blur_effect_get_type() -> GType;
    pub fn clutter_blur_effect_new() -> *mut ClutterEffect;

    //=========================================================================
    // ClutterBox
    //=========================================================================
    pub fn clutter_box_get_type() -> GType;
    pub fn clutter_box_new(manager: *mut ClutterLayoutManager) -> *mut ClutterActor;
    pub fn clutter_box_get_color(box_: *mut ClutterBox, color: *mut ClutterColor);
    pub fn clutter_box_get_layout_manager(box_: *mut ClutterBox) -> *mut ClutterLayoutManager;
    pub fn clutter_box_pack(
        box_: *mut ClutterBox,
        actor: *mut ClutterActor,
        first_property: *const c_char,
        ...
    );
    pub fn clutter_box_pack_after(
        box_: *mut ClutterBox,
        actor: *mut ClutterActor,
        sibling: *mut ClutterActor,
        first_property: *const c_char,
        ...
    );
    pub fn clutter_box_pack_at(
        box_: *mut ClutterBox,
        actor: *mut ClutterActor,
        position: c_int,
        first_property: *const c_char,
        ...
    );
    pub fn clutter_box_pack_before(
        box_: *mut ClutterBox,
        actor: *mut ClutterActor,
        sibling: *mut ClutterActor,
        first_property: *const c_char,
        ...
    );
    pub fn clutter_box_packv(
        box_: *mut ClutterBox,
        actor: *mut ClutterActor,
        n_properties: c_uint,
        properties: *const *const c_char,
        values: *const gobject::GValue,
    );
    pub fn clutter_box_set_color(box_: *mut ClutterBox, color: *const ClutterColor);
    pub fn clutter_box_set_layout_manager(
        box_: *mut ClutterBox,
        manager: *mut ClutterLayoutManager,
    );

    //=========================================================================
    // ClutterBoxLayout
    //=========================================================================
    pub fn clutter_box_layout_get_type() -> GType;
    pub fn clutter_box_layout_new() -> *mut ClutterLayoutManager;
    pub fn clutter_box_layout_get_alignment(
        layout: *mut ClutterBoxLayout,
        actor: *mut ClutterActor,
        x_align: *mut ClutterBoxAlignment,
        y_align: *mut ClutterBoxAlignment,
    );
    pub fn clutter_box_layout_get_easing_duration(layout: *mut ClutterBoxLayout) -> c_uint;
    pub fn clutter_box_layout_get_easing_mode(layout: *mut ClutterBoxLayout) -> c_ulong;
    pub fn clutter_box_layout_get_expand(
        layout: *mut ClutterBoxLayout,
        actor: *mut ClutterActor,
    ) -> gboolean;
    pub fn clutter_box_layout_get_fill(
        layout: *mut ClutterBoxLayout,
        actor: *mut ClutterActor,
        x_fill: *mut gboolean,
        y_fill: *mut gboolean,
    );
    pub fn clutter_box_layout_get_homogeneous(layout: *mut ClutterBoxLayout) -> gboolean;
    pub fn clutter_box_layout_get_orientation(layout: *mut ClutterBoxLayout) -> ClutterOrientation;
    pub fn clutter_box_layout_get_pack_start(layout: *mut ClutterBoxLayout) -> gboolean;
    pub fn clutter_box_layout_get_spacing(layout: *mut ClutterBoxLayout) -> c_uint;
    pub fn clutter_box_layout_get_use_animations(layout: *mut ClutterBoxLayout) -> gboolean;
    pub fn clutter_box_layout_get_vertical(layout: *mut ClutterBoxLayout) -> gboolean;
    pub fn clutter_box_layout_pack(
        layout: *mut ClutterBoxLayout,
        actor: *mut ClutterActor,
        expand: gboolean,
        x_fill: gboolean,
        y_fill: gboolean,
        x_align: ClutterBoxAlignment,
        y_align: ClutterBoxAlignment,
    );
    pub fn clutter_box_layout_set_alignment(
        layout: *mut ClutterBoxLayout,
        actor: *mut ClutterActor,
        x_align: ClutterBoxAlignment,
        y_align: ClutterBoxAlignment,
    );
    pub fn clutter_box_layout_set_easing_duration(layout: *mut ClutterBoxLayout, msecs: c_uint);
    pub fn clutter_box_layout_set_easing_mode(layout: *mut ClutterBoxLayout, mode: c_ulong);
    pub fn clutter_box_layout_set_expand(
        layout: *mut ClutterBoxLayout,
        actor: *mut ClutterActor,
        expand: gboolean,
    );
    pub fn clutter_box_layout_set_fill(
        layout: *mut ClutterBoxLayout,
        actor: *mut ClutterActor,
        x_fill: gboolean,
        y_fill: gboolean,
    );
    pub fn clutter_box_layout_set_homogeneous(layout: *mut ClutterBoxLayout, homogeneous: gboolean);
    pub fn clutter_box_layout_set_orientation(
        layout: *mut ClutterBoxLayout,
        orientation: ClutterOrientation,
    );
    pub fn clutter_box_layout_set_pack_start(layout: *mut ClutterBoxLayout, pack_start: gboolean);
    pub fn clutter_box_layout_set_spacing(layout: *mut ClutterBoxLayout, spacing: c_uint);
    pub fn clutter_box_layout_set_use_animations(layout: *mut ClutterBoxLayout, animate: gboolean);
    pub fn clutter_box_layout_set_vertical(layout: *mut ClutterBoxLayout, vertical: gboolean);

    //=========================================================================
    // ClutterBrightnessContrastEffect
    //=========================================================================
    pub fn clutter_brightness_contrast_effect_get_type() -> GType;
    pub fn clutter_brightness_contrast_effect_new() -> *mut ClutterEffect;
    pub fn clutter_brightness_contrast_effect_get_brightness(
        effect: *mut ClutterBrightnessContrastEffect,
        red: *mut c_float,
        green: *mut c_float,
        blue: *mut c_float,
    );
    pub fn clutter_brightness_contrast_effect_get_contrast(
        effect: *mut ClutterBrightnessContrastEffect,
        red: *mut c_float,
        green: *mut c_float,
        blue: *mut c_float,
    );
    pub fn clutter_brightness_contrast_effect_set_brightness(
        effect: *mut ClutterBrightnessContrastEffect,
        brightness: c_float,
    );
    pub fn clutter_brightness_contrast_effect_set_brightness_full(
        effect: *mut ClutterBrightnessContrastEffect,
        red: c_float,
        green: c_float,
        blue: c_float,
    );
    pub fn clutter_brightness_contrast_effect_set_contrast(
        effect: *mut ClutterBrightnessContrastEffect,
        contrast: c_float,
    );
    pub fn clutter_brightness_contrast_effect_set_contrast_full(
        effect: *mut ClutterBrightnessContrastEffect,
        red: c_float,
        green: c_float,
        blue: c_float,
    );

    //=========================================================================
    // ClutterCairoTexture
    //=========================================================================
    pub fn clutter_cairo_texture_get_type() -> GType;
    pub fn clutter_cairo_texture_new(width: c_uint, height: c_uint) -> *mut ClutterActor;
    pub fn clutter_cairo_texture_clear(self_: *mut ClutterCairoTexture);
    pub fn clutter_cairo_texture_create(self_: *mut ClutterCairoTexture) -> *mut cairo::cairo_t;
    pub fn clutter_cairo_texture_create_region(
        self_: *mut ClutterCairoTexture,
        x_offset: c_int,
        y_offset: c_int,
        width: c_int,
        height: c_int,
    ) -> *mut cairo::cairo_t;
    pub fn clutter_cairo_texture_get_auto_resize(self_: *mut ClutterCairoTexture) -> gboolean;
    pub fn clutter_cairo_texture_get_surface_size(
        self_: *mut ClutterCairoTexture,
        width: *mut c_uint,
        height: *mut c_uint,
    );
    pub fn clutter_cairo_texture_invalidate(self_: *mut ClutterCairoTexture);
    pub fn clutter_cairo_texture_invalidate_rectangle(
        self_: *mut ClutterCairoTexture,
        rect: *mut cairo::cairo_rectangle_int_t,
    );
    pub fn clutter_cairo_texture_set_auto_resize(self_: *mut ClutterCairoTexture, value: gboolean);
    pub fn clutter_cairo_texture_set_surface_size(
        self_: *mut ClutterCairoTexture,
        width: c_uint,
        height: c_uint,
    );

    //=========================================================================
    // ClutterCanvas
    //=========================================================================
    pub fn clutter_canvas_get_type() -> GType;
    pub fn clutter_canvas_new() -> *mut ClutterContent;
    pub fn clutter_canvas_get_scale_factor(canvas: *mut ClutterCanvas) -> c_int;
    pub fn clutter_canvas_set_scale_factor(canvas: *mut ClutterCanvas, scale: c_int);
    pub fn clutter_canvas_set_size(
        canvas: *mut ClutterCanvas,
        width: c_int,
        height: c_int,
    ) -> gboolean;

    //=========================================================================
    // ClutterChildMeta
    //=========================================================================
    pub fn clutter_child_meta_get_type() -> GType;
    pub fn clutter_child_meta_get_actor(data: *mut ClutterChildMeta) -> *mut ClutterActor;
    pub fn clutter_child_meta_get_container(data: *mut ClutterChildMeta) -> *mut ClutterContainer;

    //=========================================================================
    // ClutterClickAction
    //=========================================================================
    pub fn clutter_click_action_get_type() -> GType;
    pub fn clutter_click_action_new() -> *mut ClutterAction;
    pub fn clutter_click_action_get_button(action: *mut ClutterClickAction) -> c_uint;
    pub fn clutter_click_action_get_coords(
        action: *mut ClutterClickAction,
        press_x: *mut c_float,
        press_y: *mut c_float,
    );
    pub fn clutter_click_action_get_state(action: *mut ClutterClickAction) -> ClutterModifierType;
    pub fn clutter_click_action_release(action: *mut ClutterClickAction);

    //=========================================================================
    // ClutterClipNode
    //=========================================================================
    pub fn clutter_clip_node_get_type() -> GType;
    pub fn clutter_clip_node_new() -> *mut ClutterPaintNode;

    //=========================================================================
    // ClutterClone
    //=========================================================================
    pub fn clutter_clone_get_type() -> GType;
    pub fn clutter_clone_new(source: *mut ClutterActor) -> *mut ClutterActor;
    pub fn clutter_clone_get_source(self_: *mut ClutterClone) -> *mut ClutterActor;
    pub fn clutter_clone_set_source(self_: *mut ClutterClone, source: *mut ClutterActor);

    //=========================================================================
    // ClutterColorNode
    //=========================================================================
    pub fn clutter_color_node_get_type() -> GType;
    pub fn clutter_color_node_new(color: *const ClutterColor) -> *mut ClutterPaintNode;

    //=========================================================================
    // ClutterColorizeEffect
    //=========================================================================
    pub fn clutter_colorize_effect_get_type() -> GType;
    pub fn clutter_colorize_effect_new(tint: *const ClutterColor) -> *mut ClutterEffect;
    pub fn clutter_colorize_effect_get_tint(
        effect: *mut ClutterColorizeEffect,
        tint: *mut ClutterColor,
    );
    pub fn clutter_colorize_effect_set_tint(
        effect: *mut ClutterColorizeEffect,
        tint: *const ClutterColor,
    );

    //=========================================================================
    // ClutterConstraint
    //=========================================================================
    pub fn clutter_constraint_get_type() -> GType;

    //=========================================================================
    // ClutterDeformEffect
    //=========================================================================
    pub fn clutter_deform_effect_get_type() -> GType;
    pub fn clutter_deform_effect_get_back_material(
        effect: *mut ClutterDeformEffect,
    ) -> cogl::CoglHandle;
    pub fn clutter_deform_effect_get_n_tiles(
        effect: *mut ClutterDeformEffect,
        x_tiles: *mut c_uint,
        y_tiles: *mut c_uint,
    );
    pub fn clutter_deform_effect_invalidate(effect: *mut ClutterDeformEffect);
    pub fn clutter_deform_effect_set_back_material(
        effect: *mut ClutterDeformEffect,
        material: cogl::CoglHandle,
    );
    pub fn clutter_deform_effect_set_n_tiles(
        effect: *mut ClutterDeformEffect,
        x_tiles: c_uint,
        y_tiles: c_uint,
    );

    //=========================================================================
    // ClutterDesaturateEffect
    //=========================================================================
    pub fn clutter_desaturate_effect_get_type() -> GType;
    pub fn clutter_desaturate_effect_new(factor: c_double) -> *mut ClutterEffect;
    pub fn clutter_desaturate_effect_get_factor(effect: *mut ClutterDesaturateEffect) -> c_double;
    pub fn clutter_desaturate_effect_set_factor(
        effect: *mut ClutterDesaturateEffect,
        factor: c_double,
    );

    //=========================================================================
    // ClutterDeviceManager
    //=========================================================================
    pub fn clutter_device_manager_get_type() -> GType;
    pub fn clutter_device_manager_get_default() -> *mut ClutterDeviceManager;
    pub fn clutter_device_manager_get_core_device(
        device_manager: *mut ClutterDeviceManager,
        device_type: ClutterInputDeviceType,
    ) -> *mut ClutterInputDevice;
    pub fn clutter_device_manager_get_device(
        device_manager: *mut ClutterDeviceManager,
        device_id: c_int,
    ) -> *mut ClutterInputDevice;
    pub fn clutter_device_manager_list_devices(
        device_manager: *mut ClutterDeviceManager,
    ) -> *mut glib::GSList;
    pub fn clutter_device_manager_peek_devices(
        device_manager: *mut ClutterDeviceManager,
    ) -> *const glib::GSList;

    //=========================================================================
    // ClutterDragAction
    //=========================================================================
    pub fn clutter_drag_action_get_type() -> GType;
    pub fn clutter_drag_action_new() -> *mut ClutterAction;
    pub fn clutter_drag_action_get_drag_area(
        action: *mut ClutterDragAction,
        drag_area: *mut ClutterRect,
    ) -> gboolean;
    pub fn clutter_drag_action_get_drag_axis(action: *mut ClutterDragAction) -> ClutterDragAxis;
    pub fn clutter_drag_action_get_drag_handle(action: *mut ClutterDragAction)
        -> *mut ClutterActor;
    pub fn clutter_drag_action_get_drag_threshold(
        action: *mut ClutterDragAction,
        x_threshold: *mut c_uint,
        y_threshold: *mut c_uint,
    );
    pub fn clutter_drag_action_get_motion_coords(
        action: *mut ClutterDragAction,
        motion_x: *mut c_float,
        motion_y: *mut c_float,
    );
    pub fn clutter_drag_action_get_press_coords(
        action: *mut ClutterDragAction,
        press_x: *mut c_float,
        press_y: *mut c_float,
    );
    pub fn clutter_drag_action_set_drag_area(
        action: *mut ClutterDragAction,
        drag_area: *const ClutterRect,
    );
    pub fn clutter_drag_action_set_drag_axis(action: *mut ClutterDragAction, axis: ClutterDragAxis);
    pub fn clutter_drag_action_set_drag_handle(
        action: *mut ClutterDragAction,
        handle: *mut ClutterActor,
    );
    pub fn clutter_drag_action_set_drag_threshold(
        action: *mut ClutterDragAction,
        x_threshold: c_int,
        y_threshold: c_int,
    );

    //=========================================================================
    // ClutterDropAction
    //=========================================================================
    pub fn clutter_drop_action_get_type() -> GType;
    pub fn clutter_drop_action_new() -> *mut ClutterAction;

    //=========================================================================
    // ClutterEffect
    //=========================================================================
    pub fn clutter_effect_get_type() -> GType;
    pub fn clutter_effect_queue_repaint(effect: *mut ClutterEffect);

    //=========================================================================
    // ClutterFixedLayout
    //=========================================================================
    pub fn clutter_fixed_layout_get_type() -> GType;
    pub fn clutter_fixed_layout_new() -> *mut ClutterLayoutManager;

    //=========================================================================
    // ClutterFlowLayout
    //=========================================================================
    pub fn clutter_flow_layout_get_type() -> GType;
    pub fn clutter_flow_layout_new(
        orientation: ClutterFlowOrientation,
    ) -> *mut ClutterLayoutManager;
    pub fn clutter_flow_layout_get_column_spacing(layout: *mut ClutterFlowLayout) -> c_float;
    pub fn clutter_flow_layout_get_column_width(
        layout: *mut ClutterFlowLayout,
        min_width: *mut c_float,
        max_width: *mut c_float,
    );
    pub fn clutter_flow_layout_get_homogeneous(layout: *mut ClutterFlowLayout) -> gboolean;
    pub fn clutter_flow_layout_get_orientation(
        layout: *mut ClutterFlowLayout,
    ) -> ClutterFlowOrientation;
    pub fn clutter_flow_layout_get_row_height(
        layout: *mut ClutterFlowLayout,
        min_height: *mut c_float,
        max_height: *mut c_float,
    );
    pub fn clutter_flow_layout_get_row_spacing(layout: *mut ClutterFlowLayout) -> c_float;
    pub fn clutter_flow_layout_get_snap_to_grid(layout: *mut ClutterFlowLayout) -> gboolean;
    pub fn clutter_flow_layout_set_column_spacing(layout: *mut ClutterFlowLayout, spacing: c_float);
    pub fn clutter_flow_layout_set_column_width(
        layout: *mut ClutterFlowLayout,
        min_width: c_float,
        max_width: c_float,
    );
    pub fn clutter_flow_layout_set_homogeneous(
        layout: *mut ClutterFlowLayout,
        homogeneous: gboolean,
    );
    pub fn clutter_flow_layout_set_orientation(
        layout: *mut ClutterFlowLayout,
        orientation: ClutterFlowOrientation,
    );
    pub fn clutter_flow_layout_set_row_height(
        layout: *mut ClutterFlowLayout,
        min_height: c_float,
        max_height: c_float,
    );
    pub fn clutter_flow_layout_set_row_spacing(layout: *mut ClutterFlowLayout, spacing: c_float);
    pub fn clutter_flow_layout_set_snap_to_grid(
        layout: *mut ClutterFlowLayout,
        snap_to_grid: gboolean,
    );

    //=========================================================================
    // ClutterGestureAction
    //=========================================================================
    pub fn clutter_gesture_action_get_type() -> GType;
    pub fn clutter_gesture_action_new() -> *mut ClutterAction;
    pub fn clutter_gesture_action_cancel(action: *mut ClutterGestureAction);
    pub fn clutter_gesture_action_get_device(
        action: *mut ClutterGestureAction,
        point: c_uint,
    ) -> *mut ClutterInputDevice;
    pub fn clutter_gesture_action_get_last_event(
        action: *mut ClutterGestureAction,
        point: c_uint,
    ) -> *const ClutterEvent;
    pub fn clutter_gesture_action_get_motion_coords(
        action: *mut ClutterGestureAction,
        point: c_uint,
        motion_x: *mut c_float,
        motion_y: *mut c_float,
    );
    pub fn clutter_gesture_action_get_motion_delta(
        action: *mut ClutterGestureAction,
        point: c_uint,
        delta_x: *mut c_float,
        delta_y: *mut c_float,
    ) -> c_float;
    pub fn clutter_gesture_action_get_n_current_points(action: *mut ClutterGestureAction)
        -> c_uint;
    pub fn clutter_gesture_action_get_n_touch_points(action: *mut ClutterGestureAction) -> c_int;
    pub fn clutter_gesture_action_get_press_coords(
        action: *mut ClutterGestureAction,
        point: c_uint,
        press_x: *mut c_float,
        press_y: *mut c_float,
    );
    pub fn clutter_gesture_action_get_release_coords(
        action: *mut ClutterGestureAction,
        point: c_uint,
        release_x: *mut c_float,
        release_y: *mut c_float,
    );
    pub fn clutter_gesture_action_get_sequence(
        action: *mut ClutterGestureAction,
        point: c_uint,
    ) -> *mut ClutterEventSequence;
    pub fn clutter_gesture_action_get_threshold_trigger_distance(
        action: *mut ClutterGestureAction,
        x: *mut c_float,
        y: *mut c_float,
    );
    pub fn clutter_gesture_action_get_threshold_trigger_edge(
        action: *mut ClutterGestureAction,
    ) -> ClutterGestureTriggerEdge;
    pub fn clutter_gesture_action_get_threshold_trigger_egde(
        action: *mut ClutterGestureAction,
    ) -> ClutterGestureTriggerEdge;
    pub fn clutter_gesture_action_get_velocity(
        action: *mut ClutterGestureAction,
        point: c_uint,
        velocity_x: *mut c_float,
        velocity_y: *mut c_float,
    ) -> c_float;
    pub fn clutter_gesture_action_set_n_touch_points(
        action: *mut ClutterGestureAction,
        nb_points: c_int,
    );
    pub fn clutter_gesture_action_set_threshold_trigger_distance(
        action: *mut ClutterGestureAction,
        x: c_float,
        y: c_float,
    );
    pub fn clutter_gesture_action_set_threshold_trigger_edge(
        action: *mut ClutterGestureAction,
        edge: ClutterGestureTriggerEdge,
    );

    //=========================================================================
    // ClutterGridLayout
    //=========================================================================
    pub fn clutter_grid_layout_get_type() -> GType;
    pub fn clutter_grid_layout_new() -> *mut ClutterLayoutManager;
    pub fn clutter_grid_layout_attach(
        layout: *mut ClutterGridLayout,
        child: *mut ClutterActor,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn clutter_grid_layout_attach_next_to(
        layout: *mut ClutterGridLayout,
        child: *mut ClutterActor,
        sibling: *mut ClutterActor,
        side: ClutterGridPosition,
        width: c_int,
        height: c_int,
    );
    pub fn clutter_grid_layout_get_child_at(
        layout: *mut ClutterGridLayout,
        left: c_int,
        top: c_int,
    ) -> *mut ClutterActor;
    pub fn clutter_grid_layout_get_column_homogeneous(layout: *mut ClutterGridLayout) -> gboolean;
    pub fn clutter_grid_layout_get_column_spacing(layout: *mut ClutterGridLayout) -> c_uint;
    pub fn clutter_grid_layout_get_orientation(
        layout: *mut ClutterGridLayout,
    ) -> ClutterOrientation;
    pub fn clutter_grid_layout_get_row_homogeneous(layout: *mut ClutterGridLayout) -> gboolean;
    pub fn clutter_grid_layout_get_row_spacing(layout: *mut ClutterGridLayout) -> c_uint;
    pub fn clutter_grid_layout_insert_column(layout: *mut ClutterGridLayout, position: c_int);
    pub fn clutter_grid_layout_insert_next_to(
        layout: *mut ClutterGridLayout,
        sibling: *mut ClutterActor,
        side: ClutterGridPosition,
    );
    pub fn clutter_grid_layout_insert_row(layout: *mut ClutterGridLayout, position: c_int);
    pub fn clutter_grid_layout_set_column_homogeneous(
        layout: *mut ClutterGridLayout,
        homogeneous: gboolean,
    );
    pub fn clutter_grid_layout_set_column_spacing(layout: *mut ClutterGridLayout, spacing: c_uint);
    pub fn clutter_grid_layout_set_orientation(
        layout: *mut ClutterGridLayout,
        orientation: ClutterOrientation,
    );
    pub fn clutter_grid_layout_set_row_homogeneous(
        layout: *mut ClutterGridLayout,
        homogeneous: gboolean,
    );
    pub fn clutter_grid_layout_set_row_spacing(layout: *mut ClutterGridLayout, spacing: c_uint);

    //=========================================================================
    // ClutterGroup
    //=========================================================================
    pub fn clutter_group_get_type() -> GType;
    pub fn clutter_group_new() -> *mut ClutterActor;
    pub fn clutter_group_get_n_children(self_: *mut ClutterGroup) -> c_int;
    pub fn clutter_group_get_nth_child(
        self_: *mut ClutterGroup,
        index_: c_int,
    ) -> *mut ClutterActor;
    pub fn clutter_group_remove_all(self_: *mut ClutterGroup);

    //=========================================================================
    // ClutterImage
    //=========================================================================
    pub fn clutter_image_get_type() -> GType;
    pub fn clutter_image_new() -> *mut ClutterContent;
    pub fn clutter_image_set_area(
        image: *mut ClutterImage,
        data: *const u8,
        pixel_format: cogl::CoglPixelFormat,
        rect: *const cairo::cairo_rectangle_int_t,
        row_stride: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn clutter_image_set_bytes(
        image: *mut ClutterImage,
        data: *mut glib::GBytes,
        pixel_format: cogl::CoglPixelFormat,
        width: c_uint,
        height: c_uint,
        row_stride: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn clutter_image_set_data(
        image: *mut ClutterImage,
        data: *const u8,
        pixel_format: cogl::CoglPixelFormat,
        width: c_uint,
        height: c_uint,
        row_stride: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // ClutterInputDevice
    //=========================================================================
    pub fn clutter_input_device_get_type() -> GType;
    pub fn clutter_input_device_get_associated_device(
        device: *mut ClutterInputDevice,
    ) -> *mut ClutterInputDevice;
    pub fn clutter_input_device_get_axis(
        device: *mut ClutterInputDevice,
        index_: c_uint,
    ) -> ClutterInputAxis;
    pub fn clutter_input_device_get_axis_value(
        device: *mut ClutterInputDevice,
        axes: *mut c_double,
        axis: ClutterInputAxis,
        value: *mut c_double,
    ) -> gboolean;
    pub fn clutter_input_device_get_coords(
        device: *mut ClutterInputDevice,
        sequence: *mut ClutterEventSequence,
        point: *mut ClutterPoint,
    ) -> gboolean;
    pub fn clutter_input_device_get_device_coords(
        device: *mut ClutterInputDevice,
        x: *mut c_int,
        y: *mut c_int,
    );
    pub fn clutter_input_device_get_device_id(device: *mut ClutterInputDevice) -> c_int;
    pub fn clutter_input_device_get_device_mode(
        device: *mut ClutterInputDevice,
    ) -> ClutterInputMode;
    pub fn clutter_input_device_get_device_name(device: *mut ClutterInputDevice) -> *const c_char;
    pub fn clutter_input_device_get_device_type(
        device: *mut ClutterInputDevice,
    ) -> ClutterInputDeviceType;
    pub fn clutter_input_device_get_enabled(device: *mut ClutterInputDevice) -> gboolean;
    pub fn clutter_input_device_get_grabbed_actor(
        device: *mut ClutterInputDevice,
    ) -> *mut ClutterActor;
    pub fn clutter_input_device_get_has_cursor(device: *mut ClutterInputDevice) -> gboolean;
    pub fn clutter_input_device_get_key(
        device: *mut ClutterInputDevice,
        index_: c_uint,
        keyval: *mut c_uint,
        modifiers: *mut ClutterModifierType,
    ) -> gboolean;
    pub fn clutter_input_device_get_modifier_state(
        device: *mut ClutterInputDevice,
    ) -> ClutterModifierType;
    pub fn clutter_input_device_get_n_axes(device: *mut ClutterInputDevice) -> c_uint;
    pub fn clutter_input_device_get_n_keys(device: *mut ClutterInputDevice) -> c_uint;
    pub fn clutter_input_device_get_pointer_actor(
        device: *mut ClutterInputDevice,
    ) -> *mut ClutterActor;
    pub fn clutter_input_device_get_pointer_stage(
        device: *mut ClutterInputDevice,
    ) -> *mut ClutterStage;
    pub fn clutter_input_device_get_product_id(device: *mut ClutterInputDevice) -> *const c_char;
    pub fn clutter_input_device_get_slave_devices(
        device: *mut ClutterInputDevice,
    ) -> *mut glib::GList;
    pub fn clutter_input_device_get_vendor_id(device: *mut ClutterInputDevice) -> *const c_char;
    pub fn clutter_input_device_grab(device: *mut ClutterInputDevice, actor: *mut ClutterActor);
    pub fn clutter_input_device_keycode_to_evdev(
        device: *mut ClutterInputDevice,
        hardware_keycode: c_uint,
        evdev_keycode: *mut c_uint,
    ) -> gboolean;
    pub fn clutter_input_device_sequence_get_grabbed_actor(
        device: *mut ClutterInputDevice,
        sequence: *mut ClutterEventSequence,
    ) -> *mut ClutterActor;
    pub fn clutter_input_device_sequence_grab(
        device: *mut ClutterInputDevice,
        sequence: *mut ClutterEventSequence,
        actor: *mut ClutterActor,
    );
    pub fn clutter_input_device_sequence_ungrab(
        device: *mut ClutterInputDevice,
        sequence: *mut ClutterEventSequence,
    );
    pub fn clutter_input_device_set_enabled(device: *mut ClutterInputDevice, enabled: gboolean);
    pub fn clutter_input_device_set_key(
        device: *mut ClutterInputDevice,
        index_: c_uint,
        keyval: c_uint,
        modifiers: ClutterModifierType,
    );
    pub fn clutter_input_device_ungrab(device: *mut ClutterInputDevice);
    pub fn clutter_input_device_update_from_event(
        device: *mut ClutterInputDevice,
        event: *mut ClutterEvent,
        update_stage: gboolean,
    );

    //=========================================================================
    // ClutterInterval
    //=========================================================================
    pub fn clutter_interval_get_type() -> GType;
    pub fn clutter_interval_new(gtype: GType, ...) -> *mut ClutterInterval;
    pub fn clutter_interval_new_with_values(
        gtype: GType,
        initial: *const gobject::GValue,
        final_: *const gobject::GValue,
    ) -> *mut ClutterInterval;
    pub fn clutter_interval_register_progress_func(value_type: GType, func: ClutterProgressFunc);
    pub fn clutter_interval_clone(interval: *mut ClutterInterval) -> *mut ClutterInterval;
    pub fn clutter_interval_compute(
        interval: *mut ClutterInterval,
        factor: c_double,
    ) -> *const gobject::GValue;
    pub fn clutter_interval_compute_value(
        interval: *mut ClutterInterval,
        factor: c_double,
        value: *mut gobject::GValue,
    ) -> gboolean;
    pub fn clutter_interval_get_final_value(
        interval: *mut ClutterInterval,
        value: *mut gobject::GValue,
    );
    pub fn clutter_interval_get_initial_value(
        interval: *mut ClutterInterval,
        value: *mut gobject::GValue,
    );
    pub fn clutter_interval_get_interval(interval: *mut ClutterInterval, ...);
    pub fn clutter_interval_get_value_type(interval: *mut ClutterInterval) -> GType;
    pub fn clutter_interval_is_valid(interval: *mut ClutterInterval) -> gboolean;
    pub fn clutter_interval_peek_final_value(
        interval: *mut ClutterInterval,
    ) -> *mut gobject::GValue;
    pub fn clutter_interval_peek_initial_value(
        interval: *mut ClutterInterval,
    ) -> *mut gobject::GValue;
    pub fn clutter_interval_set_final(interval: *mut ClutterInterval, ...);
    pub fn clutter_interval_set_final_value(
        interval: *mut ClutterInterval,
        value: *const gobject::GValue,
    );
    pub fn clutter_interval_set_initial(interval: *mut ClutterInterval, ...);
    pub fn clutter_interval_set_initial_value(
        interval: *mut ClutterInterval,
        value: *const gobject::GValue,
    );
    pub fn clutter_interval_set_interval(interval: *mut ClutterInterval, ...);
    pub fn clutter_interval_validate(
        interval: *mut ClutterInterval,
        pspec: *mut gobject::GParamSpec,
    ) -> gboolean;

    //=========================================================================
    // ClutterKeyframeTransition
    //=========================================================================
    pub fn clutter_keyframe_transition_get_type() -> GType;
    pub fn clutter_keyframe_transition_new(property_name: *const c_char) -> *mut ClutterTransition;
    pub fn clutter_keyframe_transition_clear(transition: *mut ClutterKeyframeTransition);
    pub fn clutter_keyframe_transition_get_key_frame(
        transition: *mut ClutterKeyframeTransition,
        index_: c_uint,
        key: *mut c_double,
        mode: *mut ClutterAnimationMode,
        value: *mut gobject::GValue,
    );
    pub fn clutter_keyframe_transition_get_n_key_frames(
        transition: *mut ClutterKeyframeTransition,
    ) -> c_uint;
    pub fn clutter_keyframe_transition_set(
        transition: *mut ClutterKeyframeTransition,
        gtype: GType,
        n_key_frames: c_uint,
        ...
    );
    pub fn clutter_keyframe_transition_set_key_frame(
        transition: *mut ClutterKeyframeTransition,
        index_: c_uint,
        key: c_double,
        mode: ClutterAnimationMode,
        value: *const gobject::GValue,
    );
    pub fn clutter_keyframe_transition_set_key_frames(
        transition: *mut ClutterKeyframeTransition,
        n_key_frames: c_uint,
        key_frames: *const c_double,
    );
    pub fn clutter_keyframe_transition_set_modes(
        transition: *mut ClutterKeyframeTransition,
        n_modes: c_uint,
        modes: *const ClutterAnimationMode,
    );
    pub fn clutter_keyframe_transition_set_values(
        transition: *mut ClutterKeyframeTransition,
        n_values: c_uint,
        values: *const gobject::GValue,
    );

    //=========================================================================
    // ClutterLayoutManager
    //=========================================================================
    pub fn clutter_layout_manager_get_type() -> GType;
    pub fn clutter_layout_manager_allocate(
        manager: *mut ClutterLayoutManager,
        container: *mut ClutterContainer,
        allocation: *const ClutterActorBox,
        flags: ClutterAllocationFlags,
    );
    pub fn clutter_layout_manager_begin_animation(
        manager: *mut ClutterLayoutManager,
        duration: c_uint,
        mode: c_ulong,
    ) -> *mut ClutterAlpha;
    pub fn clutter_layout_manager_child_get(
        manager: *mut ClutterLayoutManager,
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
        first_property: *const c_char,
        ...
    );
    pub fn clutter_layout_manager_child_get_property(
        manager: *mut ClutterLayoutManager,
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
        property_name: *const c_char,
        value: *mut gobject::GValue,
    );
    pub fn clutter_layout_manager_child_set(
        manager: *mut ClutterLayoutManager,
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
        first_property: *const c_char,
        ...
    );
    pub fn clutter_layout_manager_child_set_property(
        manager: *mut ClutterLayoutManager,
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
        property_name: *const c_char,
        value: *const gobject::GValue,
    );
    pub fn clutter_layout_manager_end_animation(manager: *mut ClutterLayoutManager);
    pub fn clutter_layout_manager_find_child_property(
        manager: *mut ClutterLayoutManager,
        name: *const c_char,
    ) -> *mut gobject::GParamSpec;
    pub fn clutter_layout_manager_get_animation_progress(
        manager: *mut ClutterLayoutManager,
    ) -> c_double;
    pub fn clutter_layout_manager_get_child_meta(
        manager: *mut ClutterLayoutManager,
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
    ) -> *mut ClutterLayoutMeta;
    pub fn clutter_layout_manager_get_preferred_height(
        manager: *mut ClutterLayoutManager,
        container: *mut ClutterContainer,
        for_width: c_float,
        min_height_p: *mut c_float,
        nat_height_p: *mut c_float,
    );
    pub fn clutter_layout_manager_get_preferred_width(
        manager: *mut ClutterLayoutManager,
        container: *mut ClutterContainer,
        for_height: c_float,
        min_width_p: *mut c_float,
        nat_width_p: *mut c_float,
    );
    pub fn clutter_layout_manager_layout_changed(manager: *mut ClutterLayoutManager);
    pub fn clutter_layout_manager_list_child_properties(
        manager: *mut ClutterLayoutManager,
        n_pspecs: *mut c_uint,
    ) -> *mut *mut gobject::GParamSpec;
    pub fn clutter_layout_manager_set_container(
        manager: *mut ClutterLayoutManager,
        container: *mut ClutterContainer,
    );

    //=========================================================================
    // ClutterLayoutMeta
    //=========================================================================
    pub fn clutter_layout_meta_get_type() -> GType;
    pub fn clutter_layout_meta_get_manager(
        data: *mut ClutterLayoutMeta,
    ) -> *mut ClutterLayoutManager;

    //=========================================================================
    // ClutterListModel
    //=========================================================================
    pub fn clutter_list_model_get_type() -> GType;
    pub fn clutter_list_model_new(n_columns: c_uint, ...) -> *mut ClutterModel;
    pub fn clutter_list_model_newv(
        n_columns: c_uint,
        types: *mut GType,
        names: *const *const c_char,
    ) -> *mut ClutterModel;

    //=========================================================================
    // ClutterModel
    //=========================================================================
    pub fn clutter_model_get_type() -> GType;
    pub fn clutter_model_append(model: *mut ClutterModel, ...);
    pub fn clutter_model_appendv(
        model: *mut ClutterModel,
        n_columns: c_uint,
        columns: *mut c_uint,
        values: *mut gobject::GValue,
    );
    pub fn clutter_model_filter_iter(
        model: *mut ClutterModel,
        iter: *mut ClutterModelIter,
    ) -> gboolean;
    pub fn clutter_model_filter_row(model: *mut ClutterModel, row: c_uint) -> gboolean;
    pub fn clutter_model_foreach(
        model: *mut ClutterModel,
        func: ClutterModelForeachFunc,
        user_data: gpointer,
    );
    pub fn clutter_model_get_column_name(model: *mut ClutterModel, column: c_uint)
        -> *const c_char;
    pub fn clutter_model_get_column_type(model: *mut ClutterModel, column: c_uint) -> GType;
    pub fn clutter_model_get_filter_set(model: *mut ClutterModel) -> gboolean;
    pub fn clutter_model_get_first_iter(model: *mut ClutterModel) -> *mut ClutterModelIter;
    pub fn clutter_model_get_iter_at_row(
        model: *mut ClutterModel,
        row: c_uint,
    ) -> *mut ClutterModelIter;
    pub fn clutter_model_get_last_iter(model: *mut ClutterModel) -> *mut ClutterModelIter;
    pub fn clutter_model_get_n_columns(model: *mut ClutterModel) -> c_uint;
    pub fn clutter_model_get_n_rows(model: *mut ClutterModel) -> c_uint;
    pub fn clutter_model_get_sorting_column(model: *mut ClutterModel) -> c_int;
    pub fn clutter_model_insert(model: *mut ClutterModel, row: c_uint, ...);
    pub fn clutter_model_insert_value(
        model: *mut ClutterModel,
        row: c_uint,
        column: c_uint,
        value: *const gobject::GValue,
    );
    pub fn clutter_model_insertv(
        model: *mut ClutterModel,
        row: c_uint,
        n_columns: c_uint,
        columns: *mut c_uint,
        values: *mut gobject::GValue,
    );
    pub fn clutter_model_prepend(model: *mut ClutterModel, ...);
    pub fn clutter_model_prependv(
        model: *mut ClutterModel,
        n_columns: c_uint,
        columns: *mut c_uint,
        values: *mut gobject::GValue,
    );
    pub fn clutter_model_remove(model: *mut ClutterModel, row: c_uint);
    pub fn clutter_model_resort(model: *mut ClutterModel);
    pub fn clutter_model_set_filter(
        model: *mut ClutterModel,
        func: ClutterModelFilterFunc,
        user_data: gpointer,
        notify: glib::GDestroyNotify,
    );
    pub fn clutter_model_set_names(
        model: *mut ClutterModel,
        n_columns: c_uint,
        names: *const *const c_char,
    );
    pub fn clutter_model_set_sort(
        model: *mut ClutterModel,
        column: c_int,
        func: ClutterModelSortFunc,
        user_data: gpointer,
        notify: glib::GDestroyNotify,
    );
    pub fn clutter_model_set_sorting_column(model: *mut ClutterModel, column: c_int);
    pub fn clutter_model_set_types(model: *mut ClutterModel, n_columns: c_uint, types: *mut GType);

    //=========================================================================
    // ClutterModelIter
    //=========================================================================
    pub fn clutter_model_iter_get_type() -> GType;
    pub fn clutter_model_iter_copy(iter: *mut ClutterModelIter) -> *mut ClutterModelIter;
    pub fn clutter_model_iter_get(iter: *mut ClutterModelIter, ...);
    pub fn clutter_model_iter_get_model(iter: *mut ClutterModelIter) -> *mut ClutterModel;
    pub fn clutter_model_iter_get_row(iter: *mut ClutterModelIter) -> c_uint;
    //pub fn clutter_model_iter_get_valist(iter: *mut ClutterModelIter, args: /*Unimplemented*/va_list);
    pub fn clutter_model_iter_get_value(
        iter: *mut ClutterModelIter,
        column: c_uint,
        value: *mut gobject::GValue,
    );
    pub fn clutter_model_iter_is_first(iter: *mut ClutterModelIter) -> gboolean;
    pub fn clutter_model_iter_is_last(iter: *mut ClutterModelIter) -> gboolean;
    pub fn clutter_model_iter_next(iter: *mut ClutterModelIter) -> *mut ClutterModelIter;
    pub fn clutter_model_iter_prev(iter: *mut ClutterModelIter) -> *mut ClutterModelIter;
    pub fn clutter_model_iter_set(iter: *mut ClutterModelIter, ...);
    //pub fn clutter_model_iter_set_valist(iter: *mut ClutterModelIter, args: /*Unimplemented*/va_list);
    pub fn clutter_model_iter_set_value(
        iter: *mut ClutterModelIter,
        column: c_uint,
        value: *const gobject::GValue,
    );

    //=========================================================================
    // ClutterOffscreenEffect
    //=========================================================================
    pub fn clutter_offscreen_effect_get_type() -> GType;
    pub fn clutter_offscreen_effect_create_texture(
        effect: *mut ClutterOffscreenEffect,
        width: c_float,
        height: c_float,
    ) -> cogl::CoglHandle;
    pub fn clutter_offscreen_effect_get_target(
        effect: *mut ClutterOffscreenEffect,
    ) -> *mut cogl::CoglMaterial;
    pub fn clutter_offscreen_effect_get_target_rect(
        effect: *mut ClutterOffscreenEffect,
        rect: *mut ClutterRect,
    ) -> gboolean;
    pub fn clutter_offscreen_effect_get_target_size(
        effect: *mut ClutterOffscreenEffect,
        width: *mut c_float,
        height: *mut c_float,
    ) -> gboolean;
    pub fn clutter_offscreen_effect_get_texture(
        effect: *mut ClutterOffscreenEffect,
    ) -> cogl::CoglHandle;
    pub fn clutter_offscreen_effect_paint_target(effect: *mut ClutterOffscreenEffect);

    //=========================================================================
    // ClutterPageTurnEffect
    //=========================================================================
    pub fn clutter_page_turn_effect_get_type() -> GType;
    pub fn clutter_page_turn_effect_new(
        period: c_double,
        angle: c_double,
        radius: c_float,
    ) -> *mut ClutterEffect;
    pub fn clutter_page_turn_effect_get_angle(effect: *mut ClutterPageTurnEffect) -> c_double;
    pub fn clutter_page_turn_effect_get_period(effect: *mut ClutterPageTurnEffect) -> c_double;
    pub fn clutter_page_turn_effect_get_radius(effect: *mut ClutterPageTurnEffect) -> c_float;
    pub fn clutter_page_turn_effect_set_angle(effect: *mut ClutterPageTurnEffect, angle: c_double);
    pub fn clutter_page_turn_effect_set_period(
        effect: *mut ClutterPageTurnEffect,
        period: c_double,
    );
    pub fn clutter_page_turn_effect_set_radius(effect: *mut ClutterPageTurnEffect, radius: c_float);

    //=========================================================================
    // ClutterPaintNode
    //=========================================================================
    pub fn clutter_paint_node_get_type() -> GType;
    pub fn clutter_paint_node_add_child(node: *mut ClutterPaintNode, child: *mut ClutterPaintNode);
    pub fn clutter_paint_node_add_rectangle(
        node: *mut ClutterPaintNode,
        rect: *const ClutterActorBox,
    );
    pub fn clutter_paint_node_add_texture_rectangle(
        node: *mut ClutterPaintNode,
        rect: *const ClutterActorBox,
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
    );
    pub fn clutter_paint_node_ref(node: *mut ClutterPaintNode) -> *mut ClutterPaintNode;
    pub fn clutter_paint_node_set_name(node: *mut ClutterPaintNode, name: *const c_char);
    pub fn clutter_paint_node_unref(node: *mut ClutterPaintNode);

    //=========================================================================
    // ClutterPanAction
    //=========================================================================
    pub fn clutter_pan_action_get_type() -> GType;
    pub fn clutter_pan_action_new() -> *mut ClutterAction;
    pub fn clutter_pan_action_get_acceleration_factor(self_: *mut ClutterPanAction) -> c_double;
    pub fn clutter_pan_action_get_constrained_motion_delta(
        self_: *mut ClutterPanAction,
        point: c_uint,
        delta_x: *mut c_float,
        delta_y: *mut c_float,
    ) -> c_float;
    pub fn clutter_pan_action_get_deceleration(self_: *mut ClutterPanAction) -> c_double;
    pub fn clutter_pan_action_get_interpolate(self_: *mut ClutterPanAction) -> gboolean;
    pub fn clutter_pan_action_get_interpolated_coords(
        self_: *mut ClutterPanAction,
        interpolated_x: *mut c_float,
        interpolated_y: *mut c_float,
    );
    pub fn clutter_pan_action_get_interpolated_delta(
        self_: *mut ClutterPanAction,
        delta_x: *mut c_float,
        delta_y: *mut c_float,
    ) -> c_float;
    pub fn clutter_pan_action_get_motion_coords(
        self_: *mut ClutterPanAction,
        point: c_uint,
        motion_x: *mut c_float,
        motion_y: *mut c_float,
    );
    pub fn clutter_pan_action_get_motion_delta(
        self_: *mut ClutterPanAction,
        point: c_uint,
        delta_x: *mut c_float,
        delta_y: *mut c_float,
    ) -> c_float;
    pub fn clutter_pan_action_get_pan_axis(self_: *mut ClutterPanAction) -> ClutterPanAxis;
    pub fn clutter_pan_action_set_acceleration_factor(
        self_: *mut ClutterPanAction,
        factor: c_double,
    );
    pub fn clutter_pan_action_set_deceleration(self_: *mut ClutterPanAction, rate: c_double);
    pub fn clutter_pan_action_set_interpolate(
        self_: *mut ClutterPanAction,
        should_interpolate: gboolean,
    );
    pub fn clutter_pan_action_set_pan_axis(self_: *mut ClutterPanAction, axis: ClutterPanAxis);

    //=========================================================================
    // ClutterParamSpecColor
    //=========================================================================
    pub fn clutter_param_color_get_type() -> GType;

    //=========================================================================
    // ClutterParamSpecFixed
    //=========================================================================
    pub fn clutter_param_fixed_get_type() -> GType;

    //=========================================================================
    // ClutterParamSpecUnit
    //=========================================================================
    pub fn clutter_param_units_get_type() -> GType;

    //=========================================================================
    // ClutterPath
    //=========================================================================
    pub fn clutter_path_get_type() -> GType;
    pub fn clutter_path_new() -> *mut ClutterPath;
    pub fn clutter_path_new_with_description(desc: *const c_char) -> *mut ClutterPath;
    pub fn clutter_path_add_cairo_path(path: *mut ClutterPath, cpath: *const cairo::cairo_path_t);
    pub fn clutter_path_add_close(path: *mut ClutterPath);
    pub fn clutter_path_add_curve_to(
        path: *mut ClutterPath,
        x_1: c_int,
        y_1: c_int,
        x_2: c_int,
        y_2: c_int,
        x_3: c_int,
        y_3: c_int,
    );
    pub fn clutter_path_add_line_to(path: *mut ClutterPath, x: c_int, y: c_int);
    pub fn clutter_path_add_move_to(path: *mut ClutterPath, x: c_int, y: c_int);
    pub fn clutter_path_add_node(path: *mut ClutterPath, node: *const ClutterPathNode);
    pub fn clutter_path_add_rel_curve_to(
        path: *mut ClutterPath,
        x_1: c_int,
        y_1: c_int,
        x_2: c_int,
        y_2: c_int,
        x_3: c_int,
        y_3: c_int,
    );
    pub fn clutter_path_add_rel_line_to(path: *mut ClutterPath, x: c_int, y: c_int);
    pub fn clutter_path_add_rel_move_to(path: *mut ClutterPath, x: c_int, y: c_int);
    pub fn clutter_path_add_string(path: *mut ClutterPath, str: *const c_char) -> gboolean;
    pub fn clutter_path_clear(path: *mut ClutterPath);
    pub fn clutter_path_foreach(
        path: *mut ClutterPath,
        callback: ClutterPathCallback,
        user_data: gpointer,
    );
    pub fn clutter_path_get_description(path: *mut ClutterPath) -> *mut c_char;
    pub fn clutter_path_get_length(path: *mut ClutterPath) -> c_uint;
    pub fn clutter_path_get_n_nodes(path: *mut ClutterPath) -> c_uint;
    pub fn clutter_path_get_node(
        path: *mut ClutterPath,
        index_: c_uint,
        node: *mut ClutterPathNode,
    );
    pub fn clutter_path_get_nodes(path: *mut ClutterPath) -> *mut glib::GSList;
    pub fn clutter_path_get_position(
        path: *mut ClutterPath,
        progress: c_double,
        position: *mut ClutterKnot,
    ) -> c_uint;
    pub fn clutter_path_insert_node(
        path: *mut ClutterPath,
        index_: c_int,
        node: *const ClutterPathNode,
    );
    pub fn clutter_path_remove_node(path: *mut ClutterPath, index_: c_uint);
    pub fn clutter_path_replace_node(
        path: *mut ClutterPath,
        index_: c_uint,
        node: *const ClutterPathNode,
    );
    pub fn clutter_path_set_description(path: *mut ClutterPath, str: *const c_char) -> gboolean;
    pub fn clutter_path_to_cairo_path(path: *mut ClutterPath, cr: *mut cairo::cairo_t);

    //=========================================================================
    // ClutterPathConstraint
    //=========================================================================
    pub fn clutter_path_constraint_get_type() -> GType;
    pub fn clutter_path_constraint_new(
        path: *mut ClutterPath,
        offset: c_float,
    ) -> *mut ClutterConstraint;
    pub fn clutter_path_constraint_get_offset(constraint: *mut ClutterPathConstraint) -> c_float;
    pub fn clutter_path_constraint_get_path(
        constraint: *mut ClutterPathConstraint,
    ) -> *mut ClutterPath;
    pub fn clutter_path_constraint_set_offset(
        constraint: *mut ClutterPathConstraint,
        offset: c_float,
    );
    pub fn clutter_path_constraint_set_path(
        constraint: *mut ClutterPathConstraint,
        path: *mut ClutterPath,
    );

    //=========================================================================
    // ClutterPipelineNode
    //=========================================================================
    pub fn clutter_pipeline_node_get_type() -> GType;

    //=========================================================================
    // ClutterPropertyTransition
    //=========================================================================
    pub fn clutter_property_transition_get_type() -> GType;
    pub fn clutter_property_transition_new(property_name: *const c_char) -> *mut ClutterTransition;
    pub fn clutter_property_transition_get_property_name(
        transition: *mut ClutterPropertyTransition,
    ) -> *const c_char;
    pub fn clutter_property_transition_set_property_name(
        transition: *mut ClutterPropertyTransition,
        property_name: *const c_char,
    );

    //=========================================================================
    // ClutterRectangle
    //=========================================================================
    pub fn clutter_rectangle_get_type() -> GType;
    pub fn clutter_rectangle_new() -> *mut ClutterActor;
    pub fn clutter_rectangle_new_with_color(color: *const ClutterColor) -> *mut ClutterActor;
    pub fn clutter_rectangle_get_border_color(
        rectangle: *mut ClutterRectangle,
        color: *mut ClutterColor,
    );
    pub fn clutter_rectangle_get_border_width(rectangle: *mut ClutterRectangle) -> c_uint;
    pub fn clutter_rectangle_get_color(rectangle: *mut ClutterRectangle, color: *mut ClutterColor);
    pub fn clutter_rectangle_set_border_color(
        rectangle: *mut ClutterRectangle,
        color: *const ClutterColor,
    );
    pub fn clutter_rectangle_set_border_width(rectangle: *mut ClutterRectangle, width: c_uint);
    pub fn clutter_rectangle_set_color(
        rectangle: *mut ClutterRectangle,
        color: *const ClutterColor,
    );

    //=========================================================================
    // ClutterRotateAction
    //=========================================================================
    pub fn clutter_rotate_action_get_type() -> GType;
    pub fn clutter_rotate_action_new() -> *mut ClutterAction;

    //=========================================================================
    // ClutterScore
    //=========================================================================
    pub fn clutter_score_get_type() -> GType;
    pub fn clutter_score_new() -> *mut ClutterScore;
    pub fn clutter_score_append(
        score: *mut ClutterScore,
        parent: *mut ClutterTimeline,
        timeline: *mut ClutterTimeline,
    ) -> c_ulong;
    pub fn clutter_score_append_at_marker(
        score: *mut ClutterScore,
        parent: *mut ClutterTimeline,
        marker_name: *const c_char,
        timeline: *mut ClutterTimeline,
    ) -> c_ulong;
    pub fn clutter_score_get_loop(score: *mut ClutterScore) -> gboolean;
    pub fn clutter_score_get_timeline(
        score: *mut ClutterScore,
        id_: c_ulong,
    ) -> *mut ClutterTimeline;
    pub fn clutter_score_is_playing(score: *mut ClutterScore) -> gboolean;
    pub fn clutter_score_list_timelines(score: *mut ClutterScore) -> *mut glib::GSList;
    pub fn clutter_score_pause(score: *mut ClutterScore);
    pub fn clutter_score_remove(score: *mut ClutterScore, id_: c_ulong);
    pub fn clutter_score_remove_all(score: *mut ClutterScore);
    pub fn clutter_score_rewind(score: *mut ClutterScore);
    pub fn clutter_score_set_loop(score: *mut ClutterScore, loop_: gboolean);
    pub fn clutter_score_start(score: *mut ClutterScore);
    pub fn clutter_score_stop(score: *mut ClutterScore);

    //=========================================================================
    // ClutterScript
    //=========================================================================
    pub fn clutter_script_get_type() -> GType;
    pub fn clutter_script_new() -> *mut ClutterScript;
    pub fn clutter_script_add_search_paths(
        script: *mut ClutterScript,
        paths: *const *const c_char,
        n_paths: size_t,
    );
    pub fn clutter_script_add_states(
        script: *mut ClutterScript,
        name: *const c_char,
        state: *mut ClutterState,
    );
    pub fn clutter_script_connect_signals(script: *mut ClutterScript, user_data: gpointer);
    pub fn clutter_script_connect_signals_full(
        script: *mut ClutterScript,
        func: ClutterScriptConnectFunc,
        user_data: gpointer,
    );
    pub fn clutter_script_ensure_objects(script: *mut ClutterScript);
    pub fn clutter_script_get_object(
        script: *mut ClutterScript,
        name: *const c_char,
    ) -> *mut gobject::GObject;
    pub fn clutter_script_get_objects(
        script: *mut ClutterScript,
        first_name: *const c_char,
        ...
    ) -> c_int;
    pub fn clutter_script_get_states(
        script: *mut ClutterScript,
        name: *const c_char,
    ) -> *mut ClutterState;
    pub fn clutter_script_get_translation_domain(script: *mut ClutterScript) -> *const c_char;
    pub fn clutter_script_get_type_from_name(
        script: *mut ClutterScript,
        type_name: *const c_char,
    ) -> GType;
    pub fn clutter_script_list_objects(script: *mut ClutterScript) -> *mut glib::GList;
    pub fn clutter_script_load_from_data(
        script: *mut ClutterScript,
        data: *const c_char,
        length: ssize_t,
        error: *mut *mut glib::GError,
    ) -> c_uint;
    pub fn clutter_script_load_from_file(
        script: *mut ClutterScript,
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> c_uint;
    pub fn clutter_script_load_from_resource(
        script: *mut ClutterScript,
        resource_path: *const c_char,
        error: *mut *mut glib::GError,
    ) -> c_uint;
    pub fn clutter_script_lookup_filename(
        script: *mut ClutterScript,
        filename: *const c_char,
    ) -> *mut c_char;
    pub fn clutter_script_set_translation_domain(script: *mut ClutterScript, domain: *const c_char);
    pub fn clutter_script_unmerge_objects(script: *mut ClutterScript, merge_id: c_uint);

    //=========================================================================
    // ClutterScrollActor
    //=========================================================================
    pub fn clutter_scroll_actor_get_type() -> GType;
    pub fn clutter_scroll_actor_new() -> *mut ClutterActor;
    pub fn clutter_scroll_actor_get_scroll_mode(
        actor: *mut ClutterScrollActor,
    ) -> ClutterScrollMode;
    pub fn clutter_scroll_actor_scroll_to_point(
        actor: *mut ClutterScrollActor,
        point: *const ClutterPoint,
    );
    pub fn clutter_scroll_actor_scroll_to_rect(
        actor: *mut ClutterScrollActor,
        rect: *const ClutterRect,
    );
    pub fn clutter_scroll_actor_set_scroll_mode(
        actor: *mut ClutterScrollActor,
        mode: ClutterScrollMode,
    );

    //=========================================================================
    // ClutterSettings
    //=========================================================================
    pub fn clutter_settings_get_type() -> GType;
    pub fn clutter_settings_get_default() -> *mut ClutterSettings;

    //=========================================================================
    // ClutterShader
    //=========================================================================
    pub fn clutter_shader_get_type() -> GType;
    pub fn clutter_shader_new() -> *mut ClutterShader;
    pub fn clutter_shader_compile(
        shader: *mut ClutterShader,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn clutter_shader_get_cogl_fragment_shader(shader: *mut ClutterShader) -> cogl::CoglHandle;
    pub fn clutter_shader_get_cogl_program(shader: *mut ClutterShader) -> cogl::CoglHandle;
    pub fn clutter_shader_get_cogl_vertex_shader(shader: *mut ClutterShader) -> cogl::CoglHandle;
    pub fn clutter_shader_get_fragment_source(shader: *mut ClutterShader) -> *const c_char;
    pub fn clutter_shader_get_is_enabled(shader: *mut ClutterShader) -> gboolean;
    pub fn clutter_shader_get_vertex_source(shader: *mut ClutterShader) -> *const c_char;
    pub fn clutter_shader_is_compiled(shader: *mut ClutterShader) -> gboolean;
    pub fn clutter_shader_release(shader: *mut ClutterShader);
    pub fn clutter_shader_set_fragment_source(
        shader: *mut ClutterShader,
        data: *const c_char,
        length: ssize_t,
    );
    pub fn clutter_shader_set_is_enabled(shader: *mut ClutterShader, enabled: gboolean);
    pub fn clutter_shader_set_uniform(
        shader: *mut ClutterShader,
        name: *const c_char,
        value: *const gobject::GValue,
    );
    pub fn clutter_shader_set_vertex_source(
        shader: *mut ClutterShader,
        data: *const c_char,
        length: ssize_t,
    );

    //=========================================================================
    // ClutterShaderEffect
    //=========================================================================
    pub fn clutter_shader_effect_get_type() -> GType;
    pub fn clutter_shader_effect_new(shader_type: ClutterShaderType) -> *mut ClutterEffect;
    pub fn clutter_shader_effect_get_program(effect: *mut ClutterShaderEffect) -> cogl::CoglHandle;
    pub fn clutter_shader_effect_get_shader(effect: *mut ClutterShaderEffect) -> cogl::CoglHandle;
    pub fn clutter_shader_effect_set_shader_source(
        effect: *mut ClutterShaderEffect,
        source: *const c_char,
    ) -> gboolean;
    pub fn clutter_shader_effect_set_uniform(
        effect: *mut ClutterShaderEffect,
        name: *const c_char,
        gtype: GType,
        n_values: size_t,
        ...
    );
    pub fn clutter_shader_effect_set_uniform_value(
        effect: *mut ClutterShaderEffect,
        name: *const c_char,
        value: *const gobject::GValue,
    );

    //=========================================================================
    // ClutterShaderFloat
    //=========================================================================
    pub fn clutter_shader_float_get_type() -> GType;

    //=========================================================================
    // ClutterShaderInt
    //=========================================================================
    pub fn clutter_shader_int_get_type() -> GType;

    //=========================================================================
    // ClutterShaderMatrix
    //=========================================================================
    pub fn clutter_shader_matrix_get_type() -> GType;

    //=========================================================================
    // ClutterSnapConstraint
    //=========================================================================
    pub fn clutter_snap_constraint_get_type() -> GType;
    pub fn clutter_snap_constraint_new(
        source: *mut ClutterActor,
        from_edge: ClutterSnapEdge,
        to_edge: ClutterSnapEdge,
        offset: c_float,
    ) -> *mut ClutterConstraint;
    pub fn clutter_snap_constraint_get_edges(
        constraint: *mut ClutterSnapConstraint,
        from_edge: *mut ClutterSnapEdge,
        to_edge: *mut ClutterSnapEdge,
    );
    pub fn clutter_snap_constraint_get_offset(constraint: *mut ClutterSnapConstraint) -> c_float;
    pub fn clutter_snap_constraint_get_source(
        constraint: *mut ClutterSnapConstraint,
    ) -> *mut ClutterActor;
    pub fn clutter_snap_constraint_set_edges(
        constraint: *mut ClutterSnapConstraint,
        from_edge: ClutterSnapEdge,
        to_edge: ClutterSnapEdge,
    );
    pub fn clutter_snap_constraint_set_offset(
        constraint: *mut ClutterSnapConstraint,
        offset: c_float,
    );
    pub fn clutter_snap_constraint_set_source(
        constraint: *mut ClutterSnapConstraint,
        source: *mut ClutterActor,
    );

    //=========================================================================
    // ClutterStage
    //=========================================================================
    pub fn clutter_stage_get_type() -> GType;
    pub fn clutter_stage_new() -> *mut ClutterActor;
    pub fn clutter_stage_get_default() -> *mut ClutterStage;
    pub fn clutter_stage_ensure_current(stage: *mut ClutterStage);
    pub fn clutter_stage_ensure_redraw(stage: *mut ClutterStage);
    pub fn clutter_stage_ensure_viewport(stage: *mut ClutterStage);
    pub fn clutter_stage_event(stage: *mut ClutterStage, event: *mut ClutterEvent) -> gboolean;
    pub fn clutter_stage_get_accept_focus(stage: *mut ClutterStage) -> gboolean;
    pub fn clutter_stage_get_actor_at_pos(
        stage: *mut ClutterStage,
        pick_mode: ClutterPickMode,
        x: c_int,
        y: c_int,
    ) -> *mut ClutterActor;
    pub fn clutter_stage_get_color(stage: *mut ClutterStage, color: *mut ClutterColor);
    pub fn clutter_stage_get_fog(stage: *mut ClutterStage, fog: *mut ClutterFog);
    pub fn clutter_stage_get_fullscreen(stage: *mut ClutterStage) -> gboolean;
    pub fn clutter_stage_get_key_focus(stage: *mut ClutterStage) -> *mut ClutterActor;
    pub fn clutter_stage_get_minimum_size(
        stage: *mut ClutterStage,
        width: *mut c_uint,
        height: *mut c_uint,
    );
    pub fn clutter_stage_get_motion_events_enabled(stage: *mut ClutterStage) -> gboolean;
    pub fn clutter_stage_get_no_clear_hint(stage: *mut ClutterStage) -> gboolean;
    pub fn clutter_stage_get_perspective(
        stage: *mut ClutterStage,
        perspective: *mut ClutterPerspective,
    );
    pub fn clutter_stage_get_redraw_clip_bounds(
        stage: *mut ClutterStage,
        clip: *mut cairo::cairo_rectangle_int_t,
    );
    pub fn clutter_stage_get_throttle_motion_events(stage: *mut ClutterStage) -> gboolean;
    pub fn clutter_stage_get_title(stage: *mut ClutterStage) -> *const c_char;
    pub fn clutter_stage_get_use_alpha(stage: *mut ClutterStage) -> gboolean;
    pub fn clutter_stage_get_use_fog(stage: *mut ClutterStage) -> gboolean;
    pub fn clutter_stage_get_user_resizable(stage: *mut ClutterStage) -> gboolean;
    pub fn clutter_stage_hide_cursor(stage: *mut ClutterStage);
    pub fn clutter_stage_is_default(stage: *mut ClutterStage) -> gboolean;
    pub fn clutter_stage_queue_redraw(stage: *mut ClutterStage);
    pub fn clutter_stage_read_pixels(
        stage: *mut ClutterStage,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> *mut u8;
    pub fn clutter_stage_set_accept_focus(stage: *mut ClutterStage, accept_focus: gboolean);
    pub fn clutter_stage_set_color(stage: *mut ClutterStage, color: *const ClutterColor);
    pub fn clutter_stage_set_fog(stage: *mut ClutterStage, fog: *mut ClutterFog);
    pub fn clutter_stage_set_fullscreen(stage: *mut ClutterStage, fullscreen: gboolean);
    pub fn clutter_stage_set_key_focus(stage: *mut ClutterStage, actor: *mut ClutterActor);
    pub fn clutter_stage_set_minimum_size(stage: *mut ClutterStage, width: c_uint, height: c_uint);
    pub fn clutter_stage_set_motion_events_enabled(stage: *mut ClutterStage, enabled: gboolean);
    pub fn clutter_stage_set_no_clear_hint(stage: *mut ClutterStage, no_clear: gboolean);
    pub fn clutter_stage_set_perspective(
        stage: *mut ClutterStage,
        perspective: *mut ClutterPerspective,
    );
    pub fn clutter_stage_set_throttle_motion_events(stage: *mut ClutterStage, throttle: gboolean);
    pub fn clutter_stage_set_title(stage: *mut ClutterStage, title: *const c_char);
    pub fn clutter_stage_set_use_alpha(stage: *mut ClutterStage, use_alpha: gboolean);
    pub fn clutter_stage_set_use_fog(stage: *mut ClutterStage, fog: gboolean);
    pub fn clutter_stage_set_user_resizable(stage: *mut ClutterStage, resizable: gboolean);
    pub fn clutter_stage_show_cursor(stage: *mut ClutterStage);

    //=========================================================================
    // ClutterStageManager
    //=========================================================================
    pub fn clutter_stage_manager_get_type() -> GType;
    pub fn clutter_stage_manager_get_default() -> *mut ClutterStageManager;
    pub fn clutter_stage_manager_get_default_stage(
        stage_manager: *mut ClutterStageManager,
    ) -> *mut ClutterStage;
    pub fn clutter_stage_manager_list_stages(
        stage_manager: *mut ClutterStageManager,
    ) -> *mut glib::GSList;
    pub fn clutter_stage_manager_peek_stages(
        stage_manager: *mut ClutterStageManager,
    ) -> *const glib::GSList;
    pub fn clutter_stage_manager_set_default_stage(
        stage_manager: *mut ClutterStageManager,
        stage: *mut ClutterStage,
    );

    //=========================================================================
    // ClutterState
    //=========================================================================
    pub fn clutter_state_get_type() -> GType;
    pub fn clutter_state_new() -> *mut ClutterState;
    pub fn clutter_state_get_animator(
        state: *mut ClutterState,
        source_state_name: *const c_char,
        target_state_name: *const c_char,
    ) -> *mut ClutterAnimator;
    pub fn clutter_state_get_duration(
        state: *mut ClutterState,
        source_state_name: *const c_char,
        target_state_name: *const c_char,
    ) -> c_uint;
    pub fn clutter_state_get_keys(
        state: *mut ClutterState,
        source_state_name: *const c_char,
        target_state_name: *const c_char,
        object: *mut gobject::GObject,
        property_name: *const c_char,
    ) -> *mut glib::GList;
    pub fn clutter_state_get_state(state: *mut ClutterState) -> *const c_char;
    pub fn clutter_state_get_states(state: *mut ClutterState) -> *mut glib::GList;
    pub fn clutter_state_get_timeline(state: *mut ClutterState) -> *mut ClutterTimeline;
    pub fn clutter_state_remove_key(
        state: *mut ClutterState,
        source_state_name: *const c_char,
        target_state_name: *const c_char,
        object: *mut gobject::GObject,
        property_name: *const c_char,
    );
    pub fn clutter_state_set(
        state: *mut ClutterState,
        source_state_name: *const c_char,
        target_state_name: *const c_char,
        first_object: gpointer,
        first_property_name: *const c_char,
        first_mode: c_ulong,
        ...
    );
    pub fn clutter_state_set_animator(
        state: *mut ClutterState,
        source_state_name: *const c_char,
        target_state_name: *const c_char,
        animator: *mut ClutterAnimator,
    );
    pub fn clutter_state_set_duration(
        state: *mut ClutterState,
        source_state_name: *const c_char,
        target_state_name: *const c_char,
        duration: c_uint,
    );
    pub fn clutter_state_set_key(
        state: *mut ClutterState,
        source_state_name: *const c_char,
        target_state_name: *const c_char,
        object: *mut gobject::GObject,
        property_name: *const c_char,
        mode: c_uint,
        value: *const gobject::GValue,
        pre_delay: c_double,
        post_delay: c_double,
    ) -> *mut ClutterState;
    pub fn clutter_state_set_state(
        state: *mut ClutterState,
        target_state_name: *const c_char,
    ) -> *mut ClutterTimeline;
    pub fn clutter_state_warp_to_state(
        state: *mut ClutterState,
        target_state_name: *const c_char,
    ) -> *mut ClutterTimeline;

    //=========================================================================
    // ClutterSwipeAction
    //=========================================================================
    pub fn clutter_swipe_action_get_type() -> GType;
    pub fn clutter_swipe_action_new() -> *mut ClutterAction;

    //=========================================================================
    // ClutterTableLayout
    //=========================================================================
    pub fn clutter_table_layout_get_type() -> GType;
    pub fn clutter_table_layout_new() -> *mut ClutterLayoutManager;
    pub fn clutter_table_layout_get_alignment(
        layout: *mut ClutterTableLayout,
        actor: *mut ClutterActor,
        x_align: *mut ClutterTableAlignment,
        y_align: *mut ClutterTableAlignment,
    );
    pub fn clutter_table_layout_get_column_count(layout: *mut ClutterTableLayout) -> c_int;
    pub fn clutter_table_layout_get_column_spacing(layout: *mut ClutterTableLayout) -> c_uint;
    pub fn clutter_table_layout_get_easing_duration(layout: *mut ClutterTableLayout) -> c_uint;
    pub fn clutter_table_layout_get_easing_mode(layout: *mut ClutterTableLayout) -> c_ulong;
    pub fn clutter_table_layout_get_expand(
        layout: *mut ClutterTableLayout,
        actor: *mut ClutterActor,
        x_expand: *mut gboolean,
        y_expand: *mut gboolean,
    );
    pub fn clutter_table_layout_get_fill(
        layout: *mut ClutterTableLayout,
        actor: *mut ClutterActor,
        x_fill: *mut gboolean,
        y_fill: *mut gboolean,
    );
    pub fn clutter_table_layout_get_row_count(layout: *mut ClutterTableLayout) -> c_int;
    pub fn clutter_table_layout_get_row_spacing(layout: *mut ClutterTableLayout) -> c_uint;
    pub fn clutter_table_layout_get_span(
        layout: *mut ClutterTableLayout,
        actor: *mut ClutterActor,
        column_span: *mut c_int,
        row_span: *mut c_int,
    );
    pub fn clutter_table_layout_get_use_animations(layout: *mut ClutterTableLayout) -> gboolean;
    pub fn clutter_table_layout_pack(
        layout: *mut ClutterTableLayout,
        actor: *mut ClutterActor,
        column: c_int,
        row: c_int,
    );
    pub fn clutter_table_layout_set_alignment(
        layout: *mut ClutterTableLayout,
        actor: *mut ClutterActor,
        x_align: ClutterTableAlignment,
        y_align: ClutterTableAlignment,
    );
    pub fn clutter_table_layout_set_column_spacing(
        layout: *mut ClutterTableLayout,
        spacing: c_uint,
    );
    pub fn clutter_table_layout_set_easing_duration(layout: *mut ClutterTableLayout, msecs: c_uint);
    pub fn clutter_table_layout_set_easing_mode(layout: *mut ClutterTableLayout, mode: c_ulong);
    pub fn clutter_table_layout_set_expand(
        layout: *mut ClutterTableLayout,
        actor: *mut ClutterActor,
        x_expand: gboolean,
        y_expand: gboolean,
    );
    pub fn clutter_table_layout_set_fill(
        layout: *mut ClutterTableLayout,
        actor: *mut ClutterActor,
        x_fill: gboolean,
        y_fill: gboolean,
    );
    pub fn clutter_table_layout_set_row_spacing(layout: *mut ClutterTableLayout, spacing: c_uint);
    pub fn clutter_table_layout_set_span(
        layout: *mut ClutterTableLayout,
        actor: *mut ClutterActor,
        column_span: c_int,
        row_span: c_int,
    );
    pub fn clutter_table_layout_set_use_animations(
        layout: *mut ClutterTableLayout,
        animate: gboolean,
    );

    //=========================================================================
    // ClutterTapAction
    //=========================================================================
    pub fn clutter_tap_action_get_type() -> GType;
    pub fn clutter_tap_action_new() -> *mut ClutterAction;

    //=========================================================================
    // ClutterText
    //=========================================================================
    pub fn clutter_text_get_type() -> GType;
    pub fn clutter_text_new() -> *mut ClutterActor;
    pub fn clutter_text_new_full(
        font_name: *const c_char,
        text: *const c_char,
        color: *const ClutterColor,
    ) -> *mut ClutterActor;
    pub fn clutter_text_new_with_buffer(buffer: *mut ClutterTextBuffer) -> *mut ClutterActor;
    pub fn clutter_text_new_with_text(
        font_name: *const c_char,
        text: *const c_char,
    ) -> *mut ClutterActor;
    pub fn clutter_text_activate(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_coords_to_position(
        self_: *mut ClutterText,
        x: c_float,
        y: c_float,
    ) -> c_int;
    pub fn clutter_text_delete_chars(self_: *mut ClutterText, n_chars: c_uint);
    pub fn clutter_text_delete_selection(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_delete_text(self_: *mut ClutterText, start_pos: ssize_t, end_pos: ssize_t);
    pub fn clutter_text_get_activatable(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_get_attributes(self_: *mut ClutterText) -> *mut pango::PangoAttrList;
    pub fn clutter_text_get_buffer(self_: *mut ClutterText) -> *mut ClutterTextBuffer;
    pub fn clutter_text_get_chars(
        self_: *mut ClutterText,
        start_pos: ssize_t,
        end_pos: ssize_t,
    ) -> *mut c_char;
    pub fn clutter_text_get_color(self_: *mut ClutterText, color: *mut ClutterColor);
    pub fn clutter_text_get_cursor_color(self_: *mut ClutterText, color: *mut ClutterColor);
    pub fn clutter_text_get_cursor_position(self_: *mut ClutterText) -> c_int;
    pub fn clutter_text_get_cursor_rect(self_: *mut ClutterText, rect: *mut ClutterRect);
    pub fn clutter_text_get_cursor_size(self_: *mut ClutterText) -> c_uint;
    pub fn clutter_text_get_cursor_visible(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_get_editable(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_get_ellipsize(self_: *mut ClutterText) -> pango::PangoEllipsizeMode;
    pub fn clutter_text_get_font_description(
        self_: *mut ClutterText,
    ) -> *mut pango::PangoFontDescription;
    pub fn clutter_text_get_font_name(self_: *mut ClutterText) -> *const c_char;
    pub fn clutter_text_get_justify(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_get_layout(self_: *mut ClutterText) -> *mut pango::PangoLayout;
    pub fn clutter_text_get_layout_offsets(self_: *mut ClutterText, x: *mut c_int, y: *mut c_int);
    pub fn clutter_text_get_line_alignment(self_: *mut ClutterText) -> pango::PangoAlignment;
    pub fn clutter_text_get_line_wrap(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_get_line_wrap_mode(self_: *mut ClutterText) -> pango::PangoWrapMode;
    pub fn clutter_text_get_max_length(self_: *mut ClutterText) -> c_int;
    pub fn clutter_text_get_password_char(self_: *mut ClutterText) -> u32;
    pub fn clutter_text_get_selectable(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_get_selected_text_color(self_: *mut ClutterText, color: *mut ClutterColor);
    pub fn clutter_text_get_selection(self_: *mut ClutterText) -> *mut c_char;
    pub fn clutter_text_get_selection_bound(self_: *mut ClutterText) -> c_int;
    pub fn clutter_text_get_selection_color(self_: *mut ClutterText, color: *mut ClutterColor);
    pub fn clutter_text_get_single_line_mode(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_get_text(self_: *mut ClutterText) -> *const c_char;
    pub fn clutter_text_get_use_markup(self_: *mut ClutterText) -> gboolean;
    pub fn clutter_text_insert_text(
        self_: *mut ClutterText,
        text: *const c_char,
        position: ssize_t,
    );
    pub fn clutter_text_insert_unichar(self_: *mut ClutterText, wc: u32);
    pub fn clutter_text_position_to_coords(
        self_: *mut ClutterText,
        position: c_int,
        x: *mut c_float,
        y: *mut c_float,
        line_height: *mut c_float,
    ) -> gboolean;
    pub fn clutter_text_set_activatable(self_: *mut ClutterText, activatable: gboolean);
    pub fn clutter_text_set_attributes(self_: *mut ClutterText, attrs: *mut pango::PangoAttrList);
    pub fn clutter_text_set_buffer(self_: *mut ClutterText, buffer: *mut ClutterTextBuffer);
    pub fn clutter_text_set_color(self_: *mut ClutterText, color: *const ClutterColor);
    pub fn clutter_text_set_cursor_color(self_: *mut ClutterText, color: *const ClutterColor);
    pub fn clutter_text_set_cursor_position(self_: *mut ClutterText, position: c_int);
    pub fn clutter_text_set_cursor_size(self_: *mut ClutterText, size: c_int);
    pub fn clutter_text_set_cursor_visible(self_: *mut ClutterText, cursor_visible: gboolean);
    pub fn clutter_text_set_editable(self_: *mut ClutterText, editable: gboolean);
    pub fn clutter_text_set_ellipsize(self_: *mut ClutterText, mode: pango::PangoEllipsizeMode);
    pub fn clutter_text_set_font_description(
        self_: *mut ClutterText,
        font_desc: *mut pango::PangoFontDescription,
    );
    pub fn clutter_text_set_font_name(self_: *mut ClutterText, font_name: *const c_char);
    pub fn clutter_text_set_justify(self_: *mut ClutterText, justify: gboolean);
    pub fn clutter_text_set_line_alignment(
        self_: *mut ClutterText,
        alignment: pango::PangoAlignment,
    );
    pub fn clutter_text_set_line_wrap(self_: *mut ClutterText, line_wrap: gboolean);
    pub fn clutter_text_set_line_wrap_mode(
        self_: *mut ClutterText,
        wrap_mode: pango::PangoWrapMode,
    );
    pub fn clutter_text_set_markup(self_: *mut ClutterText, markup: *const c_char);
    pub fn clutter_text_set_max_length(self_: *mut ClutterText, max: c_int);
    pub fn clutter_text_set_password_char(self_: *mut ClutterText, wc: u32);
    pub fn clutter_text_set_preedit_string(
        self_: *mut ClutterText,
        preedit_str: *const c_char,
        preedit_attrs: *mut pango::PangoAttrList,
        cursor_pos: c_uint,
    );
    pub fn clutter_text_set_selectable(self_: *mut ClutterText, selectable: gboolean);
    pub fn clutter_text_set_selected_text_color(
        self_: *mut ClutterText,
        color: *const ClutterColor,
    );
    pub fn clutter_text_set_selection(
        self_: *mut ClutterText,
        start_pos: ssize_t,
        end_pos: ssize_t,
    );
    pub fn clutter_text_set_selection_bound(self_: *mut ClutterText, selection_bound: c_int);
    pub fn clutter_text_set_selection_color(self_: *mut ClutterText, color: *const ClutterColor);
    pub fn clutter_text_set_single_line_mode(self_: *mut ClutterText, single_line: gboolean);
    pub fn clutter_text_set_text(self_: *mut ClutterText, text: *const c_char);
    pub fn clutter_text_set_use_markup(self_: *mut ClutterText, setting: gboolean);

    //=========================================================================
    // ClutterTextBuffer
    //=========================================================================
    pub fn clutter_text_buffer_get_type() -> GType;
    pub fn clutter_text_buffer_new() -> *mut ClutterTextBuffer;
    pub fn clutter_text_buffer_new_with_text(
        text: *const c_char,
        text_len: ssize_t,
    ) -> *mut ClutterTextBuffer;
    pub fn clutter_text_buffer_delete_text(
        buffer: *mut ClutterTextBuffer,
        position: c_uint,
        n_chars: c_int,
    ) -> c_uint;
    pub fn clutter_text_buffer_emit_deleted_text(
        buffer: *mut ClutterTextBuffer,
        position: c_uint,
        n_chars: c_uint,
    );
    pub fn clutter_text_buffer_emit_inserted_text(
        buffer: *mut ClutterTextBuffer,
        position: c_uint,
        chars: *const c_char,
        n_chars: c_uint,
    );
    pub fn clutter_text_buffer_get_bytes(buffer: *mut ClutterTextBuffer) -> size_t;
    pub fn clutter_text_buffer_get_length(buffer: *mut ClutterTextBuffer) -> c_uint;
    pub fn clutter_text_buffer_get_max_length(buffer: *mut ClutterTextBuffer) -> c_int;
    pub fn clutter_text_buffer_get_text(buffer: *mut ClutterTextBuffer) -> *const c_char;
    pub fn clutter_text_buffer_insert_text(
        buffer: *mut ClutterTextBuffer,
        position: c_uint,
        chars: *const c_char,
        n_chars: c_int,
    ) -> c_uint;
    pub fn clutter_text_buffer_set_max_length(buffer: *mut ClutterTextBuffer, max_length: c_int);
    pub fn clutter_text_buffer_set_text(
        buffer: *mut ClutterTextBuffer,
        chars: *const c_char,
        n_chars: c_int,
    );

    //=========================================================================
    // ClutterTextNode
    //=========================================================================
    pub fn clutter_text_node_get_type() -> GType;
    pub fn clutter_text_node_new(
        layout: *mut pango::PangoLayout,
        color: *const ClutterColor,
    ) -> *mut ClutterPaintNode;

    //=========================================================================
    // ClutterTexture
    //=========================================================================
    pub fn clutter_texture_get_type() -> GType;
    pub fn clutter_texture_new() -> *mut ClutterActor;
    pub fn clutter_texture_new_from_actor(actor: *mut ClutterActor) -> *mut ClutterActor;
    pub fn clutter_texture_new_from_file(
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut ClutterActor;
    pub fn clutter_texture_get_base_size(
        texture: *mut ClutterTexture,
        width: *mut c_int,
        height: *mut c_int,
    );
    pub fn clutter_texture_get_cogl_material(texture: *mut ClutterTexture) -> cogl::CoglHandle;
    pub fn clutter_texture_get_cogl_texture(texture: *mut ClutterTexture) -> cogl::CoglHandle;
    pub fn clutter_texture_get_filter_quality(
        texture: *mut ClutterTexture,
    ) -> ClutterTextureQuality;
    pub fn clutter_texture_get_keep_aspect_ratio(texture: *mut ClutterTexture) -> gboolean;
    pub fn clutter_texture_get_load_async(texture: *mut ClutterTexture) -> gboolean;
    pub fn clutter_texture_get_load_data_async(texture: *mut ClutterTexture) -> gboolean;
    pub fn clutter_texture_get_max_tile_waste(texture: *mut ClutterTexture) -> c_int;
    pub fn clutter_texture_get_pick_with_alpha(texture: *mut ClutterTexture) -> gboolean;
    pub fn clutter_texture_get_pixel_format(texture: *mut ClutterTexture) -> cogl::CoglPixelFormat;
    pub fn clutter_texture_get_repeat(
        texture: *mut ClutterTexture,
        repeat_x: *mut gboolean,
        repeat_y: *mut gboolean,
    );
    pub fn clutter_texture_get_sync_size(texture: *mut ClutterTexture) -> gboolean;
    pub fn clutter_texture_set_area_from_rgb_data(
        texture: *mut ClutterTexture,
        data: *const u8,
        has_alpha: gboolean,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        rowstride: c_int,
        bpp: c_int,
        flags: ClutterTextureFlags,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn clutter_texture_set_cogl_material(
        texture: *mut ClutterTexture,
        cogl_material: cogl::CoglHandle,
    );
    pub fn clutter_texture_set_cogl_texture(
        texture: *mut ClutterTexture,
        cogl_tex: cogl::CoglHandle,
    );
    pub fn clutter_texture_set_filter_quality(
        texture: *mut ClutterTexture,
        filter_quality: ClutterTextureQuality,
    );
    pub fn clutter_texture_set_from_file(
        texture: *mut ClutterTexture,
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn clutter_texture_set_from_rgb_data(
        texture: *mut ClutterTexture,
        data: *const u8,
        has_alpha: gboolean,
        width: c_int,
        height: c_int,
        rowstride: c_int,
        bpp: c_int,
        flags: ClutterTextureFlags,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn clutter_texture_set_from_yuv_data(
        texture: *mut ClutterTexture,
        data: *const u8,
        width: c_int,
        height: c_int,
        flags: ClutterTextureFlags,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn clutter_texture_set_keep_aspect_ratio(
        texture: *mut ClutterTexture,
        keep_aspect: gboolean,
    );
    pub fn clutter_texture_set_load_async(texture: *mut ClutterTexture, load_async: gboolean);
    pub fn clutter_texture_set_load_data_async(texture: *mut ClutterTexture, load_async: gboolean);
    pub fn clutter_texture_set_pick_with_alpha(
        texture: *mut ClutterTexture,
        pick_with_alpha: gboolean,
    );
    pub fn clutter_texture_set_repeat(
        texture: *mut ClutterTexture,
        repeat_x: gboolean,
        repeat_y: gboolean,
    );
    pub fn clutter_texture_set_sync_size(texture: *mut ClutterTexture, sync_size: gboolean);

    //=========================================================================
    // ClutterTextureNode
    //=========================================================================
    pub fn clutter_texture_node_get_type() -> GType;
    pub fn clutter_texture_node_new(
        texture: *mut cogl::CoglTexture,
        color: *const ClutterColor,
        min_filter: ClutterScalingFilter,
        mag_filter: ClutterScalingFilter,
    ) -> *mut ClutterPaintNode;

    //=========================================================================
    // ClutterTimeline
    //=========================================================================
    pub fn clutter_timeline_get_type() -> GType;
    pub fn clutter_timeline_new(msecs: c_uint) -> *mut ClutterTimeline;
    pub fn clutter_timeline_add_marker(
        timeline: *mut ClutterTimeline,
        marker_name: *const c_char,
        progress: c_double,
    );
    pub fn clutter_timeline_add_marker_at_time(
        timeline: *mut ClutterTimeline,
        marker_name: *const c_char,
        msecs: c_uint,
    );
    pub fn clutter_timeline_advance(timeline: *mut ClutterTimeline, msecs: c_uint);
    pub fn clutter_timeline_advance_to_marker(
        timeline: *mut ClutterTimeline,
        marker_name: *const c_char,
    );
    pub fn clutter_timeline_clone(timeline: *mut ClutterTimeline) -> *mut ClutterTimeline;
    pub fn clutter_timeline_get_auto_reverse(timeline: *mut ClutterTimeline) -> gboolean;
    pub fn clutter_timeline_get_cubic_bezier_progress(
        timeline: *mut ClutterTimeline,
        c_1: *mut ClutterPoint,
        c_2: *mut ClutterPoint,
    ) -> gboolean;
    pub fn clutter_timeline_get_current_repeat(timeline: *mut ClutterTimeline) -> c_int;
    pub fn clutter_timeline_get_delay(timeline: *mut ClutterTimeline) -> c_uint;
    pub fn clutter_timeline_get_delta(timeline: *mut ClutterTimeline) -> c_uint;
    pub fn clutter_timeline_get_direction(
        timeline: *mut ClutterTimeline,
    ) -> ClutterTimelineDirection;
    pub fn clutter_timeline_get_duration(timeline: *mut ClutterTimeline) -> c_uint;
    pub fn clutter_timeline_get_duration_hint(timeline: *mut ClutterTimeline) -> i64;
    pub fn clutter_timeline_get_elapsed_time(timeline: *mut ClutterTimeline) -> c_uint;
    pub fn clutter_timeline_get_loop(timeline: *mut ClutterTimeline) -> gboolean;
    pub fn clutter_timeline_get_progress(timeline: *mut ClutterTimeline) -> c_double;
    pub fn clutter_timeline_get_progress_mode(
        timeline: *mut ClutterTimeline,
    ) -> ClutterAnimationMode;
    pub fn clutter_timeline_get_repeat_count(timeline: *mut ClutterTimeline) -> c_int;
    pub fn clutter_timeline_get_step_progress(
        timeline: *mut ClutterTimeline,
        n_steps: *mut c_int,
        step_mode: *mut ClutterStepMode,
    ) -> gboolean;
    pub fn clutter_timeline_has_marker(
        timeline: *mut ClutterTimeline,
        marker_name: *const c_char,
    ) -> gboolean;
    pub fn clutter_timeline_is_playing(timeline: *mut ClutterTimeline) -> gboolean;
    pub fn clutter_timeline_list_markers(
        timeline: *mut ClutterTimeline,
        msecs: c_int,
        n_markers: *mut size_t,
    ) -> *mut *mut c_char;
    pub fn clutter_timeline_pause(timeline: *mut ClutterTimeline);
    pub fn clutter_timeline_remove_marker(
        timeline: *mut ClutterTimeline,
        marker_name: *const c_char,
    );
    pub fn clutter_timeline_rewind(timeline: *mut ClutterTimeline);
    pub fn clutter_timeline_set_auto_reverse(timeline: *mut ClutterTimeline, reverse: gboolean);
    pub fn clutter_timeline_set_cubic_bezier_progress(
        timeline: *mut ClutterTimeline,
        c_1: *const ClutterPoint,
        c_2: *const ClutterPoint,
    );
    pub fn clutter_timeline_set_delay(timeline: *mut ClutterTimeline, msecs: c_uint);
    pub fn clutter_timeline_set_direction(
        timeline: *mut ClutterTimeline,
        direction: ClutterTimelineDirection,
    );
    pub fn clutter_timeline_set_duration(timeline: *mut ClutterTimeline, msecs: c_uint);
    pub fn clutter_timeline_set_loop(timeline: *mut ClutterTimeline, loop_: gboolean);
    pub fn clutter_timeline_set_progress_func(
        timeline: *mut ClutterTimeline,
        func: ClutterTimelineProgressFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
    );
    pub fn clutter_timeline_set_progress_mode(
        timeline: *mut ClutterTimeline,
        mode: ClutterAnimationMode,
    );
    pub fn clutter_timeline_set_repeat_count(timeline: *mut ClutterTimeline, count: c_int);
    pub fn clutter_timeline_set_step_progress(
        timeline: *mut ClutterTimeline,
        n_steps: c_int,
        step_mode: ClutterStepMode,
    );
    pub fn clutter_timeline_skip(timeline: *mut ClutterTimeline, msecs: c_uint);
    pub fn clutter_timeline_start(timeline: *mut ClutterTimeline);
    pub fn clutter_timeline_stop(timeline: *mut ClutterTimeline);

    //=========================================================================
    // ClutterTransition
    //=========================================================================
    pub fn clutter_transition_get_type() -> GType;
    pub fn clutter_transition_get_animatable(
        transition: *mut ClutterTransition,
    ) -> *mut ClutterAnimatable;
    pub fn clutter_transition_get_interval(
        transition: *mut ClutterTransition,
    ) -> *mut ClutterInterval;
    pub fn clutter_transition_get_remove_on_complete(
        transition: *mut ClutterTransition,
    ) -> gboolean;
    pub fn clutter_transition_set_animatable(
        transition: *mut ClutterTransition,
        animatable: *mut ClutterAnimatable,
    );
    pub fn clutter_transition_set_from(transition: *mut ClutterTransition, value_type: GType, ...);
    pub fn clutter_transition_set_from_value(
        transition: *mut ClutterTransition,
        value: *const gobject::GValue,
    );
    pub fn clutter_transition_set_interval(
        transition: *mut ClutterTransition,
        interval: *mut ClutterInterval,
    );
    pub fn clutter_transition_set_remove_on_complete(
        transition: *mut ClutterTransition,
        remove_complete: gboolean,
    );
    pub fn clutter_transition_set_to(transition: *mut ClutterTransition, value_type: GType, ...);
    pub fn clutter_transition_set_to_value(
        transition: *mut ClutterTransition,
        value: *const gobject::GValue,
    );

    //=========================================================================
    // ClutterTransitionGroup
    //=========================================================================
    pub fn clutter_transition_group_get_type() -> GType;
    pub fn clutter_transition_group_new() -> *mut ClutterTransition;
    pub fn clutter_transition_group_add_transition(
        group: *mut ClutterTransitionGroup,
        transition: *mut ClutterTransition,
    );
    pub fn clutter_transition_group_remove_all(group: *mut ClutterTransitionGroup);
    pub fn clutter_transition_group_remove_transition(
        group: *mut ClutterTransitionGroup,
        transition: *mut ClutterTransition,
    );

    //=========================================================================
    // ClutterZoomAction
    //=========================================================================
    pub fn clutter_zoom_action_get_type() -> GType;
    pub fn clutter_zoom_action_new() -> *mut ClutterAction;
    pub fn clutter_zoom_action_get_focal_point(
        action: *mut ClutterZoomAction,
        point: *mut ClutterPoint,
    );
    pub fn clutter_zoom_action_get_transformed_focal_point(
        action: *mut ClutterZoomAction,
        point: *mut ClutterPoint,
    );
    pub fn clutter_zoom_action_get_zoom_axis(action: *mut ClutterZoomAction) -> ClutterZoomAxis;
    pub fn clutter_zoom_action_set_zoom_axis(action: *mut ClutterZoomAction, axis: ClutterZoomAxis);

    //=========================================================================
    // ClutterAnimatable
    //=========================================================================
    pub fn clutter_animatable_get_type() -> GType;
    pub fn clutter_animatable_animate_property(
        animatable: *mut ClutterAnimatable,
        animation: *mut ClutterAnimation,
        property_name: *const c_char,
        initial_value: *const gobject::GValue,
        final_value: *const gobject::GValue,
        progress: c_double,
        value: *mut gobject::GValue,
    ) -> gboolean;
    pub fn clutter_animatable_find_property(
        animatable: *mut ClutterAnimatable,
        property_name: *const c_char,
    ) -> *mut gobject::GParamSpec;
    pub fn clutter_animatable_get_initial_state(
        animatable: *mut ClutterAnimatable,
        property_name: *const c_char,
        value: *mut gobject::GValue,
    );
    pub fn clutter_animatable_interpolate_value(
        animatable: *mut ClutterAnimatable,
        property_name: *const c_char,
        interval: *mut ClutterInterval,
        progress: c_double,
        value: *mut gobject::GValue,
    ) -> gboolean;
    pub fn clutter_animatable_set_final_state(
        animatable: *mut ClutterAnimatable,
        property_name: *const c_char,
        value: *const gobject::GValue,
    );

    //=========================================================================
    // ClutterContainer
    //=========================================================================
    pub fn clutter_container_get_type() -> GType;
    pub fn clutter_container_class_find_child_property(
        klass: *mut gobject::GObjectClass,
        property_name: *const c_char,
    ) -> *mut gobject::GParamSpec;
    pub fn clutter_container_class_list_child_properties(
        klass: *mut gobject::GObjectClass,
        n_properties: *mut c_uint,
    ) -> *mut *mut gobject::GParamSpec;
    pub fn clutter_container_add(
        container: *mut ClutterContainer,
        first_actor: *mut ClutterActor,
        ...
    );
    pub fn clutter_container_add_actor(container: *mut ClutterContainer, actor: *mut ClutterActor);
    //pub fn clutter_container_add_valist(container: *mut ClutterContainer, first_actor: *mut ClutterActor, var_args: /*Unimplemented*/va_list);
    pub fn clutter_container_child_get(
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
        first_prop: *const c_char,
        ...
    );
    pub fn clutter_container_child_get_property(
        container: *mut ClutterContainer,
        child: *mut ClutterActor,
        property: *const c_char,
        value: *mut gobject::GValue,
    );
    pub fn clutter_container_child_notify(
        container: *mut ClutterContainer,
        child: *mut ClutterActor,
        pspec: *mut gobject::GParamSpec,
    );
    pub fn clutter_container_child_set(
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
        first_prop: *const c_char,
        ...
    );
    pub fn clutter_container_child_set_property(
        container: *mut ClutterContainer,
        child: *mut ClutterActor,
        property: *const c_char,
        value: *const gobject::GValue,
    );
    pub fn clutter_container_create_child_meta(
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
    );
    pub fn clutter_container_destroy_child_meta(
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
    );
    pub fn clutter_container_find_child_by_name(
        container: *mut ClutterContainer,
        child_name: *const c_char,
    ) -> *mut ClutterActor;
    pub fn clutter_container_foreach(
        container: *mut ClutterContainer,
        callback: ClutterCallback,
        user_data: gpointer,
    );
    pub fn clutter_container_foreach_with_internals(
        container: *mut ClutterContainer,
        callback: ClutterCallback,
        user_data: gpointer,
    );
    pub fn clutter_container_get_child_meta(
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
    ) -> *mut ClutterChildMeta;
    pub fn clutter_container_get_children(container: *mut ClutterContainer) -> *mut glib::GList;
    pub fn clutter_container_lower_child(
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
        sibling: *mut ClutterActor,
    );
    pub fn clutter_container_raise_child(
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
        sibling: *mut ClutterActor,
    );
    pub fn clutter_container_remove(
        container: *mut ClutterContainer,
        first_actor: *mut ClutterActor,
        ...
    );
    pub fn clutter_container_remove_actor(
        container: *mut ClutterContainer,
        actor: *mut ClutterActor,
    );
    //pub fn clutter_container_remove_valist(container: *mut ClutterContainer, first_actor: *mut ClutterActor, var_args: /*Unimplemented*/va_list);
    pub fn clutter_container_sort_depth_order(container: *mut ClutterContainer);

    //=========================================================================
    // ClutterContent
    //=========================================================================
    pub fn clutter_content_get_type() -> GType;
    pub fn clutter_content_get_preferred_size(
        content: *mut ClutterContent,
        width: *mut c_float,
        height: *mut c_float,
    ) -> gboolean;
    pub fn clutter_content_invalidate(content: *mut ClutterContent);

    //=========================================================================
    // ClutterMedia
    //=========================================================================
    pub fn clutter_media_get_type() -> GType;
    pub fn clutter_media_get_audio_volume(media: *mut ClutterMedia) -> c_double;
    pub fn clutter_media_get_buffer_fill(media: *mut ClutterMedia) -> c_double;
    pub fn clutter_media_get_can_seek(media: *mut ClutterMedia) -> gboolean;
    pub fn clutter_media_get_duration(media: *mut ClutterMedia) -> c_double;
    pub fn clutter_media_get_playing(media: *mut ClutterMedia) -> gboolean;
    pub fn clutter_media_get_progress(media: *mut ClutterMedia) -> c_double;
    pub fn clutter_media_get_subtitle_font_name(media: *mut ClutterMedia) -> *mut c_char;
    pub fn clutter_media_get_subtitle_uri(media: *mut ClutterMedia) -> *mut c_char;
    pub fn clutter_media_get_uri(media: *mut ClutterMedia) -> *mut c_char;
    pub fn clutter_media_set_audio_volume(media: *mut ClutterMedia, volume: c_double);
    pub fn clutter_media_set_filename(media: *mut ClutterMedia, filename: *const c_char);
    pub fn clutter_media_set_playing(media: *mut ClutterMedia, playing: gboolean);
    pub fn clutter_media_set_progress(media: *mut ClutterMedia, progress: c_double);
    pub fn clutter_media_set_subtitle_font_name(media: *mut ClutterMedia, font_name: *const c_char);
    pub fn clutter_media_set_subtitle_uri(media: *mut ClutterMedia, uri: *const c_char);
    pub fn clutter_media_set_uri(media: *mut ClutterMedia, uri: *const c_char);

    //=========================================================================
    // ClutterScriptable
    //=========================================================================
    // pub fn clutter_scriptable_get_type() -> GType;
    // pub fn clutter_scriptable_get_id(scriptable: *mut ClutterScriptable) -> *const c_char;
    // pub fn clutter_scriptable_parse_custom_node(
    //     scriptable: *mut ClutterScriptable,
    //     script: *mut ClutterScript,
    //     value: *mut gobject::GValue,
    //     name: *const c_char,
    //     node: *mut json::JsonNode,
    // ) -> gboolean;
    // pub fn clutter_scriptable_set_custom_property(
    //     scriptable: *mut ClutterScriptable,
    //     script: *mut ClutterScript,
    //     name: *const c_char,
    //     value: *const gobject::GValue,
    // );
    // pub fn clutter_scriptable_set_id(scriptable: *mut ClutterScriptable, id_: *const c_char);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn clutter_base_init();
    pub fn clutter_cairo_clear(cr: *mut cairo::cairo_t);
    pub fn clutter_cairo_set_source_color(cr: *mut cairo::cairo_t, color: *const ClutterColor);
    pub fn clutter_check_version(major: c_uint, minor: c_uint, micro: c_uint) -> gboolean;
    pub fn clutter_check_windowing_backend(backend_type: *const c_char) -> gboolean;
    pub fn clutter_clear_glyph_cache();
    pub fn clutter_disable_accessibility();
    pub fn clutter_do_event(event: *mut ClutterEvent);
    pub fn clutter_events_pending() -> gboolean;
    pub fn clutter_feature_available(feature: ClutterFeatureFlags) -> gboolean;
    pub fn clutter_feature_get_all() -> ClutterFeatureFlags;
    pub fn clutter_frame_source_add(fps: c_uint, func: glib::GSourceFunc, data: gpointer)
        -> c_uint;
    pub fn clutter_frame_source_add_full(
        priority: c_int,
        fps: c_uint,
        func: glib::GSourceFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
    ) -> c_uint;
    pub fn clutter_get_accessibility_enabled() -> gboolean;
    pub fn clutter_get_actor_by_gid(id_: u32) -> *mut ClutterActor;
    pub fn clutter_get_current_event() -> *const ClutterEvent;
    pub fn clutter_get_current_event_time() -> u32;
    pub fn clutter_get_debug_enabled() -> gboolean;
    pub fn clutter_get_default_backend() -> *mut ClutterBackend;
    pub fn clutter_get_default_frame_rate() -> c_uint;
    pub fn clutter_get_default_text_direction() -> ClutterTextDirection;
    pub fn clutter_get_font_flags() -> ClutterFontFlags;
    pub fn clutter_get_font_map() -> *mut pango::PangoFontMap;
    pub fn clutter_get_input_device_for_id(id_: c_int) -> *mut ClutterInputDevice;
    pub fn clutter_get_keyboard_grab() -> *mut ClutterActor;
    pub fn clutter_get_motion_events_enabled() -> gboolean;
    pub fn clutter_get_option_group() -> *mut glib::GOptionGroup;
    pub fn clutter_get_option_group_without_init() -> *mut glib::GOptionGroup;
    pub fn clutter_get_pointer_grab() -> *mut ClutterActor;
    pub fn clutter_get_script_id(gobject: *mut gobject::GObject) -> *const c_char;
    pub fn clutter_get_show_fps() -> gboolean;
    pub fn clutter_get_timestamp() -> c_ulong;
    pub fn clutter_grab_keyboard(actor: *mut ClutterActor);
    pub fn clutter_grab_pointer(actor: *mut ClutterActor);
    pub fn clutter_grab_pointer_for_device(actor: *mut ClutterActor, id_: c_int);
    pub fn clutter_init(argc: *mut c_int, argv: *mut *mut *mut c_char) -> ClutterInitError;
    pub fn clutter_init_with_args(
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        parameter_string: *const c_char,
        entries: *mut glib::GOptionEntry,
        translation_domain: *const c_char,
        error: *mut *mut glib::GError,
    ) -> ClutterInitError;
    pub fn clutter_keysym_to_unicode(keyval: c_uint) -> u32;
    pub fn clutter_main();
    pub fn clutter_main_level() -> c_int;
    pub fn clutter_main_quit();
    pub fn clutter_param_spec_color(
        name: *const c_char,
        nick: *const c_char,
        blurb: *const c_char,
        default_value: *const ClutterColor,
        flags: gobject::GParamFlags,
    ) -> *mut gobject::GParamSpec;
    pub fn clutter_param_spec_fixed(
        name: *const c_char,
        nick: *const c_char,
        blurb: *const c_char,
        minimum: cogl::CoglFixed,
        maximum: cogl::CoglFixed,
        default_value: cogl::CoglFixed,
        flags: gobject::GParamFlags,
    ) -> *mut gobject::GParamSpec;
    pub fn clutter_param_spec_units(
        name: *const c_char,
        nick: *const c_char,
        blurb: *const c_char,
        default_type: ClutterUnitType,
        minimum: c_float,
        maximum: c_float,
        default_value: c_float,
        flags: gobject::GParamFlags,
    ) -> *mut gobject::GParamSpec;
    pub fn clutter_redraw(stage: *mut ClutterStage);
    pub fn clutter_set_default_frame_rate(frames_per_sec: c_uint);
    pub fn clutter_set_font_flags(flags: ClutterFontFlags);
    pub fn clutter_set_motion_events_enabled(enable: gboolean);
    pub fn clutter_set_windowing_backend(backend_type: *const c_char);
    pub fn clutter_test_add(test_path: *const c_char, test_func: glib::GTestFunc);
    pub fn clutter_test_add_data(
        test_path: *const c_char,
        test_func: glib::GTestDataFunc,
        test_data: gpointer,
    );
    pub fn clutter_test_add_data_full(
        test_path: *const c_char,
        test_func: glib::GTestDataFunc,
        test_data: gpointer,
        test_notify: glib::GDestroyNotify,
    );
    pub fn clutter_test_check_actor_at_point(
        stage: *mut ClutterActor,
        point: *const ClutterPoint,
        actor: *mut ClutterActor,
        result: *mut *mut ClutterActor,
    ) -> gboolean;
    pub fn clutter_test_check_color_at_point(
        stage: *mut ClutterActor,
        point: *const ClutterPoint,
        color: *const ClutterColor,
        result: *mut ClutterColor,
    ) -> gboolean;
    pub fn clutter_test_get_stage() -> *mut ClutterActor;
    pub fn clutter_test_init(argc: *mut c_int, argv: *mut *mut *mut c_char);
    pub fn clutter_test_run() -> c_int;
    pub fn clutter_threads_add_frame_source(
        fps: c_uint,
        func: glib::GSourceFunc,
        data: gpointer,
    ) -> c_uint;
    pub fn clutter_threads_add_frame_source_full(
        priority: c_int,
        fps: c_uint,
        func: glib::GSourceFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
    ) -> c_uint;
    pub fn clutter_threads_add_idle(func: glib::GSourceFunc, data: gpointer) -> c_uint;
    pub fn clutter_threads_add_idle_full(
        priority: c_int,
        func: glib::GSourceFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
    ) -> c_uint;
    pub fn clutter_threads_add_repaint_func(
        func: glib::GSourceFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
    ) -> c_uint;
    pub fn clutter_threads_add_repaint_func_full(
        flags: ClutterRepaintFlags,
        func: glib::GSourceFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
    ) -> c_uint;
    pub fn clutter_threads_add_timeout(
        interval: c_uint,
        func: glib::GSourceFunc,
        data: gpointer,
    ) -> c_uint;
    pub fn clutter_threads_add_timeout_full(
        priority: c_int,
        interval: c_uint,
        func: glib::GSourceFunc,
        data: gpointer,
        notify: glib::GDestroyNotify,
    ) -> c_uint;
    pub fn clutter_threads_enter();
    pub fn clutter_threads_init();
    pub fn clutter_threads_leave();
    pub fn clutter_threads_remove_repaint_func(handle_id: c_uint);
    pub fn clutter_threads_set_lock_functions(
        enter_fn: gobject::GCallback,
        leave_fn: gobject::GCallback,
    );
    pub fn clutter_ungrab_keyboard();
    pub fn clutter_ungrab_pointer();
    pub fn clutter_ungrab_pointer_for_device(id_: c_int);
    pub fn clutter_unicode_to_keysym(wc: u32) -> c_uint;
    pub fn clutter_util_next_p2(a: c_int) -> c_int;
    pub fn clutter_value_dup_paint_node(value: *const gobject::GValue) -> *mut ClutterPaintNode;
    pub fn clutter_value_get_color(value: *const gobject::GValue) -> *const ClutterColor;
    pub fn clutter_value_get_fixed(value: *const gobject::GValue) -> cogl::CoglFixed;
    pub fn clutter_value_get_paint_node(value: *const gobject::GValue) -> *mut ClutterPaintNode;
    pub fn clutter_value_get_shader_float(
        value: *const gobject::GValue,
        length: *mut size_t,
    ) -> *const c_float;
    pub fn clutter_value_get_shader_int(
        value: *const gobject::GValue,
        length: *mut size_t,
    ) -> *const c_int;
    pub fn clutter_value_get_shader_matrix(
        value: *const gobject::GValue,
        length: *mut size_t,
    ) -> *const c_float;
    pub fn clutter_value_get_units(value: *const gobject::GValue) -> *const ClutterUnits;
    pub fn clutter_value_set_color(value: *mut gobject::GValue, color: *const ClutterColor);
    pub fn clutter_value_set_fixed(value: *mut gobject::GValue, fixed_: cogl::CoglFixed);
    pub fn clutter_value_set_paint_node(value: *mut gobject::GValue, node: *mut ClutterPaintNode);
    pub fn clutter_value_set_shader_float(
        value: *mut gobject::GValue,
        size: c_int,
        floats: *const c_float,
    );
    pub fn clutter_value_set_shader_int(
        value: *mut gobject::GValue,
        size: c_int,
        ints: *const c_int,
    );
    pub fn clutter_value_set_shader_matrix(
        value: *mut gobject::GValue,
        size: c_int,
        matrix: *const c_float,
    );
    pub fn clutter_value_set_units(value: *mut gobject::GValue, units: *const ClutterUnits);
    pub fn clutter_value_take_paint_node(value: *mut gobject::GValue, node: *mut ClutterPaintNode);

}
