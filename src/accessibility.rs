use bevy::prelude::*;

/// Accessibility roles for UI elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Role {
    /// A clickable button element
    Button,
    /// A slider control for selecting a value
    Slider,
    /// A checkbox input for toggling a setting
    Checkbox,
    /// A radio button for selecting one option from many
    Radio,
    /// A tab in a tabbed interface
    Tab,
    /// A text element
    Text,
}

/// Accessibility node component
#[derive(Component, Debug, Reflect)]
/// Accessibility information for a UI node
pub struct AccessibilityNode {
    /// The semantic role of this node
    pub role: Role,
    /// Optional name for the node (used by screen readers)
    pub name: Option<String>,
    /// Optional extended description for the node
    pub description: Option<String>,
}

impl From<Role> for AccessibilityNode {
    fn from(role: Role) -> Self {
        Self {
            role,
            name: None,
            description: None,
        }
    }
}
