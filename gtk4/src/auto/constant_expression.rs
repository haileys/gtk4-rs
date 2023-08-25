// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GtkConstantExpression")]
    pub struct ConstantExpression(Shared<ffi::GtkConstantExpression>);

    match fn {
        ref => |ptr| ffi::gtk_expression_ref(ptr as *mut ffi::GtkExpression),
        unref => |ptr| ffi::gtk_expression_unref(ptr as *mut ffi::GtkExpression),
    }
}

impl glib::StaticType for ConstantExpression {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_constant_expression_get_type()) }
    }
}

impl ConstantExpression {
    #[doc(alias = "gtk_constant_expression_new_for_value")]
    #[doc(alias = "new_for_value")]
    pub fn for_value(value: &glib::Value) -> ConstantExpression {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constant_expression_new_for_value(
                value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constant_expression_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self) -> glib::Value {
        unsafe {
            from_glib_none(ffi::gtk_constant_expression_get_value(
                self.to_glib_none().0,
            ))
        }
    }
}
