//! relm4-kit Demo Application
//!
//! Showcases every component provided by the library.  The app is
//! structured as a sidebar-driven shell with one page per component.

use gtk4::prelude::*;
use relm4::{ComponentController, Controller};
use relm4_kit::containers::{AppShell, AppShellBuilder, NavItem};
use relm4_kit::theme;

mod pages;

// ============================================================================
// Entry point
// ============================================================================

fn main() -> glib::ExitCode {
    let app = gtk4::Application::new(Some("com.relm4-kit.demo"), Default::default());

    app.connect_activate(|app| {
        // Initialise the theme system.  Must happen after the application is
        // created so that GTK and the display are available for the CSS
        // provider.
        theme::init();

        // ---- Sidebar navigation items ----

        let sidebar_items = vec![
            NavItem::section("Getting Started"),
            NavItem::new("Welcome", "starred-symbolic", "welcome"),
            NavItem::new("Theme Showcase", "applications-graphics-symbolic", "theme"),
            NavItem::section("Primitives"),
            NavItem::new("Card", "x-office-document-symbolic", "card"),
            NavItem::new("Button", "input-mouse-symbolic", "button"),
            NavItem::new("Toggle", "toggle-on-symbolic", "toggle"),
            NavItem::new("Badge", "emblem-important-symbolic", "badge"),
            NavItem::section("Containers"),
            NavItem::new("Sidebar", "view-list-symbolic", "sidebar"),
            NavItem::new("Toast", "dialog-information-symbolic", "toast"),
            NavItem::new("Dialog", "help-about-symbolic", "dialog"),
            NavItem::new("Empty State", "folder-drag-accept-symbolic", "empty-state"),
        ];

        // ---- Build the application shell ----

        let shell: Controller<AppShell> = AppShellBuilder::default()
            .title("relm4-kit Demo")
            .size(1200, 800)
            .sidebar(sidebar_items)
            .build();

        // ---- Register the shell's window with the application ----

        // The AppShell creates an `AdwApplicationWindow` in its `view!` macro,
        // but it is not automatically associated with the GtkApplication.
        // We must add it here so that the window becomes visible.
        let window = shell.widget();
        app.add_window(window.upcast_ref::<gtk4::Window>());
        window.present();

        // ---- Build all page widgets and add them to the content stack ----

        let model = shell.model();

        // Getting Started
        let welcome_page = pages::welcome::create();
        model
            .content_stack
            .add_titled(&welcome_page, Some("welcome"), "Welcome");

        let theme_page = pages::theme_showcase::create();
        model
            .content_stack
            .add_titled(&theme_page, Some("theme"), "Theme Showcase");

        // Primitives
        let card_page = pages::card_demo::create();
        model
            .content_stack
            .add_titled(&card_page, Some("card"), "Card Demo");

        let button_page = pages::button_demo::create();
        model
            .content_stack
            .add_titled(&button_page, Some("button"), "Button Demo");

        let toggle_page = pages::toggle_demo::create();
        model
            .content_stack
            .add_titled(&toggle_page, Some("toggle"), "Toggle Demo");

        let badge_page = pages::badge_demo::create();
        model
            .content_stack
            .add_titled(&badge_page, Some("badge"), "Badge Demo");

        // Containers
        let sidebar_page = pages::sidebar_demo::create();
        model
            .content_stack
            .add_titled(&sidebar_page, Some("sidebar"), "Sidebar Demo");

        let toast_page = pages::toast_demo::create();
        model
            .content_stack
            .add_titled(&toast_page, Some("toast"), "Toast Demo");

        let dialog_page = pages::dialog_demo::create();
        model
            .content_stack
            .add_titled(&dialog_page, Some("dialog"), "Dialog Demo");

        let empty_state_page = pages::empty_state_demo::create();
        model
            .content_stack
            .add_titled(&empty_state_page, Some("empty-state"), "Empty State Demo");

        // Start on the welcome page.
        if let Some(first_page) = model.content_stack.child_by_name("welcome") {
            model.content_stack.set_visible_child(&first_page);
        }

        // shell is a detached controller so it stays alive for the
        // lifetime of the window.
    });

    app.run()
}
