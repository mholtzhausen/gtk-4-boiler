//! Theme Showcase page — colour, spacing, radius, and typography demo.

use gtk4::prelude::*;

/// Build the theme showcase page.
pub fn create() -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.set_valign(gtk4::Align::Center);
    container.set_halign(gtk4::Align::Center);
    container.set_spacing(12);

    let title = gtk4::Label::new(Some("Theme Showcase"));
    title.add_css_class("title-2");
    container.append(&title);

    let desc = gtk4::Label::new(Some("Theme tokens and CSS variables coming soon."));
    desc.set_halign(gtk4::Align::Center);
    container.append(&desc);

    container
}
