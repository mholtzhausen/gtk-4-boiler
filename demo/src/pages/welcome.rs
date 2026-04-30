//! Welcome page — project overview and quick-start.
//!
//! Shows a large centred hero with the project name, subtitle,
//! description, and a getting-started hint.

use gtk4::prelude::*;

/// Build the welcome page widget.
pub fn create() -> gtk4::Box {
    // Outer container fills the content area with a page background.
    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer.set_vexpand(true);
    outer.set_hexpand(true);
    outer.add_css_class("relm4-page");

    // Scroll wrapper for small windows.
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);
    outer.append(&scrolled);

    // Centred content column.
    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    content.set_valign(gtk4::Align::Center);
    content.set_halign(gtk4::Align::Center);
    content.set_spacing(16);
    content.set_margin_top(48);
    content.set_margin_bottom(48);
    content.set_margin_start(32);
    content.set_margin_end(32);

    // ---- Hero icon ----
    let icon = gtk4::Image::from_icon_name("applications-engineering-symbolic");
    icon.set_pixel_size(64);
    icon.set_opacity(0.7);
    content.append(&icon);

    // ---- Title "relm4-kit" ----
    let title = gtk4::Label::new(Some("relm4-kit"));
    title.add_css_class("relm4-page-title");
    title.set_margin_top(8);
    content.append(&title);

    // ---- Subtitle ----
    let subtitle = gtk4::Label::new(Some("Curated components for GTK4 + relm4"));
    subtitle.add_css_class("relm4-page-subtitle");
    content.append(&subtitle);

    // ---- Description ----
    let desc = gtk4::Label::new(Some(
        "Browse the sidebar to explore each component in action.\n\
         Click any navigation item to see demos, source examples,\n\
         and CSS class references.",
    ));
    desc.set_wrap(true);
    desc.set_max_width_chars(55);
    desc.set_halign(gtk4::Align::Center);
    desc.set_justify(gtk4::Justification::Center);
    desc.set_margin_top(8);
    content.append(&desc);

    // ---- Separator ----
    let separator = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    separator.set_margin_top(16);
    separator.set_margin_bottom(8);
    separator.set_size_request(200, -1);
    separator.set_halign(gtk4::Align::Center);
    content.append(&separator);

    // ---- Getting-started hint ----
    let hint = gtk4::Label::new(Some(
        "Select a page from the sidebar to start exploring.",
    ));
    hint.set_halign(gtk4::Align::Center);
    hint.set_opacity(0.6);
    content.append(&hint);

    scrolled.set_child(Some(&content));
    outer
}
