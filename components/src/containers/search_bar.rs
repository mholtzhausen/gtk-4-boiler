//! SearchBar — a debounced search input with dropdown results and keyboard
//! navigation.
//!
//! The `SearchBar` is a relm4 [`SimpleComponent`] built around
//! [`gtk4::SearchEntry`] and a [`gtk4::Popover`] containing a
//! [`gtk4::ListBox`] of results.
//!
//! # Data flow
//!
//! 1. The user types in the search entry.
//! 2. After a configurable debounce delay (default 300 ms), the component
//!    emits [`SearchBarOutput::SearchRequested`] with the current query
//!    string.
//! 3. The parent component computes results and sends them back via
//!    [`SearchBarMsg::SetResults`].
//! 4. The popover appears showing the results.  The user can navigate
//!    with ↑/↓ keys, select with Enter, or dismiss with Escape.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::SearchBar;
//!
//! let search = SearchBar::builder()
//!     .placeholder("Search…")
//!     .debounce_ms(200)
//!     .width(350)
//!     .launch(())
//!     .forward(sender.input_sender(), |output| match output {
//!         SearchBarOutput::SearchRequested(query) => Msg::Search(query),
//!         SearchBarOutput::ResultChosen(id) => Msg::Navigate(id),
//!         _ => Msg::None,
//!     });
//! ```

use gtk4::prelude::*;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

// ============================================================================
// SearchResult
// ============================================================================

/// A single search result item displayed in the result dropdown.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Primary display text.
    pub label: String,
    /// Optional secondary text shown below the label.
    pub description: Option<String>,
    /// Optional symbolic icon name shown before the label.
    pub icon: Option<String>,
    /// Unique identifier returned when this result is selected.
    pub id: String,
}

impl SearchResult {
    /// Create a new search result with a label and a unique id.
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            icon: None,
            id: id.into(),
        }
    }

    /// Set an optional description line.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set an optional symbolic icon name (e.g. `"document-symbolic"`).
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

// ============================================================================
// Messages & Output
// ============================================================================

/// Messages sent to the [`SearchBar`] component.
#[derive(Debug, Clone)]
pub enum SearchBarMsg {
    /// The search entry text changed (internal, from signal handler).
    QueryChanged(String),
    /// Set the result list shown in the dropdown.
    SetResults(Vec<SearchResult>),
    /// A result was selected by index (internal, from list box).
    ResultSelected(usize),
    /// Navigate selection up or down (internal, from key controller).
    NavigateSelection(i32),
    /// The search entry gained keyboard focus (internal).
    FocusGained,
    /// The search entry lost keyboard focus (internal).
    FocusLost,
    /// Dismiss the results popover.
    Dismiss,
}

/// Messages emitted by the [`SearchBar`] component to its parent.
#[derive(Debug, Clone)]
pub enum SearchBarOutput {
    /// A search query is ready for processing.  The parent should compute
    /// results and send them back via [`SearchBarMsg::SetResults`].
    SearchRequested(String),
    /// The user selected a result with the given `id`.
    ResultChosen(String),
    /// The popover was dismissed without selection.
    Dismissed,
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for configuring a [`SearchBar`] component.
///
/// Obtain via [`SearchBar::builder`], then configure and call
/// [`launch`](relm4::SimpleComponent::builder) to create the component.
#[derive(Debug)]
pub struct SearchBarBuilder {
    /// Placeholder text for the search entry.
    pub placeholder: Option<String>,
    /// Debounce delay in milliseconds (default 300).
    pub debounce_ms: u32,
    /// Width of the results popover in pixels (default 300).
    pub width: i32,
    /// Maximum height of the results popover in pixels (default 400).
    pub height: i32,
}

impl Default for SearchBarBuilder {
    fn default() -> Self {
        Self {
            placeholder: None,
            debounce_ms: 300,
            width: 300,
            height: 400,
        }
    }
}

impl SearchBarBuilder {
    /// Set placeholder text displayed inside the search entry when empty.
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set the debounce delay in milliseconds (default 300).
    ///
    /// After the user stops typing, the component waits this long before
    /// emitting [`SearchBarOutput::SearchRequested`].
    pub fn debounce_ms(mut self, ms: u32) -> Self {
        self.debounce_ms = ms;
        self
    }

    /// Set the width of the results popover in pixels (default 300).
    pub fn width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    /// Set the maximum height of the results popover in pixels
    /// (default 400).
    pub fn height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }
}

// ============================================================================
// Model
// ============================================================================

/// A debounced search input with a dropdown results panel.
///
/// Built with [`SearchBar::builder()`] or via the
/// [`relm4::SimpleComponent`] trait.
pub struct SearchBar {
    /// Current query text in the search entry.
    query: String,
    /// Current result list.
    results: Vec<SearchResult>,
    /// Index of the currently selected result (for keyboard navigation).
    selected_index: Option<usize>,
    /// Pending debounce timer source id.
    debounce_source: Option<glib::SourceId>,
    /// Reference to the search entry widget (needed for focus management).
    search_entry: gtk4::SearchEntry,
    /// Reference to the results list box (needed for row highlighting).
    results_list: gtk4::ListBox,
    /// Reference to the popover (needed for show/hide).
    popover: gtk4::Popover,
    /// Debounce delay in milliseconds.
    debounce_ms: u32,
}

// ============================================================================
// Component implementation
// ============================================================================

#[relm4::component(pub)]
impl SimpleComponent for SearchBar {
    /// Initialisation data — the builder with configuration options.
    type Init = SearchBarBuilder;
    /// Messages received by this component.
    type Input = SearchBarMsg;
    /// Messages emitted to the parent component.
    type Output = SearchBarOutput;

    view! {
        #[root]
        gtk4::Box {
            set_css_classes: &["relm4-search-bar"],
            set_orientation: gtk4::Orientation::Vertical,

            #[name = "search_entry"]
            gtk4::SearchEntry {
                set_placeholder_text: Some(init.placeholder.as_deref().unwrap_or("")),
                set_hexpand: true,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let search_entry = widgets.search_entry.clone();

        // ----- Create the results popover -----
        let popover = gtk4::Popover::new();
        popover.set_parent(&search_entry);
        popover.set_position(gtk4::PositionType::Bottom);
        popover.set_default_widget(Some(&search_entry));
        popover.set_size_request(init.width, init.height);

        // Scrollable results list inside the popover.
        let results_list = gtk4::ListBox::new();
        results_list.set_css_classes(&["relm4-search-results"]);
        results_list.set_selection_mode(gtk4::SelectionMode::None);

        let scrolled = gtk4::ScrolledWindow::new();
        scrolled.set_child(Some(&results_list));
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        popover.set_child(Some(&scrolled));

        // ----- Signals on the search entry -----

        // Text changed → reset debounce timer.
        let sender_clone = sender.clone();
        search_entry.connect_search_changed(move |entry| {
            let text = entry.text().to_string();
            sender_clone.input(SearchBarMsg::QueryChanged(text));
        });

        // Focus controller — replaces connect_focus_in/out_event.
        let sender_clone = sender.clone();
        let focus_controller = gtk4::EventControllerFocus::new();
        let sender_clone2 = sender_clone.clone();
        focus_controller.connect_enter(move |_| {
            sender_clone.input(SearchBarMsg::FocusGained);
        });
        focus_controller.connect_leave(move |_| {
            sender_clone2.input(SearchBarMsg::FocusLost);
        });
        search_entry.add_controller(focus_controller);

        // ----- Keyboard navigation on the search entry -----
        let sender_clone = sender.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_controller, keyval, _code, _mods| {
            match keyval {
                gtk4::gdk::Key::Down => {
                    sender_clone.input(SearchBarMsg::NavigateSelection(1));
                    glib::Propagation::Stop
                }
                gtk4::gdk::Key::Up => {
                    sender_clone.input(SearchBarMsg::NavigateSelection(-1));
                    glib::Propagation::Stop
                }
                gtk4::gdk::Key::Escape => {
                    sender_clone.input(SearchBarMsg::Dismiss);
                    glib::Propagation::Stop
                }
                gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                    // Select the currently highlighted result via a message.
                    // The component's update() will read selected_index.
                    sender_clone.input(SearchBarMsg::NavigateSelection(0));
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });
        search_entry.add_controller(key_controller);

        // ----- Signals on the results list -----
        let sender_clone = sender.clone();
        results_list.connect_row_activated(move |_lb, row| {
            let index = row.index() as usize;
            sender_clone.input(SearchBarMsg::ResultSelected(index));
        });

        let model = SearchBar {
            query: String::new(),
            results: Vec::new(),
            selected_index: None,
            debounce_source: None,
            search_entry: search_entry.clone(),
            results_list,
            popover,
            debounce_ms: init.debounce_ms,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            SearchBarMsg::QueryChanged(query) => {
                // Update query text.
                self.query = query.clone();

                // Cancel any pending debounce timer.
                if let Some(source_id) = self.debounce_source.take() {
                    source_id.remove();
                }

                // If query is empty, dismiss and clear results.
                if query.is_empty() {
                    self.results.clear();
                    self.selected_index = None;
                    self.popover.popdown();
                    self.rebuild_results();
                    return;
                }

                // Schedule a new debounce timer.
                let sender_clone = sender.clone();
                let source_id = glib::timeout_add_local_once(
                    std::time::Duration::from_millis(self.debounce_ms as u64),
                    move || {
                        // The query variable from the closure is the query at
                        // the time the timer was set. We emit it to the parent.
                        let _ = sender_clone
                            .output(SearchBarOutput::SearchRequested(query.clone()));
                    },
                );
                self.debounce_source = Some(source_id);
            }
            SearchBarMsg::SetResults(results) => {
                self.results = results;
                self.selected_index = None;
                self.rebuild_results();

                // Show popover if there are results.
                if !self.results.is_empty() && self.search_entry.has_focus() {
                    self.popover.popup();
                } else if self.results.is_empty() {
                    self.popover.popdown();
                }
            }
            SearchBarMsg::ResultSelected(index) => {
                if index < self.results.len() {
                    let id = self.results[index].id.clone();
                    // Clear state.
                    self.popover.popdown();
                    self.search_entry.set_text("");
                    self.results.clear();
                    self.selected_index = None;
                    self.rebuild_results();
                    // Emit selection.
                    let _ = sender.output(SearchBarOutput::ResultChosen(id));
                }
            }
            SearchBarMsg::NavigateSelection(delta) => {
                if self.results.is_empty() {
                    return;
                }
                let n = self.results.len();
                let new_index = match self.selected_index {
                    None if delta >= 0 => Some(0),
                    None => Some(n.saturating_sub(1)),
                    Some(idx) => {
                        let next = if delta == 0 {
                            // Enter pressed with selection — activate it.
                            let id = self.results[idx].id.clone();
                            self.popover.popdown();
                            self.search_entry.set_text("");
                            self.results.clear();
                            self.selected_index = None;
                            self.rebuild_results();
                            let _ = sender.output(SearchBarOutput::ResultChosen(id));
                            return;
                        } else {
                            (idx as i32 + delta).rem_euclid(n as i32) as usize
                        };
                        Some(next)
                    }
                };
                self.selected_index = new_index;
                self.rebuild_results();
            }
            SearchBarMsg::FocusGained => {
                // If there are existing results, show the popover again.
                if !self.results.is_empty() {
                    self.popover.popup();
                }
            }
            SearchBarMsg::FocusLost => {
                // Don't immediately dismiss — let the popover's own
                // auto-dismiss behaviour handle it.  However, if the
                // click went outside, Popover handles that automatically.
            }
            SearchBarMsg::Dismiss => {
                self.popover.popdown();
                self.search_entry.set_text("");
                self.query.clear();
                self.results.clear();
                self.selected_index = None;
                self.rebuild_results();
                let _ = sender.output(SearchBarOutput::Dismissed);
            }
        }
    }
}

// ============================================================================
// Internal helpers
// ============================================================================

impl SearchBar {
    /// Rebuild all rows in the results list box and update selection
    /// highlight.
    fn rebuild_results(&self) {
        // Remove all existing rows.
        while let Some(child) = self.results_list.first_child() {
            self.results_list.remove(&child);
        }

        for result in &self.results {
            let row = build_result_row(result);
            self.results_list.append(&row);
        }

        // Update selection highlight on the selected row.
        if let Some(idx) = self.selected_index {
            if let Some(row) = self.results_list.row_at_index(idx as i32) {
                row.add_css_class("relm4-search-result--selected");
            }
        }
    }
}

/// Build a single result row widget from a [`SearchResult`].
fn build_result_row(result: &SearchResult) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.add_css_class("relm4-search-result");
    row.set_selectable(false);

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    hbox.set_margin_start(12);
    hbox.set_margin_end(12);
    hbox.set_margin_top(8);
    hbox.set_margin_bottom(8);

    // Optional icon.
    if let Some(icon_name) = &result.icon {
        let icon = gtk4::Image::from_icon_name(icon_name);
        icon.set_icon_size(gtk4::IconSize::Normal);
        icon.set_valign(gtk4::Align::Start);
        hbox.append(&icon);
    }

    // Text content: label + optional description.
    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    text_box.set_hexpand(true);

    let label = gtk4::Label::new(Some(&result.label));
    label.set_halign(gtk4::Align::Start);
    label.set_xalign(0.0);
    label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    text_box.append(&label);

    if let Some(desc) = &result.description {
        let desc_label = gtk4::Label::new(Some(desc));
        desc_label.set_halign(gtk4::Align::Start);
        desc_label.set_xalign(0.0);
        desc_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        desc_label.add_css_class("dim-label");
        text_box.append(&desc_label);
    }

    hbox.append(&text_box);
    row.set_child(Some(&hbox));
    row
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_result_construction() {
        let r = SearchResult::new("Hello", "hello-id")
            .with_description("A greeting")
            .with_icon("face-smile-symbolic");
        assert_eq!(r.label, "Hello");
        assert_eq!(r.id, "hello-id");
        assert_eq!(r.description.as_deref(), Some("A greeting"));
        assert_eq!(r.icon.as_deref(), Some("face-smile-symbolic"));
    }

    #[test]
    fn search_result_minimal() {
        let r = SearchResult::new("Minimal", "min");
        assert_eq!(r.label, "Minimal");
        assert_eq!(r.id, "min");
        assert!(r.description.is_none());
        assert!(r.icon.is_none());
    }

    #[test]
    fn builder_defaults() {
        let b = SearchBarBuilder::default();
        assert!(b.placeholder.is_none());
        assert_eq!(b.debounce_ms, 300);
        assert_eq!(b.width, 300);
        assert_eq!(b.height, 400);
    }

    #[test]
    fn builder_configured() {
        let b = SearchBarBuilder::default()
            .placeholder("Type to search…")
            .debounce_ms(500)
            .width(400)
            .height(500);
        assert_eq!(b.placeholder.as_deref(), Some("Type to search…"));
        assert_eq!(b.debounce_ms, 500);
        assert_eq!(b.width, 400);
        assert_eq!(b.height, 500);
    }
}
