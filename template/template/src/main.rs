use gtk4::prelude::*;
use relm4::ComponentController;
use relm4_kit::containers::{AppShellBuilder, NavItem};
use relm4_kit::theme;

mod pages;

fn main() -> glib::ExitCode {
    // Create a GTK application.
    let app = gtk4::Application::new(Some("{{app-id}}"), Default::default());

    app.connect_activate(|app| {
        // Initialise the theme system.
        // Must happen after the application is created so that GTK and the
        // display are available for the CSS provider.
        theme::init();

        // ---- Build the application shell ----

        let shell = AppShellBuilder::default()
            .title("{{project-name}}")
            .size(1200, 800)
            .sidebar(vec![
                NavItem::new("Dashboard", "dashboard-symbolic", "dashboard"),
                NavItem::new("Settings", "settings-symbolic", "settings"),
            ])
            .build();

        // ---- Register the shell's window with the application ----

        let window = shell.widget();
        app.add_window(window.upcast_ref::<gtk4::Window>());
        window.present();

        // ---- Build page widgets and add them to the content stack ----

        let model = shell.model();

        let dashboard = pages::dashboard::page();
        model
            .content_stack
            .add_titled(&dashboard, Some("dashboard"), "Dashboard");

        let settings = pages::settings::page();
        model
            .content_stack
            .add_titled(&settings, Some("settings"), "Settings");

        // Start on the dashboard page.
        if let Some(first_page) = model.content_stack.child_by_name("dashboard") {
            model.content_stack.set_visible_child(&first_page);
        }
    });

    app.run()
}
