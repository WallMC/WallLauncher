use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::builder()
        .application_id("org.karimrir1.WallMC")
        .build();
    /* const ver: &str = env!("CARGO_PKG_VERSION"); */

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(1284)
            .default_height(695)
            .title("WallMC v0.1.0")
            .build();
        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        win.add(&button);
        

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}
