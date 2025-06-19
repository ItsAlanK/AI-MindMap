// Entry point for the AI-Assisted Mind Map app.
// This file sets up the app and launches the egui window.
// Main logic is in app.rs, and node data structures are in node.rs.

mod node;
mod app;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.viewport = egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]);
    eframe::run_native(
        "AI Mind Map",
        options,
        Box::new(|_cc| Ok(Box::new(app::MindMapApp::default()))),
    )
}
