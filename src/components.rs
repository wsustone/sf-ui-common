//! UI components for StrategyForge

use bevy::prelude::*;

/// A UI button component with visual states
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct UiButton {
    /// Whether the button is currently pressed
    pub pressed: bool,
    /// Whether the button is currently hovered
    pub hovered: bool,
    /// Whether the button is disabled
    pub disabled: bool,
    /// Optional tooltip text
    pub tooltip: Option<String>,
}

/// A checkbox component with toggle state
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct UiCheckbox {
    /// Whether the checkbox is checked
    pub checked: bool,
    /// Whether the checkbox is disabled
    pub disabled: bool,
    /// Optional tooltip text
    pub tooltip: Option<String>,
}

/// A slider component for numeric input
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct UiSlider {
    /// Current value of the slider
    pub value: f32,
    /// Minimum value
    pub min: f32,
    /// Maximum value
    pub max: f32,
    /// Optional step size
    pub step: Option<f32>,
    /// Whether the slider is disabled
    pub disabled: bool,
    /// Optional tooltip text
    pub tooltip: Option<String>,
}

/// A dropdown menu component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct UiDropdown {
    /// Whether the dropdown is currently open
    pub open: bool,
    /// The currently selected option index
    pub selected: Option<usize>,
    /// The available options
    pub options: Vec<String>,
    /// Whether the dropdown is disabled
    pub disabled: bool,
}

/// A tooltip component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct UiTooltip {
    /// The tooltip text
    pub text: String,
    /// The position offset from the parent
    pub offset: Vec2,
}

/// A scrollable container component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Scrollable {
    /// The scroll position
    pub scroll: f32,
    /// Whether the content is scrollable vertically
    pub vertical: bool,
    /// Whether the content is scrollable horizontally
    pub horizontal: bool,
}

/// A tab container component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TabContainer {
    /// The currently active tab index
    pub active_tab: usize,
}

/// A tab component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tab {
    /// The index of this tab
    pub index: usize,
    /// Whether this tab is the active tab
    pub active: bool,
}

/// A panel component for grouping UI elements
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Panel {
    /// The panel title
    pub title: Option<String>,
    /// Whether the panel is collapsible
    pub collapsible: bool,
    /// Whether the panel is currently collapsed
    pub collapsed: bool,
}

/// A progress bar component
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct ProgressBar {
    /// Current value (0.0 to 1.0)
    pub value: f32,
    /// Background color
    pub background_color: Color,
    /// Fill color
    pub fill_color: Color,
    /// Whether to show text percentage
    pub show_text: bool,
}
