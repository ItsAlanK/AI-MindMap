use eframe::egui;
use crate::node::Node;

/// The main application state for the mind map app.
/// Holds all nodes and manages their creation and interaction.
pub struct MindMapApp {
    /// All nodes currently in the mind map.
    pub nodes: Vec<Node>,
    /// The next unique id to assign to a new node.
    pub next_id: usize,
}

impl Default for MindMapApp {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            next_id: 0,
        }
    }
}

impl eframe::App for MindMapApp {
    /// Called each frame to update the UI and handle user interaction.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with the "Add Node" button
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Add Node").clicked() {
                // Add a new node at a default position with a unique label
                self.nodes.push(Node {
                    id: self.next_id,
                    pos: egui::pos2(200.0 + 30.0 * self.next_id as f32, 200.0),
                    label: format!("Node {}", self.next_id + 1),
                });
                self.next_id += 1;
            }
        });

        // Central panel for drawing and interacting with nodes
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            // Capture clicks on the background (for future features)
            let _response = ui.allocate_response(available_size, egui::Sense::click());

            // First, handle dragging and update node positions
            for node in &mut self.nodes {
                // Each node is represented as a rectangle; make it draggable
                let node_rect = egui::Rect::from_center_size(node.pos, egui::vec2(60.0, 40.0));
                let response = ui.allocate_rect(node_rect, egui::Sense::drag());
                if response.dragged() {
                    node.pos += response.drag_delta();
                }
            }

            // Now, draw all nodes
            let painter = ui.painter();
            for node in &self.nodes {
                let node_rect = egui::Rect::from_center_size(node.pos, egui::vec2(60.0, 40.0));
                // Draw the node as a rounded rectangle
                painter.rect_filled(node_rect, 10.0, egui::Color32::from_rgb(100, 150, 250));
                // Draw the node's label centered in the rectangle
                painter.text(node.pos, egui::Align2::CENTER_CENTER, &node.label, egui::FontId::proportional(18.0), egui::Color32::WHITE);
            }
        });
    }
} 