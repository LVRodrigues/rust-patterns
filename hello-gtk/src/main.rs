use gtk::{Application, glib, prelude::{ApplicationExtManual, ApplicationExt}, ApplicationWindow, traits::{GtkWindowExt, ButtonExt}, Button};

const APP_ID: &str = "io.github.lvrodrigues.hello-gtk";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    println!("{app}");
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Clique me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    button.connect_clicked(|b| {
        b.set_label("Hello, Luciano!");
    });
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello RUST with GTK 4!")
        .child(&button)
        .width_request(300)
        .build();
    window.present();
}