use crate::{Color, PaintNode, PipelineNode, ScalingFilter};
use glib::{
    object::{Cast, IsA},
    translate::*,
};
use std::fmt;

glib_wrapper! {
    pub struct TextureNode(Object<ffi::ClutterTextureNode, ffi::ClutterTextureNodeClass, TextureNodeClass>) @extends PipelineNode, PaintNode;

    match fn {
        get_type => || ffi::clutter_texture_node_get_type(),
    }
}

impl TextureNode {
    /// Creates a new `PaintNode` that will paint the passed `texture`.
    ///
    /// This function will take a reference on `texture`, so it is safe to
    /// call `cogl_object_unref` on `texture` when it returns.
    ///
    /// The `color` must not be pre-multiplied with its `Color.alpha`
    /// channel value; if `color` is `None`, a fully opaque white color will
    /// be used for blending.
    /// ## `texture`
    /// a `cogl::Texture`
    /// ## `color`
    /// a `Color` used for blending, or `None`
    /// ## `min_filter`
    /// the minification filter for the texture
    /// ## `mag_filter`
    /// the magnification filter for the texture
    ///
    /// # Returns
    ///
    /// the newly created `PaintNode`.
    ///  Use `PaintNodeExt::unref` when done
    pub fn new<P: IsA<cogl::Texture>>(
        texture: &P,
        color: Option<&Color>,
        min_filter: ScalingFilter,
        mag_filter: ScalingFilter,
    ) -> TextureNode {
        unsafe {
            PaintNode::from_glib_full(ffi::clutter_texture_node_new(
                texture.as_ref().to_glib_none().0,
                color.to_glib_none().0,
                min_filter.to_glib(),
                mag_filter.to_glib(),
            ))
            .unsafe_cast()
        }
    }
}

impl fmt::Display for TextureNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureNode")
    }
}
