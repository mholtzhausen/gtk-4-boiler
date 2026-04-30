//! Badge primitive — a small label for counts and status indicators.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::prelude::*;
//!
//! let badge = Badge::new()
//!     .text("3")
//!     .variant(BadgeVariant::Success)
//!     .size(BadgeSize::Small)
//!     .build();
//! ```

use gtk4::prelude::*;

/// The visual variant of a badge.
#[derive(Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    /// Green-tinted badge for positive states (e.g. "Online", "Active").
    Success,
    /// Red-tinted badge for destructive or error states.
    Danger,
    /// Yellow-tinted badge for cautionary states.
    Warning,
    /// Blue-tinted badge for informational states.
    Info,
    /// Neutral/grey badge for default or non-semantic labels.
    Neutral,
}

/// The size of a badge.
#[derive(Clone, Copy, PartialEq)]
pub enum BadgeSize {
    /// Compact badge — smaller font, tighter padding.
    Small,
    /// Default badge — standard font and padding.
    Medium,
}

/// A badge builder that constructs a styled [`gtk4::Label`].
///
/// Badges are small, non-interactive labels used for status indicators,
/// notification counts, and metadata tags.
pub struct Badge {
    text: Option<String>,
    variant: BadgeVariant,
    size: BadgeSize,
}

impl Default for Badge {
    fn default() -> Self {
        Self {
            text: None,
            variant: BadgeVariant::Neutral,
            size: BadgeSize::Medium,
        }
    }
}

impl Badge {
    /// Create a new badge builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the badge text (e.g. a count like `"3"` or a label like `"New"`).
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set the visual variant.
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the badge size.
    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    /// Build the badge label widget.
    ///
    /// Returns a configured [`gtk4::Label`] with the `.relm4-badge`
    /// CSS class plus variant and size classes.
    pub fn build(self) -> gtk4::Label {
        let label = gtk4::Label::new(Some(&self.text.unwrap_or_default()));
        label.add_css_class("relm4-badge");

        // --- Variant CSS class ---
        let variant_class = match self.variant {
            BadgeVariant::Success => "badge-success",
            BadgeVariant::Danger => "badge-danger",
            BadgeVariant::Warning => "badge-warning",
            BadgeVariant::Info => "badge-info",
            BadgeVariant::Neutral => "badge-neutral",
        };
        label.add_css_class(variant_class);

        // --- Size CSS class ---
        if self.size == BadgeSize::Small {
            label.add_css_class("badge-small");
        }

        label
    }
}
