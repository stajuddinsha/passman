use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Box as GtkBox, Orientation, Label};

const APP_ID: &str = "com.keytui.test";

pub fn run_minimal_test() -> Result<()> {
    let application = Application::builder()
        .application_id(APP_ID)
        .build();

    application.connect_activate(|app| {
        // Create a simple window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Minimal Test")
            .default_width(400)
            .default_height(300)
            .decorated(true)
            .build();

        // Create a simple button
        let button = Button::builder()
            .label("Test Button")
            .build();

        // Create a status label
        let status_label = Label::new(Some("Ready"));

        // Create a container
        let vbox = GtkBox::new(Orientation::Vertical, 12);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);

        // Add widgets
        vbox.append(&button);
        vbox.append(&status_label);

        // Connect button click
        button.connect_clicked(move |_| {
            status_label.set_text("Button clicked! It works!");
        });

        // Set window content
        window.set_child(Some(&vbox));
        window.present();
    });

    application.run();
    Ok(())
}
