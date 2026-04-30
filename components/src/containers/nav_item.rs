//! NavItem — a data struct representing a navigation item for the sidebar.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::NavItem;
//!
//! let item = NavItem::new("Dashboard", "dashboard-symbolic", "dashboard")
//!     .with_badge(3)
//!     .with_children(vec![
//!         NavItem::new("Overview", "overview-symbolic", "overview"),
//!         NavItem::new("Analytics", "chart-symbolic", "analytics"),
//!     ]);
//!
//! let section = NavItem::section("Workspace");
//! ```

/// A navigation item for use with [`Sidebar`](super::sidebar::Sidebar).
///
/// NavItem is a pure-data struct — it stores the label, icon, id, badge
/// count, child items, and whether it is a section header. It does not
/// create any widgets; the [`Sidebar`] component reads these items and
/// renders them.
#[derive(Clone)]
pub struct NavItem {
    pub label: String,
    pub icon: Option<String>,
    pub id: String,
    pub badge: Option<u32>,
    pub children: Vec<NavItem>,
    pub section: bool,
}

impl NavItem {
    /// Create a new navigation item with an icon.
    ///
    /// - `label` — displayed text for the item.
    /// - `icon` — named icon (e.g. `"dashboard-symbolic"`).
    /// - `id` — unique identifier used for navigation matching.
    ///
    /// For items without an icon, use [`Self::without_icon`].
    pub fn new(
        label: impl Into<String>,
        icon: impl Into<String>,
        id: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            icon: Some(icon.into()),
            id: id.into(),
            badge: None,
            children: Vec::new(),
            section: false,
        }
    }

    /// Create a navigation item without an icon.
    pub fn without_icon(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            id: id.into(),
            badge: None,
            children: Vec::new(),
            section: false,
        }
    }

    /// Set a badge count on this item (shown as a small number label).
    pub fn with_badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }

    /// Add child items, making this a collapsible parent entry.
    pub fn with_children(mut self, children: Vec<NavItem>) -> Self {
        self.children = children;
        self
    }

    /// Create a section header item (non-interactive, used to visually
    /// group items in the sidebar).
    pub fn section(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            id: String::new(),
            badge: None,
            children: Vec::new(),
            section: true,
        }
    }
}
