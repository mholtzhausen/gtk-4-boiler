use relm4::RelmApp;
use relm4_kit::prelude::*;

fn main() -> glib::ExitCode {
    // Initialize the relm4-kit theme (loads CSS, sets up dark mode)
    theme::init();

    // Build the application shell
    let shell = AppShell::builder()
        .title("{{project-name}}")
        .size(1200, 800)
        .sidebar(vec![
            NavItem::new("Dashboard", Some("dashboard-symbolic"), "dashboard"),
            NavItem::new("Settings", Some("settings-symbolic"), "settings"),
        ])
        .build();

    // Run the application
    let app = RelmApp::new("{{app-id}}");
    app.run(shell)
}
