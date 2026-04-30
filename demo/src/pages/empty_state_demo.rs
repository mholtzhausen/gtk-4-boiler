//! EmptyState Demo page — interactive preview of the empty state
//! placeholder component with different icons, titles, descriptions,
//! and optional action buttons.
//!
//! Shows:
//! - Basic empty state (icon + title + description, no action)
//! - Different icons (search, folder, error, info, mail)
//! - Empty state with an action button that prints to console
//! - Interactive playground: dynamically update the empty state

use std::rc::Rc;

use gtk4::prelude::*;
use relm4_kit::containers::EmptyState;

// ============================================================================
// Constants
// ============================================================================

const LONG_TEXT: &str =
    "EmptyState is a placeholder widget for empty lists, search results \
     that return nothing, or any situation where there is no content to \
     display.\n\
     \u{2022} Icon — a symbolic icon centred above the title.\n\
     \u{2022} Title — short heading describing the empty state.\n\
     \u{2022} Description — optional explanation or suggestion.\n\
     \u{2022} Action — optional primary button for a call to action.";

/// Icon names for the gallery section.
const GALLERY_ICONS: &[(&str, &str, &str)] = &[
    ("folder-drag-accept-symbolic", "No files yet", "Drag files here or use the upload button to get started."),
    ("edit-find-symbolic", "No results found", "Try adjusting your search terms or filters."),
    ("dialog-error-symbolic", "Connection lost", "Check your internet connection and try again."),
    ("dialog-information-symbolic", "Welcome!", "You're all set up. Start exploring the app."),
    ("mail-unread-symbolic", "Inbox zero", "You've read all your messages. Great job!"),
];

// ============================================================================
// Public API
// ============================================================================

/// Build the empty state demo page widget.
pub fn create() -> gtk4::Box {
    // ---- Page-specific CSS ----
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("empty_state_demo.css"));
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

    // ====================================================================
    // Page header
    // ====================================================================
    let title = gtk4::Label::new(Some("Empty State Demo"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    let desc = gtk4::Label::new(Some(LONG_TEXT));
    desc.add_css_class("relm4-page-subtitle");
    desc.set_wrap(true);
    desc.set_xalign(0.0);
    desc.set_margin_bottom(16);
    container.append(&desc);

    // ====================================================================
    // Section 1: Basic empty state
    // ====================================================================
    let section1_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    section1_sep.set_margin_bottom(16);
    container.append(&section1_sep);

    let section1_header = gtk4::Label::new(Some("Basic Empty State"));
    section1_header.add_css_class("empty-state-demo-section-header");
    section1_header.set_halign(gtk4::Align::Start);
    section1_header.set_margin_bottom(8);
    container.append(&section1_header);

    let section1_desc = gtk4::Label::new(Some(
        "The simplest form: an icon, a short title, and a description. \
         No action button — purely informational.",
    ));
    section1_desc.add_css_class("empty-state-demo-description");
    section1_desc.set_wrap(true);
    section1_desc.set_xalign(0.0);
    section1_desc.set_margin_bottom(12);
    container.append(&section1_desc);

    let basic_empty = EmptyState::new()
        .icon("folder-drag-accept-symbolic")
        .title("Nothing here yet")
        .description("Your content will appear here once you add some items. \
                      Use the button above to get started.")
        .build();
    basic_empty.add_css_class("empty-state-demo-preview");
    container.append(&basic_empty);

    // ====================================================================
    // Section 2: Different icons gallery
    // ====================================================================
    let section2_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    section2_sep.add_css_class("empty-state-demo-separator");
    container.append(&section2_sep);

    let section2_header = gtk4::Label::new(Some("Different Icons"));
    section2_header.add_css_class("empty-state-demo-section-header");
    section2_header.set_halign(gtk4::Align::Start);
    section2_header.set_margin_bottom(8);
    container.append(&section2_header);

    let section2_desc = gtk4::Label::new(Some(
        "Choose an icon that matches the context — search, files, \
         errors, or informational states.",
    ));
    section2_desc.add_css_class("empty-state-demo-description");
    section2_desc.set_wrap(true);
    section2_desc.set_xalign(0.0);
    section2_desc.set_margin_bottom(12);
    container.append(&section2_desc);

    let gallery = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    gallery.add_css_class("empty-state-demo-gallery");

    for (icon_name, gallery_title, gallery_desc) in GALLERY_ICONS {
        let card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        card.add_css_class("empty-state-demo-card");

        let header = gtk4::Label::new(Some(*icon_name));
        header.set_markup(&format!("<tt>{}</tt>", icon_name));
        header.set_halign(gtk4::Align::Start);
        header.add_css_class("empty-state-demo-description");
        header.set_margin_bottom(4);
        card.append(&header);

        let es = EmptyState::new()
            .icon(*icon_name)
            .title(*gallery_title)
            .description(*gallery_desc)
            .build();
        card.append(&es);

        gallery.append(&card);
    }

    container.append(&gallery);

    // ====================================================================
    // Section 3: With action button
    // ====================================================================
    let section3_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    section3_sep.add_css_class("empty-state-demo-separator");
    container.append(&section3_sep);

    let section3_header = gtk4::Label::new(Some("With Action Button"));
    section3_header.add_css_class("empty-state-demo-section-header");
    section3_header.set_halign(gtk4::Align::Start);
    section3_header.set_margin_bottom(8);
    container.append(&section3_header);

    let section3_desc = gtk4::Label::new(Some(
        "An optional primary action button guides the user toward the \
         next step. The button callback fires each time it is clicked.",
    ));
    section3_desc.add_css_class("empty-state-demo-description");
    section3_desc.set_wrap(true);
    section3_desc.set_xalign(0.0);
    section3_desc.set_margin_bottom(12);
    container.append(&section3_desc);

    let action_empty = EmptyState::new()
        .icon("document-new-symbolic")
        .title("No documents")
        .description("Create a new document or import an existing one from your files.")
        .action("Create Document", || {
            println!("EmptyState action clicked: Create Document");
        })
        .build();
    action_empty.add_css_class("empty-state-demo-preview");
    container.append(&action_empty);

    // ====================================================================
    // Section 4: Interactive playground
    // ====================================================================
    let section4_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    section4_sep.add_css_class("empty-state-demo-separator");
    container.append(&section4_sep);

    let section4_header = gtk4::Label::new(Some("Interactive Playground"));
    section4_header.add_css_class("empty-state-demo-section-header");
    section4_header.set_halign(gtk4::Align::Start);
    section4_header.set_margin_bottom(8);
    container.append(&section4_header);

    let section4_desc = gtk4::Label::new(Some(
        "Use the controls below to build a custom empty state. \
         The preview updates in real time.",
    ));
    section4_desc.add_css_class("empty-state-demo-description");
    section4_desc.set_wrap(true);
    section4_desc.set_xalign(0.0);
    section4_desc.set_margin_bottom(12);
    container.append(&section4_desc);

    // ---- Playground area ----
    let playground = gtk4::Box::new(gtk4::Orientation::Horizontal, 24);
    playground.add_css_class("empty-state-demo-playground");

    // Left: preview
    let preview_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    preview_box.set_hexpand(true);
    preview_box.set_valign(gtk4::Align::Center);

    // The live preview — use `set_visible_child` to swap between a
    // "before" label and the actual empty state widget.  Because we
    // need to rebuild the EmptyState dynamically, we use a stack.
    let preview_stack = gtk4::Stack::new();
    preview_stack.set_vexpand(true);
    preview_stack.set_hexpand(true);
    preview_stack.set_transition_type(gtk4::StackTransitionType::Crossfade);

    // Placeholder "before" label.
    let placeholder = gtk4::Label::new(Some("Adjust the controls →"));
    placeholder.add_css_class("empty-state-demo-status");
    placeholder.set_valign(gtk4::Align::Center);
    preview_stack.add_named(&placeholder, Some("placeholder"));

    preview_box.append(&preview_stack);
    playground.append(&preview_box);

    // Right: controls
    let controls = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    controls.add_css_class("empty-state-demo-controls");
    controls.set_valign(gtk4::Align::Center);

    let controls_title = gtk4::Label::new(Some("Controls"));
    controls_title.add_css_class("empty-state-demo-control-label");
    controls_title.set_halign(gtk4::Align::Start);
    controls.append(&controls_title);

    // Icon selector (entry).
    let icon_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    icon_row.set_halign(gtk4::Align::Fill);
    icon_row.set_hexpand(true);

    let icon_label = gtk4::Label::new(Some("Icon:"));
    icon_label.set_xalign(0.0);
    icon_label.set_width_chars(6);
    icon_row.append(&icon_label);

    let icon_entry = gtk4::Entry::new();
    icon_entry.set_placeholder_text(Some("Icon name (e.g. edit-find-symbolic)"));
    icon_entry.set_text("folder-drag-accept-symbolic");
    icon_entry.set_hexpand(true);
    icon_row.append(&icon_entry);

    controls.append(&icon_row);

    // Title entry.
    let title_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    title_row.set_halign(gtk4::Align::Fill);
    title_row.set_hexpand(true);

    let title_label = gtk4::Label::new(Some("Title:"));
    title_label.set_xalign(0.0);
    title_label.set_width_chars(6);
    title_row.append(&title_label);

    let title_entry = gtk4::Entry::new();
    title_entry.set_placeholder_text(Some("Title text"));
    title_entry.set_text("Nothing here");
    title_entry.set_hexpand(true);
    title_row.append(&title_entry);

    controls.append(&title_row);

    // Description entry.
    let desc_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    desc_row.set_halign(gtk4::Align::Fill);
    desc_row.set_hexpand(true);

    let desc_label = gtk4::Label::new(Some("Desc:"));
    desc_label.set_xalign(0.0);
    desc_label.set_width_chars(6);
    desc_row.append(&desc_label);

    let desc_entry = gtk4::Entry::new();
    desc_entry.set_placeholder_text(Some("Description text"));
    desc_entry.set_text("Add some content to get started.");
    desc_entry.set_hexpand(true);
    desc_row.append(&desc_entry);

    controls.append(&desc_row);

    // Show action checkbox.
    let show_action_check = gtk4::CheckButton::with_label("Show action button");
    show_action_check.set_active(true);
    controls.append(&show_action_check);

    // Action label entry.
    let action_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    action_row.set_halign(gtk4::Align::Fill);
    action_row.set_hexpand(true);

    let action_label = gtk4::Label::new(Some("Action:"));
    action_label.set_xalign(0.0);
    action_label.set_width_chars(6);
    action_row.append(&action_label);

    let action_entry = gtk4::Entry::new();
    action_entry.set_placeholder_text(Some("Button label"));
    action_entry.set_text("Add Item");
    action_entry.set_hexpand(true);
    action_row.append(&action_entry);

    controls.append(&action_row);

    // Update button.
    let update_btn = gtk4::Button::with_label("Update Preview");
    update_btn.add_css_class("relm4-btn");
    update_btn.add_css_class("relm4-btn-primary");
    update_btn.set_halign(gtk4::Align::End);
    controls.append(&update_btn);

    // Action button status.
    let action_status = gtk4::Label::new(Some("Action button not clicked yet."));
    action_status.add_css_class("empty-state-demo-status");
    action_status.set_halign(gtk4::Align::Fill);
    action_status.set_hexpand(true);
    controls.append(&action_status);

    playground.append(&controls);
    container.append(&playground);

    // ---- Wire up the interactive playground ----
    let stack = Rc::new(preview_stack.clone());
    let icon_entry_c = icon_entry.clone();
    let title_entry_c = title_entry.clone();
    let desc_entry_c = desc_entry.clone();
    let show_action_check_c = show_action_check.clone();
    let action_entry_c = action_entry.clone();
    let action_status_c = action_status.clone();

    update_btn.connect_clicked(move |_| {
        let icon_text = icon_entry_c.text().to_string();
        let title_text = title_entry_c.text().to_string();
        let desc_text = desc_entry_c.text().to_string();
        let show_action = show_action_check_c.is_active();
        let action_text = action_entry_c.text().to_string();

        // Build a fresh empty state.
        let mut builder = EmptyState::new()
            .icon(icon_text)
            .title(title_text)
            .description(desc_text);

        if show_action && !action_text.is_empty() {
            let status = action_status_c.clone();
            let action_text_for_closure = action_text.clone();
            builder = builder.action(action_text, move || {
                status.set_markup(&format!(
                    "<b>Action clicked:</b> <span color='#3584e4'>{}</span>",
                    action_text_for_closure
                ));
            });
        }

        let new_widget = builder.build();
        new_widget.set_vexpand(true);
        new_widget.set_hexpand(true);

        // Remove the old empty state from the stack (if any) and add the new one.
        // Using a fixed child name so we can replace it.
        if let Some(old) = stack.child_by_name("live-preview") {
            stack.remove(&old);
        }
        stack.add_named(&new_widget, Some("live-preview"));
        stack.set_visible_child(&new_widget);
    });

    // ---- Initialize the playground preview ----
    // (Trigger the update-able click to show initial state.)
    update_btn.emit_clicked();

    // ====================================================================
    // Code panel
    // ====================================================================
    let code_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    code_sep.add_css_class("empty-state-demo-separator");
    container.append(&code_sep);

    let code_header = gtk4::Label::new(Some("Source Code Example"));
    code_header.add_css_class("empty-state-demo-section-header");
    code_header.set_halign(gtk4::Align::Start);
    code_header.set_margin_bottom(8);
    container.append(&code_header);

    let code_panel = create_code_panel();
    container.append(&code_panel);

    outer
}

// ============================================================================
// Code panel
// ============================================================================

/// Build a styled code panel showing the EmptyState API in action.
fn create_code_panel() -> gtk4::Box {
    let panel = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    panel.add_css_class("empty-state-demo-code-panel");
    panel.set_hexpand(true);

    // Title bar.
    let title_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_bar.add_css_class("empty-state-demo-code-title");
    panel.append(&title_bar);

    let title_text = gtk4::Label::new(Some("Rust — EmptyState Builder"));
    title_text.add_css_class("empty-state-demo-code-title-text");
    title_text.set_halign(gtk4::Align::Start);
    title_bar.append(&title_text);

    // Code content.
    let code_text = gtk4::Label::new(Some(CODE_SAMPLE));
    code_text.add_css_class("empty-state-demo-code-text");
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

/// A complete working example of EmptyState usage.
const CODE_SAMPLE: &str = r#"use relm4_kit::containers::EmptyState;

// Basic empty state (no action button)
let empty = EmptyState::new()
    .icon("folder-drag-accept-symbolic")
    .title("No files yet")
    .description("Drag files here or use the upload button.")
    .build();

// Empty state with action button
let empty = EmptyState::new()
    .icon("document-new-symbolic")
    .title("No documents")
    .description("Create or import a document to get started.")
    .action("Create Document", || {
        println!("User wants to create a document!");
    })
    .build();

// Search results (action clears the filter)
let empty = EmptyState::new()
    .icon("edit-find-symbolic")
    .title("No results found")
    .description("Try adjusting your search terms or filters.")
    .action("Clear Filters", move || {
        sender.input(Msg::ClearFilters);
    })
    .build();
"#;
