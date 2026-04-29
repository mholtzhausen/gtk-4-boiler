//! Dialog — modal confirm, alert, and custom content dialogs.

/// Static methods for showing modals.
pub struct Dialog;

impl Dialog {
    /// Create a confirmation dialog builder.
    pub fn confirm(_title: impl Into<String>, _description: impl Into<String>) -> DialogBuilder {
        DialogBuilder
    }
}

/// A dialog builder for configuring and presenting a dialog.
pub struct DialogBuilder;

impl DialogBuilder {
    pub fn confirm_label(self, _label: impl Into<String>) -> Self { self }
    pub fn cancel_label(self, _label: impl Into<String>) -> Self { self }
    pub fn dangerous(self, _dangerous: bool) -> Self { self }
}
