//! Welcome page — project overview, quick-start, and documentation links.
//!
//! Shows a centred hero with the project name, subtitle, description,
//! a quick-start code block demonstrating basic usage, and links to
//! documentation resources.

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
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    outer.append(&scrolled);

    // Centred content column.
    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    content.set_valign(gtk4::Align::Start);
    content.set_halign(gtk4::Align::Center);
    content.set_spacing(0);
    content.set_margin_top(48);
    content.set_margin_bottom(64);
    content.set_margin_start(32);
    content.set_margin_end(32);

    // ====================================================================
    // Hero section
    // ====================================================================

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
        "A component library & project template to go from idea to polished,\n\
         shippable GTK4 application — fast. Beautiful defaults, no CSS required.\n\
         Browse the sidebar to explore every component in action.",
    ));
    desc.set_wrap(true);
    desc.set_max_width_chars(55);
    desc.set_halign(gtk4::Align::Center);
    desc.set_justify(gtk4::Justification::Center);
    desc.set_margin_top(12);
    desc.set_margin_bottom(4);
    content.append(&desc);

    // ====================================================================
    // Quick-start code block
    // ====================================================================

    let code_section = create_code_section();
    content.append(&code_section);

    // ====================================================================
    // Documentation links
    // ====================================================================

    let docs_section = create_docs_section();
    content.append(&docs_section);

    // ====================================================================
    // Finishing hint
    // ====================================================================

    let hint = gtk4::Label::new(Some(
        "Select a page from the sidebar to start exploring.",
    ));
    hint.set_halign(gtk4::Align::Center);
    hint.set_opacity(0.5);
    hint.set_margin_top(32);
    content.append(&hint);

    scrolled.set_child(Some(&content));
    outer
}

// -----------------------------------------------------------------------
// Quick-start code block
// -----------------------------------------------------------------------

/// Build the "Quick Start" section with a code example.
fn create_code_section() -> gtk4::Box {
    let section = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    section.set_margin_top(28);
    section.set_halign(gtk4::Align::Center);

    // Section heading
    let heading = gtk4::Label::new(Some("Quick Start"));
    heading.add_css_class("relm4-page-subtitle");
    heading.set_halign(gtk4::Align::Start);
    heading.set_margin_bottom(8);
    section.append(&heading);

    // Code block container — styled as a dark code editor surface.
    let code_block = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    code_block.set_halign(gtk4::Align::Fill);
    code_block.set_size_request(520, -1);

    // Load the code-block and doc-link CSS for the entire display.
    // The class names used (`.code-*`, `.doc-link-*`) are specific enough
    // to avoid conflicts with other pages.
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(include_str!("welcome.css"));
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default()
            .expect("no display for CSS provider"),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Code title bar
    let title_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_bar.set_margin_top(0);
    title_bar.set_margin_bottom(0);
    title_bar.add_css_class("code-title-bar");
    code_block.append(&title_bar);

    let title_dot = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    title_dot.add_css_class("code-title-dot");
    title_bar.append(&title_dot);

    let code_title = gtk4::Label::new(Some("src/main.rs"));
    code_title.add_css_class("code-title-text");
    code_title.set_hexpand(true);
    code_title.set_halign(gtk4::Align::Center);
    title_bar.append(&code_title);

    // Code content area
    let code_text = gtk4::Label::new(Some(
        "\
use relm4_kit::prelude::*;\n\
use relm4_kit::theme;\n\
\n\
fn main() {\n\
    let app = gtk4::Application::new(\n\
        Some(\"com.example.app\"),\n\
        Default::default(),\n\
    );\n\
\n\
    app.connect_activate(|app| {\n\
        theme::init();\n\
\n\
        let card = Card::new()\n\
            .title(\"Hello, world!\")\n\
            .style(CardStyle::Elevated)\n\
            .build();\n\
\n\
        let btn = Button::new()\n\
            .label(\"Click me\")\n\
            .variant(ButtonVariant::Primary)\n\
            .on_click(Msg::Clicked)\n\
            .build();\n\
\n\
        let window = gtk4::ApplicationWindow::new(app);\n\
        window.set_child(Some(&card));\n\
        window.present();\n\
    });\n\
\n\
    app.run()\n\
}",
    ));
    code_text.set_wrap(false);
    code_text.set_selectable(true);
    code_text.set_halign(gtk4::Align::Start);
    code_text.set_margin_top(12);
    code_text.set_margin_bottom(12);
    code_text.set_margin_start(16);
    code_text.set_margin_end(16);
    code_text.add_css_class("code-text");

    code_block.append(&code_text);

    section.append(&code_block);

    section
}

// -----------------------------------------------------------------------
// Documentation links section
// -----------------------------------------------------------------------

/// Build the documentation links section.
fn create_docs_section() -> gtk4::Box {
    let section = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    section.set_margin_top(28);
    section.set_halign(gtk4::Align::Center);

    // Section heading
    let heading = gtk4::Label::new(Some("Documentation"));
    heading.add_css_class("relm4-page-subtitle");
    heading.set_halign(gtk4::Align::Start);
    heading.set_margin_bottom(8);
    section.append(&heading);

    // Links container — a row of doc link cards.
    let links_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    links_box.set_halign(gtk4::Align::Center);

    // Helper to create link entries — uses static labels as placeholders
    // since GTK4 doesn't have native hyperlink support. Users can
    // click to select content.
    let links = [
        (
            "architecture-symbolic",
            "Architecture",
            "How components work,\ncomposing Controller<T>,\nproject structure.",
        ),
        (
            "system-help-symbolic",
            "Components",
            "Full API reference,\nbuilder methods,\ncode examples.",
        ),
        (
            "preferences-system-symbolic",
            "Theming",
            "Design tokens,\nCSS overrides,\ndark mode guide.",
        ),
        (
            "help-contribute-symbolic",
            "Contributing",
            "Setting up the dev env,\ncoding standards,\nPR process.",
        ),
    ];

    for (icon_name, title_text, desc_text) in &links {
        let card = create_doc_link_card(icon_name, title_text, desc_text);
        links_box.append(&card);
    }

    section.append(&links_box);

    section
}

/// Build a single documentation link card.
fn create_doc_link_card(
    icon_name: &str,
    title_text: &str,
    desc_text: &str,
) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    card.set_size_request(150, -1);
    card.add_css_class("doc-link-card");

    let icon = gtk4::Image::from_icon_name(icon_name);
    icon.set_pixel_size(24);
    icon.set_margin_top(12);
    icon.set_opacity(0.6);
    card.append(&icon);

    let title = gtk4::Label::new(Some(title_text));
    title.add_css_class("doc-link-title");
    title.set_margin_top(4);
    card.append(&title);

    let desc = gtk4::Label::new(Some(desc_text));
    desc.set_wrap(true);
    desc.set_justify(gtk4::Justification::Center);
    desc.set_halign(gtk4::Align::Center);
    desc.set_max_width_chars(20);
    desc.set_opacity(0.6);
    desc.set_margin_bottom(12);
    card.append(&desc);

    card
}
