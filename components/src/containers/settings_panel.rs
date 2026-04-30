//! SettingsPanel — wraps AdwPreferencesPage with a fluent row-definition API.
//!
//! Provides a builder pattern for constructing an AdwPreferencesPage with
//! groups of setting rows: toggles, dropdowns, text entries, and sliders.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::{SettingsPanel, SettingsRow};
//!
//! let page = SettingsPanel::new()
//!     .group("Notifications")
//!         .add_row(SettingsRow::toggle("Email alerts", |active| {
//!             sender.input(Msg::ToggleEmail(active))
//!         }))
//!         .add_row(SettingsRow::toggle("Push notifications", |active| {
//!             sender.input(Msg::TogglePush(active))
//!         }))
//!     .group("Display")
//!         .add_row(SettingsRow::dropdown("Theme", &["Light", "Dark", "System"], |idx| {
//!             sender.input(Msg::SetTheme(idx))
//!         }))
//!         .add_row(SettingsRow::slider("Text size", 10.0, 24.0, 14.0, |val| {
//!             sender.input(Msg::SetFontSize(val as u32))
//!         }))
//!     .group("Account")
//!         .add_row(SettingsRow::entry("Display name", |name| {
//!             sender.input(Msg::SetDisplayName(name))
//!         }))
//!     .build();
//! ```

use gtk4::prelude::*;
use libadwaita::prelude::*;

// ============================================================================
// SettingsRow
// ============================================================================

/// A single setting row to be added to a [`SettingsPanel`] group.
///
/// Each variant corresponds to a different libadwaita preferences row widget.
pub enum SettingsRow {
    /// An on/off switch with a label.
    Toggle {
        /// The label text.
        label: String,
        /// Optional subtitle displayed below the label.
        subtitle: Option<String>,
        /// Called with the new active state when the switch is toggled.
        on_toggle: Box<dyn Fn(bool) + 'static>,
    },
    /// A dropdown/combobox with a list of string options.
    Dropdown {
        /// The label text.
        label: String,
        /// Optional subtitle displayed below the label.
        subtitle: Option<String>,
        /// The list of selectable options.
        options: Vec<String>,
        /// Called with the selected index when the selection changes.
        on_select: Box<dyn Fn(u32) + 'static>,
    },
    /// A single-line text entry field.
    Entry {
        /// The label text.
        label: String,
        /// Called with the current text when the user presses Enter.
        on_activate: Box<dyn Fn(String) + 'static>,
    },
    /// A horizontal slider for numeric values.
    Slider {
        /// The label text.
        label: String,
        /// Optional subtitle displayed below the label.
        subtitle: Option<String>,
        /// Minimum value of the slider.
        min: f64,
        /// Maximum value of the slider.
        max: f64,
        /// Default/initial value of the slider.
        default: f64,
        /// Called with the current value as the slider is adjusted.
        on_change: Box<dyn Fn(f64) + 'static>,
    },
}

// ---------------------------------------------------------------------------
// Convenience constructors
// ---------------------------------------------------------------------------

impl SettingsRow {
    /// Create a toggle (switch) row.
    pub fn toggle(
        label: impl Into<String>,
        on_toggle: impl Fn(bool) + 'static,
    ) -> Self {
        Self::Toggle {
            label: label.into(),
            subtitle: None,
            on_toggle: Box::new(on_toggle),
        }
    }

    /// Create a dropdown (combo) row.
    pub fn dropdown(
        label: impl Into<String>,
        options: &[&str],
        on_select: impl Fn(u32) + 'static,
    ) -> Self {
        Self::Dropdown {
            label: label.into(),
            subtitle: None,
            options: options.iter().map(|s| s.to_string()).collect(),
            on_select: Box::new(on_select),
        }
    }

    /// Create a text entry row.
    pub fn entry(
        label: impl Into<String>,
        on_activate: impl Fn(String) + 'static,
    ) -> Self {
        Self::Entry {
            label: label.into(),
            on_activate: Box::new(on_activate),
        }
    }

    /// Create a slider row.
    pub fn slider(
        label: impl Into<String>,
        min: f64,
        max: f64,
        default: f64,
        on_change: impl Fn(f64) + 'static,
    ) -> Self {
        Self::Slider {
            label: label.into(),
            subtitle: None,
            min,
            max,
            default,
            on_change: Box::new(on_change),
        }
    }

    /// Set a subtitle on this row.
    ///
    /// Works with Toggle, Dropdown, and Slider variants.
    /// Entry rows do not support subtitles.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        match &mut self {
            Self::Toggle { subtitle: s, .. }
            | Self::Dropdown { subtitle: s, .. }
            | Self::Slider { subtitle: s, .. } => {
                *s = Some(subtitle.into());
            }
            Self::Entry { .. } => {}
        }
        self
    }
}

// ============================================================================
// SettingsPanel
// ============================================================================

/// A builder for constructing an AdwPreferencesPage with groups of setting
/// rows.
///
/// # Builder Pattern
///
/// 1. Create with [`SettingsPanel::new()`].
/// 2. Call `.group("Group Name")` to start a new group.
/// 3. Call `.add(SettingsRow::...)` one or more times to add rows.
/// 4. Repeat steps 2–3 for additional groups.
/// 5. Call `.build()` to obtain the [`AdwPreferencesPage`] widget.
pub struct SettingsPanel {
    current_group: Option<(String, Vec<libadwaita::PreferencesRow>)>,
    groups: Vec<(String, Vec<libadwaita::PreferencesRow>)>,
}

impl SettingsPanel {
    /// Create a new settings panel builder.
    pub fn new() -> Self {
        Self {
            current_group: None,
            groups: Vec::new(),
        }
    }

    /// Start a new preferences group with the given title.
    ///
    /// If a group was already in progress (rows were added since the last
    /// `.group()` call), that group is finalised and added to the page.
    pub fn group(mut self, title: impl Into<String>) -> Self {
        // Finalise any in-progress group.
        if let Some((title, rows)) = self.current_group.take() {
            self.groups.push((title, rows));
        }
        self.current_group = Some((title.into(), Vec::new()));
        self
    }

    /// Add a [`SettingsRow`] to the current group.
    ///
    /// # Panics
    ///
    /// Panics if `.group()` has not been called yet.
    pub fn add_row(mut self, row: SettingsRow) -> Self {
        let widget = Self::build_row(row);
        match &mut self.current_group {
            Some((_, rows)) => rows.push(widget),
            None => panic!("SettingsPanel::add_row() called before SettingsPanel::group()"),
        }
        self
    }

    /// Build the preferences page widget.
    ///
    /// Consumes the builder and returns the fully assembled
    /// [`AdwPreferencesPage`].
    pub fn build(mut self) -> libadwaita::PreferencesPage {
        // Finalise any in-progress group.
        if let Some((title, rows)) = self.current_group.take() {
            self.groups.push((title, rows));
        }

        let page = libadwaita::PreferencesPage::new();

        for (title, rows) in self.groups {
            let group = libadwaita::PreferencesGroup::new();
            group.set_title(&title);
            for row in rows {
                group.add(&row);
            }
            page.add(&group);
        }

        page
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------

    /// Convert a `SettingsRow` into an `AdwPreferencesRow` widget.
    fn build_row(row: SettingsRow) -> libadwaita::PreferencesRow {
        let widget: gtk4::Widget = match row {
            SettingsRow::Toggle {
                label,
                subtitle,
                on_toggle,
            } => Self::build_toggle_row(&label, subtitle, on_toggle).upcast(),
            SettingsRow::Dropdown {
                label,
                subtitle,
                options,
                on_select,
            } => Self::build_dropdown_row(&label, subtitle, &options, on_select).upcast(),
            SettingsRow::Entry {
                label,
                on_activate,
            } => Self::build_entry_row(&label, on_activate).upcast(),
            SettingsRow::Slider {
                label,
                subtitle,
                min,
                max,
                default,
                on_change,
            } => Self::build_slider_row(&label, subtitle, min, max, default, on_change).upcast(),
        };
        widget
            .downcast::<libadwaita::PreferencesRow>()
            .expect("SettingsRow variant produced a non-PreferencesRow widget")
    }

    fn build_toggle_row(
        label: &str,
        subtitle: Option<String>,
        on_toggle: Box<dyn Fn(bool) + 'static>,
    ) -> libadwaita::SwitchRow {
        let row = libadwaita::SwitchRow::builder().title(label).build();

        if let Some(sub) = subtitle {
            row.set_subtitle(&sub);
        }

        row.connect_active_notify(move |sw| {
            on_toggle(sw.is_active());
        });

        row
    }

    fn build_dropdown_row(
        label: &str,
        subtitle: Option<String>,
        options: &[String],
        on_select: Box<dyn Fn(u32) + 'static>,
    ) -> libadwaita::ComboRow {
        let model = gtk4::StringList::new(&options.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let row = libadwaita::ComboRow::builder()
            .title(label)
            .model(&model)
            .build();

        if let Some(sub) = subtitle {
            row.set_subtitle(&sub);
        }

        row.connect_selected_notify(move |combo| {
            on_select(combo.selected());
        });

        row
    }

    fn build_entry_row(
        label: &str,
        on_activate: Box<dyn Fn(String) + 'static>,
    ) -> libadwaita::EntryRow {
        let row = libadwaita::EntryRow::builder().title(label).build();

        row.connect_entry_activated(move |entry| {
            // AdwEntryRow implements gtk::Editable, so text() is available.
            let text = entry.text().to_string();
            on_activate(text);
        });

        row
    }

    fn build_slider_row(
        label: &str,
        subtitle: Option<String>,
        min: f64,
        max: f64,
        default: f64,
        on_change: Box<dyn Fn(f64) + 'static>,
    ) -> libadwaita::ActionRow {
        use gtk4::prelude::RangeExt;

        let row = libadwaita::ActionRow::builder().title(label).build();

        if let Some(sub) = subtitle {
            row.set_subtitle(&sub);
        }

        let step = (max - min) / 20.0;
        let scale =
            gtk4::Scale::with_range(gtk4::Orientation::Horizontal, min, max, step);
        scale.set_value(default);
        scale.set_draw_value(false);
        scale.set_size_request(180, -1);
        scale.set_halign(gtk4::Align::End);
        scale.set_valign(gtk4::Align::Center);
        scale.add_css_class("relm4-settings-slider");

        scale.connect_value_changed(move |s| {
            on_change(s.value());
        });

        row.add_suffix(&scale);
        row.set_activatable_widget(Some(&scale));

        row
    }
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_construction_succeeds() {
        let _panel = SettingsPanel::new();
    }

    #[test]
    fn subtitle_fluent_method() {
        let row = SettingsRow::toggle("Test", |_| {}).subtitle("A subtitle");
        match row {
            SettingsRow::Toggle { subtitle, .. } => {
                assert_eq!(subtitle.unwrap(), "A subtitle");
            }
            _ => panic!("expected Toggle variant"),
        }
    }

    #[test]
    #[should_panic(expected = "SettingsPanel::add_row() called before SettingsPanel::group()")]
    fn add_without_group_panics() {
        // Validate that .add() before .group() panics.
        // Note: .add() calls build_row() which constructs GTK widgets,
        // so we need GTK init even for the panic path.
        let _ = gtk4::init().ok();
        let _panel = SettingsPanel::new().add_row(SettingsRow::toggle("x", |_| {}));
    }
}
