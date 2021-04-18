use crate::{ActorMeta, Effect, Rect};
use glib::{object as gobject, object::IsA, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct OffscreenEffect(Object<ffi::ClutterOffscreenEffect, ffi::ClutterOffscreenEffectClass, OffscreenEffectClass>) @extends Effect, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_offscreen_effect_get_type(),
    }
}

pub const NONE_OFFSCREEN_EFFECT: Option<&OffscreenEffect> = None;

/// Trait containing all `OffscreenEffect` methods.
///
/// # Implementors
///
/// [`BlurEffect`](struct.BlurEffect.html), [`BrightnessContrastEffect`](struct.BrightnessContrastEffect.html), [`ColorizeEffect`](struct.ColorizeEffect.html), [`DeformEffect`](struct.DeformEffect.html), [`DesaturateEffect`](struct.DesaturateEffect.html), [`OffscreenEffect`](struct.OffscreenEffect.html), [`ShaderEffect`](struct.ShaderEffect.html)
pub trait OffscreenEffectExt: 'static {
    //fn create_texture(&self, width: f32, height: f32) -> /*Unimplemented*/Option<cogl::Handle>;

    // /// Retrieves the material used as a render target for the offscreen
    // /// buffer created by `self`
    // ///
    // /// You should only use the returned `cogl::Material` when painting. The
    // /// returned material might change between different frames.
    // ///
    // /// # Returns
    // ///
    // /// a `cogl::Material` or `None`. The
    // ///  returned material is owned by Clutter and it should not be
    // ///  modified or freed

    // fn get_target(&self) -> Option<cogl::Material>;

    /// Retrieves the origin and size of the offscreen buffer used by `self` to
    /// paint the actor to which it has been applied.
    ///
    /// This function should only be called by `OffscreenEffect`
    /// implementations, from within the `OffscreenEffectClass.paint_target`()
    /// virtual function.
    /// ## `rect`
    /// return location for the target area
    ///
    /// # Returns
    ///
    /// `true` if the offscreen buffer has a valid rectangle,
    ///  and `false` otherwise
    fn get_target_rect(&self) -> Option<Rect>;

    //fn get_texture(&self) -> /*Unimplemented*/Option<cogl::Handle>;

    /// Calls the `paint_target` virtual function of the `self`
    fn paint_target(&self);
}

impl<O: IsA<OffscreenEffect>> OffscreenEffectExt for O {
    //fn create_texture(&self, width: f32, height: f32) -> /*Unimplemented*/Option<cogl::Handle> {
    //    unsafe { TODO: call clutter_sys:clutter_offscreen_effect_create_texture() }
    //}

    // fn get_target(&self) -> Option<cogl::Material> {
    //     unsafe {
    //         from_glib_none(ffi::clutter_offscreen_effect_get_target(
    //             self.as_ref().to_glib_none().0,
    //         ))
    //     }
    // }

    fn get_target_rect(&self) -> Option<Rect> {
        unsafe {
            let mut rect = Rect::uninitialized();
            let ret = from_glib(ffi::clutter_offscreen_effect_get_target_rect(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none_mut().0,
            ));
            if ret {
                Some(rect)
            } else {
                None
            }
        }
    }

    //fn get_texture(&self) -> /*Unimplemented*/Option<cogl::Handle> {
    //    unsafe { TODO: call clutter_sys:clutter_offscreen_effect_get_texture() }
    //}

    fn paint_target(&self) {
        unsafe {
            ffi::clutter_offscreen_effect_paint_target(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for OffscreenEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OffscreenEffect")
    }
}
