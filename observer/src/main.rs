
use gtk::{glib, Application, gio, Builder, ApplicationWindow};
use gtk::prelude::*;

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
    let builder = Builder::from_resource("/io/github/lvrodrigues/window.ui");
    let window: ApplicationWindow = builder.object("window").unwrap();
    window.set_application(Some(app));
    window.set_visible(true);
}

