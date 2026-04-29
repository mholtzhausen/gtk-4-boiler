//! Sidebar — a navigation list with icons, badges, and collapsible nesting.

/// Messages for the Sidebar component.
pub enum SidebarMsg {
    ItemSelected(String),
}

/// Output messages from the Sidebar component.
pub enum SidebarOutput {
    Navigate(String),
}

/// A sidebar component for app navigation.
pub struct Sidebar {
    // Placeholder
}
