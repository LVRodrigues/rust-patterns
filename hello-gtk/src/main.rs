use gtk::{Application, glib, prelude::{ApplicationExtManual, ApplicationExt}, ApplicationWindow, traits::GtkWindowExt};

const APP_ID: &str = "io.github.lvrodrigues.hello-gtk";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    println!("{app}");
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello RUST with GTK 4")
        .build();
    window.present();
}