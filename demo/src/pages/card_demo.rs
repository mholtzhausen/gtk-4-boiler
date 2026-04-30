//! Card Demo page — shows the three card variants.

use gtk4::prelude::*;
use relm4_kit::primitives::Card;

/// Build the card demo page.
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

    // ---- Page title ----
    let title = gtk4::Label::new(Some("Card Demo"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    // ---- Flat card ----
    let flat_card = Card::<()>::new()
        .title("Flat Card")
        .subtitle("No shadow, sits flush with the surface.")
        .build();
    container.append(&flat_card);

    // ---- Elevated card ----
    let elevated_card = Card::<()>::new()
        .title("Elevated Card")
        .subtitle("Lifted with a subtle drop shadow.")
        .build();
    container.append(&elevated_card);

    // ---- Outlined card ----
    let outlined_card = Card::<()>::new()
        .title("Outlined Card")
        .subtitle("Bordered but shadowless.")
        .build();
    container.append(&outlined_card);

    scrolled.set_child(Some(&container));
    outer
}
