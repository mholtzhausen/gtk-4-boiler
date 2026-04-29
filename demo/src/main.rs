use gtk4::prelude::*;

fn main() -> glib::ExitCode {
    let app = gtk4::Application::new(Some("com.relm4-kit.demo"), Default::default());
    app.connect_activate(|_| {});
    app.run()
}
