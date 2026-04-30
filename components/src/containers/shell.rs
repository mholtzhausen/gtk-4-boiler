//! AppShell — the full application frame with sidebar, header bar, content
//! area, and toast notification overlay.
//!
//! `AppShell` is a relm4 [`SimpleComponent`] whose root is an
//! [`libadwaita::ApplicationWindow`].  It composes a
//! [`Sidebar`](super::sidebar::Sidebar) on the left, a
//! [`ToastStack`](super::toast_stack::ToastStack) overlaid on a
//! [`gtk4::Stack`] content area on the right, and automatically
//! instantiates a [`DarkModeWatcher`](crate::theme::DarkModeWatcher).
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::{AppShell, NavItem};
//!
//! let shell = AppShell::builder()
//!     .title("My App")
//!     .size(1200, 800)
//!     .sidebar(vec![
//!         NavItem::new("Home", "home-symbolic", "home"),
//!         NavItem::new("Settings", "settings-symbolic", "settings"),
//!     ])
//!     .on_navigate(|id| AppShellOutput::NavigateTo(id))
//!     .build();
//! ```

use gtk4::prelude::*;
use relm4::{Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent};

// ============================================================================
// Messages & Output
// ============================================================================

/// Messages sent to the [`AppShell`] component.
#[derive(Debug, Clone)]
pub enum AppShellMsg {
    /// A sidebar item was selected; forward the target page id.
    SidebarNavigated(String),
}

/// Messages emitted by [`AppShell`] to its parent component.
#[derive(Debug, Clone)]
pub enum AppShellOutput {
    /// Navigate to the page identified by the given id.
    NavigateTo(String),
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for configuring and constructing an [`AppShell`].
///
/// Collects window title, size, sidebar items, and an optional
/// navigation callback, then produces a [`Controller<AppShell>`]
/// via [`AppShellBuilder::build`].
pub struct AppShellBuilder {
    title: String,
    width: i32,
    height: i32,
    sidebar_items: Vec<super::NavItem>,
    _on_navigate: Option<Box<dyn Fn(String) -> AppShellOutput>>,
}

impl Default for AppShellBuilder {
    fn default() -> Self {
        Self {
            title: "App".into(),
            width: 1200,
            height: 800,
            sidebar_items: Vec::new(),
            _on_navigate: None,
        }
    }
}

impl AppShellBuilder {
    /// Set the window title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the initial window size.
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the sidebar navigation items.
    pub fn sidebar(mut self, items: Vec<super::NavItem>) -> Self {
        self.sidebar_items = items;
        self
    }

    /// Provide a callback that maps a navigation target id to the
    /// [`AppShellOutput`] variant to emit when a sidebar item is selected.
    pub fn on_navigate(mut self, f: impl Fn(String) -> AppShellOutput + 'static) -> Self {
        self._on_navigate = Some(Box::new(f));
        self
    }

    /// Build and launch the [`AppShell`] component, returning a
    /// [`Controller`] that keeps it alive.
    ///
    /// The returned controller should be kept alive for the lifetime of
    /// the application (e.g. stored in an [`Option`] field or leaked with
    /// [`Box::leak`]).
    pub fn build(self) -> Controller<AppShell> {
        AppShell::builder().launch(self).detach()
    }
}

// ============================================================================
// AppShell model
// ============================================================================

/// The main application shell — a full-window frame with a sidebar,
/// header bar, content stack, and toast notification overlay.
///
/// Built with [`AppShell::builder()`].
pub struct AppShell {
    /// Controller for the sidebar component.
    pub sidebar: Controller<super::Sidebar>,
    /// The content stack holding application pages.
    pub content_stack: gtk4::Stack,
    /// Controller for the toast notification stack.
    pub toasts: Controller<super::ToastStack>,
    /// Dark-mode watcher component.
    pub dark_mode: Controller<crate::theme::DarkModeWatcher>,
}

// ============================================================================
// Component implementation
// ============================================================================

#[relm4::component(pub)]
impl SimpleComponent for AppShell {
    /// Initialisation data — the builder with all configuration.
    type Init = AppShellBuilder;
    /// Messages that drive [`AppShell::update`].
    type Input = AppShellMsg;
    /// Messages emitted to the parent component.
    type Output = AppShellOutput;

    view! {
        #[root]
        libadwaita::ApplicationWindow {
            add_css_class: "relm4-shell",

            // Outer vertical box: header bar on top, main content below.
            #[name = "main_box"]
            gtk4::Box {
                set_orientation: gtk4::Orientation::Vertical,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        // ----- Window properties (set after model construction) -----
        root.set_title(Some(&init.title));
        root.set_default_size(init.width, init.height);

        // ----- Header bar -----
        let header = libadwaita::HeaderBar::new();
        header.add_css_class("relm4-header");
        widgets.main_box.append(&header);

        // ----- Horizontal paned: sidebar | content area -----
        let paned = gtk4::Paned::new(gtk4::Orientation::Horizontal);
        paned.set_wide_handle(false);
        paned.set_position(220);
        paned.set_vexpand(true);
        paned.set_hexpand(true);

        // ----- Sidebar (left pane) -----
        let sidebar = super::Sidebar::builder()
            .launch(init.sidebar_items)
            .forward(sender.input_sender(), |output| match output {
                super::SidebarOutput::Navigate(id) => AppShellMsg::SidebarNavigated(id),
            });
        paned.set_start_child(Some(sidebar.widget()));

        // ----- Content stack (right pane) -----
        let content_stack = gtk4::Stack::new();
        content_stack.set_vexpand(true);
        content_stack.set_hexpand(true);
        content_stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        content_stack.set_hhomogeneous(false);
        content_stack.set_vhomogeneous(false);

        // ----- Toast overlay wrapping the content stack -----
        let toasts = super::ToastStack::builder()
            .launch(content_stack.clone().upcast::<gtk4::Widget>())
            .detach();
        paned.set_end_child(Some(toasts.widget()));

        widgets.main_box.append(&paned);

        // ----- Dark-mode watcher (auto-started) -----
        let dark_mode = crate::theme::DarkModeWatcher::new();

        let model = AppShell {
            sidebar,
            content_stack,
            toasts,
            dark_mode,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppShellMsg::SidebarNavigated(id) => {
                log::debug!("AppShell: navigating to page '{}'", id);
                let _ = sender.output(AppShellOutput::NavigateTo(id));
            }
        }
    }
}
