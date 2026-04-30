//! Button Demo page — interactive preview of all button variants, sizes,
//! icon support, disabled states, and a live click-feedback area.
//!
//! Every variant (Primary, Secondary, Ghost, Danger, Link) is shown in
//! each size (Small, Medium, Large), both with and without icons, plus a
//! disabled row and an interactive section that logs clicks to a status
//! label.

use gtk4::prelude::*;
use relm4_kit::primitives::{Button, ButtonVariant, ButtonSize};

// Short alias for display buttons that have no click handler message.
type Btn = Button<()>;

// ============================================================================
// Constants
// ============================================================================

/// All button variants in display order.
const VARIANTS: &[(&str, ButtonVariant, &str)] = &[
    ("Primary", ButtonVariant::Primary, "Primary action button"),
    ("Secondary", ButtonVariant::Secondary, "Secondary/alternative action"),
    ("Ghost", ButtonVariant::Ghost, "Subtle, borderless action"),
    ("Danger", ButtonVariant::Danger, "Destructive action"),
    ("Link", ButtonVariant::Link, "Inline text-link style"),
];

/// All sizes in display order.
const SIZES: &[(ButtonSize, &str)] = &[
    (ButtonSize::Small, "Small"),
    (ButtonSize::Medium, "Medium"),
    (ButtonSize::Large, "Large"),
];

const LONG_TEXT: &str =
    "Buttons are the most fundamental interactive element. Use the correct \
     variant to communicate the importance of each action:\n\
     \u{2022} Primary — the single most important action on the screen.\n\
     \u{2022} Secondary — alternative actions that are less prominent.\n\
     \u{2022} Ghost — low-priority actions like Cancel.\n\
     \u{2022} Danger — destructive irreversible actions.\n\
     \u{2022} Link — inline navigational actions.";

// ============================================================================
// Public API
// ============================================================================

/// Build the button demo page widget.
pub fn create() -> gtk4::Box {
    // ---- Page-specific CSS ----
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("button_demo.css"));
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
    container.add_css_class("btn-demo-layout");
    container.set_margin_top(24);
    container.set_margin_bottom(48);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_spacing(0);

    // ---- Page header ----
    let title = gtk4::Label::new(Some("Button Demo"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    let desc = gtk4::Label::new(Some(LONG_TEXT));
    desc.add_css_class("relm4-page-subtitle");
    desc.set_wrap(true);
    desc.set_xalign(0.0);
    desc.set_margin_bottom(16);
    container.append(&desc);

    // ---- Interactive area (click-feedback) ----
    let feedback_label = gtk4::Label::new(
        Some("Click any button below to see its description here."),
    );
    feedback_label.add_css_class("btn-demo-feedback");
    feedback_label.set_wrap(true);
    feedback_label.set_xalign(0.0);
    container.append(&feedback_label);

    // ---- Separator ----
    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.set_margin_top(16);
    sep.set_margin_bottom(16);
    container.append(&sep);

    // ========================================================================
    // Section 1: Variants (Medium size, no icon)
    // ========================================================================
    let section_label = gtk4::Label::new(Some("Variants (Medium, no icon)"));
    section_label.add_css_class("btn-demo-section-label");
    section_label.set_halign(gtk4::Align::Start);
    section_label.set_margin_bottom(8);
    container.append(&section_label);

    let variants_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    variants_row.add_css_class("btn-demo-row");
    variants_row.set_halign(gtk4::Align::Start);

    for (name, variant, _desc) in VARIANTS {
        let btn = Btn::new()
            .label(*name)
            .variant(*variant)
            .build();
        variants_row.append(&btn);
    }
    container.append(&variants_row);

    // ========================================================================
    // Section 2: Sizes (Primary, no icon)
    // ========================================================================
    let sizes_label = gtk4::Label::new(Some("Sizes (Primary variant)"));
    sizes_label.add_css_class("btn-demo-section-label");
    sizes_label.set_halign(gtk4::Align::Start);
    sizes_label.set_margin_top(24);
    sizes_label.set_margin_bottom(8);
    container.append(&sizes_label);

    let sizes_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    sizes_row.add_css_class("btn-demo-row");
    sizes_row.set_halign(gtk4::Align::Center);

    for (size, name) in SIZES {
        let btn = Btn::new()
            .label(*name)
            .variant(ButtonVariant::Primary)
            .size(*size)
            .build();
        sizes_row.append(&btn);
    }
    container.append(&sizes_row);

    // ---- Sizes × all variants ----
    let sizes_grid = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    sizes_grid.add_css_class("btn-demo-row");
    sizes_grid.set_margin_top(8);

    for (variant_name, variant, _desc) in VARIANTS {
        let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row.set_halign(gtk4::Align::Start);

        // Variant label.
        let lbl = gtk4::Label::new(Some(*variant_name));
        lbl.set_width_chars(12);
        lbl.set_xalign(1.0);
        lbl.add_css_class("btn-demo-grid-label");
        row.append(&lbl);

        for (size, _size_name) in SIZES {
            let btn = Btn::new()
                .label("Button")
                .variant(*variant)
                .size(*size)
                .build();
            row.append(&btn);
        }
        sizes_grid.append(&row);
    }
    container.append(&sizes_grid);

    // ========================================================================
    // Section 3: With icons
    // ========================================================================
    let icons_label = gtk4::Label::new(Some("With Icons (Medium, all variants)"));
    icons_label.add_css_class("btn-demo-section-label");
    icons_label.set_halign(gtk4::Align::Start);
    icons_label.set_margin_top(24);
    icons_label.set_margin_bottom(8);
    container.append(&icons_label);

    let icons_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    icons_row.add_css_class("btn-demo-row");
    icons_row.set_halign(gtk4::Align::Start);

    for (name, variant, _desc) in VARIANTS {
        let btn = Btn::new()
            .label(*name)
            .variant(*variant)
            .icon(match variant {
                ButtonVariant::Primary => "document-save-symbolic",
                ButtonVariant::Secondary => "edit-undo-symbolic",
                ButtonVariant::Ghost => "user-trash-symbolic",
                ButtonVariant::Danger => "edit-delete-symbolic",
                ButtonVariant::Link => "web-browser-symbolic",
            })
            .build();
        icons_row.append(&btn);
    }
    container.append(&icons_row);

    // ---- Icon-only buttons (no label) ----
    let icon_only_label = gtk4::Label::new(Some("Icon-only buttons"));
    icon_only_label.add_css_class("btn-demo-subsection-label");
    icon_only_label.set_halign(gtk4::Align::Start);
    icon_only_label.set_margin_top(12);
    icon_only_label.set_margin_bottom(4);
    container.append(&icon_only_label);

    let icon_only_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    icon_only_row.set_halign(gtk4::Align::Start);

    for (variant, icon) in &[
        (ButtonVariant::Primary, "document-new-symbolic"),
        (ButtonVariant::Secondary, "document-open-symbolic"),
        (ButtonVariant::Ghost, "document-print-symbolic"),
        (ButtonVariant::Danger, "process-stop-symbolic"),
        (ButtonVariant::Link, "internet-web-browser-symbolic"),
    ] {
        let btn = Btn::new()
            .label("")
            .variant(*variant)
            .icon(*icon)
            .build();
        icon_only_row.append(&btn);
    }
    container.append(&icon_only_row);

    // ========================================================================
    // Section 4: Disabled state
    // ========================================================================
    let disabled_label = gtk4::Label::new(Some("Disabled State"));
    disabled_label.add_css_class("btn-demo-section-label");
    disabled_label.set_halign(gtk4::Align::Start);
    disabled_label.set_margin_top(24);
    disabled_label.set_margin_bottom(8);
    container.append(&disabled_label);

    let disabled_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    disabled_row.add_css_class("btn-demo-row");
    disabled_row.set_halign(gtk4::Align::Start);

    for (name, variant, _desc) in VARIANTS {
        let btn = Btn::new()
            .label(*name)
            .variant(*variant)
            .disabled(true)
            .build();
        disabled_row.append(&btn);
    }
    container.append(&disabled_row);

    // ---- Disabled with icons ----
    let disabled_icons_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    disabled_icons_row.add_css_class("btn-demo-row");
    disabled_icons_row.set_halign(gtk4::Align::Start);
    disabled_icons_row.set_margin_top(8);

    for (name, variant, _desc) in VARIANTS {
        let btn = Btn::new()
            .label(*name)
            .variant(*variant)
            .icon("dialog-question-symbolic")
            .disabled(true)
            .build();
        disabled_icons_row.append(&btn);
    }
    container.append(&disabled_icons_row);

    // ========================================================================
    // Section 5: Interactive live-preview
    // ========================================================================
    let interactive_label = gtk4::Label::new(
        Some("Interactive — click to see feedback"),
    );
    interactive_label.add_css_class("btn-demo-section-label");
    interactive_label.set_halign(gtk4::Align::Start);
    interactive_label.set_margin_top(24);
    interactive_label.set_margin_bottom(8);
    container.append(&interactive_label);

    // A status label that updates when any interactive button is clicked.
    let status_label = gtk4::Label::new(Some("No button clicked yet."));
    status_label.add_css_class("btn-demo-status");
    status_label.set_wrap(true);
    status_label.set_xalign(0.0);
    status_label.set_margin_bottom(8);
    container.append(&status_label);

    // Interactive grid — each variant × each size, all clickable.
    let interactive_grid = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    interactive_grid.add_css_class("btn-demo-row");

    for (variant_name, variant, variant_desc) in VARIANTS {
        let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row.set_halign(gtk4::Align::Start);

        let lbl = gtk4::Label::new(Some(*variant_name));
        lbl.set_width_chars(12);
        lbl.set_xalign(1.0);
        lbl.add_css_class("btn-demo-grid-label");
        row.append(&lbl);

        for (size, size_name) in SIZES {
            let btn = gtk4::Button::with_label(size_name);
            btn.add_css_class("relm4-btn");

            // Apply variant class.
            let variant_class = match variant {
                ButtonVariant::Primary => "relm4-btn-primary",
                ButtonVariant::Secondary => "relm4-btn-secondary",
                ButtonVariant::Ghost => "relm4-btn-ghost",
                ButtonVariant::Danger => "relm4-btn-danger",
                ButtonVariant::Link => "relm4-btn-link",
            };
            btn.add_css_class(variant_class);

            // Apply size class.
            let size_class = match size {
                ButtonSize::Small => "relm4-btn-small",
                ButtonSize::Medium => "",
                ButtonSize::Large => "relm4-btn-large",
            };
            if !size_class.is_empty() {
                btn.add_css_class(size_class);
            }

            // Wire click to update status label.
            let status_clone = status_label.clone();
            let variant_name_owned = variant_name.to_string();
            let size_name_owned = size_name.to_string();
            let desc = variant_desc.to_string();
            btn.connect_clicked(move |_| {
                status_clone.set_markup(
                    &format!(
                        "<b>Clicked:</b> <span color='#3584e4'>{variant_name_owned}</span> \
                         ({size_name_owned}) — {desc}",
                    ),
                );
            });
            row.append(&btn);
        }
        interactive_grid.append(&row);
    }
    container.append(&interactive_grid);

    // ---- Interactive with icons ----
    let interactive_icons_label = gtk4::Label::new(Some("With icons (clickable)"));
    interactive_icons_label.add_css_class("btn-demo-subsection-label");
    interactive_icons_label.set_halign(gtk4::Align::Start);
    interactive_icons_label.set_margin_top(12);
    interactive_icons_label.set_margin_bottom(4);
    container.append(&interactive_icons_label);

    let interactive_icons_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    interactive_icons_row.set_halign(gtk4::Align::Start);

    // Map of label → variant → icon name.
    let variant_icons: [(&str, ButtonVariant, &str); 5] = [
        ("Save", ButtonVariant::Primary, "document-save-symbolic"),
        ("Undo", ButtonVariant::Secondary, "edit-undo-symbolic"),
        ("Skip", ButtonVariant::Ghost, "media-skip-forward-symbolic"),
        ("Delete", ButtonVariant::Danger, "edit-delete-symbolic"),
        ("Visit", ButtonVariant::Link, "web-browser-symbolic"),
    ];

    for (label_text, variant, icon_name) in &variant_icons {
        let btn = Btn::new()
            .label(*label_text)
            .variant(*variant)
            .icon(*icon_name)
            .build();

        let status_clone = status_label.clone();
        let lbl = label_text.to_string();
        let icon = icon_name.to_string();
        btn.connect_clicked(move |_| {
            status_clone.set_markup(
                &format!(
                    "<b>Clicked:</b> <span color='#3584e4'>{lbl}</span> \
                     <span color='#5e5c64'>(icon: {icon})</span>",
                ),
            );
        });
        interactive_icons_row.append(&btn);
    }
    container.append(&interactive_icons_row);

    // ========================================================================
    // Code panel
    // ========================================================================
    let code_label = gtk4::Label::new(Some("Source Code Example"));
    code_label.add_css_class("btn-demo-section-label");
    code_label.set_halign(gtk4::Align::Start);
    code_label.set_margin_top(24);
    container.append(&code_label);

    let code_panel = create_code_panel();
    container.append(&code_panel);

    scrolled.set_child(Some(&container));
    outer
}

// ============================================================================
// Code panel
// ============================================================================

/// Build a styled code panel showing the Button API in action.
fn create_code_panel() -> gtk4::Box {
    let panel = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    panel.add_css_class("btn-demo-code-panel");
    panel.set_hexpand(true);

    // Title bar.
    let title_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_bar.add_css_class("btn-demo-code-title");
    panel.append(&title_bar);

    let title_text = gtk4::Label::new(Some("Rust — Button Builder"));
    title_text.add_css_class("btn-demo-code-title-text");
    title_text.set_halign(gtk4::Align::Start);
    title_bar.append(&title_text);

    // Code content.
    let code_text = gtk4::Label::new(Some(CODE_SAMPLE));
    code_text.add_css_class("btn-demo-code-text");
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

/// A complete working example of Button usage.
const CODE_SAMPLE: &str = r#"use relm4_kit::primitives::{Button, ButtonVariant, ButtonSize};

// Primary button (default)
let btn = Button::new()
    .label("Save")
    .variant(ButtonVariant::Primary)
    .build();

// Secondary with icon
let btn = Button::new()
    .label("Cancel")
    .variant(ButtonVariant::Secondary)
    .icon("edit-undo-symbolic")
    .build();

// Small ghost button
let btn = Button::new()
    .label("Skip")
    .variant(ButtonVariant::Ghost)
    .size(ButtonSize::Small)
    .build();

// Danger button (disabled)
let btn = Button::new()
    .label("Delete")
    .variant(ButtonVariant::Danger)
    .disabled(true)
    .build();

// Link-style button
let btn = Button::new()
    .label("Learn more")
    .variant(ButtonVariant::Link)
    .build();
"#;
