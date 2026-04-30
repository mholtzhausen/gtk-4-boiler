//! Toast Demo page — interactive preview of all toast notification kinds,
//! action-button support, and stacking behaviour.
//!
//! This page creates a standalone [`ToastStack`] component that wraps the
//! demo controls, so that clicking a button sends a real toast notification
//! into the stack.

use std::sync::atomic::{AtomicU32, Ordering};

use gtk4::prelude::*;
use relm4::{Component, ComponentController, Controller};
use relm4_kit::containers::{ToastKind, ToastStack, ToastStackMsg};

// ============================================================================
// Constants
// ============================================================================

/// Description shown at the top of the page.
const LONG_TEXT: &str =
    "Toast notifications provide non-blocking feedback for user actions. \
     They auto-dismiss after 4 seconds and stack up to 3 at a time.\n\
     \u{2022} Success — completed operations (green accent)\n\
     \u{2022} Error — failed operations (red accent)\n\
     \u{2022} Warning — non-critical issues (yellow accent)\n\
     \u{2022} Info — general information (blue accent)";

/// All toast kind definitions for the demo grid.
const TOAST_KINDS: &[(ToastKind, &str, &str, &str)] = &[
    (ToastKind::Success, "Success", "emblem-ok-symbolic", "File saved successfully."),
    (ToastKind::Error, "Error", "dialog-error-symbolic", "Connection to server lost."),
    (ToastKind::Warning, "Warning", "dialog-warning-symbolic", "Disk space is running low."),
    (ToastKind::Info, "Info", "dialog-information-symbolic", "Update v2.1.0 is available."),
];

/// Messages for the "with action" row.
const ACTION_MESSAGES: &[(ToastKind, &str, &str, &str)] = &[
    (ToastKind::Success, "Undo", "edit-undo-symbolic", "Item moved to trash."),
    (ToastKind::Error, "Retry", "view-refresh-symbolic", "Failed to sync changes."),
    (ToastKind::Warning, "Dismiss", "window-close-symbolic", "Battery at 15% — plug in soon."),
    (ToastKind::Info, "Update", "software-update-available-symbolic", "A new version is ready to install."),
];

// ============================================================================
// Public API
// ============================================================================

/// Build the toast demo page widget.
///
/// The returned widget is an outer [`gtk4::Box`] containing a
/// `ScrolledWindow` which in turn holds a standalone [`ToastStack`]
/// wrapping all the demo controls.
pub fn create() -> gtk4::Box {
    // ---- Page-specific CSS ----
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("toast_demo.css"));
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default()
            .expect("no display for CSS provider"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // ---- Outer layout ----
    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer.set_vexpand(true);
    outer.set_hexpand(true);
    outer.add_css_class("relm4-page");

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    outer.append(&scrolled);

    // ---- Content box (will be wrapped by the ToastStack overlay) ----
    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    content.add_css_class("toast-demo-layout");
    content.set_margin_top(24);
    content.set_margin_bottom(48);
    content.set_margin_start(24);
    content.set_margin_end(24);
    content.set_spacing(0);

    // ---- Create a standalone ToastStack wrapping the content ----
    // We create this *before* populating content so we can capture the
    // sender in button closures.  The controller is then leaked to keep
    // the runtime alive for the window's lifetime.
    let toasts: Controller<ToastStack> = ToastStack::builder()
        .launch(content.clone().upcast::<gtk4::Widget>())
        .detach();
    let toast_sender = toasts.sender().clone();

    // ====================================================================
    // Page header
    // ====================================================================
    let title = gtk4::Label::new(Some("Toast Demo"));
    title.add_css_class("relm4-page-title");
    content.append(&title);

    let desc = gtk4::Label::new(Some(LONG_TEXT));
    desc.add_css_class("relm4-page-subtitle");
    desc.set_wrap(true);
    desc.set_xalign(0.0);
    desc.set_margin_bottom(16);
    content.append(&desc);

    // ====================================================================
    // Section 1: Trigger by kind
    // ====================================================================
    let section1_label = gtk4::Label::new(Some("Trigger by Kind"));
    section1_label.add_css_class("toast-demo-section-label");
    section1_label.set_halign(gtk4::Align::Start);
    content.append(&section1_label);

    let kinds_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    kinds_row.add_css_class("toast-demo-row");
    kinds_row.set_halign(gtk4::Align::Start);
    kinds_row.set_margin_bottom(4);

    for (kind, _name, icon, message) in TOAST_KINDS {
        let btn = gtk4::Button::new();
        btn.add_css_class("relm4-btn");
        btn.add_css_class(match kind {
            ToastKind::Success => "relm4-btn-primary",
            ToastKind::Error => "relm4-btn-danger",
            ToastKind::Warning => "relm4-btn-secondary",
            ToastKind::Info => "relm4-btn-ghost",
        });

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        let img = gtk4::Image::from_icon_name(icon);
        img.set_icon_size(gtk4::IconSize::Normal);
        hbox.append(&img);

        let lbl = gtk4::Label::new(Some(message));
        hbox.append(&lbl);
        btn.set_child(Some(&hbox));

        let sender = toast_sender.clone();
        let msg = message.to_string();
        let k = *kind;
        btn.connect_clicked(move |_| {
            sender.emit(ToastStackMsg::Show(msg.clone(), k));
        });
        kinds_row.append(&btn);
    }
    content.append(&kinds_row);

    // ---- Kind badges (labels showing each kind) ----
    let badges_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    badges_row.set_halign(gtk4::Align::Start);
    badges_row.set_margin_top(4);

    for (kind, name, _icon, _msg) in TOAST_KINDS {
        let badge = gtk4::Label::new(Some(*name));
        badge.add_css_class("toast-demo-kind-badge");
        badge.add_css_class(match kind {
            ToastKind::Success => "toast-demo-kind-badge-success",
            ToastKind::Error => "toast-demo-kind-badge-error",
            ToastKind::Warning => "toast-demo-kind-badge-warning",
            ToastKind::Info => "toast-demo-kind-badge-info",
        });
        badges_row.append(&badge);
    }
    content.append(&badges_row);

    // ====================================================================
    // Section 2: With action button
    // ====================================================================
    let sep1 = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep1.set_margin_top(20);
    sep1.set_margin_bottom(16);
    content.append(&sep1);

    let section2_label = gtk4::Label::new(Some("With Action Button"));
    section2_label.add_css_class("toast-demo-section-label");
    section2_label.set_halign(gtk4::Align::Start);
    content.append(&section2_label);

    let action_desc = gtk4::Label::new(
        Some("Toasts can include an action button (e.g. \"Undo\", \"Retry\") \
              for follow-up actions. Click the action button to see it emit \
              an output signal."),
    );
    action_desc.add_css_class("toast-demo-description");
    action_desc.set_wrap(true);
    action_desc.set_xalign(0.0);
    action_desc.set_margin_bottom(8);
    content.append(&action_desc);

    let action_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    action_row.add_css_class("toast-demo-row");
    action_row.set_halign(gtk4::Align::Start);

    // A label that shows when an action is triggered.
    let action_feedback = gtk4::Label::new(Some("No action triggered yet."));
    action_feedback.add_css_class("toast-demo-description");
    action_feedback.set_wrap(true);
    action_feedback.set_xalign(0.0);
    action_feedback.set_margin_bottom(8);

    for (kind, action_label, icon, message) in ACTION_MESSAGES {
        let btn = gtk4::Button::new();
        btn.add_css_class("relm4-btn");
        btn.add_css_class(match kind {
            ToastKind::Success => "relm4-btn-primary",
            ToastKind::Error => "relm4-btn-danger",
            ToastKind::Warning => "relm4-btn-secondary",
            ToastKind::Info => "relm4-btn-ghost",
        });

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        let img = gtk4::Image::from_icon_name(icon);
        img.set_icon_size(gtk4::IconSize::Normal);
        hbox.append(&img);
        let lbl = gtk4::Label::new(Some(message));
        hbox.append(&lbl);
        btn.set_child(Some(&hbox));

        let sender = toast_sender.clone();
        let msg = message.to_string();
        let action = action_label.to_string();
        let k = *kind;
        let feedback = action_feedback.clone();
        btn.connect_clicked(move |_| {
            sender.emit(ToastStackMsg::ShowWithAction(
                msg.clone(),
                k,
                action.clone(),
            ));
            feedback.set_markup(
                &format!(
                    "<b>Sent:</b> <span color='#3584e4'>\"{msg}\"</span> \
                     <span color='#5e5c64'>with \"{action}\" action</span>",
                ),
            );
        });
        action_row.append(&btn);
    }
    content.append(&action_row);
    content.append(&action_feedback);

    // ====================================================================
    // Section 3: Rapid stacking
    // ====================================================================
    let sep2 = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep2.set_margin_top(20);
    sep2.set_margin_bottom(16);
    content.append(&sep2);

    let section3_label = gtk4::Label::new(Some("Rapid Stacking"));
    section3_label.add_css_class("toast-demo-section-label");
    section3_label.set_halign(gtk4::Align::Start);
    content.append(&section3_label);

    let stack_desc = gtk4::Label::new(
        Some("The stack limits visible toasts to 3 at a time. Click \
              \"Fire All\" to send 6 toasts in quick succession — the \
              oldest will be dismissed to make room for newer ones."),
    );
    stack_desc.add_css_class("toast-demo-description");
    stack_desc.set_wrap(true);
    stack_desc.set_xalign(0.0);
    stack_desc.set_margin_bottom(8);
    content.append(&stack_desc);

    let stack_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    stack_row.add_css_class("toast-demo-row");
    stack_row.set_halign(gtk4::Align::Start);
    stack_row.set_margin_bottom(4);

    // Fire all button — sends 6 toasts rapidly.
    let fire_all_btn = gtk4::Button::with_label("Fire All (6 toasts)");
    fire_all_btn.add_css_class("relm4-btn");
    fire_all_btn.add_css_class("relm4-btn-primary");
    {
        let sender = toast_sender.clone();
        fire_all_btn.connect_clicked(move |_| {
            let messages = [
                ("Toast #1 — Starting...", ToastKind::Info),
                ("Toast #2 — Processing...", ToastKind::Warning),
                ("Toast #3 — Almost done...", ToastKind::Success),
                ("Toast #4 — Extra info", ToastKind::Info),
                ("Toast #5 — Warning: nearing limit", ToastKind::Warning),
                ("Toast #6 — Final: oldest dismissed", ToastKind::Error),
            ];
            for (msg, kind) in &messages {
                sender.emit(ToastStackMsg::Show(msg.to_string(), *kind));
            }
        });
    }
    stack_row.append(&fire_all_btn);

    // Fire sequential button — one at a time with delay (can't do real
    // delay without async, so we just show a label suggesting it).
    let fire_seq_btn = gtk4::Button::with_label("Fire 3 (stack test)");
    fire_seq_btn.add_css_class("relm4-btn");
    fire_seq_btn.add_css_class("relm4-btn-secondary");
    {
        let sender = toast_sender.clone();
        fire_seq_btn.connect_clicked(move |_| {
            sender.emit(ToastStackMsg::Show(
                "Stack item #1".to_string(),
                ToastKind::Info,
            ));
            sender.emit(ToastStackMsg::Show(
                "Stack item #2".to_string(),
                ToastKind::Success,
            ));
            sender.emit(ToastStackMsg::Show(
                "Stack item #3".to_string(),
                ToastKind::Warning,
            ));
        });
    }
    stack_row.append(&fire_seq_btn);

    // Dismiss top button.
    let dismiss_top_btn = gtk4::Button::with_label("Dismiss Oldest");
    dismiss_top_btn.add_css_class("relm4-btn");
    dismiss_top_btn.add_css_class("relm4-btn-ghost");
    {
        let sender = toast_sender.clone();
        dismiss_top_btn.connect_clicked(move |_| {
            sender.emit(ToastStackMsg::DismissTop);
        });
    }
    stack_row.append(&dismiss_top_btn);

    content.append(&stack_row);

    // ---- Stack status counter (client-side tracking) ----
    let toast_count = std::sync::Arc::new(AtomicU32::new(0));
    let count_label = gtk4::Label::new(Some("Toasts sent this session: 0"));
    count_label.add_css_class("toast-demo-queue-label");
    count_label.set_halign(gtk4::Align::Start);
    count_label.set_margin_top(8);

    // Connect the count_label to increment on any button in this section.
    // We use a shared counter approach — wrap the count_label with an
    // updater for each button.
    {
        let count_label_clone = count_label.clone();
        let count_clone = toast_count.clone();
        fire_all_btn.connect_clicked(move |_| {
            let prev = count_clone.fetch_add(6, Ordering::Relaxed);
            count_label_clone.set_text(&format!("Toasts sent this session: {}", prev + 6));
        });
    }
    {
        let count_label_clone = count_label.clone();
        let count_clone = toast_count.clone();
        fire_seq_btn.connect_clicked(move |_| {
            let prev = count_clone.fetch_add(3, Ordering::Relaxed);
            count_label_clone.set_text(&format!("Toasts sent this session: {}", prev + 3));
        });
    }
    content.append(&count_label);

    // ====================================================================
    // Section 4: Custom message
    // ====================================================================
    let sep3 = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep3.set_margin_top(20);
    sep3.set_margin_bottom(16);
    content.append(&sep3);

    let section4_label = gtk4::Label::new(Some("Custom Toast"));
    section4_label.add_css_class("toast-demo-section-label");
    section4_label.set_halign(gtk4::Align::Start);
    content.append(&section4_label);

    let custom_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    custom_row.set_halign(gtk4::Align::Start);

    let entry = gtk4::Entry::new();
    entry.set_placeholder_text(Some("Type a custom message..."));
    entry.set_hexpand(true);
    entry.set_width_chars(40);
    custom_row.append(&entry);

    let send_btn = gtk4::Button::with_label("Send Info");
    send_btn.add_css_class("relm4-btn");
    send_btn.add_css_class("relm4-btn-primary");
    {
        let sender = toast_sender.clone();
        let entry_clone = entry.clone();
        send_btn.connect_clicked(move |_| {
            let text = entry_clone.text();
            let text = if text.is_empty() {
                "Custom toast — no message entered.".to_string()
            } else {
                text.to_string()
            };
            sender.emit(ToastStackMsg::Show(text, ToastKind::Info));
            entry_clone.set_text("");
        });
    }
    custom_row.append(&send_btn);

    let send_err_btn = gtk4::Button::with_label("Send Error");
    send_err_btn.add_css_class("relm4-btn");
    send_err_btn.add_css_class("relm4-btn-danger");
    {
        let sender = toast_sender.clone();
        let entry_clone = entry.clone();
        send_err_btn.connect_clicked(move |_| {
            let text = entry_clone.text();
            let text = if text.is_empty() {
                "Custom error — no message entered.".to_string()
            } else {
                text.to_string()
            };
            sender.emit(ToastStackMsg::Show(text, ToastKind::Error));
            entry_clone.set_text("");
        });
    }
    custom_row.append(&send_err_btn);

    content.append(&custom_row);

    // ====================================================================
    // Code panel
    // ====================================================================
    let sep4 = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep4.set_margin_top(24);
    sep4.set_margin_bottom(16);
    content.append(&sep4);

    let code_section_label = gtk4::Label::new(Some("Source Code Example"));
    code_section_label.add_css_class("toast-demo-section-label");
    code_section_label.set_halign(gtk4::Align::Start);
    content.append(&code_section_label);

    let code_panel = create_code_panel();
    content.append(&code_panel);

    // ---- Attach the ToastStack (overlay) to the scrolled window ----
    scrolled.set_child(Some(toasts.widget()));

    // ---- Leak the controller to keep the runtime alive ----
    // The controller's `detach()` call already makes the component runtime
    // independent; however, dropping the controller would still stop it.
    // We leak it so the ToastStack lives for the lifetime of the window.
    std::mem::forget(toasts);

    outer
}

// ============================================================================
// Code panel
// ============================================================================

/// Build a styled code panel showing the ToastStack API in action.
fn create_code_panel() -> gtk4::Box {
    let panel = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    panel.add_css_class("toast-demo-code-panel");
    panel.set_hexpand(true);

    // Title bar.
    let title_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_bar.add_css_class("toast-demo-code-title");
    panel.append(&title_bar);

    let title_text = gtk4::Label::new(Some("Rust — ToastStack API"));
    title_text.add_css_class("toast-demo-code-title-text");
    title_text.set_halign(gtk4::Align::Start);
    title_bar.append(&title_text);

    // Code content.
    let code_text = gtk4::Label::new(Some(CODE_SAMPLE));
    code_text.add_css_class("toast-demo-code-text");
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

/// A complete working example of ToastStack usage.
const CODE_SAMPLE: &str = r#"use relm4_kit::containers::{ToastStack, ToastStackMsg, ToastKind};

// Wrap your content widget in a ToastStack overlay:
let toasts = ToastStack::builder()
    .launch(content_widget.upcast::<gtk4::Widget>())
    .detach();
let sender = toasts.sender().clone();

// Show a simple toast:
sender.input(ToastStackMsg::Show(
    "File saved successfully.".into(),
    ToastKind::Success,
));

// Show a toast with an action button:
sender.input(ToastStackMsg::ShowWithAction(
    "Item moved to trash.".into(),
    ToastKind::Success,
    "Undo".into(),
));

// Dismiss the oldest toast:
sender.input(ToastStackMsg::DismissTop);

// Dismiss a specific toast by id:
sender.input(ToastStackMsg::Dismiss(42));
"#;
