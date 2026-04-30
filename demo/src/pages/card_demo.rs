//! Card Demo page — interactive preview of all card styles, footer actions,
//! scrollable content, and a live source code panel.
//!
//! A set of knobs (style selector, footer toggle, content toggle) drive a
//! preview that rebuilds on every change, demonstrating the full Card API.

use gtk4::prelude::*;
use relm4_kit::primitives::{Card, CardStyle, ButtonAction};

// ============================================================================
// Constants
// ============================================================================

/// Placeholder message type for the demo (cards are stateless builders, not
/// managed components, so we just use `()` for actions).
#[derive(Clone)]
enum DemoMsg {
    Clicked,
}

const LONG_CONTENT: &str =
    "This card has a long scrollable content area.\n\n\
     Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod \
     tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim \
     veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea \
     commodo consequat.\n\n\
     Duis aute irure dolor in reprehenderit in voluptate velit esse cillum \
     dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non \
     proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

// ============================================================================
// Public API
// ============================================================================

/// Build the card demo page widget.
pub fn create() -> gtk4::Box {
    // ---- Page-specific CSS ----
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("card_demo.css"));
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default()
            .expect("no display for CSS provider"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // ---- Page layout ----
    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer.set_vexpand(true);
    outer.set_hexpand(true);
    outer.add_css_class("relm4-page");

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    outer.append(&scrolled);

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.add_css_class("card-demo-layout");
    container.set_margin_top(24);
    container.set_margin_bottom(48);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_spacing(0);

    // ---- Page header ----
    let title = gtk4::Label::new(Some("Card Demo"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    let desc = gtk4::Label::new(Some(
        "Interactive demo of the Card component — experiment with styles, \
         footer actions, and scrollable content.",
    ));
    desc.add_css_class("relm4-page-subtitle");
    desc.set_wrap(true);
    desc.set_margin_bottom(16);
    container.append(&desc);

    // ---- Knobs row ----
    let knobs = create_knobs();
    container.append(&knobs);

    // ---- Separator ----
    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.set_margin_top(16);
    sep.set_margin_bottom(16);
    container.append(&sep);

    // ---- Preview area ----
    let preview_area = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    preview_area.add_css_class("card-demo-preview-area");
    preview_area.set_vexpand(true);
    preview_area.set_hexpand(true);

    let preview_label = gtk4::Label::new(Some("Preview"));
    preview_label.add_css_class("card-demo-section-label");
    preview_label.set_halign(gtk4::Align::Start);
    container.append(&preview_label);

    container.append(&preview_area);

    // ---- Build initial card preview ----
    let initial_card = build_preview_card(CardStyle::Elevated, true, false);
    preview_area.append(&initial_card);

    // ---- Code panel ----
    let code_label = gtk4::Label::new(Some("Source Code"));
    code_label.add_css_class("card-demo-section-label");
    code_label.set_halign(gtk4::Align::Start);
    code_label.set_margin_top(16);
    container.append(&code_label);

    let code_panel = create_code_panel(CardStyle::Elevated, true, false);
    container.append(&code_panel);

    // ---- Wire up knob signals ----
    wire_knobs(&knobs, &preview_area, &code_panel);

    scrolled.set_child(Some(&container));
    outer
}

// ============================================================================
// Knobs
// ============================================================================

/// Build the interactive knobs row.
///
/// Returns a box whose children are tagged with CSS names so that
/// [`wire_knobs`] can find them by name.
fn create_knobs() -> gtk4::Box {
    let knobs = gtk4::Box::new(gtk4::Orientation::Horizontal, 24);
    knobs.add_css_class("card-demo-knobs");
    knobs.set_halign(gtk4::Align::Fill);
    knobs.set_hexpand(true);

    // ---- 1. Style selector ----
    let style_group = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    style_group.set_halign(gtk4::Align::Start);

    let style_label = gtk4::Label::new(Some("Card Style"));
    style_label.add_css_class("card-demo-knob-label");
    style_label.set_halign(gtk4::Align::Start);
    style_group.append(&style_label);

    let style_dropdown = gtk4::DropDown::from_strings(&["Flat", "Elevated", "Outlined"]);
    style_dropdown.set_selected(1); // Elevated default
    style_dropdown.set_halign(gtk4::Align::Start);
    style_dropdown.set_widget_name("card-demo-style-selector");
    style_group.append(&style_dropdown);
    knobs.append(&style_group);

    // ---- 2. Footer toggle ----
    let footer_group = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    footer_group.set_halign(gtk4::Align::Start);

    let footer_label = gtk4::Label::new(Some("Footer Actions"));
    footer_label.add_css_class("card-demo-knob-label");
    footer_label.set_halign(gtk4::Align::Start);
    footer_group.append(&footer_label);

    let footer_switch = gtk4::Switch::new();
    footer_switch.set_active(true);
    footer_switch.set_halign(gtk4::Align::Start);
    footer_switch.set_widget_name("card-demo-footer-toggle");
    footer_group.append(&footer_switch);
    knobs.append(&footer_group);

    // ---- 3. Long content toggle ----
    let content_group = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    content_group.set_halign(gtk4::Align::Start);

    let content_label = gtk4::Label::new(Some("Scrollable Content"));
    content_label.add_css_class("card-demo-knob-label");
    content_label.set_halign(gtk4::Align::Start);
    content_group.append(&content_label);

    let content_switch = gtk4::Switch::new();
    content_switch.set_active(false);
    content_switch.set_halign(gtk4::Align::Start);
    content_switch.set_widget_name("card-demo-content-toggle");
    content_group.append(&content_switch);
    knobs.append(&content_group);

    knobs
}

// ============================================================================
// Preview card builder
// ============================================================================

/// Build the card preview widget based on the knob values.
fn build_preview_card(
    style: CardStyle,
    show_footer: bool,
    show_long_content: bool,
) -> gtk4::Box {
    let mut card = Card::new()
        .style(style)
        .title("Card Title")
        .subtitle("A subtitle providing additional context about this card's content.");

    // Optional: scrollable long content.
    if show_long_content {
        let content_text = gtk4::Label::new(Some(LONG_CONTENT));
        content_text.set_wrap(true);
        content_text.set_xalign(0.0);
        content_text.add_css_class("card-demo-long-content");
        content_text.set_margin_top(8);
        content_text.set_margin_bottom(8);

        // Wrap in a small scrolled window.
        let scroller = gtk4::ScrolledWindow::new();
        scroller.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scroller.set_max_content_height(120);
        scroller.set_propagate_natural_height(true);
        scroller.set_child(Some(&content_text));

        card = card.child(&scroller);
    } else {
        // Short content placeholder.
        let content_text = gtk4::Label::new(Some(
            "This is the card's content area. Children are placed between the \
             subtitle and the footer.",
        ));
        content_text.set_wrap(true);
        content_text.set_xalign(0.0);
        content_text.add_css_class("card-demo-long-content");
        content_text.set_margin_top(8);
        content_text.set_margin_bottom(8);
        card = card.child(&content_text);
    }

    // Optional: footer actions.
    if show_footer {
        card = card.footer(vec![
            ButtonAction::new("Cancel", DemoMsg::Clicked),
            ButtonAction::primary("Confirm", DemoMsg::Clicked),
        ]);
    }

    card.build()
}

// ============================================================================
// Code panel
// ============================================================================

/// Build a styled code panel showing the source needed to create the current
/// card configuration.
fn create_code_panel(style: CardStyle, show_footer: bool, show_long_content: bool) -> gtk4::Box {
    let panel = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    panel.add_css_class("card-demo-code-panel");
    panel.set_hexpand(true);

    // Title bar.
    let title_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_bar.add_css_class("card-demo-code-title");
    panel.append(&title_bar);

    let title_text = gtk4::Label::new(Some("Rust — Card Builder"));
    title_text.add_css_class("card-demo-code-title-text");
    title_text.set_halign(gtk4::Align::Start);
    title_bar.append(&title_text);

    // Code content.
    let code = generate_code_source(style, show_footer, show_long_content);

    let code_text = gtk4::Label::new(Some(&code));
    code_text.add_css_class("card-demo-code-text");
    code_text.set_wrap(false);
    code_text.set_selectable(true);
    code_text.set_xalign(0.0);
    code_text.set_margin_top(12);
    code_text.set_margin_bottom(12);
    code_text.set_margin_start(16);
    code_text.set_margin_end(16);
    panel.append(&code_text);

    panel
}

/// Generate a Rust code snippet for the current card configuration.
fn generate_code_source(style: CardStyle, show_footer: bool, show_long_content: bool) -> String {
    let style_str = match style {
        CardStyle::Flat => "CardStyle::Flat",
        CardStyle::Elevated => "CardStyle::Elevated",
        CardStyle::Outlined => "CardStyle::Outlined",
    };

    let mut code = String::from("use relm4_kit::primitives::{Card, CardStyle, ButtonAction};\n\n");
    code.push_str("let card = Card::new()\n");
    code.push_str(&format!("    .style({style_str})\n"));
    code.push_str("    .title(\"Card Title\")\n");
    code.push_str("    .subtitle(\"A subtitle providing additional context...\")\n");

    if show_long_content {
        code.push_str(
            "    // Wrap content in a ScrolledWindow for scrollable content\n\
             .child(&scrolled_content)\n",
        );
    } else {
        code.push_str("    .child(&content_widget)\n");
    }

    if show_footer {
        code.push_str("    .footer(vec![\n");
        code.push_str("        ButtonAction::new(\"Cancel\", Msg::Cancel),\n");
        code.push_str("        ButtonAction::primary(\"Confirm\", Msg::Confirm),\n");
        code.push_str("    ])\n");
    }

    code.push_str("    .build();\n");
    code
}

// ============================================================================
// Knob wiring
// ============================================================================

/// Connect signal handlers to the knobs so that changing any knob rebuilds
/// both the preview card and the code panel.
///
/// `preview_area` is a container whose single child is the current preview
/// card. `code_panel` is replaced similarly.
fn wire_knobs(
    knobs: &gtk4::Box,
    preview_area: &gtk4::Box,
    code_panel: &gtk4::Box,
) {
    // Find knob widgets by name.
    let style_dropdown = knobs
        .find_child("card-demo-style-selector")
        .and_then(|w| w.downcast::<gtk4::DropDown>().ok())
        .expect("card-demo-style-selector not found");

    let footer_switch = knobs
        .find_child("card-demo-footer-toggle")
        .and_then(|w| w.downcast::<gtk4::Switch>().ok())
        .expect("card-demo-footer-toggle not found");

    let content_switch = knobs
        .find_child("card-demo-content-toggle")
        .and_then(|w| w.downcast::<gtk4::Switch>().ok())
        .expect("card-demo-content-toggle not found");

    // Clone everything needed inside the closure.
    let preview_weak = preview_area.downgrade();
    let code_weak = code_panel.downgrade();
    let sd = style_dropdown.clone();
    let fs = footer_switch.clone();
    let cs = content_switch.clone();

    let rebuild = std::rc::Rc::new(move || {
        let Some(preview) = preview_weak.upgrade() else { return };
        let Some(code_panel) = code_weak.upgrade() else { return };

        let style_idx = sd.selected();
        let style = match style_idx {
            0 => CardStyle::Flat,
            1 => CardStyle::Elevated,
            _ => CardStyle::Outlined,
        };
        let show_footer = fs.is_active();
        let show_long_content = cs.is_active();

        // Rebuild preview: remove old child, add new one.
        if let Some(child) = preview.first_child() {
            preview.remove(&child);
        }
        let new_card = build_preview_card(style, show_footer, show_long_content);
        preview.append(&new_card);

        // Rebuild code panel: remove old child widgets, recreate.
        while let Some(child) = code_panel.first_child() {
            code_panel.remove(&child);
        }
        let new_panel = create_code_panel(style, show_footer, show_long_content);
        // Transfer children from new_panel to code_panel.
        while let Some(child) = new_panel.first_child() {
            new_panel.remove(&child);
            code_panel.append(&child);
        }
    });

    // Connect all knob signals.
    let r2 = rebuild.clone();
    style_dropdown.connect_selected_notify(move |_| r2());

    let r3 = rebuild.clone();
    footer_switch.connect_active_notify(move |_| r3());

    let r4 = rebuild.clone();
    content_switch.connect_active_notify(move |_| r4());
}

// ============================================================================
// Widget tree traversal helper
// ============================================================================

/// Extension trait to find a child widget by GTK widget name.
trait FindChild {
    fn find_child(&self, name: &str) -> Option<gtk4::Widget>;
}

fn find_in_widget(widget: &gtk4::Widget, name: &str) -> Option<gtk4::Widget> {
    if widget.widget_name() == name {
        return Some(widget.clone());
    }
    let mut child = widget.first_child();
    while let Some(ref c) = child {
        if let Some(found) = find_in_widget(c, name) {
            return Some(found);
        }
        child = c.next_sibling();
    }
    None
}

impl FindChild for gtk4::Widget {
    fn find_child(&self, name: &str) -> Option<gtk4::Widget> {
        find_in_widget(self, name)
    }
}

impl FindChild for gtk4::Box {
    fn find_child(&self, name: &str) -> Option<gtk4::Widget> {
        find_in_widget(self.upcast_ref(), name)
    }
}
