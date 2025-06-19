// Entry point for the AI-Assisted Mind Map app.
// This file sets up the app and launches the egui window.
// Main logic is in app.rs, and node data structures are in node.rs.

mod node;
mod app;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "AI Mind Map",
        options,
        Box::new(|_cc| Ok(Box::new(app::MindMapApp::default()))),
    )
}
