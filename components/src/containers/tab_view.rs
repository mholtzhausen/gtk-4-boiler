//! TabView — tabbed document/workspace views wrapping `AdwTabBar` + `AdwTabView`.
//!
//! Provides a managed relm4 component that presents tabs in a tab bar with
//! support for switching, closing, and reordering. Each tab wraps an
//! arbitrary GTK widget.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::{TabView, TabEntry};
//!
//! let tab_view = TabView::builder()
//!     .launch(vec![
//!         TabEntry::new("Home", gtk4::Label::new(Some("Home content")))
//!             .closable(false),
//!         TabEntry::new("Documents", gtk4::Label::new(Some("Documents content")))
//!             .closable(true),
//!     ])
//!     .detach();
//! ```

use gtk4::prelude::*;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

// ============================================================================
// TabEntry
// ============================================================================

/// Data for a single tab.
///
/// Use [`TabEntry::new()`] to create a basic tab and chain builder methods
/// to configure optional properties.
#[derive(Debug, Clone)]
pub struct TabEntry {
    /// Display title shown in the tab bar.
    pub title: String,
    /// The widget shown when this tab is active.
    pub child: gtk4::Widget,
    /// Whether the tab can be closed via the close button.
    pub closable: bool,
    /// Optional icon name (symbolic) for the tab.
    pub icon: Option<String>,
    /// Optional tooltip text for the tab.
    pub tooltip: Option<String>,
}

impl TabEntry {
    /// Create a new tab entry with the given title and child widget.
    pub fn new(title: impl Into<String>, child: impl IsA<gtk4::Widget>) -> Self {
        Self {
            title: title.into(),
            child: child.upcast(),
            closable: true,
            icon: None,
            tooltip: None,
        }
    }

    /// Set whether this tab shows a close button.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set an icon name (symbolic) for the tab.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set a tooltip for the tab.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

// ============================================================================
// Internal: per-tab state
// ============================================================================

/// Runtime state for a tab — pairs the user-provided entry with the
/// libadwaita TabPage returned by `TabView::append()`.
#[derive(Debug)]
struct TabState {
    /// The user-provided entry data.
    #[allow(dead_code)]
    entry: TabEntry,
    /// The libadwaita page handle returned from `append()`.
    page: libadwaita::TabPage,
}

// ============================================================================
// Messages & Output
// ============================================================================

/// Messages sent to the [`TabView`] component.
#[derive(Debug, Clone)]
pub enum TabViewMsg {
    /// Switch to the tab at the given index.
    SwitchTab(usize),
    /// Close the tab at the given index.
    CloseTab(usize),
}

/// Messages emitted by [`TabView`] to its parent.
#[derive(Debug, Clone)]
pub enum TabViewOutput {
    /// The tab at the given index was selected.
    TabSwitched(usize),
    /// The tab at the given index was closed.
    TabClosed(usize),
}

// ============================================================================
// Model
// ============================================================================

/// A managed component wrapping `AdwTabBar` and `AdwTabView`.
///
/// Holds a list of [`TabEntry`] items and renders them in the tab bar.
/// The parent receives [`TabViewOutput`] messages when the user switches
/// or closes tabs.
pub struct TabView {
    /// Runtime state for all open tabs.
    tabs: Vec<TabState>,
    /// The underlying libadwaita TabView widget.
    tab_view: libadwaita::TabView,
    /// Whether the close-page signal handler is connected (connected once).
    _close_handler_connected: bool,
}

// ============================================================================
// Component
// ============================================================================

#[relm4::component(pub)]
impl SimpleComponent for TabView {
    /// Initialisation data: the list of [`TabEntry`]s.
    type Init = Vec<TabEntry>;

    type Input = TabViewMsg;
    type Output = TabViewOutput;

    view! {
        #[root]
        gtk4::Box {
            set_orientation: gtk4::Orientation::Vertical,
            set_css_classes: &["relm4-tab-view"],

            #[name = "tab_bar"]
            libadwaita::TabBar {
                set_css_classes: &["relm4-tab-bar"],
                set_autohide: false,
                set_expand_tabs: true,
            },

            #[name = "content_stack"]
            libadwaita::TabView {
                set_vexpand: true,
                set_hexpand: true,
            }
        }
    }

    fn init(
        entries: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        // Link the TabBar to the TabView.
        widgets.tab_bar.set_view(Some(&widgets.content_stack));

        let tab_view = widgets.content_stack.clone();

        // Build initial tabs.
        let mut tabs: Vec<TabState> = Vec::new();
        for entry in entries {
            let page = tab_view.append(&entry.child);
            page.set_title(&entry.title);
            if !entry.closable {
                page.set_indicator_activatable(false);
            }
            if let Some(ref icon) = entry.icon {
                let themed_icon = gtk4::gio::ThemedIcon::new(icon);
                page.set_icon(Some(&themed_icon));
            }
            if let Some(ref tooltip) = entry.tooltip {
                page.set_tooltip(tooltip);
            }
            tabs.push(TabState { entry, page });
        }

        // Connect close-page signal — this fires when the user clicks
        // the close button on a tab.
        let sender_clone_switch = sender.clone();
        let sender_clone_close = sender.clone();
        tab_view.connect_close_page(move |view, page| {
            // Find the index of this page.
            if let Some(idx) = find_page_index(view, page) {
                // Emit the close output so the parent can react.
                if page.is_pinned() {
                    return glib::Propagation::Proceed;
                }
                let _ = sender_clone_close.output(TabViewOutput::TabClosed(idx));
            }
            glib::Propagation::Proceed
        });

        // Connect selected-page notify to detect tab switches (including
        // keyboard shortcuts and drag reorder).
        tab_view.connect_selected_page_notify(move |view| {
            if let Some(selected) = view.selected_page() {
                if let Some(idx) = find_page_index(view, &selected) {
                    let _ = sender_clone_switch.output(TabViewOutput::TabSwitched(idx));
                }
            }
        });

        let model = TabView {
            tabs,
            tab_view,
            _close_handler_connected: true,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            TabViewMsg::SwitchTab(index) => {
                if let Some(state) = self.tabs.get(index) {
                    self.tab_view.set_selected_page(&state.page);
                }
            }
            TabViewMsg::CloseTab(index) => {
                if index < self.tabs.len() {
                    let state = self.tabs.remove(index);
                    self.tab_view.close_page(&state.page);
                }
            }
        }
    }
}

// ============================================================================
// Helpers
// ============================================================================

/// Find the index of a given [`libadwaita::TabPage`] in the tab view.
fn find_page_index(view: &libadwaita::TabView, page: &libadwaita::TabPage) -> Option<usize> {
    let pages = view.pages();
    for i in 0..pages.n_items() {
        if let Some(item) = pages.item(i) {
            if let Ok(tab_page) = item.downcast::<libadwaita::TabPage>() {
                if tab_page == *page {
                    return Some(i as usize);
                }
            }
        }
    }
    None
}
