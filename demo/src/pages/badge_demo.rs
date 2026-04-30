//! Badge Demo page — interactive preview of all badge variants, sizes,
//! icon-badge overlays (mocked), and an animated live counter.
//!
//! Shows:
//! - All 5 variants (Success, Danger, Warning, Info, Neutral) in both sizes
//! - Badges positioned over icon placeholders (mocked)
//! - A live counter that increments every second to simulate notification
//!   badge animation

use gtk4::prelude::*;
use relm4_kit::primitives::{Badge, BadgeVariant, BadgeSize};

// ============================================================================
// Constants
// ============================================================================

/// All badge variants in display order with human-readable names.
const VARIANTS: &[(BadgeVariant, &str, &str)] = &[
    (BadgeVariant::Success, "Success", "Positive/completed state"),
    (BadgeVariant::Danger, "Danger", "Error/destructive state"),
    (BadgeVariant::Warning, "Warning", "Cautionary state"),
    (BadgeVariant::Info, "Info", "Informational state"),
    (BadgeVariant::Neutral, "Neutral", "Default/non-semantic"),
];

const SIZES: &[(BadgeSize, &str)] = &[(BadgeSize::Small, "Small"), (BadgeSize::Medium, "Medium")];

const LONG_TEXT: &str =
    "Badges are small non-interactive labels for status indicators, \
     notification counts, and metadata tags.\n\
     \u{2022} Success — green tint for positive states.\n\
     \u{2022} Danger — red tint for errors.\n\
     \u{2022} Warning — yellow tint for cautionary states.\n\
     \u{2022} Info — blue tint for informational states.\n\
     \u{2022} Neutral — grey for default labels.";

/// Example badge labels for the variant gallery.
const LABELS: &[&str] = &["New", "3", "Soon", "Live", "99+", "Alpha", "v2", "OFF"];

// ============================================================================
// Public API
// ============================================================================

/// Build the badge demo page widget.
pub fn create() -> gtk4::Box {
    // ---- Page-specific CSS ----
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("badge_demo.css"));
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
    container.add_css_class("badge-demo-layout");
    container.set_margin_top(24);
    container.set_margin_bottom(48);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_spacing(0);

    // ---- Page header ----
    let title = gtk4::Label::new(Some("Badge Demo"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    let desc = gtk4::Label::new(Some(LONG_TEXT));
    desc.add_css_class("relm4-page-subtitle");
    desc.set_wrap(true);
    desc.set_xalign(0.0);
    desc.set_margin_bottom(16);
    container.append(&desc);

    // ========================================================================
    // Section 1: All variants — Medium size
    // ========================================================================
    let variants_label = gtk4::Label::new(Some("Variants (Medium size)"));
    variants_label.add_css_class("badge-demo-section-label");
    variants_label.set_halign(gtk4::Align::Start);
    container.append(&variants_label);

    let variants_gallery = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    variants_gallery.add_css_class("badge-demo-gallery");

    // Row of labels for each variant.
    let var_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    var_row.set_halign(gtk4::Align::Start);

    for (variant, name, _desc) in VARIANTS {
        let cell = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        cell.add_css_class("badge-demo-cell");
        cell.set_halign(gtk4::Align::Center);

        let badge = Badge::new()
            .text(*name)
            .variant(*variant)
            .size(BadgeSize::Medium)
            .build();
        cell.append(&badge);

        let cell_lbl = gtk4::Label::new(Some(name));
        cell_lbl.add_css_class("badge-demo-cell-label");
        cell.append(&cell_lbl);

        var_row.append(&cell);
    }
    variants_gallery.append(&var_row);

    // Sample labels row (example use-case labels).
    let samples_label = gtk4::Label::new(Some("Example labels (Medium)"));
    samples_label.add_css_class("badge-demo-subsection-label");
    samples_label.set_halign(gtk4::Align::Start);
    samples_label.set_margin_top(12);
    samples_label.set_margin_bottom(4);
    variants_gallery.append(&samples_label);

    let samples_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    samples_row.set_halign(gtk4::Align::Start);


    // Pair each label with a cycling variant.
    for (i, label) in LABELS.iter().enumerate() {
        let variant = match i % 5 {
            0 => BadgeVariant::Success,
            1 => BadgeVariant::Danger,
            2 => BadgeVariant::Warning,
            3 => BadgeVariant::Info,
            _ => BadgeVariant::Neutral,
        };
        let badge = Badge::new()
            .text(*label)
            .variant(variant)
            .size(BadgeSize::Medium)
            .build();
        samples_row.append(&badge);
    }
    variants_gallery.append(&samples_row);

    container.append(&variants_gallery);

    // ========================================================================
    // Section 2: Sizes — same variant displayed at Small and Medium
    // ========================================================================
    let sizes_label = gtk4::Label::new(Some("Sizes (Small vs Medium)"));
    sizes_label.add_css_class("badge-demo-section-label");
    sizes_label.set_halign(gtk4::Align::Start);
    sizes_label.set_margin_top(24);
    container.append(&sizes_label);

    let sizes_gallery = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    sizes_gallery.add_css_class("badge-demo-gallery");

    // For each variant, show small + medium side-by-side.
    for (variant, name, _desc) in VARIANTS {
        let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 20);
        row.set_halign(gtk4::Align::Start);
        row.set_margin_top(2);
        row.set_margin_bottom(2);

        // Variant name label.
        let name_lbl = gtk4::Label::new(Some(*name));
        name_lbl.set_width_chars(10);
        name_lbl.set_xalign(1.0);
        name_lbl.add_css_class("badge-demo-cell-label");
        row.append(&name_lbl);

        for (size, size_name) in SIZES {
            let cell = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
            cell.set_halign(gtk4::Align::Center);

            let badge = Badge::new()
                .text(*name)
                .variant(*variant)
                .size(*size)
                .build();
            cell.append(&badge);

            let size_lbl = gtk4::Label::new(Some(size_name));
            size_lbl.add_css_class("badge-demo-cell-label");
            cell.append(&size_lbl);

            row.append(&cell);
        }
        sizes_gallery.append(&row);
    }

    container.append(&sizes_gallery);

    // ========================================================================
    // Section 3: Badges on icons (mocked)
    // ========================================================================
    let icon_label = gtk4::Label::new(Some("Badges on Icons (mocked)"));
    icon_label.add_css_class("badge-demo-section-label");
    icon_label.set_halign(gtk4::Align::Start);
    icon_label.set_margin_top(24);
    container.append(&icon_label);

    let icon_mock = gtk4::Box::new(gtk4::Orientation::Horizontal, 32);
    icon_mock.add_css_class("badge-demo-icon-mock");
    icon_mock.set_halign(gtk4::Align::Start);
    icon_mock.set_hexpand(true);

    // App icon mockups with badges.
    let app_icons: &[(&str, &str, BadgeVariant)] = &[
        ("Inbox", "99+", BadgeVariant::Danger),
        ("Updates", "3", BadgeVariant::Success),
        ("Alerts", "7", BadgeVariant::Warning),
        ("Messages", "12", BadgeVariant::Info),
        ("Tasks", "5", BadgeVariant::Neutral),
    ];

    for (app_name, badge_text, variant) in app_icons {
        let col = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        col.set_halign(gtk4::Align::Center);

        // Icon frame with badge overlay.
        let frame = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        frame.add_css_class("badge-demo-icon-frame");
        frame.set_halign(gtk4::Align::Center);

        // Placeholder icon.
        let icon = gtk4::Image::from_icon_name("applications-graphics-symbolic");
        icon.set_pixel_size(32);
        frame.append(&icon);

        // Badge positioned via overlay.
        let badge = Badge::new()
            .text(*badge_text)
            .variant(*variant)
            .size(BadgeSize::Small)
            .build();
        badge.add_css_class("badge-demo-overlay");
        badge.set_halign(gtk4::Align::End);
        badge.set_valign(gtk4::Align::Start);
        // We use a fixed container to overlay badge at top-right.
        let overlay = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        overlay.set_halign(gtk4::Align::End);
        overlay.set_valign(gtk4::Align::Start);
        overlay.append(&badge);

        let fixed = gtk4::Fixed::new();
        fixed.set_size_request(48, 48);
        fixed.put(&frame, 0.0, 0.0);
        fixed.put(&overlay, 24.0, -4.0);

        col.append(&fixed);

        let app_lbl = gtk4::Label::new(Some(app_name));
        app_lbl.add_css_class("badge-demo-cell-label");
        col.append(&app_lbl);

        icon_mock.append(&col);
    }

    container.append(&icon_mock);

    // ========================================================================
    // Section 4: Animated badge counter
    // ========================================================================
    let counter_label = gtk4::Label::new(Some("Animated Badge Counter"));
    counter_label.add_css_class("badge-demo-section-label");
    counter_label.set_halign(gtk4::Align::Start);
    counter_label.set_margin_top(24);
    container.append(&counter_label);

    let counter_card = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    counter_card.add_css_class("badge-demo-counter-card");
    counter_card.set_halign(gtk4::Align::Fill);
    counter_card.set_hexpand(true);

    // Instruction text.
    let counter_desc = gtk4::Label::new(Some(
        "This counter auto-increments every second — the badge updates in real time.",
    ));
    counter_desc.set_wrap(true);
    counter_desc.set_xalign(0.0);
    counter_desc.add_css_class("badge-demo-status");
    counter_card.append(&counter_desc);

    // Large count display.
    let count_label = gtk4::Label::new(Some("0"));
    count_label.add_css_class("badge-demo-counter-value");
    count_label.set_halign(gtk4::Align::Center);
    counter_card.append(&count_label);

    // Badge showing the current count.
    let live_badge = Badge::new()
        .text("0")
        .variant(BadgeVariant::Danger)
        .size(BadgeSize::Medium)
        .build();
    live_badge.set_halign(gtk4::Align::Center);
    live_badge.set_margin_bottom(8);
    counter_card.append(&live_badge);

    // Control buttons row.
    let controls = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    controls.set_halign(gtk4::Align::Center);

    let start_btn = gtk4::Button::with_label("Start");
    start_btn.add_css_class("relm4-btn");
    start_btn.add_css_class("relm4-btn-primary");
    controls.append(&start_btn);

    let stop_btn = gtk4::Button::with_label("Stop");
    stop_btn.add_css_class("relm4-btn");
    stop_btn.add_css_class("relm4-btn-secondary");
    controls.append(&stop_btn);

    let reset_btn = gtk4::Button::with_label("Reset");
    reset_btn.add_css_class("relm4-btn");
    reset_btn.add_css_class("relm4-btn-ghost");
    controls.append(&reset_btn);

    counter_card.append(&controls);
    container.append(&counter_card);

    // ---- Wire up animated counter ----
    // Use Rc for shared state that multiple closures need.
    let count = std::rc::Rc::new(std::cell::RefCell::new(0u32));
    let source_id = std::rc::Rc::new(std::cell::RefCell::new(None::<glib::SourceId>));
    let live_badge_c = std::rc::Rc::new(live_badge.clone());
    let count_label_c = std::rc::Rc::new(count_label.clone());

    // Start button.
    {
        let count = count.clone();
        let source_id = source_id.clone();
        let live_badge_c = live_badge_c.clone();
        let count_label_c = count_label_c.clone();

        start_btn.connect_clicked(move |_| {
            if source_id.borrow().is_some() {
                return; // Already running.
            }

            let badge = live_badge_c.clone();
            let label = count_label_c.clone();
            let count_inner = count.clone();

            let id = glib::timeout_add_seconds_local(1, move || {
                let mut c = count_inner.borrow_mut();
                *c += 1;
                let text = c.to_string();
                badge.set_text(&text);
                label.set_text(&text);

                // Cycle the badge variant as the count grows for visual variety.
                let variant = match *c % 5 {
                    0 => BadgeVariant::Success,
                    1 => BadgeVariant::Danger,
                    2 => BadgeVariant::Warning,
                    3 => BadgeVariant::Info,
                    _ => BadgeVariant::Neutral,
                };
                // Apply variant class: remove all, add the new one.
                for cls in &[
                    "badge-success", "badge-danger", "badge-warning",
                    "badge-info", "badge-neutral",
                ] {
                    badge.remove_css_class(cls);
                }
                let new_cls = match variant {
                    BadgeVariant::Success => "badge-success",
                    BadgeVariant::Danger => "badge-danger",
                    BadgeVariant::Warning => "badge-warning",
                    BadgeVariant::Info => "badge-info",
                    BadgeVariant::Neutral => "badge-neutral",
                };
                badge.add_css_class(new_cls);

                glib::ControlFlow::Continue
            });
            *source_id.borrow_mut() = Some(id);
        });
    }

    // Stop button.
    {
        let source_id = source_id.clone();
        stop_btn.connect_clicked(move |_| {
            if let Some(id) = source_id.borrow_mut().take() {
                id.remove();
            }
        });
    }

    // Reset button.
    {
        let count = count.clone();
        let live_badge_c = live_badge_c.clone();
        let count_label_c = count_label_c.clone();
        reset_btn.connect_clicked(move |_| {
            // Clear the counter.
            *count.borrow_mut() = 0u32;
            live_badge_c.set_text("0");
            count_label_c.set_text("0");
            // Reset badge variant to Danger.
            for cls in &[
                "badge-success", "badge-danger", "badge-warning",
                "badge-info", "badge-neutral",
            ] {
                live_badge_c.remove_css_class(cls);
            }
            live_badge_c.add_css_class("badge-danger");
        });
    }

    // ========================================================================
    // Code panel
    // ========================================================================
    let code_label = gtk4::Label::new(Some("Source Code Example"));
    code_label.add_css_class("badge-demo-section-label");
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

/// Build a styled code panel showing the Badge API in action.
fn create_code_panel() -> gtk4::Box {
    let panel = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    panel.add_css_class("badge-demo-code-panel");
    panel.set_hexpand(true);

    // Title bar.
    let title_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_bar.add_css_class("badge-demo-code-title");
    panel.append(&title_bar);

    let title_text = gtk4::Label::new(Some("Rust — Badge Builder"));
    title_text.add_css_class("badge-demo-code-title-text");
    title_text.set_halign(gtk4::Align::Start);
    title_bar.append(&title_text);

    // Code content.
    let code_text = gtk4::Label::new(Some(CODE_SAMPLE));
    code_text.add_css_class("badge-demo-code-text");
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

/// A complete working example of Badge usage.
const CODE_SAMPLE: &str = r#"use relm4_kit::primitives::{Badge, BadgeVariant, BadgeSize};

// Simple badge (default: Neutral, Medium)
let badge = Badge::new()
    .text("New")
    .build();

// Status badge
let badge = Badge::new()
    .text("Online")
    .variant(BadgeVariant::Success)
    .build();

// Danger badge (small)
let badge = Badge::new()
    .text("3")
    .variant(BadgeVariant::Danger)
    .size(BadgeSize::Small)
    .build();

// Warning badge
let badge = Badge::new()
    .text("Expiring")
    .variant(BadgeVariant::Warning)
    .build();

// Info badge
let badge = Badge::new()
    .text("Updated")
    .variant(BadgeVariant::Info)
    .build();
"#;
