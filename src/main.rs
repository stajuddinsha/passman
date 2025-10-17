use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use std::env;

mod app;
mod config;
mod vault;
mod clipboard;
mod ui;

use app::KeytuiApp;

fn main() -> Result<()> {
    // Initialize logging
    println!("Starting Keytui GUI...");

    // Create GTK application
    let app = Application::builder()
        .application_id("com.keytui.gui")
        .build();

    // Handle application startup
    app.connect_activate(|app| {
        let keytui_app = match KeytuiApp::new() {
            Ok(app) => app,
            Err(e) => {
                eprintln!("Failed to initialize Keytui: {}", e);
                std::process::exit(1);
            }
        };

        // Create main window (overlay)
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Keytui")
            .default_width(640)
            .default_height(420)
            .decorated(false)
            .resizable(false)
            .build();

        keytui_app.setup_ui(&window);
        window.present();
    });

    // Handle command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--daemon" {
        // Run as background daemon
        app.run();
    } else {
        // Run as overlay
        app.run();
    }

    Ok(())
}