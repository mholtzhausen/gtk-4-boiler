//! Welcome page — project overview and quick-start.

use gtk4::prelude::*;

/// Build the welcome page widget.
pub fn create() -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.set_valign(gtk4::Align::Center);
    container.set_halign(gtk4::Align::Center);
    container.set_spacing(12);

    let title = gtk4::Label::new(Some("relm4-kit"));
    title.add_css_class("title-1");
    container.append(&title);

    let subtitle = gtk4::Label::new(Some("Curated components for GTK4 + relm4"));
    subtitle.add_css_class("subtitle-1");
    container.append(&subtitle);

    let desc = gtk4::Label::new(Some(
        "Browse the sidebar to see each component in action.",
    ));
    desc.set_wrap(true);
    desc.set_max_width_chars(60);
    desc.set_halign(gtk4::Align::Center);
    container.append(&desc);

    container
}
