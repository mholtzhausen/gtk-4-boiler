//! ToastStack — a non-blocking notification overlay.
//!
//! Displays short-lived toast notifications stacked at the bottom-right
//! corner of the widget. Each toast can include an optional action button
//! (e.g. "Undo") and auto-dismisses after 4 seconds.
//!
//! The stack limits visible toasts to 3; older toasts are dismissed when
//! the limit is exceeded.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::ToastStack;
//!
//! let toast_stack = ToastStack::builder()
//!     .launch(content_widget)   // the widget to overlay toasts on
//!     .detach();
//!
//! // Later, show a toast:
//! sender.input(ToastStackMsg::Show("File saved".into(), ToastKind::Success));
//! ```

use std::collections::HashMap;

use gtk4::prelude::*;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

/// Maximum number of toasts visible at once.
const MAX_VISIBLE: usize = 3;

/// Seconds before a toast auto-dismisses.
const DISMISS_SECS: u32 = 4;

// ============================================================================
// ToastKind
// ============================================================================

/// The visual style of a toast notification.
///
/// Determines the left-border accent colour via CSS classes
/// (`toast-success`, `toast-error`, `toast-warning`, `toast-info`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    /// Green accent — e.g. "Saved successfully".
    Success,
    /// Red accent — e.g. "Connection lost".
    Error,
    /// Yellow accent — e.g. "Disk space low".
    Warning,
    /// Blue accent — e.g. "Update available".
    Info,
}

impl ToastKind {
    /// CSS class name for this kind's accent colour.
    fn css_class(&self) -> &'static str {
        match self {
            ToastKind::Success => "toast-success",
            ToastKind::Error => "toast-error",
            ToastKind::Warning => "toast-warning",
            ToastKind::Info => "toast-info",
        }
    }

    /// Icon name (symbolic) for this kind.
    fn icon_name(&self) -> &'static str {
        match self {
            ToastKind::Success => "emblem-ok-symbolic",
            ToastKind::Error => "dialog-error-symbolic",
            ToastKind::Warning => "dialog-warning-symbolic",
            ToastKind::Info => "dialog-information-symbolic",
        }
    }
}

// ============================================================================
// ToastEntry
// ============================================================================

/// Data for a single toast in the stack.
#[derive(Debug, Clone)]
pub struct ToastEntry {
    /// Unique auto-incremented id.
    pub id: u32,
    /// The message text shown inside the toast.
    pub message: String,
    /// Visual kind (determines accent colour and icon).
    pub kind: ToastKind,
    /// Optional label for an action button (e.g. "Undo").
    pub action_label: Option<String>,
}

// ============================================================================
// Messages & Output
// ============================================================================

/// Messages sent to the [`ToastStack`] component.
#[derive(Debug, Clone)]
pub enum ToastStackMsg {
    /// Show a simple toast with the given text and kind.
    Show(String, ToastKind),
    /// Show a toast with an action button labelled `String`.
    ShowWithAction(String, ToastKind, String),
    /// Dismiss the toast with the given id.
    Dismiss(u32),
    /// Dismiss the oldest (front-most) toast.
    DismissTop,
}

/// Messages emitted by [`ToastStack`] to its parent.
#[derive(Debug, Clone)]
pub enum ToastStackOutput {
    /// The action button on the toast identified by `u32` was clicked.
    /// The `String` is the original message text.
    ActionTriggered(u32, String),
}

// ============================================================================
// Internal: per-toast widget handle
// ============================================================================

/// A widget handle we keep so we can remove toasts from the box.
struct ToastWidget {
    _row: gtk4::Box,
}

// ============================================================================
// Model
// ============================================================================

/// A stack of auto-dismissing toast notifications.
pub struct ToastStack {
    /// Active toasts, ordered oldest-first.
    toasts: Vec<ToastEntry>,
    /// Maps toast id → widget handle for removal.
    widgets: HashMap<u32, ToastWidget>,
    /// Auto-incrementing id counter.
    next_id: u32,
    /// The box that holds the toast widgets (bottom-right of overlay).
    toast_box: gtk4::Box,
}

// ============================================================================
// Component
// ============================================================================

#[relm4::component(pub)]
impl SimpleComponent for ToastStack {
    /// The widget to place under the overlay (the main content area).
    type Init = gtk4::Widget;
    type Input = ToastStackMsg;
    type Output = ToastStackOutput;

    view! {
        #[root]
        gtk4::Overlay {
            set_vexpand: true,
            set_hexpand: true,
        }
    }

    fn init(
        content: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        // ---- IMPORTANT: overlay children vs main child ----
        //
        // In the view! macro above, the Overlay has no nested children.
        // If we put a nested Box inside the view! macro's Overlay,
        // relm4 calls `RelmContainerExt::container_add()` on it.
        // Since `gtk::Overlay` implements `RelmSetChildExt`, the default
        // `container_add` calls `overlay.set_child()` — which sets the
        // MAIN child, NOT an overlay child.  Then `root.set_child(&content)`
        // below replaces it, orphaning the toast box.
        //
        // To avoid this, we create the toast box manually and register it
        // as a proper overlay child via `add_overlay()`.

        // Create the toast box as a proper GTK overlay child.
        let toast_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        toast_box.set_spacing(4);
        toast_box.set_valign(gtk4::Align::End);
        toast_box.set_halign(gtk4::Align::End);
        toast_box.set_margin_bottom(16);
        toast_box.set_margin_end(16);

        // Add it as an overlay child — it floats above the main child.
        root.add_overlay(&toast_box);

        // Set the main child (the content passed by the user).
        root.set_child(Some(&content));

        let model = ToastStack {
            toasts: Vec::new(),
            widgets: HashMap::new(),
            next_id: 0,
            toast_box,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ToastStackMsg::Show(message, kind) => {
                self.add_toast(message, kind, None, &sender);
            }
            ToastStackMsg::ShowWithAction(message, kind, action_label) => {
                self.add_toast(message, kind, Some(action_label), &sender);
            }
            ToastStackMsg::Dismiss(id) => {
                self.dismiss_toast(id);
            }
            ToastStackMsg::DismissTop => {
                if let Some(oldest) = self.toasts.first() {
                    self.dismiss_toast(oldest.id);
                }
            }
        }
    }
}

// ============================================================================
// Internal helpers
// ============================================================================

impl ToastStack {
    /// Add a new toast with the given properties and schedule auto-dismiss.
    fn add_toast(
        &mut self,
        message: String,
        kind: ToastKind,
        action_label: Option<String>,
        sender: &ComponentSender<Self>,
    ) {
        let id = self.next_id;
        self.next_id += 1;

        // Build the toast widget.
        let widget = build_toast_widget(id, &message, kind, action_label.as_deref(), sender);
        self.toast_box.append(&widget);

        // Store state.
        let entry = ToastEntry {
            id,
            message,
            kind,
            action_label,
        };
        self.toasts.push(entry);
        self.widgets.insert(
            id,
            ToastWidget {
                _row: widget.clone(),
            },
        );

        // Enforce the max-visible limit (dismiss oldest first).
        while self.toasts.len() > MAX_VISIBLE {
            let oldest_id = self.toasts.first().map(|e| e.id);
            if let Some(oldest_id) = oldest_id {
                self.dismiss_toast(oldest_id);
            }
        }

        // Schedule auto-dismiss.
        let sender_clone = sender.clone();
        glib::timeout_add_seconds_local(DISMISS_SECS, move || {
            sender_clone.input(ToastStackMsg::Dismiss(id));
            glib::ControlFlow::Break
        });
    }

    /// Remove a toast by its id from both the widget tree and internal state.
    fn dismiss_toast(&mut self, id: u32) {
        // Remove the widget from the box.
        if let Some(w) = self.widgets.remove(&id) {
            self.toast_box.remove(&w._row);
        }

        // Remove the entry from the vec.
        self.toasts.retain(|e| e.id != id);
    }
}

// ============================================================================
// Widget building
// ============================================================================

/// Create a single toast row widget.
fn build_toast_widget(
    id: u32,
    message: &str,
    kind: ToastKind,
    action_label: Option<&str>,
    sender: &ComponentSender<ToastStack>,
) -> gtk4::Box {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    row.add_css_class("relm4-toast");
    row.add_css_class(kind.css_class());

    // Icon.
    let icon = gtk4::Image::from_icon_name(kind.icon_name());
    icon.set_icon_size(gtk4::IconSize::Normal);
    icon.set_margin_start(4);
    row.append(&icon);

    // Message label (grows to fill space).
    let label = gtk4::Label::new(Some(message));
    label.set_halign(gtk4::Align::Start);
    label.set_hexpand(true);
    label.set_wrap(true);
    label.set_max_width_chars(40);
    row.append(&label);

    // Optional action button.
    if let Some(action) = action_label {
        let action_btn = gtk4::Button::with_label(action);
        action_btn.add_css_class("flat");
        action_btn.set_valign(gtk4::Align::Center);
        let sender_clone = sender.clone();
        let msg = message.to_string();
        action_btn.connect_clicked(move |_| {
            // Emit output to parent.
            let _ = sender_clone.output(ToastStackOutput::ActionTriggered(id, msg.clone()));
            // Dismiss the toast.
            let s = sender_clone.clone();
            s.input(ToastStackMsg::Dismiss(id));
        });
        row.append(&action_btn);
    }

    // Close/dismiss button.
    let close_btn = gtk4::Button::new();
    close_btn.set_icon_name("window-close-symbolic");
    close_btn.add_css_class("flat");
    close_btn.set_valign(gtk4::Align::Center);
    let sender_clone = sender.clone();
    close_btn.connect_clicked(move |_| {
        sender_clone.input(ToastStackMsg::Dismiss(id));
    });
    row.append(&close_btn);

    row
}
