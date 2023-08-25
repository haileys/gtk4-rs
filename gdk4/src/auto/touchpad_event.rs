// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TouchpadGesturePhase;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GdkTouchpadEvent")]
    pub struct TouchpadEvent(Shared<ffi::GdkTouchpadEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

impl glib::StaticType for TouchpadEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_touchpad_event_get_type()) }
    }
}

impl TouchpadEvent {
    #[doc(alias = "gdk_touchpad_event_get_deltas")]
    #[doc(alias = "get_deltas")]
    pub fn deltas(&self) -> (f64, f64) {
        unsafe {
            let mut dx = std::mem::MaybeUninit::uninit();
            let mut dy = std::mem::MaybeUninit::uninit();
            ffi::gdk_touchpad_event_get_deltas(
                self.to_glib_none().0,
                dx.as_mut_ptr(),
                dy.as_mut_ptr(),
            );
            (dx.assume_init(), dy.assume_init())
        }
    }

    #[doc(alias = "gdk_touchpad_event_get_gesture_phase")]
    #[doc(alias = "get_gesture_phase")]
    pub fn gesture_phase(&self) -> TouchpadGesturePhase {
        unsafe {
            from_glib(ffi::gdk_touchpad_event_get_gesture_phase(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_touchpad_event_get_n_fingers")]
    #[doc(alias = "get_n_fingers")]
    pub fn n_fingers(&self) -> u32 {
        unsafe { ffi::gdk_touchpad_event_get_n_fingers(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_touchpad_event_get_pinch_angle_delta")]
    #[doc(alias = "get_pinch_angle_delta")]
    pub fn pinch_angle_delta(&self) -> f64 {
        unsafe { ffi::gdk_touchpad_event_get_pinch_angle_delta(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_touchpad_event_get_pinch_scale")]
    #[doc(alias = "get_pinch_scale")]
    pub fn pinch_scale(&self) -> f64 {
        unsafe { ffi::gdk_touchpad_event_get_pinch_scale(self.to_glib_none().0) }
    }
}
