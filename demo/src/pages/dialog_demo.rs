//! Dialog Demo page — interactive preview of confirm, info, and custom
//! modal dialogs.
//!
//! Each section provides a button that opens a modal dialog.  The user's
//! choice (confirm/cancel) is displayed in a result label below the
//! button.

use std::rc::Rc;

use gtk4::prelude::*;
use relm4_kit::containers::Dialog;

// ============================================================================
// Constants
// ============================================================================

const LONG_TEXT: &str =
    "Modal dialogs require user attention before returning to the main \
     interface.  Use them for confirmations, alerts, and critical prompts.\n\
     \u{2022} Confirm — ask the user to confirm a potentially destructive action.\n\
     \u{2022} Info — present information that the user must acknowledge.\n\
     \u{2022} Custom — tailor the button labels and appearance for your use case.";

// ============================================================================
// Public API
// ============================================================================

/// Build the dialog demo page widget.
///
/// `parent_window` is used as the transient parent for modal dialogs.
/// Accepts any widget that is part of the window hierarchy (typically the
/// [`libadwaita::ApplicationWindow`] from the app shell).
pub fn create(parent_window: &impl IsA<gtk4::Widget>) -> gtk4::Box {
    // ---- Page-specific CSS ----
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("dialog_demo.css"));
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("no display for CSS provider"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // ---- Outer layout ----
    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer.set_vexpand(true);
    outer.set_hexpand(true);
    outer.add_css_class("relm4-page");

    // ---- Scrolled content ----
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    outer.append(&scrolled);

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.set_margin_top(24);
    container.set_margin_bottom(48);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_spacing(0);

    scrolled.set_child(Some(&container));

    // Wrap the parent in an Rc so it can be shared across closures.
    // We clone the widget reference (the underlying GTK object is
    // ref-counted by glib).
    let parent = Rc::new(parent_window.clone().upcast::<gtk4::Widget>());

    // ====================================================================
    // Page header
    // ====================================================================
    let title = gtk4::Label::new(Some("Dialog Demo"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    let desc = gtk4::Label::new(Some(LONG_TEXT));
    desc.add_css_class("relm4-page-subtitle");
    desc.set_wrap(true);
    desc.set_xalign(0.0);
    desc.set_margin_bottom(16);
    container.append(&desc);

    // ====================================================================
    // Section 1: Confirm Dialog
    // ====================================================================
    let section1_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    section1_sep.set_margin_bottom(16);
    container.append(&section1_sep);

    let section1_header = gtk4::Label::new(Some("Confirm Dialog"));
    section1_header.add_css_class("dialog-demo-section-header");
    section1_header.set_halign(gtk4::Align::Start);
    section1_header.set_margin_bottom(8);
    container.append(&section1_header);

    let section1_desc = gtk4::Label::new(
        Some("Prompt the user to confirm a critical action before proceeding. \
              The \"Dangerous\" variant styles the confirm button with a \
              red accent to indicate destructiveness."),
    );
    section1_desc.add_css_class("dialog-demo-description");
    section1_desc.set_wrap(true);
    section1_desc.set_xalign(0.0);
    section1_desc.set_margin_bottom(12);
    container.append(&section1_desc);

    // ---- Result label for section 1 ----
    let confirm_result = gtk4::Label::new(Some("No dialog result yet."));
    confirm_result.add_css_class("dialog-demo-result");
    confirm_result.set_halign(gtk4::Align::Start);
    confirm_result.set_margin_bottom(8);

    // ---- Button: Delete File (dangerous) ----
    add_dangerous_dialog_button(&container, &confirm_result, &parent);
    // ---- Button: Save Changes (normal confirm) ----
    add_normal_confirm_button(&container, &confirm_result, &parent);
    container.append(&confirm_result);

    // ====================================================================
    // Section 2: Info Dialog
    // ====================================================================
    let section2_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    section2_sep.add_css_class("dialog-demo-separator");
    container.append(&section2_sep);

    let section2_header = gtk4::Label::new(Some("Info Dialog"));
    section2_header.add_css_class("dialog-demo-section-header");
    section2_header.set_halign(gtk4::Align::Start);
    section2_header.set_margin_bottom(8);
    container.append(&section2_header);

    let section2_desc = gtk4::Label::new(
        Some("Present information that the user must acknowledge. Info dialogs \
              have a single \"OK\" button by default — no cancel option."),
    );
    section2_desc.add_css_class("dialog-demo-description");
    section2_desc.set_wrap(true);
    section2_desc.set_xalign(0.0);
    section2_desc.set_margin_bottom(12);
    container.append(&section2_desc);

    // ---- Result label for section 2 ----
    let info_result = gtk4::Label::new(Some("No dialog result yet."));
    info_result.add_css_class("dialog-demo-result");
    info_result.set_halign(gtk4::Align::Start);
    info_result.set_margin_bottom(8);

    // ---- Buttons ----
    add_update_info_button(&container, &info_result, &parent);
    add_welcome_info_button(&container, &info_result, &parent);
    container.append(&info_result);

    // ====================================================================
    // Section 3: Custom Dialog
    // ====================================================================
    let section3_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    section3_sep.add_css_class("dialog-demo-separator");
    container.append(&section3_sep);

    let section3_header = gtk4::Label::new(Some("Custom Dialog"));
    section3_header.add_css_class("dialog-demo-section-header");
    section3_header.set_halign(gtk4::Align::Start);
    section3_header.set_margin_bottom(8);
    container.append(&section3_header);

    let section3_desc = gtk4::Label::new(
        Some("Customise the dialog buttons, labels, and appearance for \
              specialised use cases.  The example below combines custom \
              confirm/cancel text with a dangerous action style."),
    );
    section3_desc.add_css_class("dialog-demo-description");
    section3_desc.set_wrap(true);
    section3_desc.set_xalign(0.0);
    section3_desc.set_margin_bottom(12);
    container.append(&section3_desc);

    // ---- Result label for section 3 ----
    let custom_result = gtk4::Label::new(Some("No dialog result yet."));
    custom_result.add_css_class("dialog-demo-result");
    custom_result.set_halign(gtk4::Align::Start);
    custom_result.set_margin_bottom(8);

    add_custom_dialog_button(&container, &custom_result, &parent);
    container.append(&custom_result);

    // ====================================================================
    // Section 4: Code Panel
    // ====================================================================
    let code_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    code_sep.add_css_class("dialog-demo-separator");
    container.append(&code_sep);

    let code_header = gtk4::Label::new(Some("Source Code Example"));
    code_header.add_css_class("dialog-demo-section-header");
    code_header.set_halign(gtk4::Align::Start);
    code_header.set_margin_bottom(8);
    container.append(&code_header);

    let code_panel = create_code_panel();
    container.append(&code_panel);

    outer
}

// ============================================================================
// Helper: add a dangerous confirmation button
// ============================================================================

fn add_dangerous_dialog_button(
    container: &gtk4::Box,
    result: &gtk4::Label,
    parent: &Rc<gtk4::Widget>,
) {
    let btn = gtk4::Button::with_label("Delete File (Dangerous)");
    btn.add_css_class("relm4-btn");
    btn.add_css_class("relm4-btn-danger");
    btn.set_margin_bottom(4);

    let result = Rc::new(result.clone());
    let parent = Rc::clone(parent);
    btn.connect_clicked(move |_| {
        let confirm_result = Rc::clone(&result);
        let cancel_result = Rc::clone(&result);
        let parent = Rc::clone(&parent);
        Dialog::confirm("Delete File", "This action cannot be undone. The file will be permanently removed.")
            .confirm_label("Delete")
            .cancel_label("Keep File")
            .dangerous(true)
            .on_confirm(move || {
                confirm_result.set_markup(
                    "<b>Confirmed:</b> <span color='#e66156'>Delete File</span> — file was deleted."
                );
                confirm_result.remove_css_class("dialog-demo-result-cancelled");
                confirm_result.add_css_class("dialog-demo-result-danger");
            })
            .on_cancel(move || {
                cancel_result.set_markup(
                    "<b>Cancelled:</b> <span color='#5e5c64'>Delete File</span> — file was kept."
                );
                cancel_result.remove_css_class("dialog-demo-result-danger");
                cancel_result.add_css_class("dialog-demo-result-cancelled");
            })
            .present(&*parent);
    });
    container.append(&btn);
}

// ============================================================================
// Helper: add a normal confirmation button
// ============================================================================

fn add_normal_confirm_button(
    container: &gtk4::Box,
    result: &gtk4::Label,
    parent: &Rc<gtk4::Widget>,
) {
    let btn = gtk4::Button::with_label("Confirm Save Changes");
    btn.add_css_class("relm4-btn");
    btn.add_css_class("relm4-btn-primary");
    btn.set_margin_bottom(4);

    let result = Rc::new(result.clone());
    let parent = Rc::clone(parent);
    btn.connect_clicked(move |_| {
        let confirm_result = Rc::clone(&result);
        let cancel_result = Rc::clone(&result);
        let parent = Rc::clone(&parent);
        Dialog::confirm("Unsaved Changes", "You have unsaved changes. Do you want to save them before continuing?")
            .confirm_label("Save")
            .cancel_label("Discard")
            .on_confirm(move || {
                confirm_result.set_markup(
                    "<b>Confirmed:</b> <span color='#33d17a'>Save Changes</span> — changes saved."
                );
                confirm_result.remove_css_class("dialog-demo-result-cancelled");
                confirm_result.remove_css_class("dialog-demo-result-danger");
                confirm_result.add_css_class("dialog-demo-result-success");
            })
            .on_cancel(move || {
                cancel_result.set_markup(
                    "<b>Cancelled:</b> <span color='#5e5c64'>Discard Changes</span> — changes lost."
                );
                cancel_result.remove_css_class("dialog-demo-result-success");
                cancel_result.remove_css_class("dialog-demo-result-danger");
                cancel_result.add_css_class("dialog-demo-result-cancelled");
            })
            .present(&*parent);
    });
    container.append(&btn);
}

// ============================================================================
// Helper: add an update info button
// ============================================================================

fn add_update_info_button(
    container: &gtk4::Box,
    result: &gtk4::Label,
    parent: &Rc<gtk4::Widget>,
) {
    let btn = gtk4::Button::with_label("Show Info Dialog");
    btn.add_css_class("relm4-btn");
    btn.add_css_class("relm4-btn-secondary");
    btn.set_margin_bottom(4);

    let result = Rc::new(result.clone());
    let parent = Rc::clone(parent);
    btn.connect_clicked(move |_| {
        let result = Rc::clone(&result);
        let parent = Rc::clone(&parent);
        Dialog::info(
            "Application Updated",
            "Version 2.1.0 is now installed.\n\nNew features:\n• Improved performance\n• Bug fixes\n• New keyboard shortcuts",
        )
        .confirm_label("Got it")
        .on_confirm(move || {
            result.set_markup(
                "<b>Acknowledged:</b> <span color='#3584e4'>Update info</span> — user dismissed the dialog."
            );
            result.remove_css_class("dialog-demo-result-cancelled");
            result.add_css_class("dialog-demo-result-info");
        })
        .present(&*parent);
    });
    container.append(&btn);
}

// ============================================================================
// Helper: add a welcome info button
// ============================================================================

fn add_welcome_info_button(
    container: &gtk4::Box,
    result: &gtk4::Label,
    parent: &Rc<gtk4::Widget>,
) {
    let btn = gtk4::Button::with_label("Show Welcome Dialog");
    btn.add_css_class("relm4-btn");
    btn.add_css_class("relm4-btn-ghost");
    btn.set_margin_bottom(4);

    let result = Rc::new(result.clone());
    let parent = Rc::clone(parent);
    btn.connect_clicked(move |_| {
        let result = Rc::clone(&result);
        let parent = Rc::clone(&parent);
        Dialog::info(
            "Welcome to relm4-kit!",
            "A curated component library for GTK4 + relm4 applications.\n\nThis dialog demonstrates a simple info message with no secondary action button.",
        )
        .on_confirm(move || {
            result.set_markup(
                "<b>Acknowledged:</b> <span color='#3584e4'>Welcome dialog</span> — user clicked OK."
            );
            result.remove_css_class("dialog-demo-result-cancelled");
            result.add_css_class("dialog-demo-result-info");
        })
        .present(&*parent);
    });
    container.append(&btn);
}

// ============================================================================
// Helper: add a fully custom dialog button
// ============================================================================

fn add_custom_dialog_button(
    container: &gtk4::Box,
    result: &gtk4::Label,
    parent: &Rc<gtk4::Widget>,
) {
    let btn = gtk4::Button::with_label("Open Custom Dialog");
    btn.add_css_class("relm4-btn");
    btn.add_css_class("relm4-btn-primary");
    btn.set_margin_bottom(4);

    let result = Rc::new(result.clone());
    let parent = Rc::clone(parent);
    btn.connect_clicked(move |_| {
        let confirm_result = Rc::clone(&result);
        let cancel_result = Rc::clone(&result);
        let parent = Rc::clone(&parent);
        Dialog::confirm(
            "Reset All Settings",
            "This will restore all application settings to their default values.\n\n• Custom themes will be removed\n• Keyboard shortcuts will be reset\n• Data and files will not be affected",
        )
        .confirm_label("Reset Everything")
        .cancel_label("Keep My Settings")
        .dangerous(true)
        .on_confirm(move || {
            confirm_result.set_markup(
                "<b>Confirmed:</b> <span color='#e66156'>Reset Settings</span> — all settings restored to defaults."
            );
            confirm_result.remove_css_class("dialog-demo-result-cancelled");
            confirm_result.remove_css_class("dialog-demo-result-info");
            confirm_result.add_css_class("dialog-demo-result-danger");
        })
        .on_cancel(move || {
            cancel_result.set_markup(
                "<b>Cancelled:</b> <span color='#5e5c64'>Reset Settings</span> — settings were preserved."
            );
            cancel_result.remove_css_class("dialog-demo-result-danger");
            cancel_result.remove_css_class("dialog-demo-result-info");
            cancel_result.add_css_class("dialog-demo-result-cancelled");
        })
        .present(&*parent);
    });
    container.append(&btn);
}

// ============================================================================
// Code panel
// ============================================================================

/// Build a styled code panel showing the Dialog API in action.
fn create_code_panel() -> gtk4::Box {
    let panel = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    panel.add_css_class("dialog-demo-code-panel");
    panel.set_hexpand(true);

    // Title bar.
    let title_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_bar.add_css_class("toast-demo-code-title");
    panel.append(&title_bar);

    let title_text = gtk4::Label::new(Some("Rust — Dialog API"));
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

/// A complete working example of Dialog usage.
const CODE_SAMPLE: &str = r#"use relm4_kit::containers::Dialog;

// --- Confirm dialog (with dangerous action) ---
Dialog::confirm("Delete File?", "This action cannot be undone.")
    .confirm_label("Delete")
    .cancel_label("Cancel")
    .dangerous(true)
    .on_confirm(|| {
        println!("User confirmed deletion");
    })
    .on_cancel(|| {
        println!("User cancelled");
    })
    .present(&window);

// --- Info dialog (single button) ---
Dialog::info("Update Available", "Version 2.0 is ready.")
    .confirm_label("Install")
    .on_confirm(|| {
        println!("User acknowledged");
    })
    .present(&window);

// --- Custom dialog ---
Dialog::confirm("Reset Settings", "Restore all defaults?")
    .confirm_label("Reset Everything")
    .cancel_label("Keep My Settings")
    .dangerous(true)
    .on_confirm(|| {
        println!("Reset confirmed");
    })
    .on_cancel(|| {
        println!("Reset cancelled");
    })
    .present(&window);
"#;
