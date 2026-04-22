use taffy::{Layout, NodeId, Style, TaffyTree};
use zenthra_core::Rect;

/// Wraps a taffy tree — owns the layout computation for the whole widget tree.
pub struct LayoutEngine {
    tree: TaffyTree,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            tree: TaffyTree::new(),
        }
    }

    /// Add a leaf node (no children).
    pub fn new_leaf(&mut self, style: Style) -> LayoutNode {
        let id = self.tree.new_leaf(style).unwrap();
        LayoutNode(id)
    }

    /// Add a node with children.
    pub fn new_with_children(&mut self, style: Style, children: &[LayoutNode]) -> LayoutNode {
        let ids: Vec<NodeId> = children.iter().map(|n| n.0).collect();
        let id = self.tree.new_with_children(style, &ids).unwrap();
        LayoutNode(id)
    }

    /// Append a child to an existing node.
    pub fn add_child(&mut self, parent: LayoutNode, child: LayoutNode) {
        self.tree.add_child(parent.0, child.0).unwrap();
    }

    /// Compute layout for the whole tree rooted at `root`.
    pub fn compute(&mut self, root: LayoutNode, available_width: f32, available_height: f32) {
        let space = taffy::Size {
            width: taffy::AvailableSpace::Definite(available_width),
            height: taffy::AvailableSpace::Definite(available_height),
        };
        self.tree.compute_layout(root.0, space).unwrap();
    }

    /// Read back the computed rect for a node (in local space).
    pub fn layout(&self, node: LayoutNode) -> Rect {
        let l: &Layout = self.tree.layout(node.0).unwrap();
        Rect::new(l.location.x, l.location.y, l.size.width, l.size.height)
    }

    /// Remove all nodes — call before rebuilding the tree each frame.
    pub fn clear(&mut self) {
        self.tree = TaffyTree::new();
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// A handle to a node inside LayoutEngine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayoutNode(pub NodeId);
