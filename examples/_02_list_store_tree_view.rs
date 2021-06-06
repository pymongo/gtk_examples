use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(None, Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_position(gtk::WindowPosition::Center);

    // init_layout
    let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vertical_layout);
    let tree_view = gtk::TreeView::new();
    let bottom_input_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    vertical_layout.add(&tree_view);
    vertical_layout.add(&bottom_input_container);
    let text_input = gtk::Entry::new();
    let send_button = gtk::Button::with_label("send");
    bottom_input_container.add(&text_input);
    bottom_input_container.add(&send_button);

    // init tree_view
    tree_view.set_headers_visible(false);
    let column = gtk::TreeViewColumn::new();
    let column_index = 0;
    let cell = gtk::CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", column_index);
    tree_view.append_column(&column);
    let list_store = gtk::ListStore::new(&[String::static_type()]);
    list_store.insert_with_values(None, &[(0, &"First chat messages")]);
    tree_view.set_model(Some(&list_store));
    
    // init button
    let send_button_list_store = list_store.clone();
    let send_button_text_input = text_input.clone();
    send_button.connect_clicked(move |_| {
        let user_input = send_button_text_input.text().to_string();
        send_button_text_input.set_text("");
        send_button_list_store.insert_with_values(None, &[(0, &user_input)]);
    });

    window.show_all();
}
