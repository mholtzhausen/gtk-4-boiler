//! Sidebar — a navigation list with icons, badges, and collapsible nesting.
//!
//! Renders a list of [`NavItem`]s in a [`gtk4::ListBox`] with support for
//! section headers, expandable parent items, badges, and active-item
//! highlighting.
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::{NavItem, Sidebar};
//!
//! let sidebar = Sidebar::builder()
//!     .launch(vec![
//!         NavItem::section("Main"),
//!         NavItem::new("Dashboard", "dashboard-symbolic", "dashboard"),
//!         NavItem::new("Settings", "settings-symbolic", "settings"),
//!         NavItem::section("Workspace"),
//!         NavItem::new("Files", "folder-symbolic", "files")
//!             .with_children(vec![
//!                 NavItem::new("Recent", "document-open-recent-symbolic", "recent"),
//!                 NavItem::new("Shared", "emblem-shared-symbolic", "shared"),
//!             ]),
//!     ])
//!     .detach();
//! ```

use std::collections::HashSet;

use gtk4::prelude::*;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

use crate::containers::NavItem;

// ============================================================================
// Messages & Output
// ============================================================================

/// Messages sent to the [`Sidebar`] component.
#[derive(Debug, Clone)]
pub enum SidebarMsg {
    /// A navigation item was selected (by its `id`).
    ItemSelected(String),
    /// Toggle collapse/expand state of a parent item.
    ToggleCollapse(String),
}

/// Messages emitted by the [`Sidebar`] component to its parent.
#[derive(Debug, Clone)]
pub enum SidebarOutput {
    /// Navigate to the page identified by `id`.
    Navigate(String),
}

// ============================================================================
// Sidebar model
// ============================================================================

/// A sidebar component for app navigation.
///
/// Renders [`NavItem`]s in a scrollable [`gtk4::ListBox`]. Supports
/// section headers, collapsible parent items, icons, badges, and
/// active-item highlighting.
pub struct Sidebar {
    items: Vec<NavItem>,
    active_id: Option<String>,
    collapsed: HashSet<String>,
    list_box: gtk4::ListBox,
}

// ============================================================================
// Component implementation
// ============================================================================

#[relm4::component(pub)]
impl SimpleComponent for Sidebar {
    /// Initialisation data: the list of [`NavItem`]s to render.
    type Init = Vec<NavItem>;
    type Input = SidebarMsg;
    type Output = SidebarOutput;

    view! {
        #[root]
        gtk4::Box {
            set_css_classes: &["relm4-sidebar"],
            set_orientation: gtk4::Orientation::Vertical,
            set_width_request: 220,

            gtk4::ScrolledWindow {
                set_policy: (gtk4::PolicyType::Never, gtk4::PolicyType::Automatic),
                set_vexpand: true,

                #[name = "list_box"]
                gtk4::ListBox {
                    set_css_classes: &["relm4-sidebar-list"],
                    set_selection_mode: gtk4::SelectionMode::Browse,
                }
            }
        }
    }

    fn init(
        items: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let list_box = widgets.list_box.clone();

        // Connect row-activated — look up the item id stored on each row.
        let sender_clone = sender.clone();
        list_box.connect_row_activated(move |_lb, row| {
            if let Some(id) = get_row_data(row, "nav-id") {
                sender_clone.input(SidebarMsg::ItemSelected(id));
            }
        });

        // Build initial rows.
        rebuild_rows(&list_box, &items, &None, &HashSet::new(), 0);

        let model = Sidebar {
            items,
            active_id: None,
            collapsed: HashSet::new(),
            list_box,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            SidebarMsg::ItemSelected(id) => {
                // If the item has children, treat as collapse toggle rather
                // than navigation.
                let has_children = self
                    .items
                    .iter()
                    .chain(self.items.iter().flat_map(|i| &i.children))
                    .any(|item| item.id == id && !item.children.is_empty());

                if has_children {
                    // Toggle collapse state.
                    if self.collapsed.contains(&id) {
                        self.collapsed.remove(&id);
                    } else {
                        self.collapsed.insert(id);
                    }
                } else {
                    // Leaf item — navigate.
                    self.active_id = Some(id.clone());
                    let _ = sender.output(SidebarOutput::Navigate(id));
                }

                // Rebuild all rows to reflect new active/collapsed state.
                rebuild_rows(
                    &self.list_box,
                    &self.items,
                    &self.active_id,
                    &self.collapsed,
                    0,
                );
            }
            SidebarMsg::ToggleCollapse(id) => {
                if self.collapsed.contains(&id) {
                    self.collapsed.remove(&id);
                } else {
                    self.collapsed.insert(id);
                }
                rebuild_rows(
                    &self.list_box,
                    &self.items,
                    &self.active_id,
                    &self.collapsed,
                    0,
                );
            }
        }
    }
}

// ============================================================================
// Row-building helpers
// ============================================================================

/// Remove all rows from the list box and rebuild from scratch.
fn rebuild_rows(
    list_box: &gtk4::ListBox,
    items: &[NavItem],
    active_id: &Option<String>,
    collapsed: &HashSet<String>,
    depth: u32,
) {
    // Remove all existing child widgets (ListBoxRows).
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    // Build recursively.
    append_rows(list_box, items, active_id, collapsed, depth);
}

/// Append rows for the given items to the list box.
fn append_rows(
    list_box: &gtk4::ListBox,
    items: &[NavItem],
    active_id: &Option<String>,
    collapsed: &HashSet<String>,
    depth: u32,
) {
    for item in items {
        if item.section {
            // ---- Section header ----
            let row = build_section_row(item);
            list_box.append(&row);
        } else if !item.children.is_empty() {
            // ---- Parent item with children ----
            let is_collapsed = collapsed.contains(&item.id);
            let is_active = active_id.as_ref() == Some(&item.id);
            let row = build_parent_row(item, is_active, is_collapsed, depth);
            list_box.append(&row);

            // Append children if expanded.
            if !is_collapsed {
                append_rows(list_box, &item.children, active_id, collapsed, depth + 1);
            }
        } else {
            // ---- Leaf item ----
            let is_active = active_id.as_ref() == Some(&item.id);
            let row = build_leaf_row(item, is_active, depth);
            list_box.append(&row);
        }
    }
}

/// Build a section header row.
fn build_section_row(item: &NavItem) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.set_selectable(false);
    row.set_css_classes(&["relm4-sidebar-section"]);

    let label = gtk4::Label::new(Some(&item.label));
    label.set_halign(gtk4::Align::Start);
    label.set_xalign(0.0);
    label.set_margin_start(12);
    label.set_margin_top(8);
    label.set_margin_bottom(4);
    label.set_margin_end(12);

    row.set_child(Some(&label));
    row
}

/// Build a row for a parent item (has children, collapsible).
fn build_parent_row(
    item: &NavItem,
    is_active: bool,
    is_collapsed: bool,
    depth: u32,
) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.set_selectable(false); // We handle clicks manually via child buttons/boxes
    set_row_data(&row, "nav-id", item.id.clone());

    // Active CSS class.
    if is_active {
        row.add_css_class("relm4-sidebar-item--active");
    }
    row.add_css_class("relm4-sidebar-item");

    // --- Content box ---
    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    let indent = 12 + depth * 16;
    let indent_i32 = indent as i32;
    hbox.set_margin_start(indent_i32);
    hbox.set_margin_top(6);
    hbox.set_margin_bottom(6);
    hbox.set_margin_end(12);

    // Expand/collapse arrow.
    let arrow_icon = if is_collapsed {
        "pan-end-symbolic"
    } else {
        "pan-down-symbolic"
    };
    let arrow = gtk4::Image::from_icon_name(arrow_icon);
    arrow.set_icon_size(gtk4::IconSize::Normal);
    hbox.append(&arrow);

    // Item icon.
    if let Some(icon_name) = &item.icon {
        let icon = gtk4::Image::from_icon_name(icon_name);
        icon.set_icon_size(gtk4::IconSize::Normal);
        hbox.append(&icon);
    }

    // Label.
    let label = gtk4::Label::new(Some(&item.label));
    label.set_halign(gtk4::Align::Start);
    label.set_xalign(0.0);
    label.set_hexpand(true);
    hbox.append(&label);

    // Badge.
    if let Some(count) = item.badge {
        let badge = gtk4::Label::new(Some(&count.to_string()));
        badge.add_css_class("relm4-badge");
        badge.add_css_class("badge-neutral");
        badge.add_css_class("badge-small");
        hbox.append(&badge);
    }

    row.set_child(Some(&hbox));
    row
}

/// Build a row for a leaf item (no children).
fn build_leaf_row(
    item: &NavItem,
    is_active: bool,
    depth: u32,
) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    set_row_data(&row, "nav-id", item.id.clone());

    // Active CSS class.
    if is_active {
        row.add_css_class("relm4-sidebar-item--active");
    }
    row.add_css_class("relm4-sidebar-item");

    // --- Content box ---
    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    let indent = 12 + depth * 16;
    let indent_i32 = indent as i32;
    hbox.set_margin_start(indent_i32);
    hbox.set_margin_top(6);
    hbox.set_margin_bottom(6);
    hbox.set_margin_end(12);

    // Item icon.
    if let Some(icon_name) = &item.icon {
        let icon = gtk4::Image::from_icon_name(icon_name);
        icon.set_icon_size(gtk4::IconSize::Normal);
        hbox.append(&icon);
    }

    // Label.
    let label = gtk4::Label::new(Some(&item.label));
    label.set_halign(gtk4::Align::Start);
    label.set_xalign(0.0);
    label.set_hexpand(true);
    hbox.append(&label);

    // Badge.
    if let Some(count) = item.badge {
        let badge = gtk4::Label::new(Some(&count.to_string()));
        badge.add_css_class("relm4-badge");
        badge.add_css_class("badge-neutral");
        badge.add_css_class("badge-small");
        hbox.append(&badge);
    }

    row.set_child(Some(&hbox));
    row
}

// ============================================================================
// Safe wrappers for glib::ObjectExt unsafe data API
// ============================================================================

/// Store an item id on a [`gtk4::ListBoxRow`] so it can be retrieved when
/// the row is activated.
fn set_row_data(row: &gtk4::ListBoxRow, key: &str, value: String) {
    // Safety: `String` is `'static` and we always read back the same type.
    unsafe {
        row.set_data(key, value);
    }
}

/// Retrieve the item id previously stored on a [`gtk4::ListBoxRow`].
fn get_row_data(row: &gtk4::ListBoxRow, key: &str) -> Option<String> {
    // Safety: We only store `String` values with this key.
    unsafe {
        row.data::<String>(key).map(|ptr| ptr.as_ref().clone())
    }
}
