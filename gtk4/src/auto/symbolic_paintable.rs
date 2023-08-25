// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GtkSymbolicPaintable")]
    pub struct SymbolicPaintable(Interface<ffi::GtkSymbolicPaintable, ffi::GtkSymbolicPaintableInterface>) @requires gdk::Paintable;

    match fn {
        type_ => || ffi::gtk_symbolic_paintable_get_type(),
    }
}

impl SymbolicPaintable {
    pub const NONE: Option<&'static SymbolicPaintable> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::SymbolicPaintable>> Sealed for T {}
}

pub trait SymbolicPaintableExt: IsA<SymbolicPaintable> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_symbolic_paintable_snapshot_symbolic")]
    fn snapshot_symbolic(
        &self,
        snapshot: &impl IsA<gdk::Snapshot>,
        width: f64,
        height: f64,
        colors: &[gdk::RGBA],
    ) {
        let n_colors = colors.len() as _;
        unsafe {
            ffi::gtk_symbolic_paintable_snapshot_symbolic(
                self.as_ref().to_glib_none().0,
                snapshot.as_ref().to_glib_none().0,
                width,
                height,
                colors.to_glib_none().0,
                n_colors,
            );
        }
    }
}

impl<O: IsA<SymbolicPaintable>> SymbolicPaintableExt for O {}
