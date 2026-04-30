//! Badge Demo page — shows all badge variants and sizes.

use gtk4::prelude::*;

/// Build the badge demo page.
pub fn create() -> gtk4::Box {
    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer.set_vexpand(true);
    outer.set_hexpand(true);
    outer.add_css_class("relm4-page");

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);
    outer.append(&scrolled);

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.set_margin_top(24);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_spacing(16);

    let title = gtk4::Label::new(Some("Badge Demo"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    let label = gtk4::Label::new(Some("Badge demo — coming soon."));
    label.add_css_class("relm4-page-subtitle");
    container.append(&label);

    scrolled.set_child(Some(&container));
    outer
}
