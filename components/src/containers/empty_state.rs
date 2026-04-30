//! EmptyState — a placeholder for empty lists, searches, and other states
//! where there is nothing to display.
//!
//! The builder creates a centered vertical [`gtk4::Box`] with an optional
//! icon, title, description, and action button.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::EmptyState;
//!
//! let widget = EmptyState::new()
//!     .icon("search-symbolic")
//!     .title("No results found")
//!     .description("Try adjusting your search terms or filters.")
//!     .action("Clear filters", || sender.input(Msg::ClearFilters))
//!     .build();
//! ```

use gtk4::prelude::*;

/// A builder for an empty-state placeholder widget.
///
/// Constructed via [`EmptyState::new`].  After setting the desired
/// properties, call [`build`](EmptyState::build) to obtain the
/// [`gtk4::Box`] widget.
///
/// CSS classes applied to the output widget:
///
/// | Element | CSS class |
/// |---------|-----------|
/// | Root container | `.relm4-empty-state` |
/// | Icon | `.relm4-empty-state-icon` |
/// | Title | `.relm4-empty-state-title` |
/// | Description | `.relm4-empty-state-description` |
/// | Action button | `.relm4-btn.relm4-btn-primary` |
#[derive(Default)]
pub struct EmptyState {
    icon: Option<String>,
    title: Option<String>,
    description: Option<String>,
    action_label: Option<String>,
    on_action: Option<Box<dyn Fn() + 'static>>,
}

impl EmptyState {
    /// Create a new `EmptyState` builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the symbolic icon name displayed above the title.
    ///
    /// Example: `"search-symbolic"`, `"folder-open-symbolic"`,
    /// `"emblem-important-symbolic"`.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the title text.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the description text shown below the title.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add a primary action button with a label and a callback.
    ///
    /// The callback fires every time the button is clicked.  Because the
    /// underlying GTK signal handler persists for the widget's lifetime,
    /// the closure must implement `Fn` (not `FnOnce`).  In relm4
    /// components this is typically `|| sender.input(…)` — `ComponentSender`
    /// is `Clone`, so this works naturally.
    ///
    /// If this method is not called, no action button is shown.
    pub fn action(mut self, label: impl Into<String>, on_action: impl Fn() + 'static) -> Self {
        self.action_label = Some(label.into());
        self.on_action = Some(Box::new(on_action));
        self
    }

    /// Build the empty state widget.
    ///
    /// Returns a vertically stacked [`gtk4::Box`] with all elements
    /// centred:
    ///
    /// ```text
    /// ┌─────────────────────────┐
    /// │      (icon image)       │  ← .relm4-empty-state-icon
    /// │       Title Text        │  ← .relm4-empty-state-title
    /// │   Description text…     │  ← .relm4-empty-state-description
    /// │    [ Action Button ]    │  ← .relm4-btn-primary
    /// └─────────────────────────┘
    /// ```
    pub fn build(self) -> gtk4::Box {
        let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        container.add_css_class("relm4-empty-state");
        container.set_valign(gtk4::Align::Center);
        container.set_halign(gtk4::Align::Center);
        container.set_spacing(0);

        // ---- Icon ----
        if let Some(ref icon_name) = self.icon {
            let image = gtk4::Image::from_icon_name(icon_name);
            image.add_css_class("relm4-empty-state-icon");
            image.set_pixel_size(64);
            container.append(&image);
        }

        // ---- Title ----
        if let Some(ref title) = self.title {
            let title_label = gtk4::Label::new(Some(title));
            title_label.add_css_class("relm4-empty-state-title");
            title_label.set_halign(gtk4::Align::Center);
            title_label.set_xalign(0.5);
            container.append(&title_label);
        }

        // ---- Description ----
        if let Some(ref desc) = self.description {
            let desc_label = gtk4::Label::new(Some(desc));
            desc_label.add_css_class("relm4-empty-state-description");
            desc_label.set_halign(gtk4::Align::Center);
            desc_label.set_xalign(0.5);
            desc_label.set_wrap(true);
            desc_label.set_max_width_chars(50);
            container.append(&desc_label);
        }

        // ---- Action button ----
        if let Some(label) = self.action_label {
            let button = gtk4::Button::with_label(&label);
            button.add_css_class("relm4-btn");
            button.add_css_class("relm4-btn-primary");
            button.set_halign(gtk4::Align::Center);
            button.set_margin_top(8);

            if let Some(callback) = self.on_action {
                button.connect_clicked(move |_| {
                    callback();
                });
            }

            let button_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            button_box.set_halign(gtk4::Align::Center);
            button_box.append(&button);
            container.append(&button_box);
        }

        container
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_construction_succeeds() {
        // Should not panic — only tests the data struct, no widget created.
        let _state = EmptyState::new();
    }

    #[test]
    fn builder_methods_set_fields() {
        // Verify builder chaining does not panic by constructing
        // the builder (but not calling build — no GTK widget created).
        let builder = EmptyState::new()
            .icon("search-symbolic")
            .title("Empty")
            .description("Nothing here.")
            .action("Go", || {});
        let _ = builder;
    }

    // Widget-level build tests are omitted here because they require
    // GTK initialisation on the main thread (`gtk4::init()`), which
    // conflicts with Rust's parallel test runner.  These belong in
    // the integration test suite under `tests/` (Phase 8).
}
