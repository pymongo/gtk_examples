use gtk::prelude::*;
fn main() {
    let application = gtk::Application::new(None, Default::default());
    application.connect_activate(|application| {
        let gtk_builder = gtk::Builder::new();
        gtk_builder
            .add_from_string(include_str!("_01_button_toggle_text_view_visible.glade"))
            .unwrap();
        // like js's document.querySelector or android's findViewById
        let window = gtk_builder.object::<gtk::Window>("window").unwrap();
        let button = gtk_builder.object::<gtk::Button>("button").unwrap();
        let label = gtk_builder.object::<gtk::Label>("label").unwrap();
        // move label to button_on_click_listener or use glib::clone! to share label's reference
        button.connect_clicked(move |_| {
            if label.get_visible() {
                label.set_visible(false);
            } else {
                label.set_visible(true);
            }
        });
        window.set_application(Some(application));
        window.show_all();
    });
    application.run();
}
