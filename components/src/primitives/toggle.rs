//! Toggle primitive — a switch with a label and optional description.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::prelude::*;
//!
//! let toggle = Toggle::new()
//!     .title("Enable notifications")
//!     .description("Receive alerts when new items arrive")
//!     .active(true)
//!     .on_toggle(|is_active| Msg::Toggled(is_active))
//!     .build();
//! ```

use gtk4::prelude::*;

/// A toggle (switch) builder.
///
/// Produces a horizontal [`gtk4::Box`] with a title label and optional
/// description on the left, and a [`gtk4::Switch`] on the right.
///
/// The toggle's active state is exposed via the callback given to
/// [`on_toggle`](Toggle::on_toggle), which receives the new boolean
/// state whenever the user flips the switch.
pub struct Toggle<Msg> {
    title: Option<String>,
    description: Option<String>,
    active: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Msg>>,
}

impl<Msg> Default for Toggle<Msg> {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            active: false,
            on_toggle: None,
        }
    }
}

impl<Msg: 'static> Toggle<Msg> {
    /// Create a new toggle builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the toggle title/label.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the toggle description (shown below the title in a smaller
    /// font with secondary text colour).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the initial active (on/true) state of the switch.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set the callback invoked whenever the user toggles the switch.
    ///
    /// The callback receives the new boolean state (`true` = active/on)
    /// and must return a message of type `Msg`.
    pub fn on_toggle(mut self, f: impl Fn(bool) -> Msg + 'static) -> Self {
        self.on_toggle = Some(Box::new(f));
        self
    }

    /// Build the toggle widget.
    ///
    /// Returns a horizontal [`gtk4::Box`] with the `.relm4-toggle` CSS
    /// class containing:
    ///
    /// ```ignore
    /// ├── Box (vertical, labels)
    /// │   ├── Label.toggle-title       (if title is set)
    /// │   └── Label.toggle-description (if description is set)
    /// └── Switch
    /// ```
    pub fn build(self) -> gtk4::Box {
        let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        container.add_css_class("relm4-toggle");
        container.set_halign(gtk4::Align::Fill);

        // --- Label area (left side) ---
        // Use a vertical box to stack title + optional description.
        let label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        label_box.set_hexpand(true);
        label_box.set_halign(gtk4::Align::Start);
        label_box.set_valign(gtk4::Align::Center);

        if let Some(title) = self.title {
            let title_label = gtk4::Label::new(Some(&title));
            title_label.add_css_class("toggle-title");
            title_label.set_halign(gtk4::Align::Start);
            title_label.set_xalign(0.0);
            title_label.set_wrap(true);
            label_box.append(&title_label);
        }

        if let Some(desc) = self.description {
            let desc_label = gtk4::Label::new(Some(&desc));
            desc_label.add_css_class("toggle-description");
            desc_label.set_halign(gtk4::Align::Start);
            desc_label.set_xalign(0.0);
            desc_label.set_wrap(true);
            label_box.append(&desc_label);
        }

        container.append(&label_box);

        // --- Switch (right side) ---
        let switch = gtk4::Switch::new();
        switch.set_valign(gtk4::Align::Center);
        switch.set_active(self.active);

        // Wire up the toggle callback.
        if let Some(callback) = self.on_toggle {
            switch.connect_active_notify(move |sw| {
                let is_active = sw.is_active();
                let _ = callback(is_active);
            });
        }

        container.append(&switch);

        container
    }
}
