use eframe::egui;

/// Represents a single node in the mind map.
/// Each node has a unique id, a position on the canvas, a label, a color, and a size.
#[derive(Debug)]
pub struct Node {
    /// Unique identifier for the node (not currently used, but useful for future features like edges).
    pub id: usize,
    /// The position of the node's center in the mind map canvas (in egui coordinates).
    pub pos: egui::Pos2,
    /// The text label displayed inside the node.
    pub label: String,
    /// The color of the node.
    pub color: egui::Color32,
    /// The size of the node (width, height).
    pub size: egui::Vec2,
} 