//! Settings page — application preferences built with SettingsPanel.

use gtk4::prelude::*;
use relm4::RelmWidgetExt;
use relm4_kit::containers::{SettingsPanel, SettingsRow};

/// Build the settings content widget.
pub fn page() -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.set_margin_all(24);

    let page = SettingsPanel::new()
        .group("General")
            .add_row(SettingsRow::toggle("Enable notifications", |_active| {
                // In a real app this would send a message to your component.
            }).subtitle("Receive alerts for important updates"))
        .group("Appearance")
            .add_row(SettingsRow::dropdown("Theme", &["Light", "Dark", "System"], |_idx| {
                // Wire to a real theme-switching message.
            }).subtitle("Choose your preferred colour scheme"))
            .add_row(SettingsRow::slider("Text size", 10.0, 24.0, 14.0, |_val| {
                // Adjust the application font size.
            }).subtitle("Customise the interface text size"))
        .group("Account")
            .add_row(SettingsRow::entry("Display name", |_name| {
                // Update the user's display name.
            }))
        .build();

    page.add_css_class("relm4-settings-page");
    container.append(&page);

    container
}
