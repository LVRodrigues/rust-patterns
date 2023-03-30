mod window;

use adw::prelude::*;
use adw::Application;
use gtk::{glib, gio};
use window::Window;

const APP_ID: &str = "io.github.lvrodrigues.Observer";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("observer.gresource")
        .expect("Falha ao registrar os recursos.");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}

