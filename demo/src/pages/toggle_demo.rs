//! Toggle Demo page — interactive preview of toggles with labels,
//! optional descriptions, active state display, and live feedback.
//!
//! Shows a variety of toggle configurations and an interactive area
//! that logs state changes in real time.

use gtk4::prelude::*;
use relm4_kit::primitives::Toggle;

// ============================================================================
// Constants
// ============================================================================

const LONG_TEXT: &str =
    "Toggles (switches) are used for binary on/off settings. They are \
     preferable to checkboxes when the setting takes immediate effect.\n\
     \u{2022} A title clearly labels what is being toggled.\n\
     \u{2022} An optional description provides additional context.\n\
     \u{2022} The switch reflects the current state and animates on change.";

// ============================================================================
// Public API
// ============================================================================

/// Build the toggle demo page widget.
pub fn create() -> gtk4::Box {
    // ---- Page-specific CSS ----
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("toggle_demo.css"));
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
    container.add_css_class("toggle-demo-layout");
    container.set_margin_top(24);
    container.set_margin_bottom(48);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_spacing(0);

    // ---- Page header ----
    let title = gtk4::Label::new(Some("Toggle Demo"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    let desc = gtk4::Label::new(Some(LONG_TEXT));
    desc.add_css_class("relm4-page-subtitle");
    desc.set_wrap(true);
    desc.set_xalign(0.0);
    desc.set_margin_bottom(16);
    container.append(&desc);

    // ========================================================================
    // Section 1: Basic toggles (title only)
    // ========================================================================
    let basic_label = gtk4::Label::new(Some("Basic Toggles (title only)"));
    basic_label.add_css_class("toggle-demo-section-label");
    basic_label.set_halign(gtk4::Align::Start);
    container.append(&basic_label);

    let basic_group = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    basic_group.add_css_class("toggle-demo-card");

    let toggle1 = Toggle::<()>::new()
        .title("Enable notifications")
        .active(true)
        .build();
    basic_group.append(&toggle1);

    let toggle2 = Toggle::<()>::new()
        .title("Dark mode")
        .build();
    basic_group.append(&toggle2);

    let toggle3 = Toggle::<()>::new()
        .title("Auto-save")
        .active(true)
        .build();
    basic_group.append(&toggle3);

    let toggle4 = Toggle::<()>::new()
        .title("Show hidden files")
        .build();
    basic_group.append(&toggle4);

    container.append(&basic_group);

    // ========================================================================
    // Section 2: Toggles with descriptions
    // ========================================================================
    let desc_label = gtk4::Label::new(Some("Toggles with Descriptions"));
    desc_label.add_css_class("toggle-demo-section-label");
    desc_label.set_halign(gtk4::Align::Start);
    desc_label.set_margin_top(24);
    container.append(&desc_label);

    let desc_group = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    desc_group.add_css_class("toggle-demo-card");

    let toggle5 = Toggle::<()>::new()
        .title("Email notifications")
        .description("Receive email alerts when someone mentions you")
        .active(true)
        .build();
    desc_group.append(&toggle5);

    let toggle6 = Toggle::<()>::new()
        .title("Weekly digest")
        .description("A summary of activity delivered every Monday morning")
        .build();
    desc_group.append(&toggle6);

    let toggle7 = Toggle::<()>::new()
        .title("Beta features")
        .description("Try out experimental features before they are released to everyone")
        .build();
    desc_group.append(&toggle7);

    container.append(&desc_group);

    // ========================================================================
    // Section 3: All on / all off (illustrating active state)
    // ========================================================================
    let active_label = gtk4::Label::new(Some("Active vs Inactive State"));
    active_label.add_css_class("toggle-demo-section-label");
    active_label.set_halign(gtk4::Align::Start);
    active_label.set_margin_top(24);
    container.append(&active_label);

    let active_group = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    active_group.add_css_class("toggle-demo-card");

    // Long descriptions to show wrapping.
    let toggle8 = Toggle::<()>::new()
        .title("Performance mode")
        .description("Increases CPU priority for the current task. May reduce battery life.")
        .active(true)
        .build();
    active_group.append(&toggle8);

    let toggle9 = Toggle::<()>::new()
        .title("Reduce motion")
        .description("Minimises animations and transitions throughout the interface for accessibility")
        .build();
    active_group.append(&toggle9);

    container.append(&active_group);

    // ========================================================================
    // Section 4: Interactive live-feedback
    // ========================================================================
    let interactive_label = gtk4::Label::new(Some("Interactive — toggle switches to see state changes"));
    interactive_label.add_css_class("toggle-demo-section-label");
    interactive_label.set_halign(gtk4::Align::Start);
    interactive_label.set_margin_top(24);
    container.append(&interactive_label);

    let status_label = gtk4::Label::new(Some("Toggle any switch below to see state changes here."));
    status_label.add_css_class("toggle-demo-status");
    status_label.set_wrap(true);
    status_label.set_xalign(0.0);
    status_label.set_margin_bottom(4);
    container.append(&status_label);

    let state_log = gtk4::Label::new(Some("Last change: none"));
    state_log.add_css_class("toggle-demo-state-log");
    state_log.set_wrap(true);
    state_log.set_xalign(0.0);
    state_log.set_margin_bottom(8);
    container.append(&state_log);

    let interactive_group = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    interactive_group.add_css_class("toggle-demo-card");

    // Interactive toggles — we use gtk4::Switch directly so we can wire
    // GTK signals to update the status labels.
    let interactive_toggles = [
        ("Wi-Fi", "Connect to wireless networks", true),
        ("Bluetooth", "Enable Bluetooth devices", false),
        ("Airplane mode", "Disable all wireless communications", false),
        ("Do not disturb", "Silence all notifications", false),
        ("Location services", "Allow apps to request your location", true),
    ];

    for (title_text, desc_text, initial) in &interactive_toggles {
        let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row.set_halign(gtk4::Align::Fill);
        row.set_margin_top(4);
        row.set_margin_bottom(4);

        // Label area (left).
        let label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        label_box.set_hexpand(true);
        label_box.set_halign(gtk4::Align::Start);
        label_box.set_valign(gtk4::Align::Center);

        let title_lbl = gtk4::Label::new(Some(title_text));
        title_lbl.add_css_class("toggle-title");
        title_lbl.set_halign(gtk4::Align::Start);
        title_lbl.set_xalign(0.0);
        title_lbl.set_wrap(true);
        label_box.append(&title_lbl);

        let desc_lbl = gtk4::Label::new(Some(desc_text));
        desc_lbl.add_css_class("toggle-description");
        desc_lbl.set_halign(gtk4::Align::Start);
        desc_lbl.set_xalign(0.0);
        desc_lbl.set_wrap(true);
        label_box.append(&desc_lbl);

        row.append(&label_box);

        // Switch (right).
        let sw = gtk4::Switch::new();
        sw.set_valign(gtk4::Align::Center);
        sw.set_active(*initial);

        let status_clone = status_label.clone();
        let log_clone = state_log.clone();
        let title_owned = title_text.to_string();
        sw.connect_active_notify(move |switch| {
            let is_on = switch.is_active();
            let state_str = if is_on { "ON" } else { "OFF" };
            status_clone.set_markup(
                &format!(
                    "<b>{title_owned}</b> is now <span color='{}'>{state_str}</span>",
                    if is_on { "#33d17a" } else { "#e66156" },
                ),
            );
            log_clone.set_text(&format!("Last change: \"{title_owned}\" → {state_str}"));
        });

        row.append(&sw);
        interactive_group.append(&row);
    }

    container.append(&interactive_group);

    // ---- Separator ----
    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.set_margin_top(24);
    sep.set_margin_bottom(16);
    container.append(&sep);

    // ========================================================================
    // Code panel
    // ========================================================================
    let code_label = gtk4::Label::new(Some("Source Code Example"));
    code_label.add_css_class("toggle-demo-section-label");
    code_label.set_halign(gtk4::Align::Start);
    container.append(&code_label);

    let code_panel = create_code_panel();
    container.append(&code_panel);

    scrolled.set_child(Some(&container));
    outer
}

// ============================================================================
// Code panel
// ============================================================================

/// Build a styled code panel showing the Toggle API in action.
fn create_code_panel() -> gtk4::Box {
    let panel = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    panel.add_css_class("toggle-demo-code-panel");
    panel.set_hexpand(true);

    // Title bar.
    let title_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_bar.add_css_class("toggle-demo-code-title");
    panel.append(&title_bar);

    let title_text = gtk4::Label::new(Some("Rust — Toggle Builder"));
    title_text.add_css_class("toggle-demo-code-title-text");
    title_text.set_halign(gtk4::Align::Start);
    title_bar.append(&title_text);

    // Code content.
    let code_text = gtk4::Label::new(Some(CODE_SAMPLE));
    code_text.add_css_class("toggle-demo-code-text");
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

/// A complete working example of Toggle usage.
const CODE_SAMPLE: &str = r#"use relm4_kit::primitives::Toggle;

// Basic toggle (title only, initial state = off)
let toggle = Toggle::new()
    .title("Enable notifications")
    .build();

// Toggle with description (initial state = on)
let toggle = Toggle::new()
    .title("Dark mode")
    .description("Use dark background colours throughout")
    .active(true)
    .build();

// Toggle with callback
let toggle = Toggle::new()
    .title("Auto-save")
    .description("Automatically save your work every 5 minutes")
    .active(true)
    .on_toggle(|is_active| Msg::Toggled(is_active))
    .build();
"#;
