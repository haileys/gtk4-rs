// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Buildable, Filter, MultiFilter};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GtkAnyFilter")]
    pub struct AnyFilter(Object<ffi::GtkAnyFilter, ffi::GtkAnyFilterClass>) @extends MultiFilter, Filter, @implements gio::ListModel, Buildable;

    match fn {
        type_ => || ffi::gtk_any_filter_get_type(),
    }
}

impl AnyFilter {
    #[doc(alias = "gtk_any_filter_new")]
    pub fn new() -> AnyFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_any_filter_new()) }
    }
}

impl Default for AnyFilter {
    fn default() -> Self {
        Self::new()
    }
}
