//! Avatar primitive — a circular initial/icon widget for user profiles.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::prelude::*;
//!
//! let avatar = Avatar::new()
//!     .initials("JD")
//!     .size(40)
//!     .build();
//!
//! let avatar_with_icon = Avatar::new()
//!     .icon("user-avatar-symbolic")
//!     .size(32)
//!     .build();
//! ```

use gtk4::prelude::*;

/// An avatar widget builder that constructs an [`libadwaita::Avatar`].
///
/// Avatars display a user's initials as text, or an icon if set.
/// The widget is always circular with the configured size.
pub struct Avatar {
    initials: Option<String>,
    icon: Option<String>,
    size: i32,
}

impl Default for Avatar {
    fn default() -> Self {
        Self {
            initials: None,
            icon: None,
            size: 40,
        }
    }
}

impl Avatar {
    /// Create a new avatar builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the initials text displayed on the avatar.
    ///
    /// Typically 1-2 characters representing a user's name
    /// (e.g. `"JD"` for "John Doe"). The initials are shown
    /// as white text on the avatar's background colour.
    pub fn initials(mut self, initials: impl Into<String>) -> Self {
        self.initials = Some(initials.into());
        self
    }

    /// Set an icon name to display instead of initials.
    ///
    /// When set, the icon overrides the initials text.
    /// The icon should be a named icon (e.g. `"user-avatar-symbolic"`).
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the avatar size in pixels.
    ///
    /// Defaults to `40`. The widget is rendered as a circle
    /// with this diameter.
    pub fn size(mut self, size: i32) -> Self {
        self.size = size;
        self
    }

    /// Build the avatar widget.
    ///
    /// Returns a configured [`libadwaita::Avatar`] with the
    /// `.relm4-avatar` CSS class applied.
    pub fn build(self) -> libadwaita::Avatar {
        let text = self.initials.as_deref();
        let avatar = libadwaita::Avatar::new(
            self.size,
            text,
            self.icon.is_none(), // show_initials only when no icon is set
        );
        avatar.add_css_class("relm4-avatar");

        if let Some(icon_name) = &self.icon {
            avatar.set_icon_name(Some(icon_name));
        }

        avatar
    }
}
