use glib::subclass::InitializingObject;
use adw::subclass::prelude::*;
use gtk::{glib, Label, CompositeTemplate};

use observer::Observer;

use crate::fibonacci::Fibonacci;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/io/github/lvrodrigues/window.ui")]
pub struct Window {
    #[template_child]
    pub previous: TemplateChild<Label>,
    #[template_child]
    pub current: TemplateChild<Label>,
    #[template_child]
    pub steps: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_actions();
        // self.button.connect_clicked(move |button| {
        //     button.set_label("Hello World!");
        // });
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}

impl Observer<Fibonacci> for Window {
    fn receive(&self, value: &Fibonacci) {
        self.previous.set_text(value.previous.to_string().as_str());
        self.current.set_text(value.current.to_string().as_str());
        self.steps.set_text(value.steps.to_string().as_str());
    }
}