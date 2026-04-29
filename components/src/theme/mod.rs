//! Theme initialization and dark mode watching.
//!
//! Provides `init()` and `init_with_overrides()` to load the embedded
//! theme CSS, plus a [`DarkModeWatcher`] relm4 component that
//! automatically toggles the `.dark` CSS class on all toplevel
//! windows when the system dark mode preference changes.

pub mod tokens;

use gtk4::prelude::{ObjectExt, WidgetExt};
use relm4::{Component, ComponentParts, ComponentSender, Controller, SimpleComponent};
use std::fmt::Debug;

pub use tokens::*;

// ============================================================================
// Re-exports
// ============================================================================

/// The full theme CSS stylesheet, embedded at compile time.
///
/// Contains all CSS variables, dark mode overrides, and pre-styled
/// component classes for every relm4-kit primitive and container.
pub const THEME_CSS: &str = include_str!("theme.css");

// ============================================================================
// Public API — init functions
// ============================================================================

/// Initialise the relm4-kit theme system.
///
/// Loads the embedded `theme.css` into a [`gtk4::CssProvider`] and
/// attaches it to the default display at
/// [`gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION`].
///
/// Must be called exactly once **before** any widgets are created
/// (typically right after `gtk::Application::new()`).
///
/// # Panics
///
/// Panics if no default display is available.
///
/// # Example
///
/// ```ignore
/// use relm4_kit::theme;
///
/// theme::init();
/// ```
pub fn init() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(THEME_CSS);

    let display = gtk4::gdk::Display::default()
        .expect("relm4-kit: no default display available – did you call init() after creating the application?");
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Initialise the theme with additional CSS overrides.
///
/// Same as [`init()`], but appends `extra_css` after the default theme
/// CSS so applications can override variables or component styles
/// without modifying the theme itself.
///
/// # Panics
///
/// Panics if no default display is available.
///
/// # Example
///
/// ```ignore
/// use relm4_kit::theme;
///
/// theme::init_with_overrides(
///     r#"
///     :root {
///         --color-primary: #9334E6;
///     }
///     "#
/// );
/// ```
pub fn init_with_overrides(extra_css: &str) {
    let combined = format!("{}\n{}", THEME_CSS, extra_css);
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(&combined);

    let display = gtk4::gdk::Display::default()
        .expect("relm4-kit: no default display available – did you call init() after creating the application?");
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

// ============================================================================
// DarkModeWatcher
// ============================================================================

/// Messages sent to the [`DarkModeWatcher`] component.
#[derive(Debug, Clone)]
pub enum DarkModeMsg {
    /// The system dark-mode preference changed.
    DarkModeChanged(bool),
}

/// A relm4 component that watches the system dark-mode preference and
/// toggles the `.dark` CSS class on every toplevel window.
///
/// When the user switches between light and dark mode (either via the
/// system setting or through [`libadwaita::StyleManager`]), the watcher
/// adds or removes `"dark"` from each toplevel window's CSS class list
/// so that the `:root.dark { … }` overrides in `theme.css` take effect.
///
/// # Usage
///
/// ```ignore
/// use relm4_kit::theme::DarkModeWatcher;
///
/// let _watcher = DarkModeWatcher::new();
/// ```
pub struct DarkModeWatcher {
    /// Signal handler id for the `notify::is-dark` callback, kept
    /// alive for the lifetime of the component.
    _signal_id: glib::SignalHandlerId,
}

impl Debug for DarkModeWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DarkModeWatcher").finish()
    }
}

#[relm4::component(pub)]
impl SimpleComponent for DarkModeWatcher {
    type Init = ();
    type Input = DarkModeMsg;
    type Output = ();

    view! {
        #[root]
        gtk4::Box {
            set_visible: false,
            set_can_target: false,
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let style_manager = libadwaita::StyleManager::default();

        // Apply the initial dark mode state.
        toggle_dark_class(style_manager.is_dark());

        // Watch for future dark-mode changes.
        let sender_clone = sender.clone();
        let signal_id = style_manager.connect_notify(Some("is-dark"), move |sm, _| {
            let is_dark = sm.is_dark();
            sender_clone.input(DarkModeMsg::DarkModeChanged(is_dark));
        });

        let model = DarkModeWatcher {
            _signal_id: signal_id,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            DarkModeMsg::DarkModeChanged(is_dark) => {
                toggle_dark_class(is_dark);
            }
        }
    }
}

impl DarkModeWatcher {
    /// Create a new [`DarkModeWatcher`] component controller.
    ///
    /// Must be called **after** [`init()`] and after at least one
    /// toplevel window has been created (so the `.dark` class can be
    /// applied).
    pub fn new() -> Controller<Self> {
        Self::builder()
            .launch(())
            .detach()
    }
}

// ============================================================================
// Internal helpers
// ============================================================================

/// Add or remove the `"dark"` CSS class on every toplevel window.
fn toggle_dark_class(is_dark: bool) {
    for widget in gtk4::Window::list_toplevels() {
        if is_dark {
            widget.add_css_class("dark");
        } else {
            widget.remove_css_class("dark");
        }
    }
}
