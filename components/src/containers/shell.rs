//! AppShell — the full application frame with sidebar, header bar, content area, and toasts.

/// The AppShell builder.
pub struct AppShellBuilder {
    // Placeholder
}

impl AppShellBuilder {
    pub fn title(self, _title: impl Into<String>) -> Self { self }
    pub fn size(self, _width: i32, _height: i32) -> Self { self }
    pub fn sidebar(self, _items: Vec<super::NavItem>) -> Self { self }
}

/// The main application shell component.
pub struct AppShell {
    // Placeholder
}

impl AppShell {
    pub fn builder() -> AppShellBuilder {
        AppShellBuilder {}
    }
}
