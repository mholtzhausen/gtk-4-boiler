//! NavItem — a data struct representing a navigation item for the sidebar.

/// A navigation item for use with `Sidebar`.
pub struct NavItem {
    pub label: String,
    pub icon: Option<String>,
    pub id: String,
    pub badge: Option<u32>,
    pub children: Vec<NavItem>,
    pub section: bool,
}

impl NavItem {
    /// Create a new navigation item.
    pub fn new(label: impl Into<String>, icon: Option<impl Into<String>>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: icon.map(|i| i.into()),
            id: id.into(),
            badge: None,
            children: Vec::new(),
            section: false,
        }
    }

    /// Set a badge count on this item.
    pub fn with_badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }

    /// Add child items (makes this a collapsible section).
    pub fn with_children(mut self, children: Vec<NavItem>) -> Self {
        self.children = children;
        self
    }

    /// Create a section header item (non-interactive).
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
