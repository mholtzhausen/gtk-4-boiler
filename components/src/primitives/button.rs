//! Button primitive — a styled button with variants and sizes.

/// The visual variant of a button.
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
    Link,
}

/// The size of a button.
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// A button builder that constructs a [`gtk4::Button`].
pub struct Button<Msg> {
    label: Option<String>,
    variant: ButtonVariant,
    size: ButtonSize,
    icon: Option<String>,
    on_click: Option<Msg>,
}

impl<Msg> Button<Msg> {
    /// Create a new button builder.
    pub fn new() -> Self {
        Self {
            label: None,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Medium,
            icon: None,
            on_click: None,
        }
    }

    /// Set the button label.
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

    /// Set an icon name (symbolic icon name).
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the message sent when the button is clicked.
    pub fn on_click(mut self, msg: Msg) -> Self {
        self.on_click = Some(msg);
        self
    }

    /// Build the button widget.
    pub fn build(self) -> gtk4::Button {
        gtk4::Button::new()
    }
}
