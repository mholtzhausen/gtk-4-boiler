//! Dialog — modal confirm, alert, and info dialogs.
//!
//! This module provides static builder methods for showing modal dialogs
//! built on top of [`libadwaita::AlertDialog`].  `Dialog` is **not** a
//! managed relm4 component — it is a simple wrapper around libadwaita's
//! async alert dialog API.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::Dialog;
//!
//! Dialog::confirm("Delete file?", "This action cannot be undone.")
//!     .confirm_label("Delete")
//!     .dangerous(true)
//!     .on_confirm(|| sender.input(AppMsg::Confirmed))
//!     .on_cancel(|| sender.input(AppMsg::Cancelled))
//!     .present(&window);
//! ```

use libadwaita::prelude::*;

// ============================================================================
// Dialog
// ============================================================================

/// Static dialog creation methods.
///
/// `Dialog` is never instantiated — use [`Dialog::confirm`] or
/// [`Dialog::info`] to obtain a [`DialogBuilder`], configure it, and call
/// [`DialogBuilder::present`] to show the dialog.
pub struct Dialog;

impl Dialog {
    /// Create a confirmation dialog with a heading and body.
    ///
    /// The resulting builder defaults to "Cancel" / "OK" buttons.
    pub fn confirm(
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> DialogBuilder {
        DialogBuilder {
            heading: title.into(),
            body: description.into(),
            confirm_label: "OK".into(),
            cancel_label: Some("Cancel".into()),
            dangerous: false,
            on_confirm: None,
            on_cancel: None,
        }
    }

    /// Create an informational dialog with a heading and body.
    ///
    /// The resulting builder defaults to a single "OK" button (no cancel).
    pub fn info(
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> DialogBuilder {
        DialogBuilder {
            heading: title.into(),
            body: description.into(),
            confirm_label: "OK".into(),
            cancel_label: None,
            dangerous: false,
            on_confirm: None,
            on_cancel: None,
        }
    }
}

// ============================================================================
// DialogBuilder
// ============================================================================

/// Builder for configuring and presenting a modal dialog.
///
/// Obtained via [`Dialog::confirm`] or [`Dialog::info`].  After setting
/// options, call [`present`](DialogBuilder::present) to show the dialog
/// modally.
pub struct DialogBuilder {
    heading: String,
    body: String,
    confirm_label: String,
    cancel_label: Option<String>,
    dangerous: bool,
    on_confirm: Option<Box<dyn FnOnce() + 'static>>,
    on_cancel: Option<Box<dyn FnOnce() + 'static>>,
}

impl DialogBuilder {
    /// Set the text of the confirm (primary action) button.
    ///
    /// Defaults to `"OK"`.
    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    /// Set the text of the cancel (secondary action) button.
    ///
    /// If unset, no cancel button is shown.  Call this to add one.
    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = Some(label.into());
        self
    }

    /// Mark the confirm action as destructive (typically renders the
    /// button red).
    ///
    /// Defaults to `false`.
    pub fn dangerous(mut self, dangerous: bool) -> Self {
        self.dangerous = dangerous;
        self
    }

    /// Provide a closure to run when the confirm button is pressed.
    ///
    /// The closure is consumed once the dialog is dismissed.
    pub fn on_confirm(mut self, f: impl FnOnce() + 'static) -> Self {
        self.on_confirm = Some(Box::new(f));
        self
    }

    /// Provide a closure to run when the cancel button is pressed.
    ///
    /// The closure is consumed once the dialog is dismissed.
    pub fn on_cancel(mut self, f: impl FnOnce() + 'static) -> Self {
        self.on_cancel = Some(Box::new(f));
        self
    }

    /// Build the [`libadwaita::AlertDialog`] and present it modally
    /// over the given parent widget.
    ///
    /// `parent` should be the `gtk::Window` (or window-like widget) that
    /// the dialog will be centred on.
    pub fn present(self, parent: &impl IsA<gtk4::Widget>) {
        // Create the underlying AlertDialog.
        let dialog = libadwaita::AlertDialog::new(Some(&self.heading), Some(&self.body));

        // Add response buttons.
        // We add cancel first so it appears first (leftmost) in
        // libadwaita's layout.
        if let Some(cancel_label) = &self.cancel_label {
            dialog.add_response("cancel", cancel_label);
        }
        dialog.add_response("confirm", &self.confirm_label);

        // Configure response appearance.
        if self.dangerous {
            dialog.set_response_appearance("confirm", libadwaita::ResponseAppearance::Destructive);
        }

        // Set default and close responses.
        dialog.set_default_response(
            self.cancel_label
                .as_ref()
                .map(|_| "cancel")
                .or(Some("confirm")),
        );
        dialog.set_close_response(
            self.cancel_label
                .as_ref()
                .map_or("confirm", |_| "cancel"),
        );

        // Take the callbacks so we can move them into the async handler.
        let on_confirm = self.on_confirm;
        let on_cancel = self.on_cancel;

        // Present asynchronously.
        dialog.choose(
            parent,
            Option::<&libadwaita::gio::Cancellable>::None,
            move |response| {
                match response.as_str() {
                    "confirm" => {
                        if let Some(f) = on_confirm {
                            f();
                        }
                    }
                    _ => {
                        if let Some(f) = on_cancel {
                            f();
                        }
                    }
                }
            },
        );
    }
}
