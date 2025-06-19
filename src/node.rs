use eframe::egui;

/// Represents a single node in the mind map.
/// Each node has a unique id, a position on the canvas, and a label.
#[derive(Debug)]
pub struct Node {
    /// Unique identifier for the node (not currently used, but useful for future features like edges).
    pub id: usize,
    /// The position of the node's center in the mind map canvas (in egui coordinates).
    pub pos: egui::Pos2,
    /// The text label displayed inside the node.
    pub label: String,
} 