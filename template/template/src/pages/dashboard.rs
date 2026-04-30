//! Dashboard page — welcome card with quick stats and getting-started hints.

use gtk4::prelude::*;
use relm4::RelmWidgetExt;
use relm4_kit::primitives::{Card, CardStyle};

/// Build the dashboard content widget.
pub fn page() -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 24);
    container.set_margin_all(24);

    // ---- Welcome card ----
    let welcome = Card::<()>::new()
        .title("Welcome to {{project-name}}")
        .subtitle("Your new GTK4 + relm4 application, powered by relm4-kit.")
        .style(CardStyle::Elevated)
        .build();
    container.append(&welcome);

    // ---- Getting-started hints ----
    let hints_card = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    hints_card.add_css_class("relm4-card");
    hints_card.add_css_class("card-flat");
    hints_card.set_margin_all(16);

    let hints_title = gtk4::Label::new(Some("Getting Started"));
    hints_title.add_css_class("card-title");
    hints_title.set_halign(gtk4::Align::Start);
    hints_card.append(&hints_title);

    let hints = vec![
        ("\u{1f9ed}", "Add pages", "Create page modules under src/pages/ and add them to the content stack."),
        ("\u{1f3a8}", "Customise the theme", "Override styles in resources/style.css or use theme::init_with_overrides()."),
        ("\u{1f4e6}", "Use components", "Import primitives and containers from relm4_kit."),
    ];

    for (icon, title, desc) in hints {
        let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row.set_margin_start(8);
        row.set_margin_end(8);
        row.set_margin_top(4);
        row.set_margin_bottom(4);

        let icon_label = gtk4::Label::new(Some(icon));
        icon_label.set_css_classes(&["relm4-badge", "badge-info"]);
        icon_label.set_valign(gtk4::Align::Start);

        let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        let title_label = gtk4::Label::new(Some(title));
        title_label.set_halign(gtk4::Align::Start);
        title_label.add_css_class("heading");
        let desc_label = gtk4::Label::new(Some(desc));
        desc_label.set_halign(gtk4::Align::Start);
        desc_label.set_wrap(true);
        desc_label.add_css_class("caption");

        text_box.append(&title_label);
        text_box.append(&desc_label);
        row.append(&icon_label);
        row.append(&text_box);
        hints_card.append(&row);
    }

    container.append(&hints_card);

    container
}
