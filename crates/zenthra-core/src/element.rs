// crates/zenthra-core/src/element.rs

use crate::Id;
use crate::rect::Rect;

/// Represents the semantic role of a widget for accessibility and automation.
/// This matches standard Web/ARIA roles but is tailored for Zenthra's native widgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    /// A top-level window.
    Window,
    /// A clickable button.
    Button,
    /// A navigational link.
    Link,
    /// A toggleable check box.
    CheckBox,
    /// A set of mutual exclusive options.
    RadioButton,
    /// A single-line text input.
    TextInput,
    /// A multi-line text editor.
    TextArea,
    /// A numeric range selector.
    Slider,
    /// Static text content.
    Label,
    /// A content heading.
    Heading,
    /// A collection of items.
    List,
    /// An item within a list.
    ListItem,
    /// A graphical asset.
    Image,
    /// A generic layout container (non-semantic by default).
    Container,
    /// A region that can be scrolled.
    ScrollRegion,
    /// A popup or contextual menu.
    Menu,
}

/// A node in the semantic/accessibility tree.
/// This exists independently of the draw order and describes the "meaning" of the UI.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticNode {
    /// The unique, persistent ID of the widget.
    pub id: Id,
    /// The semantic role of the widget.
    pub role: Role,
    /// The human-readable label or description (e.g. Button text).
    pub label: Option<String>,
    /// The screen-space bounding box.
    pub rect: Rect,
    /// The IDs of children nodes in the semantic hierarchy.
    pub children: Vec<Id>,
    /// Whether this node is currently focused by the keyboard.
    pub focused: bool,
    /// Whether this node is currently disabled (non-interactive).
    pub disabled: bool,
}

impl SemanticNode {
    pub fn new(id: Id, role: Role, rect: Rect) -> Self {
        Self {
            id,
            role,
            label: None,
            rect,
            children: Vec::new(),
            focused: false,
            disabled: false,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_focus(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}
