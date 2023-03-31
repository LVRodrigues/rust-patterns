mod imp;

use glib::Object;
use gtk::{gio::{self, SimpleAction}, glib, prelude::ActionMapExt};
use adw::Application;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_actions(&self) {
        let action_start = SimpleAction::new("start", None);
        action_start.connect_activate(|action, _| {
            println!("{} - Start", action.to_string());
        });
        self.add_action(&action_start);

        let action_stop = SimpleAction::new("stop", None);
        action_stop.connect_activate(|action, _| {
            println!("{} - Stop", action.to_string());
        });
        self.add_action(&action_stop);

        let action_restore = SimpleAction::new("restore", None);
        action_restore.connect_activate(|action, _| {
            println!("{} - Restore", action.to_string());
        });
        self.add_action(&action_restore);
    }
}