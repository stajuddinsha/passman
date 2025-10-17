#[cfg(feature = "gui")]
use anyhow::Result;
#[cfg(feature = "gui")]
use gtk4::prelude::*;
#[cfg(feature = "gui")]
use gtk4::{Application, ApplicationWindow};
use std::env;

#[cfg(feature = "gui")]
mod app;
mod config;
mod vault;
mod clipboard;
mod search;
#[cfg(feature = "gui")]
mod ui;
#[cfg(feature = "gui")]
mod minimal_test;

#[cfg(feature = "gui")]
use app::KeytuiApp;
#[cfg(feature = "gui")]
use minimal_test::run_minimal_test;

#[cfg(feature = "gui")]
fn main() -> Result<()> {
    println!("[MAIN] Starting main function...");

    // Check for minimal test mode
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--minimal" {
        println!("[MAIN] Running minimal test...");
        return run_minimal_test();
    }

    // Initialize logging
    println!("[MAIN] Starting Keytui GUI...");

    // Create GTK application
    println!("[MAIN] Creating GTK application...");
    let app = Application::builder()
        .application_id("com.keytui.gui")
        .build();
    println!("[MAIN] GTK application created successfully");

    // Handle application startup
    println!("[MAIN] Setting up application activate handler...");
    app.connect_activate(|app| {
        println!("[ACTIVATE] Application activate callback triggered");

        println!("[ACTIVATE] Creating KeytuiApp...");
        let keytui_app = match KeytuiApp::new() {
            Ok(app) => {
                println!("[ACTIVATE] KeytuiApp created successfully");
                app
            },
            Err(e) => {
                eprintln!("[ACTIVATE] Failed to initialize Keytui: {}", e);
                std::process::exit(1);
            }
        };

        // Create main window (overlay)
        println!("[ACTIVATE] Creating main window...");
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Keytui")
            .default_width(640)
            .default_height(420)
            .decorated(true)
            .resizable(false)
            .build();
        println!("[ACTIVATE] Main window created successfully");

        println!("[ACTIVATE] Setting up UI...");
        keytui_app.setup_ui(&window);
        println!("[ACTIVATE] UI setup completed");

        println!("[ACTIVATE] Presenting window...");
        window.present();
        println!("[ACTIVATE] Window presented successfully");
    });

    // Handle command line arguments
    if args.len() > 1 && args[1] == "--daemon" {
        // Run as background daemon
        println!("[MAIN] Running as background daemon...");
        app.run();
    } else {
        // Run as overlay
        println!("[MAIN] Running as overlay...");
        app.run();
    }

    println!("[MAIN] Application run completed");
    Ok(())
}

#[cfg(not(feature = "gui"))]
fn main() {
    println!("GUI feature not enabled. Use 'cargo run --bin keytui-tui' for terminal version.");
}