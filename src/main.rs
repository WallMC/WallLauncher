use gtk::prelude::*;
use std::process::exit;
use gtk::{Application, ApplicationWindow, Button, ButtonBox, Align};

fn main() {
    pub const APP_ID: &str = "org.WallMC.WallLauncher";
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    /* const ver: &str = env!("CARGO_PKG_VERSION"); */

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(1284)
            .default_height(695)
            .title("WallMC v1.0.0")
            .build();
        win.connect_destroy(|_| {
            println!("Exiting with exit code 0");
            exit(0);
        });
        let button = Button::with_label("Click me!");
        let buttonbox = ButtonBox::builder()
            .opacity(0.6)
            .valign(Align::End)
            .build();
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
       /* let _button2 = Button::clone(&button);
        let container = ButtonBoxBuilder::new();
        container.build();
        container.add(&_button2);
        */
       buttonbox.add(&button);
       win.add(&buttonbox);
        

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}
