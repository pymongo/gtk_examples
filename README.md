# gtk notes

## widgets

### label(text)

#### align center

packing->expand(true), `set_vexpan(true); set_hexpand(true)`

#### font size

general->edit_attributes

#### tooltip

mouse hover text, like title for html a tag

common->tooltip

#### preview-only text like tools:text?

No

### window

#### popup on screen center

general->position->center, or code: `set_position(gtk::WindowPosition::Center);`
