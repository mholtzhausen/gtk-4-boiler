//! Badge primitive — a small label for counts and status indicators.

/// The visual variant of a badge.
pub enum BadgeVariant {
    Success,
    Danger,
    Warning,
    Info,
    Neutral,
}

/// The size of a badge.
pub enum BadgeSize {
    Small,
    Medium,
}

/// A badge builder that constructs a [`gtk4::Label`].
pub struct Badge {
    text: Option<String>,
    variant: BadgeVariant,
    size: BadgeSize,
}

impl Badge {
    /// Create a new badge builder.
    pub fn new() -> Self {
        Self {
            text: None,
            variant: BadgeVariant::Neutral,
            size: BadgeSize::Medium,
        }
    }

    /// Set the badge text.
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
    pub fn build(self) -> gtk4::Label {
        gtk4::Label::new(None)
    }
}
