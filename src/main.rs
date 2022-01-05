use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Scale, Orientation};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let scale = Scale::with_range(Orientation::Vertical, 0.0, 10.0, 2.0);
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Equalizer")
            .child(&scale)
            .build();

        window.show();
    });

    app.run();
}
