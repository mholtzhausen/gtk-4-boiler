//! Sidebar Demo page — embedded sidebar preview with icons, badges,
//! nested children, section headers, collapsible groups, and a code panel.
//!
//! Shows a fully functional Sidebar component embedded in the content area
//! (not the app shell's sidebar). Selecting an item updates a status label
//! and content preview area.

use gtk4::prelude::*;
use relm4::{Component, ComponentController};
use relm4_kit::containers::{NavItem, Sidebar};

// ============================================================================
// Demo sidebar items
// ============================================================================

/// Build the sidebar items for the embedded demo sidebar.
fn demo_sidebar_items() -> Vec<NavItem> {
    vec![
        NavItem::section("General"),
        NavItem::new("Dashboard", "starred-symbolic", "dashboard")
            .with_badge(4),
        NavItem::new("Notifications", "alarm-symbolic", "notifications")
            .with_badge(12),
        NavItem::section("Workspace"),
        NavItem::new("Projects", "folder-symbolic", "projects")
            .with_children(vec![
                NavItem::new("Active", "document-open-recent-symbolic", "active")
                    .with_badge(3),
                NavItem::new("Archived", "folder-drag-accept-symbolic", "archived"),
                NavItem::new("Templates", "document-new-symbolic", "templates"),
            ]),
        NavItem::new("Team", "people-symbolic", "team")
            .with_children(vec![
                NavItem::new("Members", "avatar-default-symbolic", "members")
                    .with_badge(8),
                NavItem::new("Roles", "preferences-system-symbolic", "roles"),
                NavItem::new("Activity", "view-list-symbolic", "activity"),
            ]),
        NavItem::new("Analytics", "chart-line-symbolic", "analytics"),
        NavItem::section("System"),
        NavItem::new("Settings", "preferences-system-symbolic", "settings"),
        NavItem::new("Help", "help-browser-symbolic", "help"),
    ]
}

// ============================================================================
// Content map
// ============================================================================

/// Data for the content area shown when a sidebar item is selected.
struct ContentInfo {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
}

fn item_content(id: &str) -> ContentInfo {
    match id {
        "dashboard" => ContentInfo {
            icon: "starred-symbolic",
            title: "Dashboard",
            description: "Overview of your projects and recent activity. You have 4 new notifications.",
        },
        "notifications" => ContentInfo {
            icon: "alarm-symbolic",
            title: "Notifications",
            description: "12 unread notifications across all workspaces.",
        },
        "projects" => ContentInfo {
            icon: "folder-symbolic",
            title: "Projects",
            description: "Expand the Projects section to see Active, Archived, and Templates sub-pages.",
        },
        "active" => ContentInfo {
            icon: "document-open-recent-symbolic",
            title: "Active Projects",
            description: "3 active projects requiring attention. Click to view details.",
        },
        "archived" => ContentInfo {
            icon: "folder-drag-accept-symbolic",
            title: "Archived Projects",
            description: "Completed or shelved projects. Restore them to make active again.",
        },
        "templates" => ContentInfo {
            icon: "document-new-symbolic",
            title: "Project Templates",
            description: "Start a new project from a predefined template.",
        },
        "team" => ContentInfo {
            icon: "people-symbolic",
            title: "Team",
            description: "Expand the Team section to see Members, Roles, and Activity sub-pages.",
        },
        "members" => ContentInfo {
            icon: "avatar-default-symbolic",
            title: "Team Members",
            description: "8 team members in your workspace. Manage invites and permissions.",
        },
        "roles" => ContentInfo {
            icon: "preferences-system-symbolic",
            title: "Roles & Permissions",
            description: "Configure role-based access control for your workspace.",
        },
        "activity" => ContentInfo {
            icon: "view-list-symbolic",
            title: "Team Activity",
            description: "Recent activity log across all team members and projects.",
        },
        "analytics" => ContentInfo {
            icon: "chart-line-symbolic",
            title: "Analytics",
            description: "Usage metrics, performance data, and trend reports.",
        },
        "settings" => ContentInfo {
            icon: "preferences-system-symbolic",
            title: "Settings",
            description: "Application preferences, account settings, and workspace configuration.",
        },
        "help" => ContentInfo {
            icon: "help-browser-symbolic",
            title: "Help & Documentation",
            description: "Browse documentation, FAQs, and contact support.",
        },
        _ => ContentInfo {
            icon: "dialog-question-symbolic",
            title: "Unknown Page",
            description: "Select an item from the sidebar to see its content here.",
        },
    }
}

// ============================================================================
// Content area builder
// ============================================================================

/// Build the content area widget that displays the selected item's info.
fn build_content_area(id: &str) -> gtk4::Box {
    let info = item_content(id);

    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    content.add_css_class("sidebar-demo-content-area");
    content.set_hexpand(true);
    content.set_vexpand(true);

    // Centered container.
    let center = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    center.set_halign(gtk4::Align::Center);
    center.set_valign(gtk4::Align::Center);

    // Icon.
    let icon = gtk4::Image::from_icon_name(info.icon);
    icon.set_icon_size(gtk4::IconSize::Large);
    icon.set_pixel_size(48);
    icon.set_margin_bottom(16);
    icon.set_opacity(0.6);
    center.append(&icon);

    // Title.
    let title = gtk4::Label::new(Some(info.title));
    title.add_css_class("sidebar-demo-content-title");
    title.set_xalign(0.5);
    center.append(&title);

    // Description.
    let desc = gtk4::Label::new(Some(info.description));
    desc.add_css_class("sidebar-demo-content-desc");
    desc.set_wrap(true);
    desc.set_xalign(0.5);
    desc.set_max_width_chars(40);
    center.append(&desc);

    content.append(&center);
    content
}

// ============================================================================
// Code panel
// ============================================================================

/// Build a styled code panel showing the Sidebar API usage.
fn create_code_panel() -> gtk4::Box {
    let panel = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    panel.add_css_class("sidebar-demo-code-panel");
    panel.set_hexpand(true);

    // Title bar.
    let title_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_bar.add_css_class("sidebar-demo-code-title");
    panel.append(&title_bar);

    let title_text = gtk4::Label::new(Some("Rust — Sidebar Builder"));
    title_text.add_css_class("sidebar-demo-code-title-text");
    title_text.set_halign(gtk4::Align::Start);
    title_bar.append(&title_text);

    // Code content.
    let code_text = gtk4::Label::new(Some(CODE_SAMPLE));
    code_text.add_css_class("sidebar-demo-code-text");
    code_text.set_wrap(false);
    code_text.set_selectable(true);
    code_text.set_xalign(0.0);
    code_text.set_margin_top(12);
    code_text.set_margin_bottom(12);
    code_text.set_margin_start(16);
    code_text.set_margin_end(16);
    panel.append(&code_text);

    panel
}

/// A complete working example of Sidebar usage.
const CODE_SAMPLE: &str = r#"use relm4_kit::containers::{NavItem, Sidebar};

let items = vec![
    NavItem::section("General"),
    NavItem::new("Dashboard", "starred-symbolic", "dashboard")
        .with_badge(4),
    NavItem::new("Settings", "preferences-system-symbolic", "settings"),
    NavItem::section("Workspace"),
    NavItem::new("Projects", "folder-symbolic", "projects")
        .with_children(vec![
            NavItem::new("Active", "document-open-recent-symbolic", "active"),
            NavItem::new("Archived", "folder-drag-accept-symbolic", "archived"),
        ]),
    NavItem::new("Team", "people-symbolic", "team"),
    NavItem::section("System"),
    NavItem::new("Analytics", "chart-line-symbolic", "analytics"),
];

let sidebar = Sidebar::builder()
    .launch(items)
    .detach();

// Get the root widget to embed in your layout.
let widget = sidebar.widget();
"#;

// ============================================================================
// Status bar builder
// ============================================================================

/// Build the status bar showing the last selected sidebar item.
fn build_status_bar() -> gtk4::Label {
    let status = gtk4::Label::new(
        Some("Click a sidebar item to see its content here."),
    );
    status.add_css_class("sidebar-demo-status");
    status.set_halign(gtk4::Align::Fill);
    status.set_xalign(0.0);
    status.set_wrap(true);
    status
}

// ============================================================================
// Widget tree traversal helper (reused from card_demo)
// ============================================================================

trait FindChild {
    fn find_child(&self, name: &str) -> Option<gtk4::Widget>;
}

fn find_in_widget(widget: &gtk4::Widget, name: &str) -> Option<gtk4::Widget> {
    if widget.widget_name() == name {
        return Some(widget.clone());
    }
    let mut child = widget.first_child();
    while let Some(ref c) = child {
        if let Some(found) = find_in_widget(c, name) {
            return Some(found);
        }
        child = c.next_sibling();
    }
    None
}

impl FindChild for gtk4::Widget {
    fn find_child(&self, name: &str) -> Option<gtk4::Widget> {
        find_in_widget(self, name)
    }
}

impl FindChild for gtk4::Box {
    fn find_child(&self, name: &str) -> Option<gtk4::Widget> {
        find_in_widget(self.upcast_ref(), name)
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Build the sidebar demo page widget.
pub fn create() -> gtk4::Box {
    // ---- Page-specific CSS ----
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("sidebar_demo.css"));
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default()
            .expect("no display for CSS provider"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // ---- Page layout ----
    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer.set_vexpand(true);
    outer.set_hexpand(true);
    outer.add_css_class("relm4-page");

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    outer.append(&scrolled);

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.add_css_class("sidebar-demo-layout");
    container.set_margin_top(24);
    container.set_margin_bottom(48);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_spacing(0);

    // ---- Page header ----
    let title = gtk4::Label::new(Some("Sidebar Demo"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    let desc = gtk4::Label::new(Some(
        "Embedded sidebar component with icons, badges, nested children, \
         section headers, and collapsible parent items. Click items to \
         navigate and see content previews.",
    ));
    desc.add_css_class("relm4-page-subtitle");
    desc.set_wrap(true);
    desc.set_xalign(0.0);
    desc.set_margin_bottom(16);
    container.append(&desc);

    // ---- Status bar ----
    let status_label = build_status_bar();
    container.append(&status_label);

    // ---- Separator ----
    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.set_margin_top(16);
    sep.set_margin_bottom(16);
    container.append(&sep);

    // ---- Section label ----
    let preview_label = gtk4::Label::new(Some("Live Preview"));
    preview_label.add_css_class("sidebar-demo-section-label");
    preview_label.set_halign(gtk4::Align::Start);
    preview_label.set_margin_bottom(8);
    container.append(&preview_label);

    // ---- Sidebar + Content paned layout ----
    let paned = gtk4::Paned::new(gtk4::Orientation::Horizontal);
    paned.add_css_class("sidebar-demo-paned");
    paned.set_vexpand(true);
    paned.set_hexpand(true);
    paned.set_wide_handle(true);
    paned.set_position(240);
    container.append(&paned);

    // Build the sidebar component.
    let items = demo_sidebar_items();
    let connector = Sidebar::builder().launch(items);

    // Get the root widget before leaking.
    let sidebar_widget = connector.widget().clone();

    // Keep the connector alive for the lifetime of the page.
    // Safety: the connector is leaked intentionally — this is a demo app
    // and the sidebar must remain active for the page to function.
    Box::leak(Box::new(connector));

    paned.set_start_child(Some(&sidebar_widget));

    // Build initial content area (default: dashboard).
    let content_area = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    content_area.set_hexpand(true);
    content_area.set_vexpand(true);

    let dashboard_content = build_content_area("dashboard");
    content_area.append(&dashboard_content);
    paned.set_end_child(Some(&content_area));

    // ---- Wire sidebar listbox to update content area ----
    // Traverse the sidebar widget tree to find the ListBox and connect
    // a row-activated handler that updates the content preview.
    if let Some(list_box) = sidebar_widget.find_child("list_box")
        .and_then(|w| w.downcast::<gtk4::ListBox>().ok())
    {
        let status_clone = status_label.clone();
        let content_area_clone = content_area.clone();
        list_box.connect_row_activated(move |_lb, row| {
            // Read the nav-id from the row (set by the Sidebar component).
            let id = unsafe {
                row.data::<String>("nav-id")
                    .map(|ptr| ptr.as_ref().clone())
                    .unwrap_or_else(|| String::from("dashboard"))
            };

            // Update status bar.
            let info = item_content(&id);
            status_clone.set_markup(
                &format!(
                    "<b>Selected:</b> <span color='#3584e4'>{}</span> \
                     <span color='#5e5c64'>— {}</span>",
                    info.title, info.description,
                ),
            );

            // Update content area.
            while let Some(child) = content_area_clone.first_child() {
                content_area_clone.remove(&child);
            }
            let new_content = build_content_area(&id);
            content_area_clone.append(&new_content);
        });
    }

    // ---- Separator before code panel ----
    let sep2 = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep2.set_margin_top(24);
    sep2.set_margin_bottom(16);
    container.append(&sep2);

    // ---- Code panel ----
    let code_label = gtk4::Label::new(Some("Source Code"));
    code_label.add_css_class("sidebar-demo-section-label");
    code_label.set_halign(gtk4::Align::Start);
    code_label.set_margin_bottom(8);
    container.append(&code_label);

    let code_panel = create_code_panel();
    container.append(&code_panel);

    scrolled.set_child(Some(&container));
    outer
}
