// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Path, RenderNode, Stroke};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GskStrokeNode")]
    pub struct StrokeNode(Shared<ffi::GskStrokeNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for StrokeNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_stroke_node_get_type()) }
    }
}

impl StrokeNode {
    #[doc(alias = "gsk_stroke_node_new")]
    pub fn new(child: impl AsRef<RenderNode>, path: &Path, stroke: &Stroke) -> StrokeNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gsk_stroke_node_new(
                child.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                stroke.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_stroke_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> RenderNode {
        unsafe { from_glib_none(ffi::gsk_stroke_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_stroke_node_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&self) -> Path {
        unsafe { from_glib_none(ffi::gsk_stroke_node_get_path(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_stroke_node_get_stroke")]
    #[doc(alias = "get_stroke")]
    pub fn stroke(&self) -> Stroke {
        unsafe { from_glib_none(ffi::gsk_stroke_node_get_stroke(self.to_glib_none().0)) }
    }
}
