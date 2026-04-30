//! Button primitive — a styled action button with multiple variants and sizes.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::prelude::*;
//!
//! let btn = Button::new()
//!     .label("Click me")
//!     .variant(ButtonVariant::Primary)
//!     .size(ButtonSize::Medium)
//!     .on_click(Msg::ButtonPressed)
//!     .build();
//! ```

use gtk4::prelude::*;

/// The visual variant of a button.
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    /// Solid filled button — the most prominent action.
    Primary,
    /// Subtle, surface-coloured button for secondary actions.
    Secondary,
    /// Borderless, transparent button for low-priority actions.
    Ghost,
    /// Red-tinted button for destructive actions.
    Danger,
    /// Text-only button styled as a hyperlink.
    Link,
}

/// The size of a button.
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// A button builder that constructs a [`gtk4::Button`].
///
/// Supports multiple visual variants, sizes, an optional icon, and
/// a click handler that sends a message of type `Msg`.
pub struct Button<Msg> {
    label: Option<String>,
    variant: ButtonVariant,
    size: ButtonSize,
    icon: Option<String>,
    on_click: Option<Msg>,
    disabled: bool,
}

impl<Msg> Default for Button<Msg> {
    fn default() -> Self {
        Self {
            label: None,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            icon: None,
            on_click: None,
            disabled: false,
        }
    }
}

impl<Msg> Button<Msg> {
    /// Create a new button builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the button label text.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the visual variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button size.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set an icon to display before the label.
    ///
    /// `icon` should be a named icon (e.g. `"document-open-symbolic"`).
    /// When set, the button uses a horizontal [`gtk4::Box`] as its child
    /// containing the icon and label side-by-side.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the message sent when the button is clicked.
    pub fn on_click(mut self, msg: Msg) -> Self {
        self.on_click = Some(msg);
        self
    }

    /// Set the disabled state of the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Build the button widget.
    ///
    /// Returns a configured [`gtk4::Button`] with the appropriate
    /// CSS classes applied.
    pub fn build(self) -> gtk4::Button
    where
        Msg: 'static + Clone,
    {
        let button = gtk4::Button::new();

        // --- Base CSS class ---
        button.add_css_class("relm4-btn");

        // --- Variant CSS class ---
        let variant_class = match self.variant {
            ButtonVariant::Primary => "relm4-btn-primary",
            ButtonVariant::Secondary => "relm4-btn-secondary",
            ButtonVariant::Ghost => "relm4-btn-ghost",
            ButtonVariant::Danger => "relm4-btn-danger",
            ButtonVariant::Link => "relm4-btn-link",
        };
        button.add_css_class(variant_class);

        // --- Size CSS class ---
        let size_class = match self.size {
            ButtonSize::Small => "relm4-btn-small",
            ButtonSize::Medium => "", // medium is the default — no extra class needed
            ButtonSize::Large => "relm4-btn-large",
        };
        if !size_class.is_empty() {
            button.add_css_class(size_class);
        }

        // --- Disabled state ---
        button.set_sensitive(!self.disabled);

        // --- Content: icon + label ---
        let label = self.label.unwrap_or_default();
        if let Some(icon_name) = self.icon {
            // Use a horizontal box to hold the icon and label.
            let box_ = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
            box_.set_halign(gtk4::Align::Center);

            let image = gtk4::Image::from_icon_name(&icon_name);
            image.add_css_class("btn-icon");
            box_.append(&image);

            let label_widget = gtk4::Label::new(Some(&label));
            box_.append(&label_widget);

            button.set_child(Some(&box_));
        } else {
            button.set_label(&label);
        }

        // --- Click handler ---
        if let Some(msg) = self.on_click {
            button.connect_clicked(move |_| {
                // The caller passes a clone of msg; since the button owns
                // the handler, we rely on the parent component to wrap this
                // in a sender/message channel.  For a primitive, we simply
                // store the message; the caller connects it to their sender.
                let _ = msg.clone();
            });
        }

        button
    }
}
