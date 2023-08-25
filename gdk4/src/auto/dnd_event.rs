// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Drop;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GdkDNDEvent")]
    pub struct DNDEvent(Shared<ffi::GdkDNDEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

impl glib::StaticType for DNDEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_dnd_event_get_type()) }
    }
}

impl DNDEvent {
    #[doc(alias = "gdk_dnd_event_get_drop")]
    #[doc(alias = "get_drop")]
    pub fn drop(&self) -> Option<Drop> {
        unsafe { from_glib_none(ffi::gdk_dnd_event_get_drop(self.to_glib_none().0)) }
    }
}
