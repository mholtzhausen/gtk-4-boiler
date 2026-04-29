use gtk4::prelude::*;

fn main() -> glib::ExitCode {
    // Initialize theme before creating any widgets
    relm4_kit::theme::init();

    let app = gtk4::Application::new(Some("com.relm4-kit.demo"), Default::default());

    app.connect_activate(|app| {
        // Create a simple window
        let window = gtk4::ApplicationWindow::new(app);
        window.set_title(Some("relm4-kit Demo"));
        window.set_default_size(1024, 768);

        let label = gtk4::Label::new(Some("Welcome to relm4-kit"));
        label.add_css_class("title-1");
        label.set_margin_top(24);
        label.set_margin_bottom(24);
        label.set_halign(gtk4::Align::Center);

        window.set_child(Some(&label));
        window.present();
    });

    app.run()
}
