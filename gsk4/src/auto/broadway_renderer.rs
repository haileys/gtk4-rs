// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Renderer;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GskBroadwayRenderer")]
    pub struct BroadwayRenderer(Object<ffi::GskBroadwayRenderer, ffi::GskBroadwayRendererClass>) @extends Renderer;

    match fn {
        type_ => || ffi::gsk_broadway_renderer_get_type(),
    }
}

impl BroadwayRenderer {
    #[doc(alias = "gsk_broadway_renderer_new")]
    pub fn new() -> BroadwayRenderer {
        assert_initialized_main_thread!();
        unsafe { Renderer::from_glib_full(ffi::gsk_broadway_renderer_new()).unsafe_cast() }
    }
}

impl Default for BroadwayRenderer {
    fn default() -> Self {
        Self::new()
    }
}
