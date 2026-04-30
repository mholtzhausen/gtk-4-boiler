//! Card Demo page — shows the three card variants.

use gtk4::prelude::*;
use relm4_kit::primitives::Card;

/// Build the card demo page.
pub fn create() -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    container.set_margin_top(16);
    container.set_margin_start(16);
    container.set_margin_end(16);
    container.set_spacing(16);

    let title = gtk4::Label::new(Some("Card Demo"));
    title.add_css_class("title-2");
    title.set_halign(gtk4::Align::Start);
    container.append(&title);

    // Flat card
    let flat_card = Card::<()>::new()
        .title("Flat Card")
        .subtitle("No shadow, sits flush with the surface.")
        .build();
    container.append(&flat_card);

    // Elevated card
    let elevated_card = Card::<()>::new()
        .title("Elevated Card")
        .subtitle("Lifted with a subtle drop shadow.")
        .build();
    container.append(&elevated_card);

    // Outlined card
    let outlined_card = Card::<()>::new()
        .title("Outlined Card")
        .subtitle("Bordered but shadowless.")
        .build();
    container.append(&outlined_card);

    // Wrap in a ScrolledWindow
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_child(Some(&container));
    scrolled.set_vexpand(true);

    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer.append(&scrolled);
    outer.set_vexpand(true);

    outer
}
