use glib::translate::*;
use std::fmt;
use crate::PaintNode;

glib_wrapper! {
    pub struct PipelineNode(Object<ffi::ClutterPipelineNode, ffi::ClutterPipelineNodeClass, PipelineNodeClass>) @extends PaintNode;

    match fn {
        get_type => || ffi::clutter_pipeline_node_get_type(),
    }
}

impl PipelineNode {}

pub const NONE_PIPELINE_NODE: Option<&PipelineNode> = None;

impl fmt::Display for PipelineNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PipelineNode")
    }
}
