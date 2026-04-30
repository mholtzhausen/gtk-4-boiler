//! Button Demo page — shows all button variants and sizes.

use gtk4::prelude::*;

/// Build the button demo page.
pub fn create() -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    container.set_margin_top(16);
    container.set_margin_start(16);
    container.set_margin_end(16);
    container.set_spacing(16);

    let title = gtk4::Label::new(Some("Button Demo"));
    title.add_css_class("title-2");
    title.set_halign(gtk4::Align::Start);
    container.append(&title);

    let label = gtk4::Label::new(Some("Button demo — coming soon."));
    label.set_halign(gtk4::Align::Start);
    container.append(&label);

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_child(Some(&container));
    scrolled.set_vexpand(true);

    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer.append(&scrolled);
    outer.set_vexpand(true);

    outer
}
