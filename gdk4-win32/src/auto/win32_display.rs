// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Win32HCursor;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GdkWin32Display")]
    pub struct Win32Display(Object<ffi::GdkWin32Display, ffi::GdkWin32DisplayClass>) @extends gdk::Display;

    match fn {
        type_ => || ffi::gdk_win32_display_get_type(),
    }
}

impl Win32Display {
    #[doc(alias = "gdk_win32_display_get_win32hcursor")]
    #[doc(alias = "get_win32hcursor")]
    pub fn win32hcursor(&self, cursor: &gdk::Cursor) -> Win32HCursor {
        unsafe {
            from_glib_none(ffi::gdk_win32_display_get_win32hcursor(
                self.to_glib_none().0,
                cursor.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_win32_display_set_cursor_theme")]
    pub fn set_cursor_theme(&self, name: Option<&str>, size: i32) {
        unsafe {
            ffi::gdk_win32_display_set_cursor_theme(
                self.to_glib_none().0,
                name.to_glib_none().0,
                size,
            );
        }
    }

    #[doc(alias = "gdk_win32_display_get_primary_monitor")]
    #[doc(alias = "get_primary_monitor")]
    pub fn primary_monitor(display: &impl IsA<gdk::Display>) -> gdk::Monitor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_win32_display_get_primary_monitor(
                display.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_win32_display_get_wgl_version")]
    #[doc(alias = "get_wgl_version")]
    pub fn wgl_version(display: &impl IsA<gdk::Display>) -> Option<(i32, i32)> {
        assert_initialized_main_thread!();
        unsafe {
            let mut major = std::mem::MaybeUninit::uninit();
            let mut minor = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_win32_display_get_wgl_version(
                display.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            ));
            if ret {
                Some((major.assume_init(), minor.assume_init()))
            } else {
                None
            }
        }
    }
}
