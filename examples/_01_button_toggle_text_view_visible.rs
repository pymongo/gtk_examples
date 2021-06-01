use gtk::prelude::*;
fn main() {
    let application = gtk::Application::new(None, Default::default());
    application.connect_activate(|application| {
        let builder = gtk::Builder::new();
        builder
            .add_from_string(include_str!("_01_button_toggle_text_view_visible.glade"))
            .unwrap();
        let window = builder.object::<gtk::Window>("window").unwrap(); // like js's document.querySelector or android's findViewById
        let button = builder.object::<gtk::Button>("button").unwrap();
        let label = builder.object::<gtk::Label>("label").unwrap();
        // move label to button_on_click_listener or use glib::clone! to share label's reference
        button.connect_clicked(move |_| {
            label.set_visible(!label.get_visible());
        });
        window.set_application(Some(application));
        window.show_all();
    });
    application.run();
}
