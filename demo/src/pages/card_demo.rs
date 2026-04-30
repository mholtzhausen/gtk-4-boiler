//! Card Demo page — shows the three card variants with distinct styles.

use gtk4::prelude::*;
use relm4_kit::primitives::{Card, CardStyle};

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

    // ---- Section: Flat card ----
    let flat_section_label = gtk4::Label::new(Some("Flat Card"));
    flat_section_label.add_css_class("relm4-page-subtitle");
    flat_section_label.set_halign(gtk4::Align::Start);
    flat_section_label.set_margin_top(8);
    container.append(&flat_section_label);

    let flat_card = Card::<()>::new()
        .style(CardStyle::Flat)
        .title("Flat Card")
        .subtitle("No shadow, sits flush with the surface. Different background + subtle left accent.")
        .build();
    container.append(&flat_card);

    // ---- Section: Elevated card ----
    let elevated_section_label = gtk4::Label::new(Some("Elevated Card"));
    elevated_section_label.add_css_class("relm4-page-subtitle");
    elevated_section_label.set_halign(gtk4::Align::Start);
    elevated_section_label.set_margin_top(8);
    container.append(&elevated_section_label);

    let elevated_card = Card::<()>::new()
        .style(CardStyle::Elevated)
        .title("Elevated Card")
        .subtitle("Lifted above the surface with a layered drop shadow and a blue accent.")
        .build();
    container.append(&elevated_card);

    // ---- Section: Outlined card ----
    let outlined_section_label = gtk4::Label::new(Some("Outlined Card"));
    outlined_section_label.add_css_class("relm4-page-subtitle");
    outlined_section_label.set_halign(gtk4::Align::Start);
    outlined_section_label.set_margin_top(8);
    container.append(&outlined_section_label);

    let outlined_card = Card::<()>::new()
        .style(CardStyle::Outlined)
        .title("Outlined Card")
        .subtitle("Transparent background with a clear border for lightweight separation.")
        .build();
    container.append(&outlined_card);

    scrolled.set_child(Some(&container));
    outer
}
