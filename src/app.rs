use eframe::egui;
use crate::node::Node;

/// The main application state for the mind map app.
/// Holds all nodes and manages their creation and interaction.
pub struct MindMapApp {
    /// All nodes currently in the mind map.
    pub nodes: Vec<Node>,
    /// The next unique id to assign to a new node.
    pub next_id: usize,
    /// The index of the currently selected node, if any.
    pub selected_node: Option<usize>,
    /// The current canvas offset for panning.
    pub canvas_offset: egui::Vec2,
    /// Is the user currently panning the canvas?
    pub is_panning: bool,
    /// Where did the pan drag start (screen coordinates)?
    pub pan_start: egui::Pos2,
    /// What was the offset at the start of the pan?
    pub pan_offset_start: egui::Vec2,
}

impl Default for MindMapApp {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            next_id: 0,
            selected_node: None,
            canvas_offset: egui::Vec2::ZERO,
            is_panning: false,
            pan_start: egui::Pos2::ZERO,
            pan_offset_start: egui::Vec2::ZERO,
        }
    }
}

impl eframe::App for MindMapApp {
    /// Called each frame to update the UI and handle user interaction.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with the "Add Node" button
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Add Node").clicked() {
                // Add a new node at a default position with a unique label, default color and size
                // Place it at the center of the current view (adjusted for canvas offset)
                let center = ui.ctx().screen_rect().center();
                let pos = center - self.canvas_offset;
                self.nodes.push(Node {
                    id: self.next_id,
                    pos,
                    label: format!("Node {}", self.next_id + 1),
                    color: egui::Color32::from_rgb(100, 150, 250),
                    size: egui::vec2(60.0, 40.0),
                });
                self.next_id += 1;
            }

            // Node editing panel (if a node is selected)
            if let Some(idx) = self.selected_node {
                if let Some(node) = self.nodes.get_mut(idx) {
                    ui.separator();
                    ui.heading("Edit Node");
                    ui.label("Label:");
                    ui.text_edit_singleline(&mut node.label);
                    ui.label("Color:");
                    egui::color_picker::color_edit_button_srgba(ui, &mut node.color, egui::color_picker::Alpha::Opaque);
                    ui.label("Size:");
                    ui.add(egui::Slider::new(&mut node.size.x, 30.0..=200.0).text("Width"));
                    ui.add(egui::Slider::new(&mut node.size.y, 20.0..=150.0).text("Height"));
                }
            }
        });

        // Central panel for drawing and interacting with nodes and canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            // Only allow panning with the right mouse button
            let bg_response = ui.allocate_response(available_size, egui::Sense::drag());
            let ctx = ui.ctx();
            let right_button_down = ctx.input(|i| i.pointer.button_down(egui::PointerButton::Secondary));

            if right_button_down && bg_response.drag_started() {
                self.is_panning = true;
                self.pan_start = bg_response.interact_pointer_pos().unwrap_or(egui::Pos2::ZERO);
                self.pan_offset_start = self.canvas_offset;
            }
            if self.is_panning && right_button_down && bg_response.dragged() {
                if let Some(current) = bg_response.interact_pointer_pos() {
                    let delta = current - self.pan_start;
                    self.canvas_offset = self.pan_offset_start + delta;
                }
            }
            if self.is_panning && (!right_button_down || bg_response.drag_stopped()) {
                self.is_panning = false;
            }
            if bg_response.clicked() && !right_button_down {
                self.selected_node = None;
            }

            // Handle node dragging and selection
            let mut node_screen_rects = Vec::with_capacity(self.nodes.len());
            for (i, node) in self.nodes.iter_mut().enumerate() {
                let node_screen_pos = node.pos + self.canvas_offset;
                let node_rect = egui::Rect::from_center_size(node_screen_pos, node.size);
                node_screen_rects.push((i, node_screen_pos, node_rect));
                let response = ui.allocate_rect(node_rect, egui::Sense::click_and_drag());
                if response.dragged() {
                    node.pos += response.drag_delta();
                }
                if response.clicked() {
                    self.selected_node = Some(i);
                }
            }

            // Now, draw all nodes
            let painter = ui.painter();
            for (i, node_screen_pos, node_rect) in node_screen_rects.into_iter() {
                let node = &self.nodes[i];
                painter.rect_filled(node_rect, 10.0, node.color);
                painter.text(node_screen_pos, egui::Align2::CENTER_CENTER, &node.label, egui::FontId::proportional(18.0), egui::Color32::WHITE);
                if Some(i) == self.selected_node {
                    painter.rect_stroke(
                        node_rect,
                        10.0,
                        egui::Stroke::new(2.0, egui::Color32::YELLOW),
                        egui::epaint::StrokeKind::Inside,
                    );
                }
            }
        });
    }
} 