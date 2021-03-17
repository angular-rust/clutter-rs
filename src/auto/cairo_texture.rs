// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use std::fmt;
use super::Actor;
use super::Animatable;
use super::Container;
use super::Texture;

glib_wrapper! {
    pub struct CairoTexture(Object<ffi::ClutterCairoTexture, ffi::ClutterCairoTextureClass, CairoTextureClass>) @extends Texture, Actor, @implements Animatable, Container;

    match fn {
        get_type => || ffi::clutter_cairo_texture_get_type(),
    }
}

impl CairoTexture {}

pub const NONE_CAIRO_TEXTURE: Option<&CairoTexture> = None;

impl fmt::Display for CairoTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CairoTexture")
    }
}