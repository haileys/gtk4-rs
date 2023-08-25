// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Accessible, AccessibleRole, Align, Application, Buildable, ConstraintTarget, Dialog,
    LayoutManager, Native, Overflow, PageSetup, PrintCapabilities, PrintSettings, Printer, Root,
    ShortcutManager, Widget, Window,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkPrintUnixDialog")]
    pub struct PrintUnixDialog(Object<ffi::GtkPrintUnixDialog>) @extends Dialog, Window, Widget, @implements Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;

    match fn {
        type_ => || ffi::gtk_print_unix_dialog_get_type(),
    }
}

impl PrintUnixDialog {
    #[doc(alias = "gtk_print_unix_dialog_new")]
    pub fn new(title: Option<&str>, parent: Option<&impl IsA<Window>>) -> PrintUnixDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_print_unix_dialog_new(
                title.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PrintUnixDialog`] objects.
    ///
    /// This method returns an instance of [`PrintUnixDialogBuilder`](crate::builders::PrintUnixDialogBuilder) which can be used to create [`PrintUnixDialog`] objects.
    pub fn builder() -> PrintUnixDialogBuilder {
        PrintUnixDialogBuilder::new()
    }

    #[doc(alias = "gtk_print_unix_dialog_add_custom_tab")]
    pub fn add_custom_tab(&self, child: &impl IsA<Widget>, tab_label: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_print_unix_dialog_add_custom_tab(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_get_current_page")]
    #[doc(alias = "get_current_page")]
    pub fn current_page(&self) -> i32 {
        unsafe { ffi::gtk_print_unix_dialog_get_current_page(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_unix_dialog_get_embed_page_setup")]
    #[doc(alias = "get_embed_page_setup")]
    pub fn embeds_page_setup(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_unix_dialog_get_embed_page_setup(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_get_has_selection")]
    #[doc(alias = "get_has_selection")]
    pub fn has_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_unix_dialog_get_has_selection(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_get_manual_capabilities")]
    #[doc(alias = "get_manual_capabilities")]
    pub fn manual_capabilities(&self) -> PrintCapabilities {
        unsafe {
            from_glib(ffi::gtk_print_unix_dialog_get_manual_capabilities(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_get_page_setup")]
    #[doc(alias = "get_page_setup")]
    pub fn page_setup(&self) -> PageSetup {
        unsafe {
            from_glib_none(ffi::gtk_print_unix_dialog_get_page_setup(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_get_page_setup_set")]
    #[doc(alias = "get_page_setup_set")]
    pub fn is_page_setup_set(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_unix_dialog_get_page_setup_set(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_get_selected_printer")]
    #[doc(alias = "get_selected_printer")]
    pub fn selected_printer(&self) -> Option<Printer> {
        unsafe {
            from_glib_none(ffi::gtk_print_unix_dialog_get_selected_printer(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_get_settings")]
    #[doc(alias = "get_settings")]
    pub fn settings(&self) -> PrintSettings {
        unsafe {
            from_glib_full(ffi::gtk_print_unix_dialog_get_settings(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_get_support_selection")]
    #[doc(alias = "get_support_selection")]
    pub fn supports_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_unix_dialog_get_support_selection(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_set_current_page")]
    pub fn set_current_page(&self, current_page: i32) {
        unsafe {
            ffi::gtk_print_unix_dialog_set_current_page(self.to_glib_none().0, current_page);
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_set_embed_page_setup")]
    pub fn set_embed_page_setup(&self, embed: bool) {
        unsafe {
            ffi::gtk_print_unix_dialog_set_embed_page_setup(
                self.to_glib_none().0,
                embed.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_set_has_selection")]
    pub fn set_has_selection(&self, has_selection: bool) {
        unsafe {
            ffi::gtk_print_unix_dialog_set_has_selection(
                self.to_glib_none().0,
                has_selection.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_set_manual_capabilities")]
    pub fn set_manual_capabilities(&self, capabilities: PrintCapabilities) {
        unsafe {
            ffi::gtk_print_unix_dialog_set_manual_capabilities(
                self.to_glib_none().0,
                capabilities.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_set_page_setup")]
    pub fn set_page_setup(&self, page_setup: &PageSetup) {
        unsafe {
            ffi::gtk_print_unix_dialog_set_page_setup(
                self.to_glib_none().0,
                page_setup.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_set_settings")]
    pub fn set_settings(&self, settings: Option<&PrintSettings>) {
        unsafe {
            ffi::gtk_print_unix_dialog_set_settings(
                self.to_glib_none().0,
                settings.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_print_unix_dialog_set_support_selection")]
    pub fn set_support_selection(&self, support_selection: bool) {
        unsafe {
            ffi::gtk_print_unix_dialog_set_support_selection(
                self.to_glib_none().0,
                support_selection.into_glib(),
            );
        }
    }

    #[doc(alias = "print-settings")]
    pub fn print_settings(&self) -> Option<PrintSettings> {
        ObjectExt::property(self, "print-settings")
    }

    #[doc(alias = "print-settings")]
    pub fn set_print_settings(&self, print_settings: Option<&PrintSettings>) {
        ObjectExt::set_property(self, "print-settings", print_settings)
    }

    #[doc(alias = "current-page")]
    pub fn connect_current_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_page_trampoline<F: Fn(&PrintUnixDialog) + 'static>(
            this: *mut ffi::GtkPrintUnixDialog,
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
                b"notify::current-page\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_current_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "embed-page-setup")]
    pub fn connect_embed_page_setup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_embed_page_setup_trampoline<
            F: Fn(&PrintUnixDialog) + 'static,
        >(
            this: *mut ffi::GtkPrintUnixDialog,
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
                b"notify::embed-page-setup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_embed_page_setup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-selection")]
    pub fn connect_has_selection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_selection_trampoline<F: Fn(&PrintUnixDialog) + 'static>(
            this: *mut ffi::GtkPrintUnixDialog,
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
                b"notify::has-selection\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_has_selection_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "manual-capabilities")]
    pub fn connect_manual_capabilities_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_manual_capabilities_trampoline<
            F: Fn(&PrintUnixDialog) + 'static,
        >(
            this: *mut ffi::GtkPrintUnixDialog,
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
                b"notify::manual-capabilities\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_manual_capabilities_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-setup")]
    pub fn connect_page_setup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_setup_trampoline<F: Fn(&PrintUnixDialog) + 'static>(
            this: *mut ffi::GtkPrintUnixDialog,
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
                b"notify::page-setup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_page_setup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "print-settings")]
    pub fn connect_print_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_print_settings_trampoline<F: Fn(&PrintUnixDialog) + 'static>(
            this: *mut ffi::GtkPrintUnixDialog,
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
                b"notify::print-settings\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_print_settings_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected-printer")]
    pub fn connect_selected_printer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_printer_trampoline<
            F: Fn(&PrintUnixDialog) + 'static,
        >(
            this: *mut ffi::GtkPrintUnixDialog,
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
                b"notify::selected-printer\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_printer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "support-selection")]
    pub fn connect_support_selection_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_support_selection_trampoline<
            F: Fn(&PrintUnixDialog) + 'static,
        >(
            this: *mut ffi::GtkPrintUnixDialog,
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
                b"notify::support-selection\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_support_selection_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for PrintUnixDialog {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PrintUnixDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PrintUnixDialogBuilder {
    builder: glib::object::ObjectBuilder<'static, PrintUnixDialog>,
}

impl PrintUnixDialogBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn current_page(self, current_page: i32) -> Self {
        Self {
            builder: self.builder.property("current-page", current_page),
        }
    }

    pub fn embed_page_setup(self, embed_page_setup: bool) -> Self {
        Self {
            builder: self.builder.property("embed-page-setup", embed_page_setup),
        }
    }

    pub fn has_selection(self, has_selection: bool) -> Self {
        Self {
            builder: self.builder.property("has-selection", has_selection),
        }
    }

    pub fn manual_capabilities(self, manual_capabilities: PrintCapabilities) -> Self {
        Self {
            builder: self
                .builder
                .property("manual-capabilities", manual_capabilities),
        }
    }

    pub fn page_setup(self, page_setup: &PageSetup) -> Self {
        Self {
            builder: self.builder.property("page-setup", page_setup.clone()),
        }
    }

    pub fn print_settings(self, print_settings: &PrintSettings) -> Self {
        Self {
            builder: self
                .builder
                .property("print-settings", print_settings.clone()),
        }
    }

    pub fn support_selection(self, support_selection: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("support-selection", support_selection),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn use_header_bar(self, use_header_bar: i32) -> Self {
        Self {
            builder: self.builder.property("use-header-bar", use_header_bar),
        }
    }

    pub fn application(self, application: &impl IsA<Application>) -> Self {
        Self {
            builder: self
                .builder
                .property("application", application.clone().upcast()),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn decorated(self, decorated: bool) -> Self {
        Self {
            builder: self.builder.property("decorated", decorated),
        }
    }

    pub fn default_height(self, default_height: i32) -> Self {
        Self {
            builder: self.builder.property("default-height", default_height),
        }
    }

    pub fn default_widget(self, default_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-widget", default_widget.clone().upcast()),
        }
    }

    pub fn default_width(self, default_width: i32) -> Self {
        Self {
            builder: self.builder.property("default-width", default_width),
        }
    }

    pub fn deletable(self, deletable: bool) -> Self {
        Self {
            builder: self.builder.property("deletable", deletable),
        }
    }

    pub fn destroy_with_parent(self, destroy_with_parent: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("destroy-with-parent", destroy_with_parent),
        }
    }

    pub fn display(self, display: &impl IsA<gdk::Display>) -> Self {
        Self {
            builder: self.builder.property("display", display.clone().upcast()),
        }
    }

    pub fn focus_visible(self, focus_visible: bool) -> Self {
        Self {
            builder: self.builder.property("focus-visible", focus_visible),
        }
    }

    pub fn focus_widget(self, focus_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("focus-widget", focus_widget.clone().upcast()),
        }
    }

    pub fn fullscreened(self, fullscreened: bool) -> Self {
        Self {
            builder: self.builder.property("fullscreened", fullscreened),
        }
    }

    #[cfg(feature = "v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
    pub fn handle_menubar_accel(self, handle_menubar_accel: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("handle-menubar-accel", handle_menubar_accel),
        }
    }

    pub fn hide_on_close(self, hide_on_close: bool) -> Self {
        Self {
            builder: self.builder.property("hide-on-close", hide_on_close),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn maximized(self, maximized: bool) -> Self {
        Self {
            builder: self.builder.property("maximized", maximized),
        }
    }

    pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("mnemonics-visible", mnemonics_visible),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            builder: self.builder.property("resizable", resizable),
        }
    }

    pub fn startup_id(self, startup_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("startup-id", startup_id.into()),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    pub fn titlebar(self, titlebar: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("titlebar", titlebar.clone().upcast()),
        }
    }

    pub fn transient_for(self, transient_for: &impl IsA<Window>) -> Self {
        Self {
            builder: self
                .builder
                .property("transient-for", transient_for.clone().upcast()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`PrintUnixDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PrintUnixDialog {
        self.builder.build()
    }
}
