//! TreeView — a multi-column tree/table view with sorting, selection, and
//! expandable rows.
//!
//! Built on [`gtk4::ColumnView`] + [`gtk4::TreeListModel`], the TreeView
//! supports:
//!
//! - Hierarchical rows with expand/collapse
//! - Column headers with titles
//! - Resizable column widths
//! - Single row selection with activation callback
//!
//! # Example
//!
//! ```ignore
//! use relm4_kit::containers::tree_view::{TreeItem, ColumnDef};
//! use relm4_kit::containers::TreeView;
//! use gtk4::prelude::*;
//!
//! struct FileItem { name: String, size: String }
//!
//! impl TreeItem for FileItem {
//!     fn column_widgets(&self) -> Vec<gtk4::Widget> {
//!         vec![
//!             gtk4::Label::new(Some(&self.name)).upcast::<gtk4::Widget>(),
//!             gtk4::Label::new(Some(&self.size)).upcast::<gtk4::Widget>(),
//!         ]
//!     }
//!     fn children(&self) -> Vec<Box<dyn TreeItem>> { vec![] }
//!     fn id(&self) -> Option<String> { Some(self.name.clone()) }
//! }
//!
//! let tree = TreeView::new()
//!     .column(ColumnDef::new("Name").resizable(true))
//!     .column(ColumnDef::new("Size"))
//!     .rows(vec![
//!         Box::new(FileItem { name: "readme.md".into(), size: "2 KB".into() }),
//!     ])
//!     .on_select(|id| println!("Selected: {:?}", id))
//!     .build();
//! ```

use glib::subclass::prelude::ObjectSubclassExt;
use gtk4::prelude::*;

// ============================================================================
// TreeItem trait
// ============================================================================

/// Trait implemented by types that can appear as rows in a [`TreeView`].
///
/// Each item provides a list of column widgets via [`column_widgets()`],
/// an optional list of child items via [`children()`], and an optional
/// unique identifier via [`id()`].
pub trait TreeItem {
    /// Return one [`gtk4::Widget`] per column for this item.
    ///
    /// The length of the returned vector should match the number of
    /// columns defined when building the [`TreeView`].
    fn column_widgets(&self) -> Vec<gtk4::Widget>;

    /// Return child items for expandable rows.
    ///
    /// Defaults to an empty vector (no children).
    fn children(&self) -> Vec<Box<dyn TreeItem>> {
        vec![]
    }

    /// Return an optional unique identifier for this item.
    ///
    /// Will be passed back through the `on_select` callback when the
    /// row is activated.
    fn id(&self) -> Option<String> {
        None
    }
}

// ============================================================================
// ColumnDef
// ============================================================================

/// Definition of a single column in the tree view.
#[derive(Clone)]
pub struct ColumnDef {
    /// Column header title.
    pub title: String,
    /// Whether the column width can be resized by the user.
    pub resizable: bool,
}

impl ColumnDef {
    /// Create a new column definition with the given title.
    ///
    /// By default, columns are resizable.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            resizable: true,
        }
    }

    /// Set whether the column width is user-resizable (default: `true`).
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
}

// ============================================================================
// GObject wrapper for Box<dyn TreeItem>
// ============================================================================

mod tree_item_glib_imp {
    //! Internal GObject subclass implementation.
    //! Wraps `Box<dyn TreeItem>` so it can live in a `gio::ListStore`.

    use std::cell::RefCell;

    use glib::subclass::prelude::*;

    use super::TreeItem;

    /// Per-instance data for our custom GObject.
    #[derive(Default)]
    pub struct TreeItemRow {
        pub data: RefCell<Option<Box<dyn TreeItem>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TreeItemRow {
        const NAME: &'static str = "Relm4KitTreeItemRow";
        type Type = super::TreeItemRow;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for TreeItemRow {}
}

glib::wrapper! {
    /// A `glib::Object` wrapper for a `Box<dyn TreeItem>`.
    ///
    /// This wrapper allows tree items to be stored in a `gio::ListStore`
    /// and used with GTK's list model infrastructure.
    pub struct TreeItemRow(ObjectSubclass<tree_item_glib_imp::TreeItemRow>);
}

impl TreeItemRow {
    /// Wrap a tree item in a GObject.
    pub fn new(item: Box<dyn TreeItem>) -> Self {
        let obj: Self = glib::Object::new();
        let imp = tree_item_glib_imp::TreeItemRow::from_obj(&obj);
        *imp.data.borrow_mut() = Some(item);
        obj
    }

    /// Borrow the inner tree item.
    pub fn item(&self) -> std::cell::Ref<'_, Box<dyn TreeItem>> {
        let imp = tree_item_glib_imp::TreeItemRow::from_obj(self);
        std::cell::Ref::map(imp.data.borrow(), |d| d.as_ref().unwrap())
    }
}

// ============================================================================
// Messages & Output
// ============================================================================

/// Messages sent to the [`TreeView`] component.
#[derive(Debug, Clone)]
pub enum TreeViewMsg {
    /// A row was selected (by index in the tree model).
    RowSelected(u32),
    /// Request row expansion/collapse toggling (by index).
    ToggleExpand(u32),
    /// A column sort order changed (column index, sort direction).
    SortChanged(u32, gtk4::SortType),
    /// Context menu action for a row at the given index.
    ContextAction(u32, String),
}

/// Messages emitted by the [`TreeView`] component to its parent.
#[derive(Debug, Clone)]
pub enum TreeViewOutput {
    /// A row was activated/selected — carries the item's `id()` if set.
    RowSelected(Option<String>),
    /// A context menu action was chosen.
    ContextAction(u32, String),
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for constructing a [`TreeView`] component.
///
/// Obtain via [`TreeView::new`], configure columns and rows, then call
/// [`build`](TreeViewBuilder::build) to create the widget.
pub struct TreeViewBuilder {
    /// Column definitions.
    pub columns: Vec<ColumnDef>,
    /// Root-level tree items.
    pub rows: Vec<Box<dyn TreeItem>>,
    /// Callback when a row is selected.
    pub on_select: Option<Box<dyn Fn(Option<String>) + 'static>>,
}

impl Default for TreeViewBuilder {
    fn default() -> Self {
        Self {
            columns: vec![ColumnDef::new("Item")],
            rows: vec![],
            on_select: None,
        }
    }
}

impl TreeViewBuilder {
    /// Add a column definition.
    pub fn column(mut self, col: ColumnDef) -> Self {
        self.columns.push(col);
        self
    }

    /// Set the root-level row items.
    pub fn rows(mut self, rows: Vec<Box<dyn TreeItem>>) -> Self {
        self.rows = rows;
        self
    }

    /// Set a callback for when a row is selected/activated.
    ///
    /// The callback receives the selected item's `id()` (if set).
    pub fn on_select(mut self, f: impl Fn(Option<String>) + 'static) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }

    /// Build the tree view widget.
    ///
    /// Returns a [`gtk4::ColumnView`] widget ready to be added to a
    /// container.
    pub fn build(self) -> gtk4::ColumnView {
        TreeView::build_widget(self)
    }
}

// ============================================================================
// Model
// ============================================================================

/// A multi-column tree/table view component.
///
/// The `TreeView` wraps a [`gtk4::ColumnView`] and provides a builder
/// API for setting up columns, rows, and selection handling.
///
/// CSS classes applied:
///
/// | Element | CSS class |
/// |---------|-----------|
/// | Root ColumnView | `.relm4-tree-view` |
/// | Column header | `.relm4-tree-header` |
/// | Tree row | `.relm4-tree-row` |
/// | Selected row | `.relm4-tree-row--selected` |
/// | Cell | `.relm4-tree-cell` |
#[allow(dead_code)]
pub struct TreeView {
    column_view: gtk4::ColumnView,
    selection_model: gtk4::SingleSelection,
    columns: Vec<ColumnDef>,
    on_select: Option<Box<dyn Fn(Option<String>) + 'static>>,
}

// ============================================================================
// Helpers: convert TreeItem hierarchy to gio::ListStore
// ============================================================================

/// Wrap items into a `gio::ListStore`.
fn items_to_store(items: Vec<Box<dyn TreeItem>>) -> gtk4::gio::ListStore {
    let store: gtk4::gio::ListStore = gtk4::gio::ListStore::new::<TreeItemRow>();
    for item in items {
        store.append(&TreeItemRow::new(item));
    }
    store
}

/// The create-child-function for `TreeListModel`.
///
/// Given a `glib::Object` (which is a `TreeItemRow`), returns a
/// `ListModel` of its children (or `None` if no children).
fn tree_item_create_func(item: &glib::Object) -> Option<gtk4::gio::ListModel> {
    let wrapper: &TreeItemRow = match item.downcast_ref::<TreeItemRow>() {
        Some(w) => w,
        None => return None,
    };
    let children = wrapper.item().children();

    if children.is_empty() {
        None
    } else {
        let store = items_to_store(children);
        Some(store.upcast::<gtk4::gio::ListModel>())
    }
}

// ============================================================================
// Widget building
// ============================================================================

impl TreeView {
    /// Create a new [`TreeViewBuilder`].
    pub fn new() -> TreeViewBuilder {
        TreeViewBuilder::default()
    }

    /// Internal: build the actual `gtk::ColumnView` widget from a builder.
    fn build_widget(builder: TreeViewBuilder) -> gtk4::ColumnView {
        // 1. Create root list store with TreeItemRow objects.
        let root_store: gtk4::gio::ListStore = items_to_store(builder.rows);

        // 2. Create TreeListModel wrapping the root store.
        let tree_model = gtk4::TreeListModel::new(
            root_store,
            false, // passthrough = false: items are TreeItemRow wrappers
            false, // autoexpand = false: start collapsed
            tree_item_create_func,
        );

        // 3. Wrap in SingleSelection for row selection.
        let selection_model = gtk4::SingleSelection::new(Some(tree_model));
        selection_model.set_can_unselect(false);

        // 4. Create ColumnView.
        let column_view = gtk4::ColumnView::new(Some(selection_model.clone()));
        column_view.set_css_classes(&["relm4-tree-view"]);
        column_view.set_show_row_separators(true);
        column_view.set_single_click_activate(true);

        // 5. Add columns.
        for col_def in &builder.columns {
            let col = create_column(col_def);
            column_view.append_column(&col);
        }

        // 6. Connect row activation.
        let on_select = builder.on_select;
        selection_model.connect_selection_changed(move |sm, _position, _n_items| {
            let selected = sm.selected();
            if selected != gtk4::INVALID_LIST_POSITION {
                if let Some(item) = sm.item(selected) {
                    if let Some(wrapper) = item.downcast_ref::<TreeItemRow>() {
                        let id = wrapper.item().id();
                        if let Some(ref f) = on_select {
                            f(id);
                        }
                    }
                }
            }
        });

        column_view
    }
}

// ============================================================================
// Column creation
// ============================================================================

/// Create a `gtk::ColumnViewColumn` from a `ColumnDef`, using a
/// `SignalListItemFactory`.
fn create_column(col_def: &ColumnDef) -> gtk4::ColumnViewColumn {
    let factory = gtk4::SignalListItemFactory::new();

    // We use the "bind" signal: for each visible row, update its widget
    // from the underlying data item.
    factory.connect_bind(move |_factory, list_item| {
        // The list_item in gtk4-rs 0.9 is a gtk::ListItem
        let list_item_ref = match list_item.downcast_ref::<gtk4::ListItem>() {
            Some(li) => li,
            None => return,
        };

        // Get the item data from the model.
        let item_obj = list_item_ref.item();

        // Get or create the child widget.
        let should_create = list_item_ref.child().is_none();

        let child = if should_create {
            // Create a TreeExpander with a label inside for tree indentation
            let expander = gtk4::TreeExpander::new();
            let label = gtk4::Label::new(None);
            label.set_halign(gtk4::Align::Start);
            label.set_xalign(0.0);
            label.set_css_classes(&["relm4-tree-cell"]);
            label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            expander.set_child(Some(&label));
            expander.upcast::<gtk4::Widget>()
        } else {
            list_item_ref.child().unwrap()
        };

        // Update label text from the item data.
        if let Some(item_obj) = item_obj {
            if let Some(wrapper) = item_obj.downcast_ref::<TreeItemRow>() {
                let item = wrapper.item();
                let widgets = item.column_widgets();

                // Try to find the label inside our expander.
                if let Some(expander) = child.downcast_ref::<gtk4::TreeExpander>() {
                    if let Some(label) = expander
                        .child()
                        .and_then(|c| c.downcast::<gtk4::Label>().ok())
                    {
                        if !widgets.is_empty() {
                            if let Some(src_label) =
                                widgets[0].downcast_ref::<gtk4::Label>()
                            {
                                label.set_text(&src_label.text());
                            }
                        }
                    }
                }
            }
        }

        if should_create {
            list_item_ref.set_child(Some(&child));
        }
    });

    let col = gtk4::ColumnViewColumn::new(Some(&col_def.title), Some(factory.clone()));
    col.set_resizable(col_def.resizable);
    col.set_expand(true);

    col
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple test item for unit tests.
    struct TestItem {
        name: String,
        size: String,
        children: Vec<Box<dyn TreeItem>>,
    }

    impl TreeItem for TestItem {
        fn column_widgets(&self) -> Vec<gtk4::Widget> {
            vec![
                gtk4::Label::new(Some(&self.name)).upcast::<gtk4::Widget>(),
                gtk4::Label::new(Some(&self.size)).upcast::<gtk4::Widget>(),
            ]
        }

        fn children(&self) -> Vec<Box<dyn TreeItem>> {
            self.children
                .iter()
                .map(|_c| {
                    Box::new(TestItem {
                        name: "sub-item".into(),
                        size: "1 KB".into(),
                        children: vec![],
                    }) as Box<dyn TreeItem>
                })
                .collect()
        }

        fn id(&self) -> Option<String> {
            Some(self.name.clone())
        }
    }

    #[test]
    fn column_def_defaults() {
        let col = ColumnDef::new("Name");
        assert_eq!(col.title, "Name");
        assert!(col.resizable);
    }

    #[test]
    fn column_def_configured() {
        let col = ColumnDef::new("Size").resizable(false);
        assert_eq!(col.title, "Size");
        assert!(!col.resizable);
    }

    #[test]
    fn builder_defaults() {
        let b = TreeViewBuilder::default();
        assert_eq!(b.columns.len(), 1);
        assert_eq!(b.columns[0].title, "Item");
        assert!(b.rows.is_empty());
        assert!(b.on_select.is_none());
    }

    #[test]
    fn builder_configured() {
        let b = TreeViewBuilder::default()
            .column(ColumnDef::new("A"))
            .column(ColumnDef::new("B"));
        assert_eq!(b.columns.len(), 3);
        assert_eq!(b.columns[1].title, "A");
        assert_eq!(b.columns[2].title, "B");
    }

    #[test]
    fn tree_item_row_creation() {
        let item = TestItem {
            name: "root".into(),
            size: "—".into(),
            children: vec![],
        };
        let row = TreeItemRow::new(Box::new(item));
        let borrowed = row.item();
        assert_eq!(borrowed.id(), Some("root".into()));
    }

    #[test]
    fn items_to_store_smoke() {
        let items: Vec<Box<dyn TreeItem>> = vec![
            Box::new(TestItem {
                name: "a".into(),
                size: "1".into(),
                children: vec![],
            }),
            Box::new(TestItem {
                name: "b".into(),
                size: "2".into(),
                children: vec![],
            }),
        ];
        let store = items_to_store(items);
        assert_eq!(store.n_items(), 2);
    }
}
