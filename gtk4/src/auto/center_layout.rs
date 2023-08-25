// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{BaselinePosition, LayoutManager, Orientation, Widget};
#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{prelude::*, translate::*};
#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkCenterLayout")]
    pub struct CenterLayout(Object<ffi::GtkCenterLayout, ffi::GtkCenterLayoutClass>) @extends LayoutManager;

    match fn {
        type_ => || ffi::gtk_center_layout_get_type(),
    }
}

impl CenterLayout {
    #[doc(alias = "gtk_center_layout_new")]
    pub fn new() -> CenterLayout {
        assert_initialized_main_thread!();
        unsafe { LayoutManager::from_glib_full(ffi::gtk_center_layout_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_center_layout_get_baseline_position")]
    #[doc(alias = "get_baseline_position")]
    pub fn baseline_position(&self) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_center_layout_get_baseline_position(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_center_layout_get_center_widget")]
    #[doc(alias = "get_center_widget")]
    pub fn center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_center_layout_get_center_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_center_layout_get_end_widget")]
    #[doc(alias = "get_end_widget")]
    pub fn end_widget(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_center_layout_get_end_widget(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_center_layout_get_orientation")]
    #[doc(alias = "get_orientation")]
    pub fn orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_center_layout_get_orientation(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_center_layout_get_shrink_center_last")]
    #[doc(alias = "get_shrink_center_last")]
    pub fn shrinks_center_last(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_center_layout_get_shrink_center_last(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_center_layout_get_start_widget")]
    #[doc(alias = "get_start_widget")]
    pub fn start_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_center_layout_get_start_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_center_layout_set_baseline_position")]
    pub fn set_baseline_position(&self, baseline_position: BaselinePosition) {
        unsafe {
            ffi::gtk_center_layout_set_baseline_position(
                self.to_glib_none().0,
                baseline_position.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_center_layout_set_center_widget")]
    pub fn set_center_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_center_layout_set_center_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_center_layout_set_end_widget")]
    pub fn set_end_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_center_layout_set_end_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_center_layout_set_orientation")]
    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_center_layout_set_orientation(self.to_glib_none().0, orientation.into_glib());
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_center_layout_set_shrink_center_last")]
    pub fn set_shrink_center_last(&self, shrink_center_last: bool) {
        unsafe {
            ffi::gtk_center_layout_set_shrink_center_last(
                self.to_glib_none().0,
                shrink_center_last.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_center_layout_set_start_widget")]
    pub fn set_start_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_center_layout_set_start_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "shrink-center-last")]
    pub fn connect_shrink_center_last_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shrink_center_last_trampoline<
            F: Fn(&CenterLayout) + 'static,
        >(
            this: *mut ffi::GtkCenterLayout,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shrink-center-last\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_shrink_center_last_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CenterLayout {
    fn default() -> Self {
        Self::new()
    }
}
