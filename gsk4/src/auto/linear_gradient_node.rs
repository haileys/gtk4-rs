// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ColorStop;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GskLinearGradientNode")]
    pub struct LinearGradientNode(Shared<ffi::GskLinearGradientNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for LinearGradientNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_linear_gradient_node_get_type()) }
    }
}

impl LinearGradientNode {
    #[doc(alias = "gsk_linear_gradient_node_new")]
    pub fn new(
        bounds: &graphene::Rect,
        start: &graphene::Point,
        end: &graphene::Point,
        color_stops: &[ColorStop],
    ) -> LinearGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as _;
        unsafe {
            from_glib_full(ffi::gsk_linear_gradient_node_new(
                bounds.to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_color_stops")]
    #[doc(alias = "get_color_stops")]
    pub fn color_stops(&self) -> Vec<ColorStop> {
        unsafe {
            let mut n_stops = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gsk_linear_gradient_node_get_color_stops(
                    self.to_glib_none().0,
                    n_stops.as_mut_ptr(),
                ),
                n_stops.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_end")]
    #[doc(alias = "get_end")]
    pub fn end(&self) -> graphene::Point {
        unsafe { from_glib_none(ffi::gsk_linear_gradient_node_get_end(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_n_color_stops")]
    #[doc(alias = "get_n_color_stops")]
    pub fn n_color_stops(&self) -> usize {
        unsafe { ffi::gsk_linear_gradient_node_get_n_color_stops(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_start")]
    #[doc(alias = "get_start")]
    pub fn start(&self) -> graphene::Point {
        unsafe {
            from_glib_none(ffi::gsk_linear_gradient_node_get_start(
                self.to_glib_none().0,
            ))
        }
    }
}
